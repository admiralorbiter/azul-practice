mod version;

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use log;

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
}

#[derive(Serialize, Deserialize)]
struct VersionInfo {
    engine_version: String,
    state_version: u32,
    ruleset_id: String,
}

#[derive(Serialize, Deserialize)]
struct PingResponse {
    status: String,
}

/// Returns the engine version information as JSON string
#[wasm_bindgen]
pub fn get_version() -> String {
    let info = VersionInfo {
        engine_version: version::ENGINE_VERSION.to_string(),
        state_version: version::STATE_VERSION,
        ruleset_id: version::RULESET_ID.to_string(),
    };
    
    serde_json::to_string(&info).unwrap()
}

/// Ping function that returns a simple status response as JSON string
#[wasm_bindgen]
pub fn ping() -> String {
    log::info!("Ping called from WASM");
    
    let response = PingResponse {
        status: "ok".to_string(),
    };
    
    serde_json::to_string(&response).unwrap()
}
