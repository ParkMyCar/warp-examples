use warp::Filter;

use protobuf::query_handler;

#[tokio::main]
async fn main() {
    let route = warp::get()
        .and(warp::path("what_time_is_it_mr_fox"))
        .and(warp::path::end())
        .and(warp_protobuf::body::protobuf())
        .map(query_handler);

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}