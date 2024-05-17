use std::collections::HashMap;
use anoncreds::data_types::cred_def::CredentialDefinitionId;
use anoncreds::data_types::schema::{Schema, SchemaId};
use anoncreds::verifier::verify_presentation;
use wasm_bindgen::prelude::*;
use crate::presentation::{Presentation, PresentationRequest};
use anoncreds::data_types::cred_def::{CredentialDefinition as AnoncredsCredentialDefinition};
use crate::error::AnoncredsError;

#[wasm_bindgen]
pub struct Verifier;


#[wasm_bindgen]
impl Verifier {

    #[wasm_bindgen(js_name = "createPresentationRequest")]
    pub fn create_presentation_request(
         name: String,
         version: String,
         requested_attributes: JsValue,
         requested_predicates: JsValue,
    ) -> Result<PresentationRequest, JsValue> {
        PresentationRequest::new(
            name,
            version,
            requested_attributes,
            requested_predicates
        )
    }

    #[wasm_bindgen(js_name = "verifyPresentation")]
    pub fn verify_presentation(
        presentation: Presentation,
        presentation_request: PresentationRequest,
        schemas_dict: JsValue,
        credential_definition_dict: JsValue
    ) -> Result<bool, JsValue> {
        let mut schemas = HashMap::new();
        let mut defs = HashMap::new();

        let schema_list : HashMap<SchemaId, Schema> =  serde_wasm_bindgen::from_value(schemas_dict).map_err(|e| JsValue::from(AnoncredsError::from(e)))?;
        let cred_def_list : HashMap<CredentialDefinitionId, AnoncredsCredentialDefinition> =  serde_wasm_bindgen::from_value(credential_definition_dict).map_err(|e| JsValue::from(AnoncredsError::from(e)))?;

        for (key, value) in schema_list.iter() {
            schemas.insert(key, value);
        }

        for (key, value) in cred_def_list.iter() {
            defs.insert(key, value);
        }

        match verify_presentation(
            &presentation._presentation,
            &presentation_request._presentation_request,
            &schemas,
            &defs,
            None,
            None,
            None
        ) {
            Ok(verified) => Ok(verified),
            Err(_err) => Ok(false),
        }
    }



}