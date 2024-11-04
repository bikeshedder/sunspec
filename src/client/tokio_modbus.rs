//! This module contains the actual communication methods via
//! `tokio-modbus`.

use std::sync::Arc;

use tokio::sync::Mutex;
use tokio_modbus::{
    client::{Context, Reader, Writer},
    slave::SlaveContext,
    Slave,
};

use super::{r#async::IntoAsyncModbusClient, AsyncModbusClient, ModbusError};

impl AsyncModbusClient for Arc<Mutex<Context>> {
    async fn read_registers(
        &self,
        slave_id: u8,
        addr: u16,
        len: u16,
    ) -> Result<Vec<u16>, ModbusError> {
        let mut ctx = self.lock().await;
        ctx.set_slave(Slave(slave_id));
        Ok(ctx.read_holding_registers(addr, len).await??)
    }
    async fn write_registers(
        &self,
        slave_id: u8,
        addr: u16,
        data: &[u16],
    ) -> Result<(), ModbusError> {
        let mut ctx = self.lock().await;
        ctx.set_slave(Slave(slave_id));
        Ok(ctx.write_multiple_registers(addr, data).await??)
    }
}

impl IntoAsyncModbusClient<Arc<Mutex<Context>>> for Context {
    fn into_async_modbus_client(self) -> Arc<Mutex<Context>> {
        Arc::new(Mutex::new(self))
    }
}

impl From<tokio_modbus::Error> for ModbusError {
    fn from(value: tokio_modbus::Error) -> Self {
        match value {
            tokio_modbus::Error::Protocol(e) => Self::Protocol(Box::new(e)),
            tokio_modbus::Error::Transport(e) => Self::IO(e),
        }
    }
}

impl From<tokio_modbus::ExceptionCode> for ModbusError {
    fn from(value: tokio_modbus::ExceptionCode) -> Self {
        match value {
            tokio_modbus::ExceptionCode::IllegalFunction => Self::IllegalFunction,
            tokio_modbus::ExceptionCode::IllegalDataAddress => Self::IllegalDataAddress,
            tokio_modbus::ExceptionCode::IllegalDataValue => Self::IllegalDataValue,
            tokio_modbus::ExceptionCode::ServerDeviceFailure => Self::ServerDeviceFailure,
            tokio_modbus::ExceptionCode::Acknowledge => Self::Acknowledge,
            tokio_modbus::ExceptionCode::ServerDeviceBusy => Self::ServerDeviceBusy,
            tokio_modbus::ExceptionCode::MemoryParityError => Self::MemoryParityError,
            tokio_modbus::ExceptionCode::GatewayPathUnavailable => Self::GatewayPathUnavailable,
            tokio_modbus::ExceptionCode::GatewayTargetDevice => Self::GatewayTargetDevice,
            tokio_modbus::ExceptionCode::Custom(x) => Self::Custom(x),
        }
    }
}
