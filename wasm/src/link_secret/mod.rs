
use wasm_bindgen::prelude::wasm_bindgen;
use anoncreds::data_types::link_secret::LinkSecret as AnoncredsLinkSecretType;
use anoncreds::prover;
use anoncreds::types::LinkSecret as AnoncredsLinkSecret;

#[wasm_bindgen]
pub struct LinkSecret {
    pub(crate) _link_secret : AnoncredsLinkSecret
}

#[wasm_bindgen]
impl LinkSecret {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        LinkSecret {
            _link_secret: prover::create_link_secret().expect("Unable to create link secret")
        }
    }

    #[wasm_bindgen(static_method_of = LinkSecret, js_name = fromString)]
    pub fn from_string(link_secret: String) -> Self {
        let linkstr: &str = link_secret.as_str();
        let link = AnoncredsLinkSecretType::try_from(linkstr).unwrap();
        LinkSecret {
            _link_secret:link
        }
    }

    #[wasm_bindgen( js_name = toString)]
    pub fn to_string(&self) -> String {
        let cloned = self._link_secret.try_clone().expect("Unable to instantiate create link secret");
        cloned.0.to_dec().expect("cannot convert to string")
    }
}
