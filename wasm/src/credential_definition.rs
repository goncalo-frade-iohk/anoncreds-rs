use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use anoncreds::data_types::cred_def::CredentialDefinitionPrivate as AnoncredsCredentialDefinitionPrivate;
use anoncreds::data_types::cred_def::CredentialKeyCorrectnessProof as AnoncredsCredentialKeyCorrectnessProof;
use anoncreds::data_types::cred_def::{CredentialDefinition as AnoncredsCredentialDefinition};


#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export class CredentialDefinition {
    free(): void;
    static from(definition: any): CredentialDefinition;
    readonly schema_id: string;
    readonly type: string;
    readonly tag: string;
    readonly value: Record<string, any>;
    readonly issuer_id: string;
}
"#;

#[wasm_bindgen(skip_typescript)]
pub struct CredentialDefinition {
    pub(crate) _definition: AnoncredsCredentialDefinition
}

#[wasm_bindgen(skip_typescript)]
impl CredentialDefinition {

    pub(crate) fn new(value: AnoncredsCredentialDefinition) -> Self {
        CredentialDefinition {
            _definition: value
        }
    }

    #[wasm_bindgen(static_method_of = LinkSecret, js_name = from)]
    pub fn from(credential_definition: JsValue) -> Self {
        CredentialDefinition {
            _definition: serde_wasm_bindgen::from_value(credential_definition).expect("Unable to deserialise Credential Definition")
        }
    }

    #[wasm_bindgen(getter, js_name="schemaId")]
    pub fn schema_id(&self) -> String {
        self._definition.schema_id.to_string()
    }

    #[wasm_bindgen(getter, js_name="type")]
    pub fn signature_type(&self) -> String {
        serde_wasm_bindgen::to_value(&self._definition.signature_type).unwrap().as_string().unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn tag(&self) -> String {
        self._definition.tag.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> JsValue {
        self.value()
    }
}



#[wasm_bindgen]
pub struct CredentialDefinitionPrivate {
    pub(crate) _value: AnoncredsCredentialDefinitionPrivate
}

#[wasm_bindgen]
impl CredentialDefinitionPrivate {
    pub(crate) fn new(value: AnoncredsCredentialDefinitionPrivate) -> Self {
        CredentialDefinitionPrivate {
            _value: value
        }
    }
}

#[wasm_bindgen]
pub struct CredentialKeyCorrectnessProof {
    pub(crate) _value: AnoncredsCredentialKeyCorrectnessProof
}


#[wasm_bindgen]
impl CredentialKeyCorrectnessProof {

    pub(crate) fn new(value: AnoncredsCredentialKeyCorrectnessProof) -> Self {
        CredentialKeyCorrectnessProof {
            _value: value
        }
    }
}

#[wasm_bindgen]
pub struct CredentialDefinitionPrivateResponse {
    pub(crate) credential_definition: CredentialDefinition,
    pub(crate) credential_definition_private: CredentialDefinitionPrivate,
    pub(crate) credential_key_correctness_proof: CredentialKeyCorrectnessProof,
}

#[wasm_bindgen]
impl CredentialDefinitionPrivateResponse {
    pub(crate) fn new(
         credential_definition: CredentialDefinition,
         credential_definition_private: CredentialDefinitionPrivate,
         credential_key_correctness_proof: CredentialKeyCorrectnessProof,
    ) -> Self {
        CredentialDefinitionPrivateResponse {
            credential_definition,
            credential_definition_private,
            credential_key_correctness_proof
        }
    }
}