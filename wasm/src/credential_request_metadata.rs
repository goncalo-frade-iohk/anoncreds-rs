use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use anoncreds::data_types::cred_request::CredentialRequestMetadata as AnoncredsCredentialRequestMetadata;
use crate::error::AnoncredsError;
use crate::utils::fix_js_value;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type CredentialRequestMetadataType = {
    readonly link_secret_blinding_data: {
        v_prime: string;
    };
    readonly link_secret_name: string;
    readonly nonce: string;
}
export class CredentialRequestMetadata implements CredentialRequestMetadataType {
    free(): void;
    static from(definition: CredentialRequestMetadataType): CredentialRequestMetadata;
    readonly link_secret_blinding_data: {
        v_prime: string;
    };
    readonly link_secret_name: string;
    readonly nonce: string;
    toJSON(): CredentialRequestMetadataType
}
"#;

#[wasm_bindgen(skip_typescript)]
pub struct CredentialRequestMetadata {
    pub(crate) _metadata: AnoncredsCredentialRequestMetadata
}

#[wasm_bindgen]
impl CredentialRequestMetadata {

    #[wasm_bindgen(js_name="toJSON")]
    pub fn to_json(&self) -> JsValue {
        fix_js_value(serde_wasm_bindgen::to_value(&self._metadata).unwrap())
    }

    #[wasm_bindgen(js_name = from)]
    pub fn from(request: &JsValue) -> Result<CredentialRequestMetadata, JsValue> {
        let metadata:AnoncredsCredentialRequestMetadata = serde_wasm_bindgen::from_value(fix_js_value(request.clone()))
            .map_err(|e| JsValue::from(AnoncredsError::from(e)))?;

        Ok(CredentialRequestMetadata {
            _metadata: metadata
        })
    }

    #[wasm_bindgen(getter)]
    pub fn link_secret_blinding_data(&self) -> JsValue {
        fix_js_value(serde_wasm_bindgen::to_value(&self._metadata.link_secret_blinding_data).unwrap())
    }

    #[wasm_bindgen(getter)]
    pub fn nonce(&self) -> String {
        self._metadata.nonce.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn link_secret_name(&self) -> String {
        self._metadata.link_secret_name.to_string()
    }

}

