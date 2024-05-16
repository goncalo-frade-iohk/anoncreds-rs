use std::collections::HashMap;
use anoncreds::types::{ CredentialDefinitionConfig, MakeCredentialValues, SignatureType};
use wasm_bindgen::prelude::*;
use crate::credential_definition::{CredentialDefinition, CredentialDefinitionPrivate, CredentialDefinitionPrivateResponse, CredentialKeyCorrectnessProof};
use crate::credential_offer::CredentialOffer;
use crate::credential_schema::CredentialSchema;
use crate::credential::Credential;
use crate::credential_request::CredentialRequest;
use anoncreds::data_types::cred_def::CredentialKeyCorrectnessProof as AnoncredsCredentialKeyCorrectnessProof;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Value {
    Str(String),
    Int(i32),
    Float(f64),
}

impl Value {
    fn to_string(&self) -> String {
        match self {
            Value::Str(s) => s.clone(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
        }
    }
}

#[wasm_bindgen]
pub struct Issuer;


#[wasm_bindgen]
impl Issuer {

    #[wasm_bindgen(js_name = createSchema)]
    pub fn create_schema(
        schema_name: &str,
        schema_version:&str,
        issuer_id: &str,
        attr_names: Vec<String>
    ) -> Result<CredentialSchema, JsValue> {
        CredentialSchema::create_schema(
            schema_name,
            schema_version,
            issuer_id,
            attr_names
        )
    }

    #[wasm_bindgen(js_name = createCredentialDefinition)]
    pub fn create_credential_definition(
        schema_id: &str,
        schema: CredentialSchema,
        issuer_id: &str,
        tag: &str,
    ) -> CredentialDefinitionPrivateResponse {
        let iss = issuer_id.to_string();
        let definition = anoncreds::issuer::create_credential_definition(
            schema_id,
            &schema._schema,
            iss,
            tag,
            SignatureType::CL,
            CredentialDefinitionConfig::new(false)
        ).expect("Couldn't create the credential definition");
        CredentialDefinitionPrivateResponse {
            credential_definition:CredentialDefinition::new(definition.0),
            credential_definition_private: CredentialDefinitionPrivate::new(definition.1),
            credential_key_correctness_proof: CredentialKeyCorrectnessProof {
                _value: definition.2.value
            }
        }
    }

    #[wasm_bindgen(js_name = createCredentialOffer)]
    pub fn create_credential_offer(
        schema_id: &str,
        cred_def_id: &str,
        correctness_proof: CredentialKeyCorrectnessProof
    ) -> CredentialOffer {
        let cloned_correctness: ursa::cl::CredentialKeyCorrectnessProof = serde_wasm_bindgen::from_value(serde_wasm_bindgen::to_value(&correctness_proof._value).unwrap()).unwrap();
        let offer = anoncreds::issuer::create_credential_offer(
            schema_id.to_string(),
            cred_def_id.to_string(),
            &AnoncredsCredentialKeyCorrectnessProof {
                value: cloned_correctness,
            }
        );
        CredentialOffer {
            _offer: offer.unwrap()
        }
    }

    #[wasm_bindgen(js_name = "createCredential")]
    pub fn create_credential(
        credential_definition: CredentialDefinition,
        credential_definition_private: CredentialDefinitionPrivate,
        credential_offer: CredentialOffer,
        credential_request: CredentialRequest,
        values: JsValue
    ) -> Result<Credential, JsValue> {

        let map: HashMap<String, Value> = serde_wasm_bindgen::from_value(values)?;
        let mut credential_values = MakeCredentialValues::default();

        for (key, value) in map.iter() {
            credential_values.add_raw(key, value.to_string())
                .map_err(|e| JsValue::from_str(&format!("Could not add_raw value error: {}", e)))?;
        }

        let credential = anoncreds::issuer::create_credential(
            &credential_definition._definition,
            &credential_definition_private._value,
            &credential_offer._offer,
            &credential_request._request,
            credential_values.into(),
            None,
            None,
            None
        ) .map_err(|e| JsValue::from_str(&format!("Could not create credential error: {}", e)))?;

        Ok(Credential {
            _credential: credential
        })
    }

}