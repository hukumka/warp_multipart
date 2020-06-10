use serde::Deserialize;
use super::{FromPart, Error, get_data};
use warp::multipart::Part;
use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonFile<T>(pub T);

#[async_trait]
impl<T: for<'a> Deserialize<'a>> FromPart for JsonFile<T> {
    async fn from_part(part: Part) -> Result<Self, Error> {
        let data = get_data(part).await?;
        let res = serde_json::from_slice(&data).map_err(|_| Error::Deserialize)?;
        Ok(JsonFile(res))
    }
}