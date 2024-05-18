use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::utils::fix_js_value;

use super::credential_request::CredentialRequest;
use super::credential_request_metadata::CredentialRequestMetadata;

#[wasm_bindgen]
pub struct CreateCredentialRequestResponse {
    pub(crate) request: CredentialRequest,
    pub(crate) metadata: CredentialRequestMetadata
}

#[wasm_bindgen]
impl CreateCredentialRequestResponse {

    pub(crate) fn new(request: CredentialRequest, metadata: CredentialRequestMetadata) -> Self {
        CreateCredentialRequestResponse {
            request,
            metadata
        }
    }

    #[wasm_bindgen(getter)]
    pub fn request(&self) -> Result<CredentialRequest, JsValue> {
        CredentialRequest::from(fix_js_value(serde_wasm_bindgen::to_value(&self.request._request).unwrap()).as_ref())
    }

    #[wasm_bindgen(getter)]
    pub fn metadata(&self) -> Result<CredentialRequestMetadata, JsValue> {
        CredentialRequestMetadata::from(fix_js_value(serde_wasm_bindgen::to_value(&self.metadata._metadata).unwrap()).as_ref())
    }

}


