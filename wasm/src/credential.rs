use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use anoncreds::data_types::credential::Credential as AnoncredsCredential;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use crate::error::AnoncredsError;
use crate::utils::fix_js_value;
use crate::credential_request_metadata::CredentialRequestMetadata;
use crate::link_secret::LinkSecret;
use crate::credential_definition::CredentialDefinition;


#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export interface Credential_Value {
    encoded: string;
    raw: string;
}
export type CrerdentialSignatureType = {
    p_credential: {
        m_2: string;
        a: string;
        e: string;
        v: string;
    };
}
export type CredentialType = {
    readonly schema_id: string;
    readonly cred_def_id: string;
    readonly values: Record<string, Credential_Value>;
    readonly signature: CrerdentialSignatureType;
    readonly signature_correctness_proof: {
        c: string;
        se: string;
    };
}
export class Credential implements CredentialType {
    free(): void;
    static from(credential: CredentialType): Credential;
    toJSON(): CredentialType
    readonly schema_id: string;
    readonly cred_def_id: string;
    readonly values: Record<string, Credential_Value>;
    readonly signature:CrerdentialSignatureType;
    readonly signature_correctness_proof: {
        c: string;
        se: string;
    };
}
"#;

#[wasm_bindgen(skip_typescript)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Credential {
    pub(crate) _credential: AnoncredsCredential
}

#[wasm_bindgen]
impl Credential {

    #[wasm_bindgen(js_name="toJSON")]
    pub fn to_json(&self) -> JsValue {
        fix_js_value(serde_wasm_bindgen::to_value(&self._credential).unwrap())
    }

    #[wasm_bindgen( js_name = from)]
    pub fn from(credential: &JsValue) -> Result<Credential, JsValue> {
        let credential:AnoncredsCredential = from_value::<AnoncredsCredential>(fix_js_value(credential.clone()))
            .map_err(|e| JsValue::from(AnoncredsError::from(e)))?;
        Ok(Credential {
            _credential:credential
        })
    }

    #[wasm_bindgen(getter)]
    pub fn schema_id(&self) -> String {
        self._credential.schema_id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn cred_def_id(&self) -> String {
        self._credential.cred_def_id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn values(&self) -> JsValue {
        fix_js_value(serde_wasm_bindgen::to_value(&self._credential.values).unwrap())
    }

    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> JsValue {
        fix_js_value(serde_wasm_bindgen::to_value(&self._credential.signature).unwrap())
    }

    #[wasm_bindgen(getter)]
    pub fn signature_correctness_proof(&self) -> JsValue {
        fix_js_value(serde_wasm_bindgen::to_value(&self._credential.signature_correctness_proof).unwrap())
    }

    #[wasm_bindgen]
    pub fn process(
        &self,
        credential_request_metadata: &CredentialRequestMetadata,
        link_secret: &LinkSecret,
        credential_definition: &CredentialDefinition
    ) -> Credential {
        let mut mutable_credential = self._credential.try_clone().unwrap();
        anoncreds::prover::process_credential(
            &mut mutable_credential,
            &credential_request_metadata._metadata,
            &link_secret._link_secret,
            &credential_definition._definition,
            None,
        ).map_err(|e| JsValue::from(AnoncredsError::from(e)))?;

        Credential {
            _credential:mutable_credential
        }
    }


}
