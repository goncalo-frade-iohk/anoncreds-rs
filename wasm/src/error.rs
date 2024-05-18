use js_sys::Reflect;
use serde::{Deserialize, Serialize};
use serde::de::value::Error;
use ursa::errors::UrsaCryptoError;
use wasm_bindgen::prelude::*;
use crate::utils::extract_property;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Errors {
    Error,
    AnonCredsError,
    UrsaError,
    SerializationError,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnoncredsError {
    pub code: Errors,
    pub message: String,
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

impl From<JsValue> for AnoncredsError {
    fn from(error: JsValue) -> AnoncredsError {
        let code = extract_property::<Errors>(&error, "code").expect("NO");
        let message = extract_property::<String>(&error, "message").expect("NO2");
        AnoncredsError {
            code,
            message,
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

impl From<AnoncredsError> for JsValue {
    fn from(failure: AnoncredsError) -> Self {
        let error = js_sys::Error::new(&failure.message).into();
        Reflect::set(&error, &"code".into(), &serde_wasm_bindgen::to_value(&failure.code).unwrap()).unwrap();
        error
    }
}

impl From<Errors> for JsValue {
    fn from(failure: Errors) -> Self {
        serde_wasm_bindgen::to_value(&failure).unwrap()
    }
}
