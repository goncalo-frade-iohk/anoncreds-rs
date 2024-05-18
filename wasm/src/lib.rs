#[cfg(debug_assertions)]
use log::{Level};

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


// Import the console log function from JavaScript
#[cfg(debug_assertions)]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(debug_assertions)]
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_log::init_with_level(Level::Trace).expect("error initializing log");
    console_error_panic_hook::set_once();
}
