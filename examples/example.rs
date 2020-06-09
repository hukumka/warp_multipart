use warp_multibody::{FromPart};
use warp::multipart::{FormData, Part};
use warp::{Filter, Reply, Rejection};

#[derive(FromPart, Debug)]
struct MultipartRequest {
    name: String,
    value: Option<String>,
    file: Part,
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
    let data = MultipartRequest::from_multipart(data).await
        .map_err(|_| warp::reject())?;

    println!("{}: {:?}", x, data);
    Ok(warp::reply::html(""))
}