use crate::identifiers::cred_def::CredentialDefinitionId;
use crate::identifiers::rev_reg::RevocationRegistryId;
use crate::ursa::cl::RevocationKeyPublic;
use crate::utils::qualifier::Qualifiable;
use crate::utils::validation::{Validatable, ValidationError};

pub const CL_ACCUM: &str = "CL_ACCUM";

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct RevocationRegistryConfig {
    pub issuance_type: Option<IssuanceType>,
    pub max_cred_num: Option<u32>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum IssuanceType {
    ISSUANCE_BY_DEFAULT,
    ISSUANCE_ON_DEMAND,
}

impl IssuanceType {
    pub fn to_bool(&self) -> bool {
        self.clone() == IssuanceType::ISSUANCE_BY_DEFAULT
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum RegistryType {
    CL_ACCUM,
}

impl RegistryType {
    pub fn to_str(&self) -> &'static str {
        match *self {
            RegistryType::CL_ACCUM => CL_ACCUM,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct RevocationRegistryDefinitionValue {
    pub issuance_type: IssuanceType,
    pub max_cred_num: u32,
    pub public_keys: RevocationRegistryDefinitionValuePublicKeys,
    pub tails_hash: String,
    pub tails_location: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct RevocationRegistryDefinitionValuePublicKeys {
    pub accum_key: RevocationKeyPublic,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "ver"))]
pub enum RevocationRegistryDefinition {
    #[cfg_attr(feature = "serde", serde(rename = "1.0"))]
    RevocationRegistryDefinitionV1(RevocationRegistryDefinitionV1),
}

impl RevocationRegistryDefinition {
    pub fn to_unqualified(self) -> RevocationRegistryDefinition {
        match self {
            RevocationRegistryDefinition::RevocationRegistryDefinitionV1(v1) => {
                RevocationRegistryDefinition::RevocationRegistryDefinitionV1(
                    RevocationRegistryDefinitionV1 {
                        id: v1.id.to_unqualified(),
                        revoc_def_type: v1.revoc_def_type,
                        tag: v1.tag,
                        cred_def_id: v1.cred_def_id.to_unqualified(),
                        value: v1.value,
                    },
                )
            }
        }
    }
}

impl Validatable for RevocationRegistryDefinition {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            RevocationRegistryDefinition::RevocationRegistryDefinitionV1(v1) => {
                v1.id.validate()?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct RevocationRegistryDefinitionV1 {
    pub id: RevocationRegistryId,
    pub revoc_def_type: RegistryType,
    pub tag: String,
    pub cred_def_id: CredentialDefinitionId,
    pub value: RevocationRegistryDefinitionValue,
}
