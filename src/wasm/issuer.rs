// --- ISSUER ---
use wasm_bindgen::prelude::*;

use crate::data_types::cred_def::CredentialDefinition;
use crate::data_types::cred_def::CredentialDefinitionPrivate;
use crate::data_types::cred_def::SignatureType;
use crate::data_types::cred_offer::CredentialOffer;
use crate::data_types::cred_request::CredentialRequest;
use crate::data_types::schema::Schema;

use crate::services::issuer;

use crate::types::CredentialDefinitionConfig;
use crate::types::MakeCredentialValues;

#[wasm_bindgen(js_name = issuerCreateCredentialOffer)]
pub fn issuer_create_credential_offer(js_schema: JsValue) -> Vec<JsValue> {
    let schema: Schema = serde_wasm_bindgen::from_value(js_schema).unwrap();

    let (cred_def, cred_def_priv, key_correctness_proof) =
        issuer::create_credential_definition("did:web:xyz/resource/schema",
                                            &schema,
                                            "did:web:xyz",
                                            "default-tag",
                                            SignatureType::CL,
                                            CredentialDefinitionConfig::default()
                                            ).expect("Unable to create Credential Definition");

    let credential_offer =
        issuer::create_credential_offer("did:web:xyz/resource/schema",
                                        "did:web:xyz/resource/cred-def",
                                        &key_correctness_proof,
                                        ).expect("Unable to create Credential Offer");

    let js_cred_offer = serde_wasm_bindgen::to_value(&credential_offer).unwrap();
    let js_cred_def = serde_wasm_bindgen::to_value(&cred_def).unwrap();
    let js_cred_def_priv = serde_wasm_bindgen::to_value(&cred_def_priv).unwrap();
    
    vec![js_cred_offer, js_cred_def, js_cred_def_priv]
}

#[wasm_bindgen(js_name = issuerCreateCredential)]
pub fn issuer_create_credential(
    js_cred_offer: JsValue,
    js_cred_def: JsValue,
    js_cred_def_priv: JsValue,
    js_cred_request: JsValue
) -> JsValue {
    let cred_def: CredentialDefinition = serde_wasm_bindgen::from_value(js_cred_def).unwrap();
    let cred_def_priv: CredentialDefinitionPrivate = serde_wasm_bindgen::from_value(js_cred_def_priv).unwrap();
    let credential_offer: CredentialOffer = serde_wasm_bindgen::from_value(js_cred_offer).unwrap();
    let credential_request: CredentialRequest = serde_wasm_bindgen::from_value(js_cred_request).unwrap();

    let mut credential_values = MakeCredentialValues::default();
    credential_values.add_raw("name", "john").expect("Unable to add credential value");
    credential_values.add_raw("age", "28").expect("Unable to add credential value");

    let credential = issuer::create_credential(
        &cred_def,
        &cred_def_priv,
        &credential_offer,
        &credential_request,
        credential_values.into(),
        None,
        None,
        None
    )
    .expect("Unable to create credential");

    serde_wasm_bindgen::to_value(&credential).unwrap()
}
