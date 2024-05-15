// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfers {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub from_balance: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub to_balance: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
