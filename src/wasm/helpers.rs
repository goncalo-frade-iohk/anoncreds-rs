// --- HELPERS ---
use wasm_bindgen::prelude::*;

use std::str::FromStr;

use crate::data_types::credential::Credential;
use crate::data_types::cred_def::SignatureType;
use crate::data_types::cred_def::CredentialDefinition;
use crate::data_types::cred_offer::CredentialOffer;
use crate::data_types::cred_request::CredentialRequestMetadata;
use crate::data_types::link_secret::LinkSecret;
use crate::data_types::pres_request::PresentationRequest;
use crate::data_types::schema::Schema;

use crate::services::issuer;

use crate::types::CredentialDefinitionConfig;

use crate::utils::validation::Validatable;

use super::error::ErrorCode;

#[wasm_bindgen(js_name = anoncredsSetDefaultLogger)]
pub fn anoncreds_set_default_logger() -> ErrorCode {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    debug!("Initialized default logger");

    ErrorCode::Success
}

#[wasm_bindgen(js_name = anoncredsCreateSchema)]
pub fn anoncreds_create_schema(
    name: &str,
    version: &str,
    issuer_id: &str,
    attribute_names: Vec<JsValue>,
) -> JsValue {
    let mut attribute_names_vec: Vec<String> = vec![];

    for name in &attribute_names {
        let name = name.as_string();
        if let Some(name) = name {
            attribute_names_vec.push(name.to_owned());
        }
    }

    let schema =
        issuer::create_schema(name, version, issuer_id, attribute_names_vec.into()).unwrap();

    serde_wasm_bindgen::to_value(&schema).unwrap()
}

#[wasm_bindgen(js_name = anoncredsCreateCredentialDefinition)]
pub fn anoncreds_create_credential_definition(
    schema_id: &str,
    schema: JsValue,
    tag: &str,
    issuer_id: &str,
    signature_type: &str,
    support_revocation: bool,
) -> Vec<JsValue> {
    let schema: Schema = serde_wasm_bindgen::from_value(schema).unwrap();
    let signature_type = SignatureType::from_str(signature_type)
        .map_err(err_map!(Input))
        .unwrap();
    let (cred_def, cred_def_pvt, key_proof) = issuer::create_credential_definition(
        schema_id,
        &schema,
        issuer_id,
        tag,
        signature_type,
        CredentialDefinitionConfig { support_revocation },
    )
    .unwrap();

    let cred_def = serde_wasm_bindgen::to_value(&cred_def).unwrap();
    let cred_def_pvt = serde_wasm_bindgen::to_value(&cred_def_pvt).unwrap();
    let key_proof = serde_wasm_bindgen::to_value(&key_proof).unwrap();

    vec![cred_def, cred_def_pvt, key_proof]
}

#[wasm_bindgen(js_name = anoncredsCreateCredentialDefinitionCustom)]
pub fn anoncreds_create_credential_definition_custom(
    schema_id: &str,
    schema: JsValue,
    tag: &str,
    issuer_id: &str,
) -> Vec<JsValue> {
    let schema: Schema = serde_wasm_bindgen::from_value(schema).unwrap();

    let (cred_def, cred_def_pvt, key_proof) = issuer::create_credential_definition(
        schema_id,
        &schema,
        issuer_id,
        tag,
        SignatureType::CL,
        CredentialDefinitionConfig { support_revocation: false },
    )
    .unwrap();

    let cred_def = serde_wasm_bindgen::to_value(&cred_def).unwrap();
    let cred_def_pvt = serde_wasm_bindgen::to_value(&cred_def_pvt).unwrap();
    let key_proof = serde_wasm_bindgen::to_value(&key_proof).unwrap();

    vec![cred_def, cred_def_pvt, key_proof]
}


#[wasm_bindgen(js_name = anoncredsValidateCredentialDefinitionFromJson)]
pub fn anoncreds_validate_credential_definition_from_json(
    json: JsValue
) -> Result<bool, JsValue> {
    let cred_def: CredentialDefinition = serde_wasm_bindgen::from_value(json).map_err(|e| <serde_wasm_bindgen::Error as Into<JsValue>>::into(e))?;
    cred_def.validate().map(|_| true).map_err(|e| JsValue::from_str(&e.to_string()))
}


// --- SERDE ---

pub fn deserialise_credential(credential: JsValue) -> Credential {
  serde_wasm_bindgen::from_value(credential).expect("Unable to deserialise Credential")
}

pub fn deserialise_credential_offer(cred_offer: JsValue) -> CredentialOffer {
  serde_wasm_bindgen::from_value(cred_offer).expect("Unable to deserialise Credential Offer")
}

pub fn deserialise_credential_definition(cred_def: JsValue) -> CredentialDefinition {
  serde_wasm_bindgen::from_value(cred_def).expect("Unable to deserialise Credential Definition")
}

pub fn deserialise_credential_request_metadata(cred_req_meta: JsValue) -> CredentialRequestMetadata {
  serde_wasm_bindgen::from_value(cred_req_meta).expect("Unable to deserialise Credential Request Metadata")
}

pub fn deserialise_link_secret(link_secret: &str) -> LinkSecret {
  LinkSecret::try_from(link_secret).expect("Unable to deserialise Link Secret")
}

pub fn deserialise_presentation_request(pres_req: JsValue) -> PresentationRequest {
  let json = serde_wasm_bindgen::from_value(pres_req).expect("Unable to deserialise Presentation Request");
  serde_json::from_value(json).expect("Unable to parse Presentation Request")
}
