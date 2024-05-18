use std::collections::HashMap;
use anoncreds::data_types::nonce::Nonce;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use anoncreds::data_types::pres_request::{AttributeInfo, PredicateInfo, PresentationRequest as AnoncredsPresentationRequest, PresentationRequestPayload};
use serde::{Deserialize, Serialize};
use anoncreds::data_types::presentation::Presentation as AnoncredsPresentation;
use crate::error::AnoncredsError;
use crate::utils::fix_js_value;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type RequestedAttributes = Record<
    string,
    {
        name: string;
        restrictions: Record<string, string>;
    }
>
export type RequestedPredicates = Record<
    string,
    {
        name: string;
        p_type: string;
        p_value: any;
    }
>;
export type PresentationRequestType = {
    readonly nonce: string;
    readonly name: string;
    readonly version: string;
    readonly requested_attributes: RequestedAttributes;
    readonly requested_predicates: RequestedPredicates;
}
export class PresentationRequest implements PresentationRequestType {
    free(): void;
    static from(definition: PresentationRequestType): PresentationRequest;
    readonly nonce: string;
    readonly name: string;
    readonly version: string;
    readonly requested_attributes: RequestedAttributes;
    readonly requested_predicates: RequestedPredicates;
    toJSON(): PresentationRequestType
}
export type PresentationType = {
    readonly proof: {
        proofs: {
            primary_proof: {
                eq_proof: {
                    revealed_attrs: Record<string, string>;
                    a_prime: string;
                    e: string;
                    m: Record<string, string>;
                    m2: string;
                    v: string;
                };
                ge_proofs: {
                    mj: string;
                    alpha: string;
                    r: Record<string, string>;
                    t: Record<string, string>;
                    u: Record<string, string>;
                    predicate: Record<string, any>;
                }[];
            };
        }[];
        aggregated_proof: {
            c_hash: string;
            c_list: number[][];
        };
    };
    readonly requested_proof: {
        predicates: Record<
            string,
            {
                sub_proof_index: number;
            }
        >;
        revealed_attrs: Record<
            string,
            {
                encoded: string;
                raw: string;
                sub_proof_index: number;
            }
        >;
        self_attested_attrs: Record<
            string,
            {
                sub_proof_index: number;
            }
        >;
        unrevealed_attrs: Record<
            string,
            {
                sub_proof_index: number;
            }
        >;
    };
    readonly identifiers: {
        schema_id: string;
        cred_def_id: string;
    }[];
}
export class Presentation implements PresentationType {
    free(): void;
    static from(definition: PresentationType): Presentation;
    readonly proof: {
        proofs: {
            primary_proof: {
                eq_proof: {
                    revealed_attrs: Record<string, string>;
                    a_prime: string;
                    e: string;
                    m: Record<string, string>;
                    m2: string;
                    v: string;
                };
                ge_proofs: {
                    mj: string;
                    alpha: string;
                    r: Record<string, string>;
                    t: Record<string, string>;
                    u: Record<string, string>;
                    predicate: Record<string, any>;
                }[];
            };
        }[];
        aggregated_proof: {
            c_hash: string;
            c_list: number[][];
        };
    };
    readonly requested_proof: {
        predicates: Record<
            string,
            {
                sub_proof_index: number;
            }
        >;
        revealed_attrs: Record<
            string,
            {
                encoded: string;
                raw: string;
                sub_proof_index: number;
            }
        >;
        self_attested_attrs: Record<
            string,
            {
                sub_proof_index: number;
            }
        >;
        unrevealed_attrs: Record<
            string,
            {
                sub_proof_index: number;
            }
        >;
    };
    readonly identifiers: {
        schema_id: string;
        cred_def_id: string;
    }[];
    toJSON(): PresentationType
}
"#;

#[wasm_bindgen(skip_typescript)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Presentation {
    pub(crate) _presentation: AnoncredsPresentation
}

#[wasm_bindgen]
impl Presentation {

    #[wasm_bindgen(js_name="toJSON")]
    pub fn to_json(&self) -> JsValue {
        fix_js_value(serde_wasm_bindgen::to_value(&self._presentation).unwrap())
    }

    #[wasm_bindgen(js_name = from)]
    pub fn from(presentation: JsValue) -> Result<Presentation, JsValue> {
        serde_wasm_bindgen::Serializer::json_compatible();
        let anoncreds_presentation: AnoncredsPresentation = serde_wasm_bindgen::from_value(fix_js_value(presentation))
            .map_err(|e| JsValue::from(AnoncredsError::from(e)))?;

        Ok(Presentation {
            _presentation: anoncreds_presentation
        })
    }

    #[wasm_bindgen(getter)]
    pub fn proof(&self) -> JsValue {
        fix_js_value(serde_wasm_bindgen::to_value(&self._presentation.proof).unwrap())
    }

    #[wasm_bindgen(getter)]
    pub fn requested_proof(&self) -> JsValue {
        fix_js_value(serde_wasm_bindgen::to_value(&self._presentation.requested_proof).unwrap())
    }

    #[wasm_bindgen(getter)]
    pub fn identifiers(&self) -> JsValue {
        fix_js_value(serde_wasm_bindgen::to_value(&self._presentation.identifiers).unwrap())
    }

}

#[wasm_bindgen(skip_typescript)]
#[derive(Debug, Deserialize, Serialize)]
pub struct PresentationRequest {
    pub(crate) _presentation_request: AnoncredsPresentationRequest
}

#[wasm_bindgen]
impl PresentationRequest {


    #[wasm_bindgen(js_name="toJSON")]
    pub fn to_json(&self) -> JsValue {
        fix_js_value(serde_wasm_bindgen::to_value(&self._presentation_request.value()).unwrap())
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        name: String,
        version: String,
        requested_attributes: &JsValue,
        requested_predicates: &JsValue,
    ) -> Result<PresentationRequest, JsValue> {

        let attributes: HashMap<String, AttributeInfo> = serde_wasm_bindgen::from_value(requested_attributes.clone())
            .map_err(|e| JsValue::from(AnoncredsError::from(e)))?;

        let predicates: HashMap<String, PredicateInfo> = serde_wasm_bindgen::from_value(requested_predicates.clone())
            .map_err(|e| JsValue::from(AnoncredsError::from(e)))?;

        let payload = PresentationRequestPayload {
            name,
            nonce: Nonce::new().unwrap(),
            version,
            requested_attributes: attributes,
            requested_predicates: predicates,
            non_revoked: None,
        };

        let presentation_request = AnoncredsPresentationRequest::PresentationRequestV2(payload);

        Ok(
            PresentationRequest {
                _presentation_request:presentation_request
            }
        )

    }

    #[wasm_bindgen(js_name = "from")]
    pub fn from(presentation_request: &JsValue) -> Result<PresentationRequest, JsValue> {
        let anoncreds_presentation_request: PresentationRequestPayload = serde_wasm_bindgen::from_value(fix_js_value(presentation_request.clone()))
            .map_err(|e| JsValue::from(AnoncredsError::from(e)))?;

        Ok(PresentationRequest {
            _presentation_request: AnoncredsPresentationRequest::PresentationRequestV2(anoncreds_presentation_request)
        })
    }

    #[wasm_bindgen(getter)]
    pub fn nonce(&self) -> String {
       self._presentation_request.value().nonce.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self._presentation_request.value().name.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn version(&self) -> String {
        self._presentation_request.value().version.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn requested_attributes(&self) -> JsValue {
        let all = self._presentation_request.value();
        fix_js_value(serde_wasm_bindgen::to_value(&all.requested_attributes).unwrap())
    }

    #[wasm_bindgen(getter)]
    pub fn requested_predicates(&self) -> JsValue {
        let all = self._presentation_request.value();
        fix_js_value(serde_wasm_bindgen::to_value(&all.requested_predicates).unwrap())
    }
}