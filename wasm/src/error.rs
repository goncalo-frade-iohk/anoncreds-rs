use serde::{Deserialize, Serialize};
use serde::de::value::Error;
use ursa::errors::UrsaCryptoError;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Errors {
    Error,
    AnonCredsError,
    UrsaError,
    SerializationError,
}

#[derive(Serialize, Deserialize, Debug)]
#[wasm_bindgen(inspectable)]
pub struct AnoncredsError {
    pub(crate) code: Errors,
    pub(crate) message: String,
}


impl From<anoncreds::ErrorKind>  for AnoncredsError {
    fn from(error: anoncreds::ErrorKind) -> AnoncredsError {
        AnoncredsError {
            code: Errors::AnonCredsError,
            message: format!("AnonCredsError {}",error.to_string()),
        }
    }
}

impl From<UrsaCryptoError>  for AnoncredsError {
    fn from(error: UrsaCryptoError) -> AnoncredsError {
        AnoncredsError {
            code: Errors::UrsaError,
            message: format!("UrsaError {}",error.to_string()),
        }
    }
}
impl From<anoncreds::Error> for AnoncredsError {
    fn from(error: anoncreds::Error) -> AnoncredsError {
        AnoncredsError {
            code: Errors::AnonCredsError,
            message: format!("AnoncredsError {}", error.message.unwrap()),
        }
    }
}

impl From<serde_wasm_bindgen::Error> for AnoncredsError {
    fn from(error: serde_wasm_bindgen::Error) -> AnoncredsError {
        AnoncredsError {
            code: Errors::SerializationError,
            message:format!("Serialization {}", error),
        }
    }
}

impl From<Error> for AnoncredsError {
    fn from(error:Error) -> AnoncredsError {
        AnoncredsError {
            code: Errors::SerializationError,
            message:format!("Serialization {}", error),
        }
    }
}


#[wasm_bindgen]
impl AnoncredsError {
    #[wasm_bindgen(getter)]
    pub fn code(&self) -> Errors {
        self.code.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.to_string()
    }

}