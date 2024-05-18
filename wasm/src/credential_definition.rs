use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use anoncreds::data_types::cred_def::CredentialDefinitionPrivate as AnoncredsCredentialDefinitionPrivate;
use anoncreds::data_types::cred_def::{CredentialDefinition as AnoncredsCredentialDefinition};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value};
use ursa::bn::BigNumber;
use wasm_bindgen::__rt::IntoJsResult;
use crate::error::AnoncredsError;
use crate::utils::{extract_property, fix_js_value};

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type Tuple<T1 = string, T2 = string> = [T1, T2];
export type CredentialDefinitionTypePrimaryKey = {
    n: string,
    s: string,
    r: {
        [name:string]: string
    },
    rctxt: string,
    z: string
}
export type CredentialDefinitionType = {
    readonly schemaId: string;
    readonly type: string;
    readonly tag: string;
    readonly value: {
        primary: CredentialDefinitionTypePrimaryKey
    };
    readonly issuerId: string;
}
export class CredentialDefinition implements CredentialDefinitionType {
    free(): void;w
    static from(definition: CredentialDefinitionType): CredentialDefinition;
    readonly schemaId: string;
    readonly type: string;
    readonly tag: string;
    readonly value: {
        primary: CredentialDefinitionTypePrimaryKey
    };
    readonly issuerId: string;
    toJSON(): CredentialDefinitionType
}
export type CredentialDefinitionPrivateType = {
    readonly value : {
        p_key: {
            p: string,
            q: string
        }
    }
}
export class CredentialDefinitionPrivate implements CredentialDefinitionPrivateType {
    free(): void;
    static from(definition: CredentialDefinitionPrivateType): CredentialDefinitionPrivate;
    readonly value : {
        p_key: {
            p: string,
            q: string
        }
    }
    toJSON(): CredentialDefinitionPrivateType
}
export type CredentialKeyCorrectnessProofType = {
    readonly c: string;
    readonly xr_cap: Tuple[];
    readonly xz_cap: string;
}
export class CredentialKeyCorrectnessProof implements CredentialKeyCorrectnessProofType {
    free(): void;
    static from(definition: CredentialKeyCorrectnessProofType): CredentialKeyCorrectnessProof;
    readonly c: string;
    readonly xr_cap: Tuple[];
    readonly xz_cap: string;
    toJSON(): CredentialKeyCorrectnessProofType
}
"#;

#[wasm_bindgen(skip_typescript)]
#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialDefinition {
    pub(crate) _definition: AnoncredsCredentialDefinition
}


#[wasm_bindgen]
impl CredentialDefinition {

    pub(crate) fn new(value: AnoncredsCredentialDefinition) -> Self {
        CredentialDefinition {
            _definition: value
        }
    }

    #[wasm_bindgen(js_name = "from")]
    pub fn from(credential_definition: &JsValue) -> Result<CredentialDefinition, JsValue> {
        let definition: AnoncredsCredentialDefinition = from_value(fix_js_value(credential_definition.clone()))
            .map_err(|e| JsValue::from(AnoncredsError::from(e)))?;

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
        fix_js_value(serde_wasm_bindgen::to_value(&self._definition.value).unwrap())
    }

    #[wasm_bindgen(getter, js_name="issuerId")]
    pub fn issuer_id(&self) -> String {
        self._definition.issuer_id.to_string()
    }

    #[wasm_bindgen(js_name="toJSON")]
    pub fn to_json(&self) -> JsValue {
        fix_js_value(serde_wasm_bindgen::to_value(&self._definition).unwrap())
    }

}



#[wasm_bindgen(skip_typescript)]
#[derive(Debug, Deserialize, Serialize)]
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

    #[wasm_bindgen( js_name = from)]
    pub fn from(credential_definition_private: &JsValue) -> Result<CredentialDefinitionPrivate, JsValue> {
        let value:AnoncredsCredentialDefinitionPrivate = from_value(fix_js_value(credential_definition_private.clone()))
            .map_err(|e| JsValue::from(AnoncredsError::from(e)))?;


        Ok(CredentialDefinitionPrivate {
            _value: value
        })
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> JsValue {
        fix_js_value(serde_wasm_bindgen::to_value(&self._value.value).unwrap())
    }

    #[wasm_bindgen(js_name="toJSON")]
    pub fn to_json(&self) -> JsValue {
        fix_js_value(serde_wasm_bindgen::to_value(&self._value).unwrap())
    }
}

#[wasm_bindgen(skip_typescript)]
#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialKeyCorrectnessProof {
    pub(crate) _value: ursa::cl::CredentialKeyCorrectnessProof
}

#[wasm_bindgen]
impl CredentialKeyCorrectnessProof {

    #[wasm_bindgen( js_name = from)]
    pub fn from(key_correctness_proof: &JsValue) -> Result<CredentialKeyCorrectnessProof, JsValue> {
        let value:ursa::cl::CredentialKeyCorrectnessProof = from_value(fix_js_value(key_correctness_proof.clone()))
            .map_err(|e| JsValue::from(AnoncredsError::from(e)))?;
        Ok(CredentialKeyCorrectnessProof {
            _value: value
        })
    }

    fn get_key_correctness_proof(&self) -> Result<ursa::cl::CredentialKeyCorrectnessProof, JsValue> {
        let key_correctness = serde_wasm_bindgen::to_value(&self._value).map_err(|e| JsValue::from(AnoncredsError::from(e)))?;
        from_value(fix_js_value(key_correctness)).map_err(|e| JsValue::from(AnoncredsError::from(e)))
    }

    fn to_key_correctness_proof_js(&self, credential_key_correctness_proof: ursa::cl::CredentialKeyCorrectnessProof) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&credential_key_correctness_proof)
            .map_err(|e| JsValue::from(AnoncredsError::from(e)))
    }

    #[wasm_bindgen(getter)]
    pub fn c(&self) -> Result<JsValue, JsValue> {
        let credential_key_correctness_proof  = self.get_key_correctness_proof()?;
        let value = self.to_key_correctness_proof_js(credential_key_correctness_proof)?;
        Ok(
            fix_js_value(extract_property::<String>(&value, "c").unwrap().into_js_result().unwrap())
        )
    }

    #[wasm_bindgen(getter)]
    pub fn xz_cap(&self) -> Result<JsValue, JsValue> {
        let credential_key_correctness_proof  = self.get_key_correctness_proof()?;
        let value = self.to_key_correctness_proof_js(credential_key_correctness_proof)?;
        Ok(
            fix_js_value(extract_property::<String>(&value, "xz_cap").unwrap().into_js_result().unwrap())
        )
    }

    #[wasm_bindgen(getter)]
    pub fn xr_cap(&self) -> Result<JsValue, JsValue> {
        let credential_key_correctness_proof  = self.get_key_correctness_proof()?;
        let value = self.to_key_correctness_proof_js(credential_key_correctness_proof)?;
        let xr_cap : Vec<(String, String)> = extract_property(&value, "xr_cap").unwrap();
        Ok(
            fix_js_value(
                serde_wasm_bindgen::to_value(
                    &xr_cap
                        .into_iter()
                        .map(|(s, bn) | {
                            (s,BigNumber::from_dec(bn.as_str()).unwrap())
                        })
                        .collect::<Vec<(String, BigNumber)>>()
                ).unwrap()
            )
        )
    }

    #[wasm_bindgen(js_name="toJSON")]
    pub fn to_json(&self) -> JsValue {
        fix_js_value(serde_wasm_bindgen::to_value(&self._value).unwrap())
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
        CredentialDefinition::from(serde_wasm_bindgen::to_value(&self.credential_definition._definition).unwrap().as_ref()).unwrap()
    }

    #[wasm_bindgen(getter, js_name="credentialDefinitionPrivate")]
    pub fn credential_definition_private(&self) -> CredentialDefinitionPrivate  {
        CredentialDefinitionPrivate::from(serde_wasm_bindgen::to_value(&self.credential_definition_private._value).unwrap().as_ref()).unwrap()
    }

    #[wasm_bindgen(getter, js_name="keyCorrectnessProof")]
    pub fn key_correctness_proof(&self) -> CredentialKeyCorrectnessProof  {
        let js_value = serde_wasm_bindgen::to_value(&self.credential_key_correctness_proof._value).unwrap();
        let ursa_key_correctness_proof:ursa::cl::CredentialKeyCorrectnessProof  = from_value(fix_js_value(js_value)).unwrap();
        CredentialKeyCorrectnessProof{
            _value: ursa_key_correctness_proof
        }
    }

}