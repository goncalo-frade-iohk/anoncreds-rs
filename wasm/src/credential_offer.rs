use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use anoncreds::data_types::cred_offer::CredentialOffer as AnoncredsCredentialOffer;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export class CredentialOffer {
    free(): void;
    static from(offer: any): CredentialOffer;
    readonly schema_id: string;
    readonly cred_def_id: string;
    key_correctness_proof: {
        c: string;
        xr_cap: [string, string];
        xz_cap: string;
    };
    nonce: string;
}
"#;

#[wasm_bindgen(skip_typescript)]
pub struct CredentialOffer {
    pub(crate) _offer: AnoncredsCredentialOffer
}

#[wasm_bindgen(skip_typescript)]
impl CredentialOffer {

    #[wasm_bindgen(js_name = from)]
    pub fn from(offer: JsValue) -> Self {
        CredentialOffer {
            _offer: serde_wasm_bindgen::from_value(offer).expect("Unable to deserialize Credential Offer")
        }
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
        serde_wasm_bindgen::to_value(&self._offer.key_correctness_proof).unwrap()
    }
}