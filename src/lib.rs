pub use warp_multibody_derive::*;

use async_trait::async_trait;
use warp::filters::multipart::{Part, FormData};
use bytes::Buf;
use thiserror::Error;
use std::string::FromUtf8Error;

pub mod derive_imports {
    pub use super::{Error, FromPart};
    pub use warp::filters::multipart::{Part, FormData};
    pub use futures::stream::StreamExt;
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
pub trait FromPart: Sized{
    async fn from_part(part: Part) -> Result<Self, Error>;
}

#[async_trait]
impl FromPart for String {
    async fn from_part(mut part: Part) -> Result<Self, Error> {
        let mut data = part.data().await
            .ok_or(Error::NoData)??;
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