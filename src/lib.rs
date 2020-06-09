pub use warp_multipart_derive::*;

use async_trait::async_trait;
use bytes::Buf;
use std::string::FromUtf8Error;
use thiserror::Error;
use warp::filters::multipart::Part;

pub mod derive_imports {
    pub use super::{Error, FromPart};
    pub use futures::stream::StreamExt;
    pub use warp::filters::multipart::{FormData, Part};
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Strings must have UTF-8 encoding.")]
    NotUtf8(#[from] FromUtf8Error),
    #[error("Part must contain data.")]
    NoData,
    #[error("Field {:?} is missing.", .0)]
    MissingField(String),
    #[error("Internal warp error.")]
    Internal(#[from] warp::Error),
}

#[async_trait]
pub trait FromPart: Sized {
    async fn from_part(part: Part) -> Result<Self, Error>;
}

#[async_trait]
impl FromPart for String {
    async fn from_part(mut part: Part) -> Result<Self, Error> {
        let mut data = part.data().await.ok_or(Error::NoData)??;
        let mut buffer = vec![0; data.remaining()];
        data.copy_to_slice(&mut buffer);
        Ok(String::from_utf8(buffer)?)
    }
}

#[async_trait]
impl FromPart for Part {
    async fn from_part(part: Part) -> Result<Self, Error> {
        Ok(part)
    }
}

#[async_trait]
impl<T: FromPart> FromPart for Option<T> {
    async fn from_part(part: Part) -> Result<Self, Error> {
        Ok(Some(T::from_part(part).await?))
    }
}
