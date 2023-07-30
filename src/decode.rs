use std::collections::HashMap;

use base64::{engine::general_purpose, Engine};

use crate::emoticons::{Emoticon, EmoticonJson};

/// Decodes a base64 encoded JSON string and returns the list of emoticons in the JSON
pub fn decode_data(data: &str) -> anyhow::Result<Vec<Emoticon>> {
    let mut buf = Vec::new();
    buf.resize(data.len() * 4 / 3 + 4, 0);

    let bytes_written = general_purpose::STANDARD.encode_slice(data, &mut buf)?;
    buf.truncate(bytes_written);

    let json: HashMap<String, EmoticonJson> = serde_json::from_slice(&buf)?;
    Ok(Emoticon::from_json_data(json))
}
