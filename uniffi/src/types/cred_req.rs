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
        self.core.link_secret_name.clone()
    }

    pub fn get_nonce(&self) -> Arc<Nonce> {
        return Arc::new(Nonce { anoncreds_nonce: self.core.nonce.try_clone().unwrap() })
    }
}

#[cfg(test)]
mod test {
    use anoncreds_core::data_types::cred_def::CredentialDefinition;
    use anoncreds_core::issuer::{create_credential_definition, create_credential_offer, create_schema};
    use anoncreds_core::prover::{create_credential_request};
    use anoncreds_core::types::{AttributeNames, CredentialDefinitionConfig, CredentialKeyCorrectnessProof, CredentialOffer, LinkSecret, SignatureType};
    use crate::{AnoncredsError, CredentialRequestMetadata};

    const NEW_IDENTIFIER: &str = "mock:uri";
    const LEGACY_DID_IDENTIFIER: &str = "DXoTtQJNtXtiwWaZAK3rB1";
    const LEGACY_SCHEMA_IDENTIFIER: &str = "DXoTtQJNtXtiwWaZAK3rB1:2:example:1.0";
    const LEGACY_CRED_DEF_IDENTIFIER: &str = "DXoTtQJNtXtiwWaZAK3rB1:3:CL:98153:default";

    const ENTROPY: Option<&str> = Some("entropy");
    const PROVER_DID: Option<&str> = Some(LEGACY_DID_IDENTIFIER);
    const LINK_SECRET_ID: &str = "link:secret:id";

    fn cred_def() -> anoncreds_core::Result<(CredentialDefinition, CredentialKeyCorrectnessProof)> {
        let credential_definition_issuer_id = "sample:id";

        let attr_names = AttributeNames::from(vec!["name".to_owned(), "age".to_owned()]);
        let schema = create_schema("schema:name", "1.0", "sample:uri", attr_names)?;
        let cred_def = create_credential_definition(
            "schema:id",
            &schema,
            credential_definition_issuer_id,
            "default",
            SignatureType::CL,
            CredentialDefinitionConfig {
                support_revocation: true,
            },
        )?;

        Ok((cred_def.0, cred_def.2))
    }

    fn link_secret() -> LinkSecret {
        LinkSecret::new().unwrap()
    }

    fn credential_offer(
        correctness_proof: CredentialKeyCorrectnessProof,
        is_legacy: bool,
    ) -> anoncreds_core::Result<CredentialOffer> {
        if is_legacy {
            create_credential_offer(
                LEGACY_SCHEMA_IDENTIFIER,
                LEGACY_CRED_DEF_IDENTIFIER,
                &correctness_proof,
            )
        } else {
            create_credential_offer(NEW_IDENTIFIER, NEW_IDENTIFIER, &correctness_proof)
        }
    }

    #[test]
    fn test_get_json() {
        let (cred_def, correctness_proof) = cred_def().unwrap();
        let link_secret = link_secret();
        let credential_offer = credential_offer(correctness_proof, false).unwrap();

        let res = create_credential_request(
            ENTROPY,
            None,
            &cred_def,
            &link_secret,
            LINK_SECRET_ID,
            &credential_offer,
        ).unwrap();

        let json_string = serde_json::to_string(&res.1).map_err(|err| AnoncredsError::ConversionError(err.to_string())).unwrap();

        let crm = CredentialRequestMetadata {
            core: res.1
        };

        println!("{}", crm.get_json().unwrap());

        assert_eq!(json_string, crm.get_json().unwrap().clone());
    }
}
