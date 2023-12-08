// --- PROVER ---
extern crate console_error_panic_hook;

use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use crate::data_types::cred_def::CredentialDefinition;
use crate::data_types::cred_def::CredentialDefinitionId;
use crate::data_types::schema::Schema;
use crate::data_types::schema::SchemaId;

use crate::services::prover;
use crate::services::utils::new_nonce;

use crate::types::PresentCredentials;

use super::helpers;


#[wasm_bindgen(js_name = proverCreateLinkSecret)]
pub fn prover_create_link_secret() -> String {
  let secret = prover::create_link_secret().expect("Unable to create link secret");
  let secret_str = secret.try_into().expect("Unable to convert link secret");

  return secret_str;
}

#[wasm_bindgen(js_name = proverCreateCredentialRequest)]
pub fn prover_create_credential_request(
  cred_offer: JsValue,
  cred_def: JsValue,
  link_secret: &str,
  link_secret_id: &str
) -> Vec<JsValue> {
  let credential_offer = helpers::deserialise_credential_offer(cred_offer);
  let cred_def = helpers::deserialise_credential_definition(cred_def);
  let link_secret = helpers::deserialise_link_secret(link_secret);
  let entropy = new_nonce().unwrap().to_string();
  
  let (credential_request, credential_request_metadata) =
    prover::create_credential_request(
      Some(&entropy),
      None,
      &cred_def,
      &link_secret,
      &link_secret_id,
      &credential_offer,
    )
    .expect("Unable to create Credential Request");

  let js_cred_req = serde_wasm_bindgen::to_value(&credential_request).expect("Unable to serialise Credential Request");
  let js_cred_meta = serde_wasm_bindgen::to_value(&credential_request_metadata).expect("Unable to serialise Credential Request Metadata");
  let js_entropy = serde_wasm_bindgen::to_value(&entropy).expect("Unable to serialise Entropy");

  vec![js_cred_req, js_cred_meta, js_entropy]
}

#[wasm_bindgen(js_name = proverProcessCredential)]
pub fn prover_process_credential(
  cred_def: JsValue,
  credential: JsValue,
  cred_req_meta: JsValue,
  link_secret: &str,
) -> JsValue {
  let mut credential = helpers::deserialise_credential(credential);
  let cred_def = helpers::deserialise_credential_definition(cred_def);
  let cred_req_meta = helpers::deserialise_credential_request_metadata(cred_req_meta);
  let link_secret = helpers::deserialise_link_secret(link_secret);

  prover::process_credential(
    &mut credential,
    &cred_req_meta,
    &link_secret,
    &cred_def,
    None
  )
  .expect("Unable to process the Credential");

  serde_wasm_bindgen::to_value(&credential).expect("Unable to serialise Credential")
}

#[wasm_bindgen(js_name = proverCreatePresentation)]
pub fn prover_create_presentation(
  presentation_request: JsValue,
  schema_dict: JsValue,
  cred_def_dict: JsValue,
  credential: JsValue,
  link_secret: &str,
) -> JsValue {
  let pres_request = helpers::deserialise_presentation_request(presentation_request);
  let credential = helpers::deserialise_credential(credential);
  let link_secret = helpers::deserialise_link_secret(link_secret);

  let mut schemas = HashMap::new();
  let schema_list: HashMap<SchemaId, Schema> = serde_wasm_bindgen::from_value(schema_dict)
    .expect("Unable to deserialise Schemas");

  for (key, value) in schema_list.iter() {
    schemas.insert(key, value);
  }

  let mut cred_defs = HashMap::new();
  let cred_def_list: HashMap<CredentialDefinitionId, CredentialDefinition> = serde_wasm_bindgen::from_value(cred_def_dict)
    .expect("Unable to deserialise Credential Definitions");

  for (key, value) in cred_def_list.iter() {
    cred_defs.insert(key, value);
  }

  let mut present = PresentCredentials::default();
  let mut cred1 = present.add_credential(&credential, None, None);
  let pres_req_val = pres_request.value();

  for key in pres_req_val.requested_attributes.keys() {
    cred1.add_requested_attribute(key, true);
  }

  for key in pres_req_val.requested_predicates.keys() {
    cred1.add_requested_predicate(key);
  }

  let presentation = prover::create_presentation(
    &pres_request,
    present,
    None,
    &link_secret,
    &schemas,
    &cred_defs
  )
  .expect("Unable to create Presentation");

  serde_wasm_bindgen::to_value(&presentation).expect("Unable to serialise Presentation")
}
