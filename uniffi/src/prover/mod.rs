use crate::error::AnoncredsError;
use crate::types::cred_def::CredentialDefinition;
use crate::types::cred_offer::CredentialOffer;
use crate::types::cred_req::{CredentialRequest, CredentialRequestMetadata};
use crate::types::link_secret::LinkSecret;
use crate::{Credential, Presentation};
use crate::{PresentationRequest, RevocationRegistryDefinition};
use anoncreds_core::prover;
use anoncreds_core::types::PresentCredentials as AnoncredsPresentCredentials;
use std::convert::TryInto;
use std::sync::Arc;

pub struct CreateCrendentialRequestResponse {
    pub request: Arc<CredentialRequest>,
    pub metadata: CredentialRequestMetadata,
}

pub struct Prover {}

impl Prover {
    pub fn new() -> Self {
        Prover {}
    }

    pub fn create_link_secret(&self) -> Arc<LinkSecret> {
        let secret = LinkSecret::new();
        Arc::new(secret)
    }

    pub fn create_credential_request(
        &self,
        entropy: Option<String>,
        prover_did: Option<String>,
        cred_def: Arc<CredentialDefinition>,
        link_secret: Arc<LinkSecret>,
        link_secret_id: String,
        credential_offer: Arc<CredentialOffer>,
    ) -> Result<CreateCrendentialRequestResponse, AnoncredsError> {
        let cred_def_clone = Arc::clone(&cred_def);
        let cred_def_inner = cred_def_clone.as_ref();
        let cred_def_core = cred_def_inner.clone().try_into()?;
        let link_secret_core = (*link_secret).clone().secret;
        let cred_offer_core = (*credential_offer).clone().core;

        let (request, metadata) = anoncreds_core::prover::create_credential_request(
            entropy.as_ref().map(|s| s.as_str()),
            prover_did.as_ref().map(|s| s.as_str()),
            &cred_def_core,
            &link_secret_core,
            link_secret_id.as_str(),
            &cred_offer_core,
        )
        .map_err(|err| AnoncredsError::CreateCrentialRequestError(format!("Error: {}", err)))?;

        return Ok(CreateCrendentialRequestResponse {
            request: Arc::new(CredentialRequest { core: request }),
            metadata: CredentialRequestMetadata::from(metadata),
        });
    }

    pub fn process_credential(
        &self,
        credential: Arc<Credential>,
        cred_request_metadata: CredentialRequestMetadata,
        link_secret: Arc<LinkSecret>,
        cred_def: Arc<CredentialDefinition>,
        rev_reg_def: Option<Arc<RevocationRegistryDefinition>>,
    ) -> Result<(), AnoncredsError> {
        let mut mutable_credential = (*credential)
            .core
            .try_clone()
            .map_err(|_| AnoncredsError::ConversionError)?;
        prover::process_credential(
            &mut mutable_credential,
            &cred_request_metadata.into(),
            &(*link_secret).secret,
            &(*cred_def).core,
            rev_reg_def.as_ref().map(|def| &(*def).core),
        )
        .map_err(|err| AnoncredsError::ProcessCredential(format!("Error: {}", err)))
    }

    //TODO
    pub fn create_presentation(
        &self,
        presentation_request: Arc<PresentationRequest>,
        credentials: Arc<AnoncredsPresentCredentials>,
        //     self_attested: Option<HashMap<String, String>>,
        //     link_secret: &LinkSecret,
        //     schemas: &HashMap<&SchemaId, &Schema>,
        //     cred_defs: &HashMap<&CredentialDefinitionId, &CredentialDefinition>,
    ) -> Result<(), AnoncredsError> {
        // ) -> Result<Presentation, AnoncredsError> {
        // let ret = prover::create_presentation(
        //     pres_req: &PresentationRequest,
        //     credentials: PresentCredentials,
        //     self_attested: Option<HashMap<String, String>>,
        //     link_secret: &LinkSecret,
        //     schemas: &HashMap<&SchemaId, &Schema>,
        //     cred_defs: &HashMap<&CredentialDefinitionId, &CredentialDefinition>,
        // ).map_err(|err| AnoncredsError::CreatePresentationError(format!("Error: {}", err)))

        // return ret;
        return Ok(());
    }
}
