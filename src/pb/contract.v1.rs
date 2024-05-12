// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub grt_approvals: ::prost::alloc::vec::Vec<GrtApproval>,
    #[prost(message, repeated, tag="2")]
    pub grt_minter_addeds: ::prost::alloc::vec::Vec<GrtMinterAdded>,
    #[prost(message, repeated, tag="3")]
    pub grt_minter_removeds: ::prost::alloc::vec::Vec<GrtMinterRemoved>,
    #[prost(message, repeated, tag="4")]
    pub grt_new_ownerships: ::prost::alloc::vec::Vec<GrtNewOwnership>,
    #[prost(message, repeated, tag="5")]
    pub grt_new_pending_ownerships: ::prost::alloc::vec::Vec<GrtNewPendingOwnership>,
    #[prost(message, repeated, tag="6")]
    pub grt_transfers: ::prost::alloc::vec::Vec<GrtTransfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrtApproval {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub spender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrtMinterAdded {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub account: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrtMinterRemoved {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub account: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrtNewOwnership {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub to: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrtNewPendingOwnership {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub to: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrtTransfer {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub value: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
