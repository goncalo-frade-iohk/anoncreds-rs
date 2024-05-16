use std::collections::HashMap;
use anoncreds::data_types::nonce::Nonce;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use anoncreds::data_types::pres_request::{AttributeInfo, PredicateInfo, PresentationRequest as AnoncredsPresentationRequest, PresentationRequestPayload};
use serde::{Deserialize, Serialize};
use anoncreds::data_types::presentation::Presentation as AnoncredsPresentation;
use crate::error::AnoncredsError;

#[wasm_bindgen(inspectable)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Presentation {
    pub(crate) _presentation: AnoncredsPresentation
}

#[wasm_bindgen]
impl Presentation {
    #[wasm_bindgen(static_method_of = Presentation, js_name = from)]
    pub fn from(presentation: JsValue) -> Result<Presentation, JsValue> {
        let anoncreds_presentation: AnoncredsPresentation = serde_wasm_bindgen::from_value(presentation)
            .map_err(|e| AnoncredsError::from(e))?;

        Ok(Presentation {
            _presentation: anoncreds_presentation
        })
    }

    #[wasm_bindgen(getter)]
    pub fn proof(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self._presentation.proof).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn requested_proof(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self._presentation.requested_proof).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn identifiers(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self._presentation.identifiers).unwrap()
    }

}

#[wasm_bindgen(inspectable)]
#[derive(Debug, Deserialize, Serialize)]
pub struct PresentationRequest {
    pub(crate) _presentation_request: AnoncredsPresentationRequest
}

#[wasm_bindgen]
impl PresentationRequest {

    #[wasm_bindgen(constructor)]
    pub fn new(
        name: String,
        version: String,
        requested_attributes: JsValue,
        requested_predicates: JsValue,
    ) -> Result<PresentationRequest, JsValue> {

        let attributes: HashMap<String, AttributeInfo> = serde_wasm_bindgen::from_value(requested_attributes)
            .map_err(|e| AnoncredsError::from(e))?;

        let predicates: HashMap<String, PredicateInfo> = serde_wasm_bindgen::from_value(requested_predicates)
            .map_err(|e| AnoncredsError::from(e))?;

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
    pub fn from(presentation_request: JsValue) -> Result<PresentationRequest, JsValue> {
        let anoncreds_presentation_request: PresentationRequestPayload = serde_wasm_bindgen::from_value(presentation_request)
            .map_err(|e| AnoncredsError::from(e))?;

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
        serde_wasm_bindgen::to_value(&all.requested_attributes).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn requested_predicates(&self) -> JsValue {
        let all = self._presentation_request.value();
        serde_wasm_bindgen::to_value(&all.requested_predicates).unwrap()
    }
}