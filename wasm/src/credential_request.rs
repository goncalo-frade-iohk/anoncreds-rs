use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use anoncreds::data_types::cred_request::CredentialRequest as AnoncredsCredentialRequest;
use serde::{Deserialize, Serialize};


#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export class CredentialRequest {
    free(): void;
    static from(request: any): CredentialRequest;

    readonly blinded_ms: {
        u: string;
        hidden_attributes: string[];
        committed_attributes: Record<string, any>; // supposed to be empty
    }
    readonly blinded_ms_correctness_proof:{
        c: string;
        v_dash_cap: string;
        m_caps: Record<string, string>;
        r_caps: Record<string, any>; // supposed to be empty
      } | undefined
    readonly nonce: string;
}
"#;

#[wasm_bindgen(skip_typescript)]
#[derive( Serialize, Deserialize)]
pub struct CredentialRequest {
    pub(crate) _request: AnoncredsCredentialRequest
}

#[wasm_bindgen(skip_typescript)]
impl CredentialRequest {

    #[wasm_bindgen(static_method_of = CredentialRequest, js_name = from)]
    pub fn from(request: JsValue) -> Self {
        CredentialRequest {
            _request: serde_wasm_bindgen::from_value(request).expect("Unable to deserialise Credential Request")
        }
    }

    #[wasm_bindgen(getter)]
    pub fn nonce(&self) -> String {
        self._request.nonce.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn blinded_ms(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self._request.blinded_ms).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn blinded_ms_correctness_proof(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self._request.blinded_ms_correctness_proof).unwrap()
    }
}
