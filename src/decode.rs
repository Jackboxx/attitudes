use crate::emoticons::Emoticon;

pub fn decode_data(data: &[u8]) -> anyhow::Result<Vec<Emoticon>> {
    let decoded: Vec<Emoticon> = bincode::deserialize(&data[..])?;
    Ok(decoded)
}
