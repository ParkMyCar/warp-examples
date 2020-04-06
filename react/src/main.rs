use warp::Filter;

#[tokio::main]
async fn main() {
    // GET / -> index.html
    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./my-app/build/index.html"));

    //  GET /...
    // e.g. /favicon.ico -> favicon.ico
    // e.g. /static/js/main.chunk.js -> /static/js/main.chunk.js
    let assets = warp::get().and(warp::fs::dir("./my-app/build"));

    let routes = index.or(assets);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
