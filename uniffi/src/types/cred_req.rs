use crate::types::error::AnoncredsError;
use crate::types::nonce::Nonce;
use anoncreds_core::data_types::cred_request::{CredentialRequest as AnoncredsCredentialRequest, CredentialRequestMetadata as AnoncredsCredentialRequestMetadata};
use std::sync::Arc;

pub struct CredentialRequest {
    pub core: AnoncredsCredentialRequest
}

impl CredentialRequest {
    pub fn new(json_string: String) -> Result<Self, AnoncredsError> {
        let core_def: AnoncredsCredentialRequest =
            serde_json::from_str(&json_string).map_err(|err| AnoncredsError::ConversionError(err.to_string()))?;
        return Ok(CredentialRequest { core: core_def });
    }

    pub fn get_blinded_credential_secrets_json(&self) -> String {
        serde_json::to_string(&self.core.blinded_ms).unwrap()
    }

    pub fn get_blinded_credential_secrets_correctness_proof_json(&self) -> String {
        serde_json::to_string(&self.core.blinded_ms_correctness_proof).unwrap()
    }

    pub fn get_nonce(&self) -> Arc<Nonce> {
        return Arc::new(Nonce { anoncreds_nonce: self.core.nonce.try_clone().unwrap() })
    }

    pub fn get_json(&self) -> Result<String, AnoncredsError> {
        serde_json::to_string(&self.core).map_err(|err| AnoncredsError::ConversionError(err.to_string()))
    }
}

pub struct CredentialRequestMetadata {
    pub core: AnoncredsCredentialRequestMetadata
}

impl CredentialRequestMetadata {
    pub fn new(json_string: String) -> Result<Self, AnoncredsError> {
        let core_def: AnoncredsCredentialRequestMetadata =
            serde_json::from_str(&json_string).map_err(|err| AnoncredsError::ConversionError(err.to_string()))?;
        return Ok(CredentialRequestMetadata { core: core_def });
    }

    pub fn get_json(&self) -> Result<String, AnoncredsError> {
        serde_json::to_string(&self.core).map_err(|err| AnoncredsError::ConversionError(err.to_string()))
    }

    pub fn get_link_secret_blinding_data(&self) -> String {
        serde_json::to_string(&self.core.link_secret_blinding_data).unwrap()
    }

    pub fn get_link_secret_name(&self) -> String {
        serde_json::to_string(&self.core.link_secret_name).unwrap()
    }

    pub fn get_nonce(&self) -> Arc<Nonce> {
        return Arc::new(Nonce { anoncreds_nonce: self.core.nonce.try_clone().unwrap() })
    }
}