#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(enumeration = "arg::Type", tag = "1")]
    pub r#type: i32,
    #[prost(bool, tag = "2")]
    pub is_array: bool,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `Arg`.
pub mod arg {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        I32 = 0,
        I64 = 1,
        U32 = 2,
        U64 = 3,
        F32 = 4,
        F64 = 5,
        String = 6,
        Bytes = 7,
        Bool = 8,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uint64Slice {
    #[prost(uint64, repeated, tag = "1")]
    pub slice: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringUint64Map {
    #[prost(string, repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, repeated, tag = "2")]
    pub vals: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringUint64SliceMap {
    #[prost(string, repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub vals: ::prost::alloc::vec::Vec<Uint64Slice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifiedIndex {
    #[prost(uint64, tag = "1")]
    pub index: u64,
    #[prost(bool, tag = "2")]
    pub valid: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifiedIndexSlice {
    #[prost(message, repeated, tag = "1")]
    pub slice: ::prost::alloc::vec::Vec<VerifiedIndex>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringVerifiedIndexMap {
    #[prost(string, repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub vals: ::prost::alloc::vec::Vec<VerifiedIndex>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringVerifiedIndexSliceMap {
    #[prost(string, repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub vals: ::prost::alloc::vec::Vec<VerifiedIndexSlice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringSlice {
    #[prost(string, repeated, tag = "1")]
    pub slice: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringStringSliceMap {
    #[prost(string, repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub vals: ::prost::alloc::vec::Vec<StringSlice>,
}
/// Inter-blockchain Transfer Protocol
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ibtp {
    /// ID of sending chain and sending service
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    /// ID of receiving chain and receiving service
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
    /// Index of inter-chain transaction
    #[prost(uint64, tag = "3")]
    pub index: u64,
    /// inter-chain transaction type
    #[prost(enumeration = "ibtp::Type", tag = "4")]
    pub r#type: i32,
    /// timeout height of inter-chain transaction on BitXHub
    #[prost(int64, tag = "5")]
    pub timeout_height: i64,
    /// Proof of inter-chain transactions
    #[prost(bytes = "vec", tag = "6")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// Encoded content used by inter-chain
    #[prost(bytes = "vec", tag = "7")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    /// info about other txs in the same group
    #[prost(message, optional, tag = "8")]
    pub group: ::core::option::Option<StringUint64Map>,
    /// Message version
    #[prost(string, tag = "9")]
    pub version: ::prost::alloc::string::String,
    /// Self-defined fields used by app-chain
    #[prost(bytes = "vec", tag = "10")]
    pub extra: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `IBTP`.
pub mod ibtp {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Interchain = 0,
        ReceiptSuccess = 1,
        ReceiptFailure = 2,
        Rollback = 3,
        ReceiptRollback = 4,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Category {
        Request = 0,
        Response = 1,
        Unknown = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payload {
    #[prost(bool, tag = "1")]
    pub encrypted: bool,
    #[prost(bytes = "vec", tag = "2")]
    pub content: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Content {
    #[prost(string, tag = "1")]
    pub func: ::prost::alloc::string::String,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Result {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IbtPs {
    #[prost(message, repeated, tag = "1")]
    pub ibtps: ::prost::alloc::vec::Vec<Ibtp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BxhTransaction {
    #[prost(bytes = "vec", tag = "1")]
    pub version: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "4")]
    pub timestamp: i64,
    #[prost(bytes = "vec", tag = "5")]
    pub transaction_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "6")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "7")]
    pub ibtp: ::core::option::Option<Ibtp>,
    #[prost(uint64, tag = "8")]
    pub nonce: u64,
    #[prost(string, tag = "9")]
    pub amount: ::prost::alloc::string::String,
    #[prost(uint32, tag = "10")]
    pub typ: u32,
    #[prost(bytes = "vec", tag = "11")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "12")]
    pub extra: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionData {
    #[prost(enumeration = "transaction_data::Type", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    #[prost(enumeration = "transaction_data::VmType", tag = "3")]
    pub vm_type: i32,
    #[prost(bytes = "vec", tag = "4")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub extra: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `TransactionData`.
pub mod transaction_data {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Normal = 0,
        Invoke = 1,
        Update = 2,
        Freeze = 3,
        Unfreeze = 4,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VmType {
        Bvm = 0,
        Xvm = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvokePayload {
    #[prost(string, tag = "1")]
    pub method: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub args: ::prost::alloc::vec::Vec<Arg>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionMeta {
    #[prost(bytes = "vec", tag = "1")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub block_height: u64,
    #[prost(uint64, tag = "3")]
    pub index: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrosschainTransactionExtra {
    /// to_id index
    #[prost(uint64, tag = "1")]
    pub index: u64,
    /// receipt status
    #[prost(bool, tag = "2")]
    pub status: bool,
    /// receipt result
    #[prost(bytes = "vec", tag = "3")]
    pub ret: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionTracingMeta {
    #[prost(bytes = "vec", tag = "1")]
    pub receipt_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub confirm_hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionSlice {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(message, repeated, tag = "2")]
    pub txs: ::prost::alloc::vec::Vec<BxhTransaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetExchangeInfo {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sender_on_src: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub receiver_on_src: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub asset_on_src: u64,
    #[prost(string, tag = "5")]
    pub sender_on_dst: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub receiver_on_dst: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    pub asset_on_dst: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionRecord {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(enumeration = "TransactionStatus", tag = "2")]
    pub status: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionStatus {
    Begin = 0,
    Success = 1,
    Failure = 2,
    Rollback = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssetExchangeStatus {
    Init = 0,
    Redeem = 1,
    Refund = 2,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Receipt {
    #[prost(bytes = "vec", tag = "1")]
    pub version: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub tx_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub ret: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "receipt::Status", tag = "4")]
    pub status: i32,
    #[prost(message, repeated, tag = "5")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    #[prost(uint64, tag = "6")]
    pub gas_used: u64,
    #[prost(message, repeated, tag = "7")]
    pub evm_logs: ::prost::alloc::vec::Vec<EvmLog>,
    #[prost(bytes = "vec", tag = "8")]
    pub bloom: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "9")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `Receipt`.
pub mod receipt {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Success = 0,
        Failed = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Receipts {
    #[prost(message, repeated, tag = "1")]
    pub receipts: ::prost::alloc::vec::Vec<Receipt>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// Transaction Hash
    #[prost(bytes = "vec", tag = "1")]
    pub tx_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Event Type: interchain, nodemgr
    #[prost(enumeration = "event::EventType", tag = "3")]
    pub event_type: i32,
}
/// Nested message and enum types in `Event`.
pub mod event {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventType {
        Other = 0,
        Interchain = 1,
        Nodemgr = 2,
        Wasm = 3,
        AuditProposal = 4,
        AuditAppchain = 5,
        AuditRule = 6,
        AuditService = 7,
        AuditNode = 8,
        AuditRole = 9,
        AuditInterchain = 10,
        AuditDapp = 11,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvmLog {
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub topics: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "4")]
    pub block_number: u64,
    #[prost(bytes = "vec", tag = "5")]
    pub tx_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "6")]
    pub tx_index: u64,
    #[prost(bytes = "vec", tag = "7")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "8")]
    pub index: u64,
    #[prost(bool, tag = "9")]
    pub removed: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditTxInfo {
    #[prost(message, optional, tag = "1")]
    pub tx: ::core::option::Option<BxhTransaction>,
    #[prost(message, optional, tag = "2")]
    pub rec: ::core::option::Option<Receipt>,
    #[prost(uint64, tag = "3")]
    pub block_height: u64,
    #[prost(map = "string, bytes", tag = "4")]
    pub related_chain_id_list:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
    #[prost(map = "string, bytes", tag = "5")]
    pub related_node_id_list:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditRelatedObjInfo {
    #[prost(bytes = "vec", tag = "1")]
    pub audit_obj: ::prost::alloc::vec::Vec<u8>,
    #[prost(map = "string, bytes", tag = "2")]
    pub related_chain_id_list:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
    #[prost(map = "string, bytes", tag = "3")]
    pub related_node_id_list:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditSubscriptionRequest {
    #[prost(enumeration = "audit_subscription_request::Type", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub audit_node_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub block_height: u64,
    #[prost(bytes = "vec", tag = "4")]
    pub extra: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `AuditSubscriptionRequest`.
pub mod audit_subscription_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        AuditNode = 0,
        All = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(message, optional, tag = "1")]
    pub block_header: ::core::option::Option<BlockHeader>,
    /// [(gogoproto.customtype) = "Transactions"]; // transaction set
    #[prost(bytes = "vec", tag = "2")]
    pub transactions: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub extra: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    #[prost(uint64, tag = "1")]
    pub number: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub state_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub tx_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub receipt_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub timeout_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "6")]
    pub parent_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "7")]
    pub timestamp: i64,
    #[prost(bytes = "vec", tag = "8")]
    pub version: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "9")]
    pub bloom: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HappyBlock {
    #[prost(message, optional, tag = "1")]
    pub block_header: ::core::option::Option<BlockHeader>,
    #[prost(message, repeated, tag = "2")]
    pub bxh_txs: ::prost::alloc::vec::Vec<BxhTransaction>,
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub eth_txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", tag = "4")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "6")]
    pub extra: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, repeated, tag = "7")]
    pub index: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainMeta {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub interchain_tx_count: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockHeaderRequest {
    #[prost(uint64, tag = "1")]
    pub begin: u64,
    #[prost(uint64, tag = "2")]
    pub end: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInterchainTxWrappersRequest {
    #[prost(uint64, tag = "1")]
    pub begin: u64,
    #[prost(uint64, tag = "2")]
    pub end: u64,
    #[prost(string, tag = "3")]
    pub pid: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionHashMsg {
    #[prost(string, tag = "1")]
    pub tx_hash: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockRequest {
    #[prost(enumeration = "get_block_request::Type", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Nested message and enum types in `GetBlockRequest`.
pub mod get_block_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Height = 0,
        Hash = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(enumeration = "request::Type", tag = "1")]
    pub r#type: i32,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        ChainStatus = 0,
        Network = 1,
        Validators = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignResponse {
    #[prost(map = "string, bytes", tag = "1")]
    pub sign:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksRequest {
    #[prost(uint64, tag = "1")]
    pub start: u64,
    #[prost(uint64, tag = "2")]
    pub end: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksResponse {
    #[prost(message, repeated, tag = "1")]
    pub blocks: ::prost::alloc::vec::Vec<Block>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHappyBlocksResponse {
    #[prost(message, repeated, tag = "1")]
    pub blocks: ::prost::alloc::vec::Vec<HappyBlock>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockHeadersRequest {
    #[prost(uint64, tag = "1")]
    pub start: u64,
    #[prost(uint64, tag = "2")]
    pub end: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockHeadersResponse {
    #[prost(message, repeated, tag = "1")]
    pub block_headers: ::prost::alloc::vec::Vec<BlockHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionRequest {
    #[prost(enumeration = "subscription_request::Type", tag = "1")]
    pub r#type: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub extra: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `SubscriptionRequest`.
pub mod subscription_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Block = 0,
        Event = 1,
        InterchainTx = 2,
        BlockHeader = 3,
        InterchainTxWrapper = 4,
        UnionInterchainTxWrapper = 5,
        EvmLog = 6,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMultiSignsRequest {
    #[prost(enumeration = "get_multi_signs_request::Type", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
}
/// Nested message and enum types in `GetMultiSignsRequest`.
pub mod get_multi_signs_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        AssetExchange = 0,
        Ibtp = 1,
        BlockHeader = 2,
        Burn = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionResponse {
    #[prost(message, optional, tag = "1")]
    pub tx: ::core::option::Option<BxhTransaction>,
    #[prost(message, optional, tag = "2")]
    pub tx_meta: ::core::option::Option<TransactionMeta>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifiedTx {
    #[prost(message, optional, tag = "1")]
    pub tx: ::core::option::Option<BxhTransaction>,
    #[prost(bool, tag = "2")]
    pub valid: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterchainTxWrapper {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub l2_roots: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "2")]
    pub transactions: ::prost::alloc::vec::Vec<VerifiedTx>,
    #[prost(uint64, tag = "3")]
    pub height: u64,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub timeout_l2_roots: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, repeated, tag = "5")]
    pub timeout_ibtps: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterchainTxWrappers {
    #[prost(message, repeated, tag = "1")]
    pub interchain_tx_wrappers: ::prost::alloc::vec::Vec<InterchainTxWrapper>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTpsRequest {
    #[prost(uint64, tag = "1")]
    pub begin: u64,
    #[prost(uint64, tag = "2")]
    pub end: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelVpNodeRequest {
    #[prost(string, tag = "1")]
    pub pid: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PierInfo {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub index: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub timeout: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckPierResponse {
    #[prost(enumeration = "check_pier_response::Status", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CheckPierResponse`.
pub mod check_pier_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        ErrorMaster = 0,
        HasMaster = 1,
        NoMaster = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartBeatRespones {
    #[prost(enumeration = "heart_beat_respones::Status", tag = "1")]
    pub status: i32,
}
/// Nested message and enum types in `HeartBeatRespones`.
pub mod heart_beat_respones {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Alive = 0,
        Dead = 1,
    }
}
#[doc = r" Generated client implementations."]
pub mod chain_broker_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ChainBrokerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ChainBrokerClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ChainBrokerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ChainBrokerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ChainBrokerClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn subscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscriptionRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Response>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/Subscribe");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn subscribe_audit_info(
            &mut self,
            request: impl tonic::IntoRequest<super::AuditSubscriptionRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Response>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/SubscribeAuditInfo");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_block_header(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockHeaderRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::BlockHeader>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/GetBlockHeader");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_interchain_tx_wrappers(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInterchainTxWrappersRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::InterchainTxWrappers>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/abi.ChainBroker/GetInterchainTxWrappers");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn send_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::BxhTransaction>,
        ) -> Result<tonic::Response<super::TransactionHashMsg>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/SendTransaction");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn send_view(
            &mut self,
            request: impl tonic::IntoRequest<super::BxhTransaction>,
        ) -> Result<tonic::Response<super::Receipt>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/SendView");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::TransactionHashMsg>,
        ) -> Result<tonic::Response<super::GetTransactionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/GetTransaction");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_receipt(
            &mut self,
            request: impl tonic::IntoRequest<super::TransactionHashMsg>,
        ) -> Result<tonic::Response<super::Receipt>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/GetReceipt");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_block(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockRequest>,
        ) -> Result<tonic::Response<super::Block>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/GetBlock");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_blocks(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlocksRequest>,
        ) -> Result<tonic::Response<super::GetBlocksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/GetBlocks");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_happy_blocks(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlocksRequest>,
        ) -> Result<tonic::Response<super::GetHappyBlocksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/GetHappyBlocks");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_block_headers(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockHeadersRequest>,
        ) -> Result<tonic::Response<super::GetBlockHeadersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/GetBlockHeaders");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_chain_meta(
            &mut self,
            request: impl tonic::IntoRequest<super::Request>,
        ) -> Result<tonic::Response<super::ChainMeta>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/GetChainMeta");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_info(
            &mut self,
            request: impl tonic::IntoRequest<super::Request>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/GetInfo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_account_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::Address>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/GetAccountBalance");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_multi_signs(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMultiSignsRequest>,
        ) -> Result<tonic::Response<super::SignResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/GetMultiSigns");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_tps(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTpsRequest>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/GetTPS");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_pending_nonce_by_account(
            &mut self,
            request: impl tonic::IntoRequest<super::Address>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/abi.ChainBroker/GetPendingNonceByAccount");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn check_master_pier(
            &mut self,
            request: impl tonic::IntoRequest<super::Address>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/CheckMasterPier");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_master_pier(
            &mut self,
            request: impl tonic::IntoRequest<super::PierInfo>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/SetMasterPier");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn heart_beat(
            &mut self,
            request: impl tonic::IntoRequest<super::PierInfo>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/HeartBeat");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_chain_id(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.ChainBroker/GetChainID");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod chain_broker_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ChainBrokerServer."]
    #[async_trait]
    pub trait ChainBroker: Send + Sync + 'static {
        #[doc = "Server streaming response type for the Subscribe method."]
        type SubscribeStream: futures_core::Stream<Item = Result<super::Response, tonic::Status>>
            + Send
            + 'static;
        async fn subscribe(
            &self,
            request: tonic::Request<super::SubscriptionRequest>,
        ) -> Result<tonic::Response<Self::SubscribeStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeAuditInfo method."]
        type SubscribeAuditInfoStream: futures_core::Stream<Item = Result<super::Response, tonic::Status>>
            + Send
            + 'static;
        async fn subscribe_audit_info(
            &self,
            request: tonic::Request<super::AuditSubscriptionRequest>,
        ) -> Result<tonic::Response<Self::SubscribeAuditInfoStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetBlockHeader method."]
        type GetBlockHeaderStream: futures_core::Stream<Item = Result<super::BlockHeader, tonic::Status>>
            + Send
            + 'static;
        async fn get_block_header(
            &self,
            request: tonic::Request<super::GetBlockHeaderRequest>,
        ) -> Result<tonic::Response<Self::GetBlockHeaderStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetInterchainTxWrappers method."]
        type GetInterchainTxWrappersStream: futures_core::Stream<Item = Result<super::InterchainTxWrappers, tonic::Status>>
            + Send
            + 'static;
        async fn get_interchain_tx_wrappers(
            &self,
            request: tonic::Request<super::GetInterchainTxWrappersRequest>,
        ) -> Result<tonic::Response<Self::GetInterchainTxWrappersStream>, tonic::Status>;
        async fn send_transaction(
            &self,
            request: tonic::Request<super::BxhTransaction>,
        ) -> Result<tonic::Response<super::TransactionHashMsg>, tonic::Status>;
        async fn send_view(
            &self,
            request: tonic::Request<super::BxhTransaction>,
        ) -> Result<tonic::Response<super::Receipt>, tonic::Status>;
        async fn get_transaction(
            &self,
            request: tonic::Request<super::TransactionHashMsg>,
        ) -> Result<tonic::Response<super::GetTransactionResponse>, tonic::Status>;
        async fn get_receipt(
            &self,
            request: tonic::Request<super::TransactionHashMsg>,
        ) -> Result<tonic::Response<super::Receipt>, tonic::Status>;
        async fn get_block(
            &self,
            request: tonic::Request<super::GetBlockRequest>,
        ) -> Result<tonic::Response<super::Block>, tonic::Status>;
        async fn get_blocks(
            &self,
            request: tonic::Request<super::GetBlocksRequest>,
        ) -> Result<tonic::Response<super::GetBlocksResponse>, tonic::Status>;
        async fn get_happy_blocks(
            &self,
            request: tonic::Request<super::GetBlocksRequest>,
        ) -> Result<tonic::Response<super::GetHappyBlocksResponse>, tonic::Status>;
        async fn get_block_headers(
            &self,
            request: tonic::Request<super::GetBlockHeadersRequest>,
        ) -> Result<tonic::Response<super::GetBlockHeadersResponse>, tonic::Status>;
        async fn get_chain_meta(
            &self,
            request: tonic::Request<super::Request>,
        ) -> Result<tonic::Response<super::ChainMeta>, tonic::Status>;
        async fn get_info(
            &self,
            request: tonic::Request<super::Request>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status>;
        async fn get_account_balance(
            &self,
            request: tonic::Request<super::Address>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status>;
        async fn get_multi_signs(
            &self,
            request: tonic::Request<super::GetMultiSignsRequest>,
        ) -> Result<tonic::Response<super::SignResponse>, tonic::Status>;
        async fn get_tps(
            &self,
            request: tonic::Request<super::GetTpsRequest>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status>;
        async fn get_pending_nonce_by_account(
            &self,
            request: tonic::Request<super::Address>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status>;
        async fn check_master_pier(
            &self,
            request: tonic::Request<super::Address>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status>;
        async fn set_master_pier(
            &self,
            request: tonic::Request<super::PierInfo>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status>;
        async fn heart_beat(
            &self,
            request: tonic::Request<super::PierInfo>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status>;
        async fn get_chain_id(
            &self,
            request: tonic::Request<super::Empty>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ChainBrokerServer<T: ChainBroker> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ChainBroker> ChainBrokerServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        #[doc = r" Enable decompressing requests with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.accept_compression_encodings.enable_gzip();
            self
        }
        #[doc = r" Compress responses with `gzip`, if the client supports it."]
        pub fn send_gzip(mut self) -> Self {
            self.send_compression_encodings.enable_gzip();
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ChainBrokerServer<T>
    where
        T: ChainBroker,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/abi.ChainBroker/Subscribe" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker>
                        tonic::server::ServerStreamingService<super::SubscriptionRequest>
                        for SubscribeSvc<T>
                    {
                        type Response = super::Response;
                        type ResponseStream = T::SubscribeStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscriptionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/SubscribeAuditInfo" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeAuditInfoSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker>
                        tonic::server::ServerStreamingService<super::AuditSubscriptionRequest>
                        for SubscribeAuditInfoSvc<T>
                    {
                        type Response = super::Response;
                        type ResponseStream = T::SubscribeAuditInfoStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuditSubscriptionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe_audit_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeAuditInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/GetBlockHeader" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockHeaderSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker>
                        tonic::server::ServerStreamingService<super::GetBlockHeaderRequest>
                        for GetBlockHeaderSvc<T>
                    {
                        type Response = super::BlockHeader;
                        type ResponseStream = T::GetBlockHeaderStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockHeaderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_block_header(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockHeaderSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/GetInterchainTxWrappers" => {
                    #[allow(non_camel_case_types)]
                    struct GetInterchainTxWrappersSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker>
                        tonic::server::ServerStreamingService<super::GetInterchainTxWrappersRequest>
                        for GetInterchainTxWrappersSvc<T>
                    {
                        type Response = super::InterchainTxWrappers;
                        type ResponseStream = T::GetInterchainTxWrappersStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetInterchainTxWrappersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_interchain_tx_wrappers(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetInterchainTxWrappersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/SendTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct SendTransactionSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker> tonic::server::UnaryService<super::BxhTransaction> for SendTransactionSvc<T> {
                        type Response = super::TransactionHashMsg;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BxhTransaction>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/SendView" => {
                    #[allow(non_camel_case_types)]
                    struct SendViewSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker> tonic::server::UnaryService<super::BxhTransaction> for SendViewSvc<T> {
                        type Response = super::Receipt;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BxhTransaction>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_view(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendViewSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/GetTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker> tonic::server::UnaryService<super::TransactionHashMsg>
                        for GetTransactionSvc<T>
                    {
                        type Response = super::GetTransactionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransactionHashMsg>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/GetReceipt" => {
                    #[allow(non_camel_case_types)]
                    struct GetReceiptSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker> tonic::server::UnaryService<super::TransactionHashMsg> for GetReceiptSvc<T> {
                        type Response = super::Receipt;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransactionHashMsg>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_receipt(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetReceiptSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/GetBlock" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker> tonic::server::UnaryService<super::GetBlockRequest> for GetBlockSvc<T> {
                        type Response = super::Block;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_block(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/GetBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlocksSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker> tonic::server::UnaryService<super::GetBlocksRequest> for GetBlocksSvc<T> {
                        type Response = super::GetBlocksResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlocksRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_blocks(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlocksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/GetHappyBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct GetHappyBlocksSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker> tonic::server::UnaryService<super::GetBlocksRequest> for GetHappyBlocksSvc<T> {
                        type Response = super::GetHappyBlocksResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlocksRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_happy_blocks(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetHappyBlocksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/GetBlockHeaders" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockHeadersSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker> tonic::server::UnaryService<super::GetBlockHeadersRequest>
                        for GetBlockHeadersSvc<T>
                    {
                        type Response = super::GetBlockHeadersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockHeadersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_block_headers(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockHeadersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/GetChainMeta" => {
                    #[allow(non_camel_case_types)]
                    struct GetChainMetaSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker> tonic::server::UnaryService<super::Request> for GetChainMetaSvc<T> {
                        type Response = super::ChainMeta;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Request>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_chain_meta(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetChainMetaSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/GetInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetInfoSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker> tonic::server::UnaryService<super::Request> for GetInfoSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Request>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/GetAccountBalance" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountBalanceSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker> tonic::server::UnaryService<super::Address> for GetAccountBalanceSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Address>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_account_balance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccountBalanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/GetMultiSigns" => {
                    #[allow(non_camel_case_types)]
                    struct GetMultiSignsSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker> tonic::server::UnaryService<super::GetMultiSignsRequest>
                        for GetMultiSignsSvc<T>
                    {
                        type Response = super::SignResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMultiSignsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_multi_signs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMultiSignsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/GetTPS" => {
                    #[allow(non_camel_case_types)]
                    struct GetTPSSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker> tonic::server::UnaryService<super::GetTpsRequest> for GetTPSSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTpsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_tps(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTPSSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/GetPendingNonceByAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetPendingNonceByAccountSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker> tonic::server::UnaryService<super::Address>
                        for GetPendingNonceByAccountSvc<T>
                    {
                        type Response = super::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Address>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_pending_nonce_by_account(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPendingNonceByAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/CheckMasterPier" => {
                    #[allow(non_camel_case_types)]
                    struct CheckMasterPierSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker> tonic::server::UnaryService<super::Address> for CheckMasterPierSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Address>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).check_master_pier(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CheckMasterPierSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/SetMasterPier" => {
                    #[allow(non_camel_case_types)]
                    struct SetMasterPierSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker> tonic::server::UnaryService<super::PierInfo> for SetMasterPierSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PierInfo>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_master_pier(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetMasterPierSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/HeartBeat" => {
                    #[allow(non_camel_case_types)]
                    struct HeartBeatSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker> tonic::server::UnaryService<super::PierInfo> for HeartBeatSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PierInfo>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).heart_beat(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HeartBeatSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.ChainBroker/GetChainID" => {
                    #[allow(non_camel_case_types)]
                    struct GetChainIDSvc<T: ChainBroker>(pub Arc<T>);
                    impl<T: ChainBroker> tonic::server::UnaryService<super::Empty> for GetChainIDSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Empty>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_chain_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetChainIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ChainBroker> Clone for ChainBrokerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ChainBroker> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ChainBroker> tonic::transport::NamedService for ChainBrokerServer<T> {
        const NAME: &'static str = "abi.ChainBroker";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterchainMetaS {
    #[prost(message, optional, tag = "1")]
    pub counter: ::core::option::Option<StringVerifiedIndexSliceMap>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub l2_roots: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "3")]
    pub timeout_counter: ::core::option::Option<StringStringSliceMap>,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub timeout_l2_roots: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterchainS {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub interchain_counter: ::core::option::Option<StringUint64Map>,
    #[prost(message, optional, tag = "3")]
    pub receipt_counter: ::core::option::Option<StringUint64Map>,
    #[prost(message, optional, tag = "4")]
    pub source_interchain_counter: ::core::option::Option<StringUint64Map>,
    #[prost(message, optional, tag = "5")]
    pub source_receipt_counter: ::core::option::Option<StringUint64Map>,
}
