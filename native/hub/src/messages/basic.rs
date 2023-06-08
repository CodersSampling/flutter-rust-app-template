#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SampleNumber {
    #[prost(int32, tag = "1")]
    pub value: i32,
    #[prost(int32, tag = "2")]
    pub dummy_one: i32,
    #[prost(int32, tag = "3")]
    pub dummy_two: i32,
    #[prost(int32, repeated, tag = "4")]
    pub dummy_three: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SampleLetter {
    #[prost(string, tag = "1")]
    pub letter: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub dummy_one: i32,
    #[prost(int32, tag = "3")]
    pub dummy_two: i32,
    #[prost(int32, repeated, tag = "4")]
    pub dummy_three: ::prost::alloc::vec::Vec<i32>,
}
