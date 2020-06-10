use warp::multipart::{FormData};
use warp::{Filter, Rejection, Reply};
use warp_multipart::{FromMultipart, JsonFile};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct File {
    a: u32,
    b: String,
}
#[derive(FromMultipart, Debug)]
struct MultipartRequest {
    name: String,
    #[default]
    value: Option<String>,
    file: JsonFile<File>,
}

#[tokio::main]
async fn main() {
    let promote = warp::post()
        .and(warp::path("request"))
        .and(warp::path::param::<u32>())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::multipart::form())
        .and_then(request);

    warp::serve(promote).run(([127, 0, 0, 1], 3030)).await
}

async fn request(x: u32, data: FormData) -> Result<impl Reply, Rejection> {
    let data = MultipartRequest::from_multipart(data)
        .await
        .map_err(|_| warp::reject())?;

    println!("{}: {:?}", x, data);
    Ok(warp::reply::html(""))
}
