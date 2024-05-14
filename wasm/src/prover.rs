
use wasm_bindgen::prelude::*;
use crate::credential_definition::CredentialDefinition;
use crate::credential_offer::CredentialOffer;
use crate::credential_request::{ CredentialRequest};
use crate::credential_request_metadata::CredentialRequestMetadata;
use crate::credential_request_response::CreateCredentialRequestResponse;
use crate::link_secret::LinkSecret;

#[wasm_bindgen]
pub struct Prover;


#[wasm_bindgen]
impl Prover {

    #[wasm_bindgen(static_method_of = Prover, js_name = createLinkSecret)]
    pub fn create_link_secret() -> LinkSecret {
        LinkSecret::new()
    }

    #[wasm_bindgen(static_method_of = Prover, js_name = createCredentialRequest)]
    pub fn create_credential_request(
        entropy: String,
        credential_definition: CredentialDefinition,
        link_secret: LinkSecret,
        link_secret_id: String,
        credential_offer: CredentialOffer
    ) -> CreateCredentialRequestResponse {
        let response = anoncreds::prover::create_credential_request(
            Some(&entropy),
            None,
            &credential_definition._definition,
            &link_secret._link_secret,
            &link_secret_id,
            &credential_offer._offer
        ).unwrap();
        let request = CredentialRequest {
            _request:response.0
        };
        let metadata = CredentialRequestMetadata {
            _metadata:response.1
        };

        CreateCredentialRequestResponse::new(request, metadata)
    }

}