use warp::Filter;
use warp::fs::dir;

#[tokio::main]
async fn main() {
    let dir = dir("./files");

    warp::serve(dir)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
