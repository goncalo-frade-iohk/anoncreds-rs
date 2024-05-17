use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use anoncreds::data_types::cred_request::CredentialRequestMetadata as AnoncredsCredentialRequestMetadata;
use crate::error::AnoncredsError;
use crate::utils::fix_js_value;


#[wasm_bindgen(inspectable)]
pub struct CredentialRequestMetadata {
    pub(crate) _metadata: AnoncredsCredentialRequestMetadata
}

#[wasm_bindgen]
impl CredentialRequestMetadata {

    #[wasm_bindgen(js_name = from)]
    pub fn from(request: JsValue) -> Result<CredentialRequestMetadata, JsValue> {
        let metadata:AnoncredsCredentialRequestMetadata = serde_wasm_bindgen::from_value(fix_js_value(request))
            .map_err(|e| JsValue::from(AnoncredsError::from(e)))?;

        Ok(CredentialRequestMetadata {
            _metadata: metadata
        })
    }

    #[wasm_bindgen(getter)]
    pub fn link_secret_blinding_data(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self._metadata.link_secret_blinding_data).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn nonce(&self) -> String {
        self._metadata.nonce.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn link_secret_name(&self) -> String {
        self._metadata.link_secret_name.to_string()
    }
}

