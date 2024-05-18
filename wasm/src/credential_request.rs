use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use anoncreds::data_types::cred_request::CredentialRequest as AnoncredsCredentialRequest;
use serde_wasm_bindgen::from_value;
use crate::error::AnoncredsError;
use crate::utils::{extract_property, fix_js_value};

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type CredentialRequestType = {
    readonly cred_def_id: string;
    readonly blinded_ms: {
        u: string;
        hidden_attributes: string[];
        committed_attributes: Record<string, string>;
    };
    readonly blinded_ms_correctness_proof: {
        c: string;
        v_dash_cap: string;
        m_caps: Record<string, string>;
        r_caps: Record<string, string>;
    };
    readonly entropy: string;
    readonly nonce: string;
}
export class CredentialRequest implements CredentialRequestType {
    free(): void;
    static from(definition: CredentialRequestType): CredentialRequest;
    readonly cred_def_id: string;
    readonly blinded_ms: {
        u: string;
        hidden_attributes: string[];
        committed_attributes: Record<string, string>; // supposed to be empty
    };
    readonly blinded_ms_correctness_proof: {
        c: string;
        v_dash_cap: string;
        m_caps: Record<string, string>;
        r_caps: Record<string, string>;
    };
    readonly entropy: string;
    readonly nonce: string;
    toJSON(): CredentialRequestType
}
"#;

#[wasm_bindgen(skip_typescript)]
pub struct CredentialRequest {
    pub(crate) _request: AnoncredsCredentialRequest
}

#[wasm_bindgen]
impl CredentialRequest {

    #[wasm_bindgen(js_name = from)]
    pub fn from(request: &JsValue) -> Result<CredentialRequest, JsValue> {
        let request:AnoncredsCredentialRequest = from_value(fix_js_value(request.clone()))
            .map_err(|e| JsValue::from(AnoncredsError::from(e)))?;

        Ok(CredentialRequest {
            _request: request
        })
    }

    #[wasm_bindgen(js_name="toJSON")]
    pub fn to_json(&self) -> JsValue {
        fix_js_value(serde_wasm_bindgen::to_value(&self._request).unwrap())
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
        fix_js_value(serde_wasm_bindgen::to_value(&self._request.blinded_ms).unwrap())
    }

    #[wasm_bindgen(getter)]
    pub fn blinded_ms_correctness_proof(&self) -> JsValue {
       fix_js_value( serde_wasm_bindgen::to_value(&self._request.blinded_ms_correctness_proof).unwrap())
    }

}
