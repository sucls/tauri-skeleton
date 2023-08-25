use anyhow::Error;
use std::borrow::Cow;
use thiserror::Error as ThisError;

type ErrString = Cow<'static, str>;

#[derive(Debug, ThisError)]
pub enum PluginError {
    #[error("execute error: {0}")]
    Failure(u8, String),
}

impl From<anyhow::Error> for PluginError {
    fn from(err: Error) -> Self {
        PluginError::Failure(9, format!("{:?}", err).into(),)
    }
}

#[derive(serde::Serialize)]
pub struct PluginResponse {
    pub(crate) message: String
}

pub type PluginResult = std::result::Result<PluginResponse, PluginError>;

