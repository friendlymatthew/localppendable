use warp::fs::dir;
use tracing::{Level};
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() {

    fmt::Subscriber::builder()
        .with_max_level(Level::INFO)
        .with_writer(std::io::stdout)
        .fmt_fields(fmt::format::PrettyFields::new())
        .init();

    let dir = dir("./files");

    warp::serve(dir)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
