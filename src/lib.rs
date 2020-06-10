pub use warp_multipart_derive::*;

#[cfg(feature="json")]
mod json;

#[cfg(feature="json")]
pub use crate::json::*;

use async_trait::async_trait;
use bytes::Buf;
use std::string::FromUtf8Error;
use thiserror::Error;
use warp::filters::multipart::{Part, FormData};


pub mod derive_imports {
    pub use async_trait::async_trait;
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
    #[error("Expected non-empty data")]
    Empty,
    #[error("Error deserializing data")]
    Deserialize,
}

/// Types implementing this trait could be parsed from multipart data.
#[async_trait]
pub trait FromMultipart: Sized {
    async fn from_multipart(multipart: FormData) -> Result<Self, Error>;
}

/// How to parse part of multipart.
#[async_trait]
pub trait FromPart: Sized {
    async fn from_part(part: Part) -> Result<Self, Error>;
}

#[async_trait]
impl FromPart for String {
    async fn from_part(part: Part) -> Result<Self, Error> {
        Ok(String::from_utf8(get_data(part).await?)?)
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
        let res = T::from_part(part).await;
        match res {
            Ok(value) => Ok(Some(value)),
            Err(Error::Empty) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

pub(crate) async fn get_data(mut part: Part) -> Result<Vec<u8>, Error> {
    let mut data = part.data().await.ok_or(Error::NoData)??;
    if data.remaining() == 0 {
        return Err(Error::Empty);
    }
    let mut buffer = vec![0; data.remaining()];
    data.copy_to_slice(&mut buffer);
    Ok(buffer)
}