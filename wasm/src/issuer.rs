use anoncreds::types::{AttributeNames, CredentialDefinitionConfig, SignatureType};
use wasm_bindgen::prelude::*;
use crate::credential_definition::{CredentialDefinition, CredentialDefinitionPrivate, CredentialDefinitionPrivateResponse, CredentialKeyCorrectnessProof};
use crate::credential_schema::CredentialSchema;
use crate::link_secret::LinkSecret;

#[wasm_bindgen]
pub struct Issuer;


#[wasm_bindgen]
impl Issuer {

    #[wasm_bindgen(static_method_of = Issuer, js_name = createSchema)]
    pub fn create_schema(
        schema_name: &str,
        schema_version:&str,
        issuer_id: &str,
        attr_names: Vec<String>
    ) -> CredentialSchema {
        CredentialSchema::create_schema(
            schema_name,
            schema_version,
            issuer_id,
            attr_names
        )
    }

    #[wasm_bindgen(static_method_of = Issuer, js_name = createCredentialDefinition)]
    pub fn create_credential_definition(
        schema_id: String,
        schema: CredentialSchema,
        issuer_id: String,
        tag: String,
    ) -> CredentialDefinitionPrivateResponse {
        let definition = anoncreds::issuer::create_credential_definition(
            schema_id,
            &schema._schema,
            issuer_id,
            tag.as_str(),
            SignatureType::CL,
            CredentialDefinitionConfig::new(false)
        ).expect("Couldn't create the credential definition");
        CredentialDefinitionPrivateResponse {
            credential_definition:CredentialDefinition::new(definition.0),
            credential_definition_private: CredentialDefinitionPrivate::new(definition.1),
            credential_key_correctness_proof: CredentialKeyCorrectnessProof::new(definition.2.try_clone().unwrap())
        }
    }
}