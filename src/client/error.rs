use std::error::Error;

use thiserror::Error;

use crate::DecodeError;

/// This error is returned if a communication fails because of a timeout
/// or underlying modbus error.
#[derive(Debug, Error)]
pub enum ModbusError {
    /// A timeout occured
    #[error("Timeout")]
    Timeout,
    /// The requested device does not implement modbus correctly and the
    /// underlying modbus implementation reported an error.
    #[error(transparent)]
    Protocol(Box<dyn Error + Send + Sync>),
    /// I/O error
    #[error("I/O error")]
    IO(#[from] std::io::Error),
    /// The function code received in the query is not an allowable action for
    /// the server (or slave). This may be because the function code is only
    /// applicable to newer devices, and was not implemented in the unit
    /// selected. It could also indicate that the server (or slave) is in the
    /// wrong state to process a request of this type, for example because it is
    /// unconfigured and is being asked to return register values.
    #[error("Illegal function")]
    IllegalFunction,
    /// The data address received in the query is not an allowable address for
    /// the server (or slave). More specifically, the combination of reference
    /// number and transfer length is invalid. For a controller with 100
    /// registers, the PDU addresses the first register as 0, and the last one
    /// as 99. If a request is submitted with a starting register address of 96
    /// and a quantity of registers of 4, then this request will successfully
    /// operate (address-wise at least) on registers 96, 97, 98, 99. If a
    /// request is submitted with a starting register address of 96 and a
    /// quantity of registers of 5, then this request will fail with Exception
    /// Code 0x02 “Illegal Data Address” since it attempts to operate on
    /// registers 96, 97, 98, 99 and 100, and there is no register with address
    /// 100.
    #[error("Illegal data address")]
    IllegalDataAddress,
    /// A value contained in the query data field is not an allowable value for
    /// server (or slave). This indicates a fault in the structure of the
    /// remainder of a complex request, such as that the implied length is
    /// incorrect. It specifically does NOT mean that a data item submitted for
    /// storage in a register has a value outside the expectation of the
    /// application program, since the MODBUS protocol is unaware of the
    /// significance of any particular value of any particular register.
    #[error("Illegal data value")]
    IllegalDataValue,
    /// An unrecoverable error occurred while the server (or slave) was
    /// attempting to perform the requested action.
    #[error("Server device failure")]
    ServerDeviceFailure,
    /// Specialized use in conjunction with programming
    /// commands. The server (or slave) has accepted the request and is
    /// processing it, but a long duration of time will be required to do so.
    /// This response is returned to prevent a timeout error from occurring in
    /// the client (or master). The client (or master) can next issue a Poll
    /// Program Complete message to determine if processing is completed.
    #[error("Acknowledge")]
    Acknowledge,
    /// Specialized use in conjunction with programming commands. The server (or
    /// slave) is engaged in processing a long–duration program command. The
    /// client (or master) should retransmit the message later when the server
    /// (or slave) is free.
    #[error("Server device busy")]
    ServerDeviceBusy,
    /// Specialized use in conjunction with function codes 20 and 21 and
    /// reference type 6, to indicate that the extended file area failed to pass
    /// a consistency check. The server (or slave) attempted to read record
    /// file, but detected a parity error in the memory. The client (or master)
    /// can retry the request, but service may be required on the server (or
    /// slave) device.
    #[error("Memory parity error")]
    MemoryParityError,
    /// Specialized use in conjunction with gateways, indicates that the gateway
    /// was unable to allocate an internal communication path from the input
    /// port to the output port for processing the request. Usually means that
    /// the gateway is misconfigured or overloaded.
    #[error("Gateway path unavailable")]
    GatewayPathUnavailable,
    /// Specialized use in conjunction with gateways, indicates that no response
    /// was obtained from the target device. Usually means that the device is
    /// not present on the network.
    #[error("Gateway target device")]
    GatewayTargetDevice,
    /// Custom modbus error
    #[error("Custom: {0}")]
    Custom(u8),
}

/// This error is returned if there was an error loading the
/// requested model.
#[derive(Debug, Error)]
pub enum ReadModelError {
    /// Some error occured while communicating via the modbus. This
    /// error is implementation specific.
    #[error("Modbus error: {0}")]
    Modbus(#[from] ModbusError),
    /// The decoding of the point data failed.
    #[error("Decode error: {0}")]
    DecodeError(#[from] DecodeError),
    /// A point in the model is mandatory but the value is missing.
    #[error("Missing mandatory value")]
    MissingMandatoryValue,
}

/// This error is returned if there was an error while
/// reading data from a point.
#[derive(Debug, Error)]
pub enum ReadPointError {
    /// Communication error.
    #[error("Modbus error: {0}")]
    Modbus(#[from] ModbusError),
    /// The decoding of the point data failed.
    #[error("Decode error: {0}")]
    DecodeError(#[from] DecodeError),
    /// The point is mandatory but value is missing.
    #[error("Missing mandatory value")]
    MissingMandatoryValue,
}

/// This error is returned if there was an error while
/// writing data to a point.
#[derive(Debug, Error)]
pub enum WritePointError {
    /// Communication error.
    #[error("Modbus error: {0}")]
    Modbus(#[from] ModbusError),
    /// The encoded value was too large for the point.
    #[error("Encoded value too large for point")]
    ValueTooLarge,
}
