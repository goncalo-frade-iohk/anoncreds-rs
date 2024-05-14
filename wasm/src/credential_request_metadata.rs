use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use anoncreds::data_types::cred_request::CredentialRequestMetadata as AnoncredsCredentialRequestMetadata;


#[wasm_bindgen]
pub struct CredentialRequestMetadata {
    pub(crate) _metadata: AnoncredsCredentialRequestMetadata
}

#[wasm_bindgen]
impl CredentialRequestMetadata {

    #[wasm_bindgen(static_method_of = CredentialRequestMetadata, js_name = from)]
    pub fn from(request: JsValue) -> Self {
        CredentialRequestMetadata {
            _metadata: serde_wasm_bindgen::from_value(request).expect("Unable to deserialise Credential Request Metadata")
        }
    }
}

