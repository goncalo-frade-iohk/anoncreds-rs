use std::array::IntoIter;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use anoncreds::data_types::cred_def::CredentialDefinitionPrivate as AnoncredsCredentialDefinitionPrivate;
use anoncreds::data_types::cred_def::{CredentialDefinition as AnoncredsCredentialDefinition};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use ursa::bn::BigNumber;
use wasm_bindgen::__rt::IntoJsResult;
use crate::error::AnoncredsError;
use crate::utils::extract_property;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export class CredentialDefinition {
    free(): void;
    static from(definition: any): CredentialDefinition;
    readonly schemaId: string;
    readonly type: string;
    readonly tag: string;
    readonly value: Record<string, any>;
    readonly issuerId: string;
    toJSON():any;
}
"#;

#[wasm_bindgen(skip_typescript, inspectable)]
#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialDefinition {
    pub(crate) _definition: AnoncredsCredentialDefinition
}


#[wasm_bindgen(skip_typescript, inspectable)]
impl CredentialDefinition {

    pub(crate) fn new(value: AnoncredsCredentialDefinition) -> Self {
        CredentialDefinition {
            _definition: value
        }
    }

    #[wasm_bindgen(static_method_of = CredentialDefinition, js_name = from)]
    pub fn from(credential_definition: JsValue) -> Result<CredentialDefinition, JsValue> {
        let definition: AnoncredsCredentialDefinition = from_value(credential_definition)
            .map_err(|e| AnoncredsError::from(e))?;

        Ok(CredentialDefinition {
            _definition: definition,
        })
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
        serde_wasm_bindgen::to_value(&self._definition.value).unwrap()
    }

    #[wasm_bindgen(getter, js_name="issuerId")]
    pub fn issuer_id(&self) -> String {
        self._definition.issuer_id.to_string()
    }

}



#[wasm_bindgen(inspectable)]
pub struct CredentialDefinitionPrivate {
    pub(crate) _value: AnoncredsCredentialDefinitionPrivate
}

#[wasm_bindgen(inspectable)]
impl CredentialDefinitionPrivate {
    pub(crate) fn new(value: AnoncredsCredentialDefinitionPrivate) -> Self {
        CredentialDefinitionPrivate {
            _value: value
        }
    }

    #[wasm_bindgen(static_method_of = CredentialDefinitionPrivate, js_name = from)]
    pub fn from(credential_definition_private: JsValue) -> Result<CredentialDefinitionPrivate, JsValue> {
        let value:AnoncredsCredentialDefinitionPrivate = from_value(credential_definition_private)
            .map_err(|e| AnoncredsError::from(e))?;


        Ok(CredentialDefinitionPrivate {
            _value: value
        })
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self._value.value).unwrap()
    }
}

#[wasm_bindgen(inspectable)]
pub struct CredentialKeyCorrectnessProof {
    pub(crate) _value: ursa::cl::CredentialKeyCorrectnessProof
}

#[wasm_bindgen(inspectable)]
impl CredentialKeyCorrectnessProof {

    pub(crate) fn new(value: ursa::cl::CredentialKeyCorrectnessProof) -> Self {
        CredentialKeyCorrectnessProof {
            _value: value
        }
    }

    #[wasm_bindgen(static_method_of = CredentialKeyCorrectnessProof, js_name = from)]
    pub fn from(key_correctness_proof: JsValue) -> Result<CredentialKeyCorrectnessProof, JsValue> {
        let value:ursa::cl::CredentialKeyCorrectnessProof = from_value(key_correctness_proof)
            .map_err(|e| AnoncredsError::from(e))?;
        Ok(CredentialKeyCorrectnessProof {
            _value: value
        })
    }

    #[wasm_bindgen(getter)]
    pub fn c(&self) -> JsValue {
        let credential_key_correctness_proof: ursa::cl::CredentialKeyCorrectnessProof = serde_wasm_bindgen::from_value(serde_wasm_bindgen::to_value(&self._value).unwrap()).unwrap();
        let value:JsValue = serde_wasm_bindgen::to_value(&credential_key_correctness_proof).unwrap();
        extract_property::<String>(&value, "c").unwrap().into_js_result().unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn xz_cap(&self) -> JsValue {
        let credential_key_correctness_proof: ursa::cl::CredentialKeyCorrectnessProof = from_value(serde_wasm_bindgen::to_value(&self._value).unwrap()).unwrap();
        let value:JsValue = serde_wasm_bindgen::to_value(&credential_key_correctness_proof).unwrap();
        extract_property::<String>(&value, "xz_cap").unwrap().into_js_result().unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn xr_cap(&self) -> JsValue {
        let credential_key_correctness_proof: ursa::cl::CredentialKeyCorrectnessProof = from_value(serde_wasm_bindgen::to_value(&self._value).unwrap()).unwrap();
        let value:JsValue = serde_wasm_bindgen::to_value(&credential_key_correctness_proof).unwrap();
        let xr_cap : Vec<(String, String)> = extract_property(&value, "xr_cap").unwrap();
        serde_wasm_bindgen::to_value(
                &xr_cap
                    .into_iter()
                    .map(|(s, bn) | {
                        (s,BigNumber::from_dec(bn.as_str()).unwrap())
                    })
                    .collect::<Vec<(String, BigNumber)>>()
        ).unwrap()

    }
}

#[wasm_bindgen(inspectable)]
pub struct CredentialDefinitionPrivateResponse {
    pub(crate) credential_definition: CredentialDefinition,
    pub(crate) credential_definition_private: CredentialDefinitionPrivate,
    pub(crate) credential_key_correctness_proof: CredentialKeyCorrectnessProof,
}

#[wasm_bindgen(inspectable)]
impl CredentialDefinitionPrivateResponse {

    pub fn new(
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

    #[wasm_bindgen(getter, js_name="credentialDefinition")]
    pub fn credential_definition(&self) -> CredentialDefinition  {
        CredentialDefinition::from(serde_wasm_bindgen::to_value(&self.credential_definition._definition).unwrap()).unwrap()
    }

    #[wasm_bindgen(getter, js_name="credentialDefinitionPrivate")]
    pub fn credential_definition_private(&self) -> CredentialDefinitionPrivate  {
        CredentialDefinitionPrivate::from(serde_wasm_bindgen::to_value(&self.credential_definition_private._value).unwrap()).unwrap()
    }

    #[wasm_bindgen(getter, js_name="keyCorrectnessProof")]
    pub fn key_correctness_proof(&self) -> CredentialKeyCorrectnessProof  {
        let ursa_key_correctness_proof:ursa::cl::CredentialKeyCorrectnessProof  = from_value(serde_wasm_bindgen::to_value(&self.credential_key_correctness_proof._value).unwrap()).unwrap();
        CredentialKeyCorrectnessProof{
            _value: ursa_key_correctness_proof
        }
    }
}