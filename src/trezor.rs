// use trezor_client::client::Trezor;

use alloy::serde::quantity::vec;

use crate::error::Error;

pub struct Trezor {
    device_id: String,
    model: String,
    revision: String,
    session_id: Vec<u8>,
    vendor: String
}

impl Trezor {
    pub async fn new() -> Result<Self, Error> {
        let mut trezor = Self {
            device_id: String::new(),
            model: String::new(),
            revision: String::new(),
            session_id: vec![],
            vendor: String::new()
        };

        trezor.initiate_session()?;

        Ok(trezor)
    }

    pub fn device_id(&self) -> String {
        self.device_id.clone()
    }

    pub fn model(&self) -> String {
        self.model.clone()
    }

    pub fn revision(&self) -> String {
        self.revision.clone()
    }

    pub fn vendor(&self) -> String {
        self.vendor.clone()
    }
}

impl Trezor {
    fn initiate_session(&mut self) -> Result<(), Error> {
        let mut client = trezor_client::unique(false)
            .map_err(|e| Error::TrezorError { description: e.to_string() })?;

        client.init_device(None)
            .map_err(|e| Error::TrezorError { description: e.to_string() })?;

        let features = client.features()
            .ok_or(Error::TrezorError { description: "Can't get features".to_string() })?;

        self.device_id = features.device_id().to_string();
        self.model = features.model().to_string();
        self.revision = features.internal_model().to_string();
        self.session_id = features.session_id().to_vec();
        self.vendor = features.fw_vendor().to_string();

        Ok(())
    }
}