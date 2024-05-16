use log::{Level, trace};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

// Import the console log function from JavaScript
#[wasm_bindgen]
#[cfg(debug_assertions)]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[wasm_bindgen(start)]
#[cfg(debug_assertions)]
pub fn main_js() -> Result<(), JsValue> {
    console_log::init_with_level(Level::Trace).expect("error initializing log");
    console_error_panic_hook::set_once();
    main()
}

#[cfg(debug_assertions)]
fn main() -> Result<(), JsValue> {
    Ok(())
}

mod credential_schema;
mod credential_request;
mod credential_request_metadata;
mod credential_request_response;
mod link_secret;
mod credential_definition;
mod credential_offer;
mod prover;
mod issuer;
mod credential;
mod utils;
mod presentation;
mod verifier;
mod error;




