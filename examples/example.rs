use warp::{Filter, Reply};
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
        .and(warp_multipart::extract())
        .map(request);

    warp::serve(promote).run(([127, 0, 0, 1], 3030)).await
}

fn request(x: u32, data: MultipartRequest) -> impl Reply {
    println!("{}: {:?}", x, data);
    warp::reply::html("")
}
