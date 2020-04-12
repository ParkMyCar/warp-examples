#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeReq {
    #[prost(enumeration="TimeZone", tag="1")]
    pub zone: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeResp {
    #[prost(uint64, tag="1")]
    pub timestamp: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TimeZone {
    Utc = 0,
    Local = 1,
}
