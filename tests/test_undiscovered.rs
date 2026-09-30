#![cfg(all(feature = "model1", feature = "model103"))]

use std::{
    collections::HashMap,
    future::Future,
    pin::pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Wake, Waker},
};

use sunspec::{
    client::{
        AsyncClient, AsyncDevice, AsyncModbusClient, Config, ModbusError, ReadModelError,
        ReadPointError, WritePointError,
    },
    models::{model1::Model1, model103::Model103},
    Model,
};

/// Modbus request issued by the client under test.
#[derive(Clone, Debug, PartialEq)]
enum Request {
    Read(u16, u16),
    Write(u16, Vec<u16>),
}

/// In-memory Modbus client serving a fixed register map and recording
/// all requests.
#[derive(Clone, Debug)]
struct RecordingClient {
    registers: Arc<HashMap<u16, u16>>,
    requests: Arc<Mutex<Vec<Request>>>,
}

impl RecordingClient {
    fn new(base: u16, data: &[u16]) -> Self {
        let registers = (base..).zip(data.iter().copied()).collect();
        Self {
            registers: Arc::new(registers),
            requests: Default::default(),
        }
    }
    fn take_requests(&self) -> Vec<Request> {
        std::mem::take(&mut self.requests.lock().unwrap())
    }
}

impl AsyncModbusClient for RecordingClient {
    fn read_registers(
        &self,
        _slave_id: u8,
        addr: u16,
        len: u16,
    ) -> impl Future<Output = Result<Vec<u16>, ModbusError>> + Send {
        self.requests.lock().unwrap().push(Request::Read(addr, len));
        let result = (addr..addr + len)
            .map(|addr| {
                self.registers
                    .get(&addr)
                    .copied()
                    .ok_or(ModbusError::IllegalDataAddress)
            })
            .collect();
        async move { result }
    }
    async fn write_registers(
        &self,
        _slave_id: u8,
        addr: u16,
        data: &[u16],
    ) -> Result<(), ModbusError> {
        self.requests
            .lock()
            .unwrap()
            .push(Request::Write(addr, data.to_vec()));
        Ok(())
    }
}

struct NoopWaker;

impl Wake for NoopWaker {
    fn wake(self: Arc<Self>) {}
}

/// Minimal executor for futures which never actually wait.
fn block_on<F: Future>(fut: F) -> F::Output {
    let waker = Waker::from(Arc::new(NoopWaker));
    let mut cx = Context::from_waker(&waker);
    let mut fut = pin!(fut);
    loop {
        if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
            return output;
        }
    }
}

/// Discover a device which only provides model 1 with the given length.
fn device_with_model1(len: u16) -> (AsyncDevice<RecordingClient>, RecordingClient) {
    let mut data = vec![0x5375, 0x6e53, 1, len];
    data.extend(std::iter::repeat(0).take(len.into()));
    data.extend([0xFFFF, 0]);
    let client = RecordingClient::new(40000, &data);
    let config = Config {
        discovery_addresses: vec![40000],
        read_timeout: None,
        write_timeout: None,
        ..Config::default()
    };
    let device = block_on(AsyncClient::new(client.clone(), config).device(1)).unwrap();
    let _ = client.take_requests();
    (device, client)
}

#[test]
fn test_undiscovered_model() {
    let (device, client) = device_with_model1(66);
    assert_eq!(Model103::addr(&device.models), None);

    assert!(matches!(
        block_on(device.read_model::<Model103>()),
        Err(ReadModelError::ModelNotDiscovered { model_id: 103 })
    ));
    assert!(matches!(
        block_on(device.read_any_model(&Model103::INFO)),
        Err(ReadModelError::ModelNotDiscovered { model_id: 103 })
    ));
    assert!(matches!(
        block_on(device.read_point(Model103::W)),
        Err(ReadPointError::ModelNotDiscovered { model_id: 103 })
    ));
    assert!(matches!(
        block_on(device.write_point(Model103::W, 42)),
        Err(WritePointError::ModelNotDiscovered { model_id: 103 })
    ));
    assert_eq!(client.take_requests(), []);
}

#[test]
fn test_point_outside_of_model() {
    // Model 1 reported with a length of 64 does not contain the `DA`
    // point at offset 64.
    let (device, client) = device_with_model1(64);
    assert!(matches!(
        block_on(device.read_point(Model1::DA)),
        Err(ReadPointError::PointOutOfBounds)
    ));
    assert!(matches!(
        block_on(device.write_point(Model1::DA, Some(2))),
        Err(WritePointError::PointOutOfBounds)
    ));
    assert_eq!(client.take_requests(), []);
}

#[test]
fn test_model_too_short() {
    // Model 1 reported with a length of 64 is missing the `DA` point.
    let (device, client) = device_with_model1(64);
    assert!(matches!(
        block_on(device.read_model::<Model1>()),
        Err(ReadModelError::ModelTooShort {
            model_id: 1,
            len: 64
        })
    ));
    assert!(matches!(
        block_on(device.read_any_model(&Model1::INFO)),
        Err(ReadModelError::ModelTooShort {
            model_id: 1,
            len: 64
        })
    ));
    assert_eq!(
        client.take_requests(),
        [Request::Read(40004, 64), Request::Read(40004, 64)]
    );
}

#[test]
fn test_model_without_trailing_pad() {
    // Some devices omit the trailing pad register of model 1 and
    // report a length of 65 instead of 66.
    let (device, _) = device_with_model1(65);
    assert_eq!(block_on(device.read_model::<Model1>()).unwrap().da, Some(0));
    assert!(block_on(device.read_any_model(&Model1::INFO)).is_ok());
}

#[test]
fn test_point_inside_of_model() {
    let (device, client) = device_with_model1(66);
    assert_eq!(block_on(device.read_point(Model1::DA)).unwrap(), Some(0));
    block_on(device.write_point(Model1::DA, Some(2))).unwrap();
    assert_eq!(
        client.take_requests(),
        [Request::Read(40068, 1), Request::Write(40068, vec![2])]
    );
}

#[cfg(feature = "model704")]
#[test]
fn test_len_includes_nested_groups() {
    use sunspec::{models::model704::Model704, Group};
    // 57 registers of points plus four nested groups of 2 registers each
    assert_eq!(Model704::LEN, 65);
}
