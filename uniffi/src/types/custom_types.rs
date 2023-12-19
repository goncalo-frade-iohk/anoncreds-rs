use anoncreds_core::data_types::schema::SchemaId;
use anoncreds_core::data_types::issuer_id::IssuerId;
use anoncreds_core::data_types::pres_request::{AttributeInfo, PredicateInfo, PredicateTypes};
use anoncreds_core::data_types::rev_reg::RevocationRegistryId;
use anoncreds_core::data_types::rev_reg_def::RevocationRegistryDefinitionId;
use anoncreds_core::data_types::cred_def::CredentialDefinitionId;
use anoncreds_core::types::{
    AttributeNames, 
    AttributeValues as AnoncredsAttributeValues
};
use anoncreds_core::data_types::credential::CredentialValues as AnoncredsCredentialValues;
use crate::{AnoncredsError, UniffiCustomTypeConverter};
use std::collections::HashMap;

/// Make sure [AttributeNames] implements [UniffiCustomTypeConverter] so that UniFFI can use it as
/// it is a Tuple Struct in Rust
impl UniffiCustomTypeConverter for AttributeNames {
    type Builtin = Vec<String>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(AttributeNames(val))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

/// Make sure [IssuerId] implements [UniffiCustomTypeConverter] so that UniFFI can use it as
/// it is Rust [macro_rules]
impl UniffiCustomTypeConverter for IssuerId {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(IssuerId(val))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

/// Make sure [SchemaId] implements [UniffiCustomTypeConverter] so that UniFFI can use it as
/// it is Rust [macro_rules]
impl UniffiCustomTypeConverter for SchemaId {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(SchemaId(val))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

/// Make sure [CredentialDefinitionId] implements [UniffiCustomTypeConverter] so that UniFFI can use it as
/// it is Rust [macro_rules]
impl UniffiCustomTypeConverter for CredentialDefinitionId {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(CredentialDefinitionId(val))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

/// Make sure [RevocationRegistryDefinitionId] implements [UniffiCustomTypeConverter] so that UniFFI can use it as
/// it is Rust [macro_rules]
impl UniffiCustomTypeConverter for RevocationRegistryDefinitionId {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(RevocationRegistryDefinitionId(val))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

/// Make sure [RevocationRegistryId] implements [UniffiCustomTypeConverter] so that UniFFI can use it as
/// it is a Tuple Struct in Rust
impl UniffiCustomTypeConverter for RevocationRegistryId {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(RevocationRegistryId(val))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

pub struct CredentialValues {
    pub values: HashMap<String, AttributeValues>
}

impl From<AnoncredsCredentialValues> for CredentialValues {
    fn from(acr: AnoncredsCredentialValues) -> Self {
        let mapped: HashMap<String, AttributeValues> = acr.0.iter()
            .map(|(k, v)| (k.clone(), v.clone().into()))
            .collect();
        return CredentialValues { values: mapped }
    }
}

impl From<CredentialValues> for AnoncredsCredentialValues {
    fn from(def: CredentialValues) -> AnoncredsCredentialValues {
        let mapped: HashMap<String, AnoncredsAttributeValues> = def.values.into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect();
        AnoncredsCredentialValues(mapped)
    }
}

pub struct AttributeValues {
    pub raw: String,
    pub encoded: String,
}

impl From<AnoncredsAttributeValues> for AttributeValues {
    fn from(acr: AnoncredsAttributeValues) -> Self {
        return AttributeValues { raw: acr.raw, encoded: acr.encoded }
    }
}

impl From<AttributeValues> for AnoncredsAttributeValues {
    fn from(def: AttributeValues) -> AnoncredsAttributeValues {
        AnoncredsAttributeValues { raw: def.raw, encoded: def.encoded }
    }
}

#[derive(Debug)]
pub struct AttributeInfoValue {
    pub core: AttributeInfo
}

impl AttributeInfoValue {
    pub fn new(json: String) -> Result<Self, AnoncredsError> {
        let core = serde_json::from_str(&json)
            .map_err(|err| AnoncredsError::ConversionError(err.to_string()))?;
        return Ok(AttributeInfoValue {
            core: core
        });
    }

    pub fn get_json(&self) -> Result<String, AnoncredsError> {
        return serde_json::to_string(&self.core)
            .map_err(|err| AnoncredsError::ConversionError(err.to_string()))
    }

    pub fn get_name(&self) -> String {
        return self.core.name.clone().unwrap()
    }

    pub fn get_names(&self) -> Vec<String> {
        return self.core.names.clone().unwrap()
    }
}

impl From<AttributeInfo> for AttributeInfoValue {
    fn from(value: AttributeInfo) -> AttributeInfoValue {
        return AttributeInfoValue {
            core: value
        };
    }
}

impl From<&AttributeInfo> for AttributeInfoValue {
    fn from(value: &AttributeInfo) -> AttributeInfoValue {
        return AttributeInfoValue {
            core: value.clone()
        };
    }
}

impl From<AttributeInfoValue> for AttributeInfo {
    fn from(value: AttributeInfoValue) -> Self {
        return value.core.clone();
    }
}

#[derive(Debug)]
pub struct PredicateInfoValue {
    pub core: PredicateInfo
}

impl PredicateInfoValue {
    pub fn new(json: String) -> Result<Self, AnoncredsError> {
        let core = serde_json::from_str(&json)
            .map_err(|err| AnoncredsError::ConversionError(err.to_string()))?;
        return Ok(PredicateInfoValue {
            core: core
        });
    }

    pub fn get_json(&self) -> Result<String, AnoncredsError> {
        return serde_json::to_string(&self.core)
            .map_err(|err| AnoncredsError::ConversionError(err.to_string()))
    }

    pub fn get_name(&self) -> String {
        return self.core.name.clone()
    }

    pub fn get_p_type(&self) -> PredicateTypes {
        return self.core.p_type.clone()
    }
}

impl From<PredicateInfo> for PredicateInfoValue {
    fn from(value: PredicateInfo) -> PredicateInfoValue {
        return PredicateInfoValue {
            core: value
        };
    }
}

impl From<&PredicateInfo> for PredicateInfoValue {
    fn from(value: &PredicateInfo) -> PredicateInfoValue {
        return PredicateInfoValue {
            core: value.clone()
        };
    }
}

impl From<PredicateInfoValue> for PredicateInfo {
    fn from(value: PredicateInfoValue) -> PredicateInfo {
        return value.core.clone();
    }
}
