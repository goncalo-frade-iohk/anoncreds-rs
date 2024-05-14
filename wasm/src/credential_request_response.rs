use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use anoncreds::data_types::cred_request::CredentialRequestMetadata as AnoncredsCredentialRequestMetadata;
use anoncreds::data_types::cred_request::CredentialRequest as AnoncredsCredentialRequest;
use log::Metadata;

use super::credential_request::CredentialRequest;
use super::credential_request_metadata::CredentialRequestMetadata;

#[wasm_bindgen]
pub struct CreateCredentialRequestResponse {
    pub(crate) request: CredentialRequest,
    pub(crate) metadata: CredentialRequestMetadata
}

#[wasm_bindgen]
impl CreateCredentialRequestResponse {
    #[wasm_bindgen(constructor)]
    pub fn new(request: CredentialRequest, metadata: CredentialRequestMetadata) -> Self {
        CreateCredentialRequestResponse {
            request,
            metadata
        }
    }

    #[wasm_bindgen(getter)]
    pub fn request(&self) -> CredentialRequest {
        CredentialRequest::from(serde_wasm_bindgen::to_value(&self.request).unwrap())
    }

    #[wasm_bindgen(getter)]
    pub fn metadata(&self) -> CredentialRequestMetadata {
        CredentialRequestMetadata::from(serde_wasm_bindgen::to_value(&self.request).unwrap())
    }


}


