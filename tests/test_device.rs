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
        AsyncClient, AsyncDevice, AsyncModbusClient, Config, DiscoveryError, LookupError,
        ModbusError, ReadModelError, ReadPointError, WritePointError,
    },
    models::{model1::Model1, model103::Model103},
    AnyModel,
};

/// Modbus request issued by the client under test.
#[derive(Clone, Debug, PartialEq)]
enum Request {
    Read(u16, u16),
    Write(u16, Vec<u16>),
}

/// Number of registers returned for a read request of the given length.
type ResponseLen = fn(u16) -> usize;

/// In-memory Modbus client serving a fixed register map and recording
/// all requests.
#[derive(Clone, Debug)]
struct RecordingClient {
    registers: Arc<HashMap<u16, u16>>,
    requests: Arc<Mutex<Vec<Request>>>,
    /// Number of registers returned for a read request of the given
    /// length. This is used to simulate misbehaving devices.
    response_len: Arc<Mutex<Option<ResponseLen>>>,
}

impl RecordingClient {
    fn new(base: u16, data: &[u16]) -> Self {
        let registers = (base..=u16::MAX).zip(data.iter().copied()).collect();
        Self {
            registers: Arc::new(registers),
            requests: Default::default(),
            response_len: Default::default(),
        }
    }
    fn set_response_len(&self, response_len: ResponseLen) {
        *self.response_len.lock().unwrap() = Some(response_len);
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
        let result = (u32::from(addr)..u32::from(addr) + u32::from(len))
            .map(|addr| {
                u16::try_from(addr)
                    .ok()
                    .and_then(|addr| self.registers.get(&addr).copied())
                    .ok_or(ModbusError::IllegalDataAddress)
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|mut data| {
                if let Some(response_len) = *self.response_len.lock().unwrap() {
                    data.resize(response_len(len), 0);
                }
                data
            });
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

fn config() -> Config {
    Config {
        discovery_addresses: vec![40000],
        read_timeout: None,
        write_timeout: None,
        ..Config::default()
    }
}

/// Discover a device which provides the given models. Each model is
/// given as model id and model data.
fn device_with_models(
    models: &[(u16, Vec<u16>)],
) -> (AsyncDevice<RecordingClient>, RecordingClient) {
    let mut data = vec![0x5375, 0x6e53];
    for (id, model) in models {
        data.extend([*id, model.len() as u16]);
        data.extend(model);
    }
    data.extend([0xFFFF, 0]);
    let client = RecordingClient::new(40000, &data);
    let device = block_on(AsyncClient::new(client.clone(), config()).device(1)).unwrap();
    let _ = client.take_requests();
    (device, client)
}

/// Model 1 data with the given length and `DA` point value.
fn model1(len: u16, da: u16) -> Vec<u16> {
    let mut data = vec![0; len.into()];
    if let Some(value) = data.get_mut(64) {
        *value = da;
    }
    data
}

/// Discover a device which only provides model 1 with the given length.
fn device_with_model1(len: u16) -> (AsyncDevice<RecordingClient>, RecordingClient) {
    device_with_models(&[(1, model1(len, 0))])
}

#[test]
fn test_undiscovered_model() {
    let (device, client) = device_with_model1(66);
    assert_eq!(
        device.model::<Model103>().unwrap_err(),
        LookupError::ModelNotDiscovered { model_id: 103 }
    );
    assert_eq!(device.models::<Model103>().count(), 0);
    assert_eq!(client.take_requests(), []);
}

#[test]
fn test_point_outside_of_model() {
    // Model 1 reported with a length of 64 does not contain the `DA`
    // point at offset 64.
    let (device, client) = device_with_model1(64);
    let model = device.model::<Model1>().unwrap();
    assert!(matches!(
        block_on(model.read_point(Model1::DA)),
        Err(ReadPointError::PointOutOfBounds)
    ));
    assert!(matches!(
        block_on(model.write_point(Model1::DA, Some(2))),
        Err(WritePointError::PointOutOfBounds)
    ));
    assert_eq!(client.take_requests(), []);
}

#[test]
fn test_point_inside_of_model() {
    let (device, client) = device_with_model1(66);
    let model = device.model::<Model1>().unwrap();
    assert_eq!(block_on(model.read_point(Model1::DA)).unwrap(), Some(0));
    block_on(model.write_point(Model1::DA, Some(2))).unwrap();
    assert_eq!(
        client.take_requests(),
        [Request::Read(40068, 1), Request::Write(40068, vec![2])]
    );
}

#[test]
fn test_model_too_short() {
    // Model 1 reported with a length of 64 is missing the `DA` point.
    let (device, client) = device_with_model1(64);
    assert!(matches!(
        block_on(device.model::<Model1>().unwrap().read()),
        Err(ReadModelError::ModelTooShort {
            model_id: 1,
            len: 64
        })
    ));
    assert!(matches!(
        block_on(device.models::<AnyModel>().next().unwrap().read()),
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
    let model = device.model::<Model1>().unwrap();
    assert_eq!(block_on(model.read()).unwrap().da, Some(0));
    assert!(block_on(model.untyped().read()).is_ok());
}

#[test]
fn test_multiple_models() {
    let (device, client) = device_with_models(&[(1, model1(66, 1)), (1, model1(66, 2))]);
    let addrs = device
        .models::<AnyModel>()
        .map(|model| model.addr())
        .collect::<Vec<_>>();
    assert_eq!(addrs, [40004, 40072]);

    // Selecting a model which is not unique is an error.
    assert_eq!(
        device.model::<Model1>().unwrap_err(),
        LookupError::ModelNotUnique { model_id: 1 }
    );
    assert_eq!(client.take_requests(), []);

    let das = device
        .models::<Model1>()
        .map(|model| block_on(model.read()).unwrap().da)
        .collect::<Vec<_>>();
    assert_eq!(das, [Some(1), Some(2)]);
    assert_eq!(
        client.take_requests(),
        [Request::Read(40004, 66), Request::Read(40072, 66)]
    );

    let second = device.models::<Model1>().nth(1).unwrap();
    block_on(second.write_point(Model1::DA, Some(3))).unwrap();
    assert_eq!(client.take_requests(), [Request::Write(40136, vec![3])]);
    assert!(device.models::<Model1>().nth(2).is_none());
}

#[test]
fn test_untyped_model() {
    let (device, _) = device_with_models(&[(1, model1(66, 1)), (1, model1(66, 2))]);
    let second = device.models::<AnyModel>().nth(1).unwrap();
    assert_eq!(second.info().id, 1);
    let AnyModel::M1(model) = block_on(second.read()).unwrap() else {
        panic!("Unexpected model");
    };
    assert_eq!(model.da, Some(2));

    assert!(second.downcast::<Model103>().is_none());
    let typed = second.downcast::<Model1>().unwrap();
    assert_eq!(typed.addr(), 40072);
    assert_eq!(typed.untyped().addr(), 40072);
}

#[test]
fn test_device_from_discovery() {
    let (device, client) = device_with_model1(66);
    let restored = AsyncClient::new(client.clone(), config())
        .device_from_discovery(1, device.discovery().clone());
    assert_eq!(restored.slave_id(), 1);
    assert_eq!(restored.discovery(), device.discovery());
    // No discovery requests were issued.
    assert_eq!(client.take_requests(), []);
    let model = restored.model::<Model1>().unwrap();
    assert_eq!(block_on(model.read()).unwrap().da, Some(0));
}

#[cfg(feature = "model704")]
#[test]
fn test_len_includes_nested_groups() {
    use sunspec::{models::model704::Model704, Group};
    // 57 registers of points plus four nested groups of 2 registers each
    assert_eq!(Model704::LEN, 65);
}

#[cfg(feature = "model14")]
#[test]
fn test_write_string_point() {
    use sunspec::{models::model14::Model14, Group};
    let (device, client) = device_with_models(&[(14, vec![0; Model14::LEN.into()])]);
    let model = device.model::<Model14>().unwrap();
    // `ADDR` is a string of 20 registers at offset 7, `NAM` an optional
    // string of 4 registers at offset 0.
    let addr = 40004 + 7;
    let padded = |data: &[u16], len: usize| {
        let mut data = data.to_vec();
        data.resize(len, 0);
        data
    };

    // Strings shorter than the point are padded with zeros.
    block_on(model.write_point(Model14::ADDR, "ab".into())).unwrap();
    block_on(model.write_point(Model14::ADDR, "abc".into())).unwrap();
    block_on(model.write_point(Model14::NAM, None)).unwrap();
    assert_eq!(
        client.take_requests(),
        [
            Request::Write(addr, padded(&[0x6162], 20)),
            Request::Write(addr, padded(&[0x6162, 0x6300], 20)),
            Request::Write(40004, vec![0; 4]),
        ]
    );

    // Strings filling the whole point are written as they are.
    let full = "x".repeat(40);
    block_on(model.write_point(Model14::ADDR, full)).unwrap();
    assert_eq!(
        client.take_requests(),
        [Request::Write(addr, vec![0x7878; 20])]
    );

    // Strings longer than the point are rejected.
    assert!(matches!(
        block_on(model.write_point(Model14::ADDR, "x".repeat(41))),
        Err(WritePointError::ValueTooLarge)
    ));
    assert_eq!(client.take_requests(), []);
}

#[test]
fn test_invalid_response_length_during_discovery() {
    let client = RecordingClient::new(40000, &[0x5375, 0x6e53, 0xFFFF, 0]);
    client.set_response_len(|len| usize::from(len) - 1);
    assert!(matches!(
        block_on(AsyncClient::new(client, config()).device(1)),
        Err(DiscoveryError::Modbus(ModbusError::InvalidResponseLength {
            expected: 2,
            actual: 1
        }))
    ));
}

#[test]
fn test_invalid_response_length() {
    let (device, client) = device_with_model1(66);
    let model = device.model::<Model1>().unwrap();
    client.set_response_len(|len| usize::from(len) - 1);
    assert!(matches!(
        block_on(model.read()),
        Err(ReadModelError::Modbus(ModbusError::InvalidResponseLength {
            expected: 66,
            actual: 65
        }))
    ));
    assert!(matches!(
        block_on(model.read_point(Model1::DA)),
        Err(ReadPointError::Modbus(ModbusError::InvalidResponseLength {
            expected: 1,
            actual: 0
        }))
    ));
    client.set_response_len(|len| usize::from(len) + 1);
    assert!(matches!(
        block_on(model.read()),
        Err(ReadModelError::Modbus(ModbusError::InvalidResponseLength {
            expected: 66,
            actual: 67
        }))
    ));
}

#[test]
fn test_max_read_length() {
    let (mut device, client) = device_with_model1(66);
    device.config.max_read_length = 30;
    assert!(block_on(device.model::<Model1>().unwrap().read()).is_ok());
    assert_eq!(
        client.take_requests(),
        [
            Request::Read(40004, 30),
            Request::Read(40034, 30),
            Request::Read(40064, 6)
        ]
    );
    // A maximum read length of 0 is treated as 1.
    device.config.max_read_length = 0;
    assert!(block_on(device.model::<Model1>().unwrap().read()).is_ok());
    assert_eq!(client.take_requests().len(), 66);
}

#[test]
fn test_suns_identifier_at_end_of_address_space() {
    let client = RecordingClient::new(65534, &[0x5375, 0x6e53]);
    let config = Config {
        discovery_addresses: vec![65534],
        ..config()
    };
    assert!(matches!(
        block_on(AsyncClient::new(client, config).device(1)),
        Err(DiscoveryError::AddressOverflow)
    ));
}
