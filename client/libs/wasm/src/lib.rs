use botwfddata::Payload;
use deku::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"
type CString = Uint8Array;
"#;

/// Convert bytes to JS object
#[wasm_bindgen(js_name = "parsePayload")]
pub fn parse_payload(bytes: &[u8]) -> Result<Payload, String> {
    let (_, value) = Payload::from_bytes((bytes, 0)).map_err(|e| e.to_string())?;
    Ok(value)
}