use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use anoncreds::data_types::credential::Credential as AnoncredsCredential;
use serde::{Deserialize, Serialize};
use crate::error::AnoncredsError;

#[wasm_bindgen(inspectable)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Credential {
    pub(crate) _credential: AnoncredsCredential
}

#[wasm_bindgen]
impl Credential {

    #[wasm_bindgen( js_name = from)]
    pub fn from(credential: JsValue) -> Result<Credential, JsValue> {
        let credential:AnoncredsCredential = serde_wasm_bindgen::from_value::<AnoncredsCredential>(credential)
            .map_err(|e| AnoncredsError::from(e))?;
        Ok(Credential {
            _credential:credential
        })
    }

    #[wasm_bindgen(getter)]
    pub fn schema_id(&self) -> String {
        self._credential.schema_id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn cred_def_id(&self) -> String {
        self._credential.cred_def_id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn values(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self._credential.values).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self._credential.signature).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn signature_correctness_proof(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self._credential.signature_correctness_proof).unwrap()
    }
}
