use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

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
        CredentialRequest::from(serde_wasm_bindgen::to_value(&self.request._request).unwrap())
    }

    #[wasm_bindgen(getter)]
    pub fn metadata(&self) -> Result<CredentialRequestMetadata, JsValue> {
        CredentialRequestMetadata::from(serde_wasm_bindgen::to_value(&self.metadata._metadata).unwrap())
    }
}


