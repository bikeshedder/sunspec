#![cfg(all(feature = "model1", feature = "model103"))]

use std::{
    collections::HashMap,
    future::Future,
    pin::pin,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
};

use sunspec::{
    client::{AsyncClient, AsyncModbusClient, Config, ModbusError},
    models::{model1::Model1, model103::Model103},
    AnyModel, Model, ModelInfo, MODELS,
};

/// In-memory Modbus client serving a fixed register map.
#[derive(Clone, Debug)]
struct MemoryClient {
    registers: Arc<HashMap<u16, u16>>,
}

impl MemoryClient {
    fn new(base: u16, data: &[u16]) -> Self {
        let registers = (base..).zip(data.iter().copied()).collect();
        Self {
            registers: Arc::new(registers),
        }
    }
}

impl AsyncModbusClient for MemoryClient {
    fn read_registers(
        &self,
        _slave_id: u8,
        addr: u16,
        len: u16,
    ) -> impl Future<Output = Result<Vec<u16>, ModbusError>> + Send {
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
        _addr: u16,
        _data: &[u16],
    ) -> Result<(), ModbusError> {
        Err(ModbusError::IllegalFunction)
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

#[rustfmt::skip]
const MODEL1: [u16; 66] = [
    16707, 19781, 8275, 20300, 16722, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,      // mn
    19791, 17477, 19488, 23128, 11571, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,     // md
    0, 0, 0, 0, 0, 0, 0, 0,                                                 // opt
    30256, 11824, 11636, 25971, 29696, 0, 0, 0,                             // vr
    21326, 11604, 17747, 21549, 12336, 12337, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // sn
    1,                                                                      // da
    0,                                                                      // pad
];

#[test]
fn test_read_any_model() {
    let mut data = vec![0x5375, 0x6e53, 1, MODEL1.len() as u16];
    data.extend(MODEL1);
    data.extend([0xFFFF, 0]);

    let config = Config {
        discovery_addresses: vec![40000],
        read_timeout: None,
        // Force the model to be read in multiple chunks
        max_read_length: 10,
        ..Config::default()
    };
    let client = AsyncClient::new(MemoryClient::new(40000, &data), config);
    let device = block_on(client.device(1)).unwrap();

    let info: &ModelInfo = "common".parse().unwrap();
    let addr = info.addr(&device.models).unwrap();
    assert_eq!((addr.addr(), addr.len()), (40004, 66));
    assert_eq!(Model103::INFO.addr(&device.models), None);

    let discovered = device.models.iter().collect::<Vec<_>>();
    assert_eq!(discovered.len(), 1);
    assert_eq!(discovered[0].0, info);
    assert_eq!(discovered[0].1, addr);

    let any_model = block_on(device.read_any_model(info)).unwrap();
    let model = block_on(device.read_model::<Model1>()).unwrap();
    assert_eq!(any_model.info(), &Model1::INFO);
    assert_eq!(any_model.as_dyn().info(), &Model1::INFO);
    assert_eq!(any_model, AnyModel::from(model.clone()));
    let AnyModel::M1(model1) = &any_model else {
        panic!("Unexpected model: {any_model:?}");
    };
    assert_eq!(model1.mn, "ACME SOLAR");
    assert_eq!(model1, &model);
}

#[test]
fn test_model_info() {
    let info = ModelInfo::by_id(1).unwrap();
    assert_eq!(info, &Model1::INFO);
    assert_eq!(info.id, 1);
    assert_eq!(info.name, "common");
    assert_eq!(info.label, "Common");
    assert_eq!(Model103::INFO.name, "inverter_three_phase");
    assert!(MODELS.contains(&info));

    let model = info.parse(&MODEL1).unwrap();
    assert_eq!(model, AnyModel::M1(Model1::parse(&MODEL1).unwrap()));
    assert!(info.parse(&[]).is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_any_model() {
    let model = Model1::parse(&MODEL1).unwrap();
    let any_model = AnyModel::from(model.clone());
    let json = serde_json::to_value(&any_model).unwrap();
    let mut expected = serde_json::to_value(&model).unwrap();
    let _ = expected
        .as_object_mut()
        .unwrap()
        .insert("model".into(), "common".into());
    assert_eq!(json, expected);
    assert_eq!(serde_json::from_value::<AnyModel>(json).unwrap(), any_model);
}
