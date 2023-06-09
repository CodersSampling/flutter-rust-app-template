#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DummySchema {
    #[prost(bool, tag = "1")]
    pub some: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OtherDummySchema {
    #[prost(bool, tag = "1")]
    pub some: bool,
}
