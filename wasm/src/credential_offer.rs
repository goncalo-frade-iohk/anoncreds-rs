use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use anoncreds::data_types::cred_offer::CredentialOffer as AnoncredsCredentialOffer;
use crate::utils::fix_js_value;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type CredentialOfferType = {
    readonly cred_def_id: string;
    readonly schema_id: string;
    readonly key_correctness_proof: CredentialKeyCorrectnessProofType;
    readonly nonce: string;
    readonly method_name?: string;
}
export class CredentialOffer implements CredentialOfferType {
    free(): void;
    static from(definition: CredentialOfferType): CredentialOffer;
    readonly cred_def_id: string;
    readonly schema_id: string;
    readonly key_correctness_proof: CredentialKeyCorrectnessProofType;
    readonly nonce: string;
    readonly method_name?: string;
    toJSON(): CredentialOfferType
}
"#;

#[wasm_bindgen(skip_typescript)]
pub struct CredentialOffer {
    pub(crate) _offer: AnoncredsCredentialOffer
}

#[wasm_bindgen]
impl CredentialOffer {

    #[wasm_bindgen(js_name = from)]
    pub fn from(offer: &JsValue) -> Self {
        CredentialOffer {
            _offer: serde_wasm_bindgen::from_value(fix_js_value(offer.clone())).expect("Unable to deserialize Credential Offer")
        }
    }

    #[wasm_bindgen(js_name="toJSON")]
    pub fn to_json(&self) -> JsValue {
        fix_js_value(serde_wasm_bindgen::to_value(&self._offer).unwrap())
    }

    #[wasm_bindgen(getter)]
    pub fn schema_id(&self) -> String {
        self._offer.schema_id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn cred_def_id(&self) -> String {
        self._offer.cred_def_id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn nonce(&self) -> String {
        self._offer.nonce.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn key_correctness_proof(&self) -> JsValue {
       fix_js_value(serde_wasm_bindgen::to_value(&self._offer.key_correctness_proof).unwrap())
    }

}