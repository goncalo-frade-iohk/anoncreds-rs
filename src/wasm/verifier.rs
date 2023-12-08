// --- VERIFIER ---
use wasm_bindgen::prelude::*;

use std::collections::HashMap;

use crate::data_types::cred_def::CredentialDefinition;
use crate::data_types::cred_def::CredentialDefinitionId;
use crate::data_types::presentation::Presentation;
use crate::data_types::pres_request::PresentationRequest;
use crate::data_types::schema::Schema;
use crate::data_types::schema::SchemaId;

use crate::services::verifier;

use super::helpers;

#[wasm_bindgen(js_name = verifierVerifyPresentation)]
pub fn verifier_verify_presentation(
  js_presentation: JsValue,
  js_presentation_request: JsValue,
  js_schema: JsValue, 
  js_cred_def: JsValue
) -> bool {
  let presentation: Presentation = serde_wasm_bindgen::from_value(js_presentation).unwrap();
  let pres_request: PresentationRequest = helpers::deserialise_presentation_request(js_presentation_request);

  let schema: Schema = serde_wasm_bindgen::from_value(js_schema).unwrap();
  let mut schemas = HashMap::new();
  let schema_id = SchemaId::new_unchecked("did:web:xyz/resource/schema");
  schemas.insert(&schema_id, &schema);
  
  let cred_def: CredentialDefinition = serde_wasm_bindgen::from_value(js_cred_def).unwrap();
  let mut cred_defs = HashMap::new();
  let cred_def_id = CredentialDefinitionId::new_unchecked("did:web:xyz/resource/cred-def");
  cred_defs.insert(&cred_def_id, &cred_def);

  let verified = verifier::verify_presentation(&presentation, &pres_request, &schemas, &cred_defs, None, None, None).expect("Error");
 
  return verified;
}
