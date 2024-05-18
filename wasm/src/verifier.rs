use std::collections::HashMap;
use anoncreds::data_types::cred_def::CredentialDefinitionId;
use anoncreds::data_types::schema::{Schema, SchemaId};
use anoncreds::verifier::verify_presentation;
use wasm_bindgen::prelude::*;
use crate::presentation::{Presentation, PresentationRequest};
use anoncreds::data_types::cred_def::{CredentialDefinition as AnoncredsCredentialDefinition};
use anoncreds::data_types::nonce::Nonce;
use serde_wasm_bindgen::from_value;
use crate::error::AnoncredsError;



#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type SchemasDict = Record<string, CredentialSchema>;
export type DefinitionsDict = Record<string, CredentialDefinition>;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "SchemasDict")]
    pub type SchemasDict;
    #[wasm_bindgen(typescript_type = "DefinitionsDict")]
    pub type DefinitionsDict;

    #[wasm_bindgen(typescript_type = "RequestedPredicates")]
    pub type RequestedPredicates;

    #[wasm_bindgen(typescript_type = "RequestedAttributes")]
    pub type RequestedAttributes;
}

#[wasm_bindgen]
pub struct Verifier;


#[wasm_bindgen]
impl Verifier {

    #[wasm_bindgen(js_name = "createNonce")]
    pub fn create_nonce() -> Result<String, JsValue> {
        Ok(Nonce::new().unwrap().as_native().to_dec().unwrap())
    }

    #[wasm_bindgen(js_name = "createPresentationRequest")]
    pub fn create_presentation_request(
         name: String,
         version: String,
         requested_attributes: &RequestedPredicates,
         requested_predicates: &RequestedAttributes,
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
        presentation: &Presentation,
        presentation_request: &PresentationRequest,
        schemas_dict: &SchemasDict,
        credential_definition_dict: &DefinitionsDict
    ) -> Result<bool, JsValue> {
        let mut schemas = HashMap::new();
        let mut defs = HashMap::new();

        let schemas_dict_js: JsValue = schemas_dict.into();
        let definitions_dict_js: JsValue = credential_definition_dict.into();

        let schema_list : HashMap<SchemaId, Schema> =  from_value(schemas_dict_js.clone())
            .map_err(|e| JsValue::from(AnoncredsError::from(e)))?;

        let cred_def_list : HashMap<
            CredentialDefinitionId, AnoncredsCredentialDefinition
        > =  from_value(definitions_dict_js.clone())
            .map_err(|e| JsValue::from(AnoncredsError::from(e)))?;

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