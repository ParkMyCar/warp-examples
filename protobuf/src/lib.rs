use gen::query::{TimeReq, TimeResp};
use warp::Reply;

pub mod gen;

pub fn query_handler(_req: TimeReq) -> impl Reply {
    println!("We got a request!");

    let resp = TimeResp {
        timestamp: 0_u64,
    };

    warp_protobuf::reply::protobuf(&resp)
}