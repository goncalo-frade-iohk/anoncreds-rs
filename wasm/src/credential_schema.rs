use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use anoncreds::data_types::schema::{Schema as AnoncredsSchema, Schema};
use anoncreds::types::AttributeNames;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::credential_definition::CredentialDefinition;
use crate::credential_request::CredentialRequest;
use crate::error::{AnoncredsError, Errors};


#[wasm_bindgen(inspectable)]
#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialSchema {
    pub(crate) _schema: AnoncredsSchema
}


#[wasm_bindgen(inspectable)]
impl CredentialSchema {

    #[wasm_bindgen(constructor)]
    pub fn create_schema(
        schema_name: &str,
        schema_version:&str,
        issuer_id: &str,
        attr_names: Vec<String>
    ) -> Result<CredentialSchema, JsValue> {
        let schema_json = json!({
            "name": schema_name,
            "version": schema_version,
            "attrNames": attr_names,
            "issuerId": issuer_id
        });

        let schema: Schema = anoncreds::issuer::create_schema(
            schema_name,
            schema_version,
            issuer_id.to_string(),
            AttributeNames::from(attr_names)
        )
            .map_err(|e| AnoncredsError::from(e))?;


        Ok(
            CredentialSchema {
                _schema: schema
            }
        )
    }

    #[wasm_bindgen(js_name = from)]
    pub fn from(schema: JsValue) -> Result<CredentialSchema,JsValue > {
        let schema = serde_wasm_bindgen::from_value(schema)
            .map_err(|e| AnoncredsError::from(e))?;
        Ok(CredentialSchema {
            _schema: schema
        })
    }


    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self._schema.name.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn version(&self) -> String {
        self._schema.version.to_string()
    }

    #[wasm_bindgen(getter, js_name="issuerId")]
    pub fn issuer_id(&self) -> String {
        self._schema.issuer_id.to_string()
    }

    #[wasm_bindgen(getter, js_name="attrNames")]
    pub fn attr_names(&self) -> Vec<String> {
        self._schema.attr_names.0.clone()
    }

}
