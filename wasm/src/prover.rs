use std::collections::HashMap;
use crate::credential::Credential;
use crate::credential_definition::CredentialDefinition;
use crate::credential_offer::CredentialOffer;
use crate::credential_request::{ CredentialRequest};
use crate::credential_request_metadata::CredentialRequestMetadata;
use crate::credential_request_response::CreateCredentialRequestResponse;
use crate::link_secret::LinkSecret;
use anoncreds::data_types::cred_def::{CredentialDefinition as AnoncredsCredentialDefinition, CredentialDefinitionId};
use anoncreds::data_types::nonce::Nonce;
use anoncreds::data_types::schema::{Schema, SchemaId};
use anoncreds::types::PresentCredentials;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::error::AnoncredsError;
use crate::presentation::{Presentation, PresentationRequest};



#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "SchemasDict")]
    pub type SchemasDict;

    #[wasm_bindgen(typescript_type = "DefinitionsDict")]
    pub type DefinitionsDict;
}



#[wasm_bindgen]
pub struct Prover;

#[wasm_bindgen]
impl Prover {

    #[wasm_bindgen( js_name = createLinkSecret)]
    pub fn create_link_secret() -> Result<LinkSecret, JsValue> {
        LinkSecret::new()
    }

    #[wasm_bindgen( js_name = createCredentialRequest)]
    pub fn create_credential_request(
        credential_definition: &CredentialDefinition,
        link_secret: &LinkSecret,
        link_secret_id: String,
        credential_offer: &CredentialOffer
    ) -> Result<CreateCredentialRequestResponse, JsValue> {
        let response = anoncreds::prover::create_credential_request(
            Some(Nonce::new().unwrap().as_native().to_dec().unwrap().as_str()),
            None,
            &credential_definition._definition,
            &link_secret._link_secret,
            &link_secret_id,
            &credential_offer._offer
        ).map_err(|e| JsValue::from(AnoncredsError::from(e)))?;

        let request = CredentialRequest {
            _request:response.0
        };

        let metadata = CredentialRequestMetadata {
            _metadata:response.1
        };

        Ok(CreateCredentialRequestResponse::new(request, metadata))
    }

    #[wasm_bindgen( js_name = processCredential)]
    pub fn process_credential(
        credential: &Credential,
        credential_request_metadata: &CredentialRequestMetadata,
        link_secret: &LinkSecret,
        credential_definition: &CredentialDefinition,
    ) -> Result<Credential, JsValue> {
        let mut mutable_credential = credential._credential.try_clone().unwrap();
        anoncreds::prover::process_credential(
            &mut mutable_credential,
            &credential_request_metadata._metadata,
            &link_secret._link_secret,
            &credential_definition._definition,
            None,
        ).map_err(|e| JsValue::from(AnoncredsError::from(e)))?;
        Ok( Credential {
           _credential: mutable_credential
       })
    }

    #[wasm_bindgen( js_name = createPresentation)]
    pub fn create_presentation(
        presentation_request: PresentationRequest,
        credential: &Credential,
        link_secret: &LinkSecret,
        schemas_dict: &SchemasDict,
        credential_definition_dict: &DefinitionsDict
    ) -> Result<Presentation, JsValue>{
        let mut schemas = HashMap::new();
        let mut cred_defs = HashMap::new();

        let  schema_list : HashMap<SchemaId, Schema> =  from_value(schemas_dict.obj.clone())
            .map_err(|e| JsValue::from(AnoncredsError::from(e)))?;

        let cred_def_list : HashMap<CredentialDefinitionId, AnoncredsCredentialDefinition> =  from_value(credential_definition_dict.obj.clone())
            .map_err(|e| JsValue::from(AnoncredsError::from(e)))?;

        for (key, value) in schema_list.iter() {
            schemas.insert(key, value);
        }

        for (key, value) in cred_def_list.iter() {
            cred_defs.insert(key, value);
        }

        let mut present = PresentCredentials::default();
        let mut cred = present.add_credential(&credential._credential, None, None);

        for key_name in  presentation_request._presentation_request.value().requested_attributes.keys() {
            cred.add_requested_attribute(key_name, true);
        }

        for key_name in presentation_request._presentation_request.value().requested_predicates.keys() {
            cred.add_requested_predicate(key_name);
        }

        let presentation = anoncreds::prover::create_presentation(
            &presentation_request._presentation_request,
            present,
            None,
            &link_secret._link_secret,
            &schemas,
            &cred_defs
        ).map_err(|e| JsValue::from(AnoncredsError::from(e)))?;

        Ok(
           Presentation {
               _presentation:presentation
           }
        )
    }
}