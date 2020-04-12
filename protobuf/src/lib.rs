use chrono::prelude::*;
use gen::query::{TimeReq, TimeResp, TimeZone};
use warp::Reply;

pub mod gen;

pub fn query_handler(req: TimeReq) -> impl Reply {
    let timestamp = match TimeZone::from_i32(req.zone).unwrap() {
        TimeZone::Local => Local::now().timestamp(),
        TimeZone::Utc => Utc::now().timestamp(),
    };

    let resp = TimeResp {
        timestamp,
    };
    warp_protobuf::reply::protobuf(&resp)
}