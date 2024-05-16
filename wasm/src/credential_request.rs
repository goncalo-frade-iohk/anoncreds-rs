use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use anoncreds::data_types::cred_request::CredentialRequest as AnoncredsCredentialRequest;
use serde_wasm_bindgen::from_value;
use crate::error::AnoncredsError;
use crate::utils::extract_property;


#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export class CredentialRequest {
    free(): void;
    static from(request: any): CredentialRequest;
    readonly cred_def_id: string;
    readonly blinded_ms: {
        u: string;
        hidden_attributes: string[];
        committed_attributes: Record<string, any>; // supposed to be empty
    };
    readonly blinded_ms_correctness_proof:{
        c: string;
        v_dash_cap: string;
        m_caps: Record<string, string>;
        r_caps: Record<string, any>; // supposed to be empty
      } | undefined
    readonly nonce: string;
    readonly entropy: string;
}
"#;

#[wasm_bindgen(skip_typescript, inspectable)]
pub struct CredentialRequest {
    pub(crate) _request: AnoncredsCredentialRequest
}

#[wasm_bindgen]
impl CredentialRequest {

    #[wasm_bindgen(js_name = from)]
    pub fn from(request: JsValue) -> Result<CredentialRequest, JsValue> {
        let request:AnoncredsCredentialRequest = from_value(request)
            .map_err(|e| AnoncredsError::from(e))?;

        Ok(CredentialRequest {
            _request: request
        })
    }

    #[wasm_bindgen(getter)]
    pub fn nonce(&self) -> String {
        self._request.nonce.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn cred_def_id(&self) -> String {
        let value:JsValue = serde_wasm_bindgen::to_value(&self._request).unwrap();
        extract_property::<String>(&value, "cred_def_id").unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn entropy(&self) -> String {
        let value:JsValue = serde_wasm_bindgen::to_value(&self._request).unwrap();
        extract_property::<String>(&value, "entropy").unwrap()
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
