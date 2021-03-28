#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUnspentRequest {
    /// The minimum number of confirmations to be included.
    #[prost(int32, tag = "1")]
    pub min_confs: i32,
    /// The maximum number of confirmations to be included.
    #[prost(int32, tag = "2")]
    pub max_confs: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUnspentResponse {
    /// A list of utxos satisfying the specified number of confirmations.
    #[prost(message, repeated, tag = "1")]
    pub utxos: ::std::vec::Vec<super::lnrpc::Utxo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseOutputRequest {
    ///
    ///An ID of 32 random bytes that must be unique for each distinct application
    ///using this RPC which will be used to bound the output lease to.
    #[prost(bytes, tag = "1")]
    pub id: std::vec::Vec<u8>,
    /// The identifying outpoint of the output being leased.
    #[prost(message, optional, tag = "2")]
    pub outpoint: ::std::option::Option<super::lnrpc::OutPoint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseOutputResponse {
    ///
    ///The absolute expiration of the output lease represented as a unix timestamp.
    #[prost(uint64, tag = "1")]
    pub expiration: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseOutputRequest {
    /// The unique ID that was used to lock the output.
    #[prost(bytes, tag = "1")]
    pub id: std::vec::Vec<u8>,
    /// The identifying outpoint of the output being released.
    #[prost(message, optional, tag = "2")]
    pub outpoint: ::std::option::Option<super::lnrpc::OutPoint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseOutputResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyReq {
    ///
    ///Is the key finger print of the root pubkey that this request is targeting.
    ///This allows the WalletKit to possibly serve out keys for multiple HD chains
    ///via public derivation.
    #[prost(int32, tag = "1")]
    pub key_finger_print: i32,
    ///
    ///The target key family to derive a key from. In other contexts, this is
    ///known as the "account".
    #[prost(int32, tag = "2")]
    pub key_family: i32,
}
/// No fields, as we always give out a p2wkh address.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddrRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddrResponse {
    ///
    ///The address encoded using a bech32 format.
    #[prost(string, tag = "1")]
    pub addr: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    ///
    ///The raw serialized transaction.
    #[prost(bytes, tag = "1")]
    pub tx_hex: std::vec::Vec<u8>,
    ///
    ///An optional label to save with the transaction. Limited to 500 characters.
    #[prost(string, tag = "2")]
    pub label: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishResponse {
    ///
    ///If blank, then no error occurred and the transaction was successfully
    ///published. If not the empty string, then a string representation of the
    ///broadcast error.
    ///
    ///TODO(roasbeef): map to a proper enum type
    #[prost(string, tag = "1")]
    pub publish_error: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendOutputsRequest {
    ///
    ///The number of satoshis per kilo weight that should be used when crafting
    ///this transaction.
    #[prost(int64, tag = "1")]
    pub sat_per_kw: i64,
    ///
    ///A slice of the outputs that should be created in the transaction produced.
    #[prost(message, repeated, tag = "2")]
    pub outputs: ::std::vec::Vec<super::signrpc::TxOut>,
    /// An optional label for the transaction, limited to 500 characters.
    #[prost(string, tag = "3")]
    pub label: std::string::String,
    /// The minimum number of confirmations each one of your outputs used for
    /// the transaction must satisfy.
    #[prost(int32, tag = "4")]
    pub min_confs: i32,
    /// Whether unconfirmed outputs should be used as inputs for the transaction.
    #[prost(bool, tag = "5")]
    pub spend_unconfirmed: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendOutputsResponse {
    ///
    ///The serialized transaction sent out on the network.
    #[prost(bytes, tag = "1")]
    pub raw_tx: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateFeeRequest {
    ///
    ///The number of confirmations to shoot for when estimating the fee.
    #[prost(int32, tag = "1")]
    pub conf_target: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateFeeResponse {
    ///
    ///The amount of satoshis per kw that should be used in order to reach the
    ///confirmation target in the request.
    #[prost(int64, tag = "1")]
    pub sat_per_kw: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendingSweep {
    /// The outpoint of the output we're attempting to sweep.
    #[prost(message, optional, tag = "1")]
    pub outpoint: ::std::option::Option<super::lnrpc::OutPoint>,
    /// The witness type of the output we're attempting to sweep.
    #[prost(enumeration = "WitnessType", tag = "2")]
    pub witness_type: i32,
    /// The value of the output we're attempting to sweep.
    #[prost(uint32, tag = "3")]
    pub amount_sat: u32,
    ///
    ///The fee rate we'll use to sweep the output. The fee rate is only determined
    ///once a sweeping transaction for the output is created, so it's possible for
    ///this to be 0 before this.
    #[prost(uint32, tag = "4")]
    pub sat_per_byte: u32,
    /// The number of broadcast attempts we've made to sweep the output.
    #[prost(uint32, tag = "5")]
    pub broadcast_attempts: u32,
    ///
    ///The next height of the chain at which we'll attempt to broadcast the
    ///sweep transaction of the output.
    #[prost(uint32, tag = "6")]
    pub next_broadcast_height: u32,
    /// The requested confirmation target for this output.
    #[prost(uint32, tag = "8")]
    pub requested_conf_target: u32,
    /// The requested fee rate, expressed in sat/byte, for this output.
    #[prost(uint32, tag = "9")]
    pub requested_sat_per_byte: u32,
    ///
    ///Whether this input must be force-swept. This means that it is swept even
    ///if it has a negative yield.
    #[prost(bool, tag = "7")]
    pub force: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendingSweepsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendingSweepsResponse {
    ///
    ///The set of outputs currently being swept by lnd's central batching engine.
    #[prost(message, repeated, tag = "1")]
    pub pending_sweeps: ::std::vec::Vec<PendingSweep>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BumpFeeRequest {
    /// The input we're attempting to bump the fee of.
    #[prost(message, optional, tag = "1")]
    pub outpoint: ::std::option::Option<super::lnrpc::OutPoint>,
    /// The target number of blocks that the input should be spent within.
    #[prost(uint32, tag = "2")]
    pub target_conf: u32,
    ///
    ///The fee rate, expressed in sat/byte, that should be used to spend the input
    ///with.
    #[prost(uint32, tag = "3")]
    pub sat_per_byte: u32,
    ///
    ///Whether this input must be force-swept. This means that it is swept even
    ///if it has a negative yield.
    #[prost(bool, tag = "4")]
    pub force: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BumpFeeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSweepsRequest {
    ///
    ///Retrieve the full sweep transaction details. If false, only the sweep txids
    ///will be returned. Note that some sweeps that LND publishes will have been
    ///replaced-by-fee, so will not be included in this output.
    #[prost(bool, tag = "1")]
    pub verbose: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSweepsResponse {
    #[prost(oneof = "list_sweeps_response::Sweeps", tags = "1, 2")]
    pub sweeps: ::std::option::Option<list_sweeps_response::Sweeps>,
}
pub mod list_sweeps_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransactionIDs {
        ///
        ///Reversed, hex-encoded string representing the transaction ids of the
        ///sweeps that our node has broadcast. Note that these transactions may
        ///not have confirmed yet, we record sweeps on broadcast, not confirmation.
        #[prost(string, repeated, tag = "1")]
        pub transaction_ids: ::std::vec::Vec<std::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Sweeps {
        #[prost(message, tag = "1")]
        TransactionDetails(super::super::lnrpc::TransactionDetails),
        #[prost(message, tag = "2")]
        TransactionIds(TransactionIDs),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelTransactionRequest {
    /// The txid of the transaction to label.
    #[prost(bytes, tag = "1")]
    pub txid: std::vec::Vec<u8>,
    /// The label to add to the transaction, limited to 500 characters.
    #[prost(string, tag = "2")]
    pub label: std::string::String,
    /// Whether to overwrite the existing label, if it is present.
    #[prost(bool, tag = "3")]
    pub overwrite: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelTransactionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundPsbtRequest {
    #[prost(oneof = "fund_psbt_request::Template", tags = "1, 2")]
    pub template: ::std::option::Option<fund_psbt_request::Template>,
    #[prost(oneof = "fund_psbt_request::Fees", tags = "3, 4")]
    pub fees: ::std::option::Option<fund_psbt_request::Fees>,
}
pub mod fund_psbt_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Template {
        ///
        ///Use an existing PSBT packet as the template for the funded PSBT.
        ///
        ///The packet must contain at least one non-dust output. If one or more
        ///inputs are specified, no coin selection is performed. In that case every
        ///input must be an UTXO known to the wallet that has not been locked
        ///before. The sum of all inputs must be sufficiently greater than the sum
        ///of all outputs to pay a miner fee with the specified fee rate. A change
        ///output is added to the PSBT if necessary.
        #[prost(bytes, tag = "1")]
        Psbt(std::vec::Vec<u8>),
        ///
        ///Use the outputs and optional inputs from this raw template.
        #[prost(message, tag = "2")]
        Raw(super::TxTemplate),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Fees {
        ///
        ///The target number of blocks that the transaction should be confirmed in.
        #[prost(uint32, tag = "3")]
        TargetConf(u32),
        ///
        ///The fee rate, expressed in sat/vbyte, that should be used to spend the
        ///input with.
        #[prost(uint32, tag = "4")]
        SatPerVbyte(u32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundPsbtResponse {
    ///
    ///The funded but not yet signed PSBT packet.
    #[prost(bytes, tag = "1")]
    pub funded_psbt: std::vec::Vec<u8>,
    ///
    ///The index of the added change output or -1 if no change was left over.
    #[prost(int32, tag = "2")]
    pub change_output_index: i32,
    ///
    ///The list of lock leases that were acquired for the inputs in the funded PSBT
    ///packet.
    #[prost(message, repeated, tag = "3")]
    pub locked_utxos: ::std::vec::Vec<UtxoLease>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxTemplate {
    ///
    ///An optional list of inputs to use. Every input must be an UTXO known to the
    ///wallet that has not been locked before. The sum of all inputs must be
    ///sufficiently greater than the sum of all outputs to pay a miner fee with the
    ///fee rate specified in the parent message.
    ///
    ///If no inputs are specified, coin selection will be performed instead and
    ///inputs of sufficient value will be added to the resulting PSBT.
    #[prost(message, repeated, tag = "1")]
    pub inputs: ::std::vec::Vec<super::lnrpc::OutPoint>,
    ///
    ///A map of all addresses and the amounts to send to in the funded PSBT.
    #[prost(map = "string, uint64", tag = "2")]
    pub outputs: ::std::collections::HashMap<std::string::String, u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UtxoLease {
    ///
    ///A 32 byte random ID that identifies the lease.
    #[prost(bytes, tag = "1")]
    pub id: std::vec::Vec<u8>,
    /// The identifying outpoint of the output being leased.
    #[prost(message, optional, tag = "2")]
    pub outpoint: ::std::option::Option<super::lnrpc::OutPoint>,
    ///
    ///The absolute expiration of the output lease represented as a unix timestamp.
    #[prost(uint64, tag = "3")]
    pub expiration: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizePsbtRequest {
    ///
    ///A PSBT that should be signed and finalized. The PSBT must contain all
    ///required inputs, outputs, UTXO data and partial signatures of all other
    ///signers.
    #[prost(bytes, tag = "1")]
    pub funded_psbt: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizePsbtResponse {
    /// The fully signed and finalized transaction in PSBT format.
    #[prost(bytes, tag = "1")]
    pub signed_psbt: std::vec::Vec<u8>,
    /// The fully signed and finalized transaction in the raw wire format.
    #[prost(bytes, tag = "2")]
    pub raw_final_tx: std::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WitnessType {
    UnknownWitness = 0,
    ///
    ///A witness that allows us to spend the output of a commitment transaction
    ///after a relative lock-time lockout.
    CommitmentTimeLock = 1,
    ///
    ///A witness that allows us to spend a settled no-delay output immediately on a
    ///counterparty's commitment transaction.
    CommitmentNoDelay = 2,
    ///
    ///A witness that allows us to sweep the settled output of a malicious
    ///counterparty's who broadcasts a revoked commitment transaction.
    CommitmentRevoke = 3,
    ///
    ///A witness that allows us to sweep an HTLC which we offered to the remote
    ///party in the case that they broadcast a revoked commitment state.
    HtlcOfferedRevoke = 4,
    ///
    ///A witness that allows us to sweep an HTLC output sent to us in the case that
    ///the remote party broadcasts a revoked commitment state.
    HtlcAcceptedRevoke = 5,
    ///
    ///A witness that allows us to sweep an HTLC output that we extended to a
    ///party, but was never fulfilled.  This HTLC output isn't directly on the
    ///commitment transaction, but is the result of a confirmed second-level HTLC
    ///transaction. As a result, we can only spend this after a CSV delay.
    HtlcOfferedTimeoutSecondLevel = 6,
    ///
    ///A witness that allows us to sweep an HTLC output that was offered to us, and
    ///for which we have a payment preimage. This HTLC output isn't directly on our
    ///commitment transaction, but is the result of confirmed second-level HTLC
    ///transaction. As a result, we can only spend this after a CSV delay.
    HtlcAcceptedSuccessSecondLevel = 7,
    ///
    ///A witness that allows us to sweep an HTLC that we offered to the remote
    ///party which lies in the commitment transaction of the remote party. We can
    ///spend this output after the absolute CLTV timeout of the HTLC as passed.
    HtlcOfferedRemoteTimeout = 8,
    ///
    ///A witness that allows us to sweep an HTLC that was offered to us by the
    ///remote party. We use this witness in the case that the remote party goes to
    ///chain, and we know the pre-image to the HTLC. We can sweep this without any
    ///additional timeout.
    HtlcAcceptedRemoteSuccess = 9,
    ///
    ///A witness that allows us to sweep an HTLC from the remote party's commitment
    ///transaction in the case that the broadcast a revoked commitment, but then
    ///also immediately attempt to go to the second level to claim the HTLC.
    HtlcSecondLevelRevoke = 10,
    ///
    ///A witness type that allows us to spend a regular p2wkh output that's sent to
    ///an output which is under complete control of the backing wallet.
    WitnessKeyHash = 11,
    ///
    ///A witness type that allows us to sweep an output that sends to a nested P2SH
    ///script that pays to a key solely under our control.
    NestedWitnessKeyHash = 12,
    ///
    ///A witness type that allows us to spend our anchor on the commitment
    ///transaction.
    CommitmentAnchor = 13,
}
#[doc = r" Generated client implementations."]
pub mod wallet_kit_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " WalletKit is a service that gives access to the core functionalities of the"]
    #[doc = " daemon's wallet."]
    pub struct WalletKitClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WalletKitClient<tonic::transport::Channel> {
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
    impl<T> WalletKitClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = ""]
        #[doc = "ListUnspent returns a list of all utxos spendable by the wallet with a"]
        #[doc = "number of confirmations between the specified minimum and maximum."]
        pub async fn list_unspent(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUnspentRequest>,
        ) -> Result<tonic::Response<super::ListUnspentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/ListUnspent");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "LeaseOutput locks an output to the given ID, preventing it from being"]
        #[doc = "available for any future coin selection attempts. The absolute time of the"]
        #[doc = "lock's expiration is returned. The expiration of the lock can be extended by"]
        #[doc = "successive invocations of this RPC. Outputs can be unlocked before their"]
        #[doc = "expiration through `ReleaseOutput`."]
        pub async fn lease_output(
            &mut self,
            request: impl tonic::IntoRequest<super::LeaseOutputRequest>,
        ) -> Result<tonic::Response<super::LeaseOutputResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/LeaseOutput");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "ReleaseOutput unlocks an output, allowing it to be available for coin"]
        #[doc = "selection if it remains unspent. The ID should match the one used to"]
        #[doc = "originally lock the output."]
        pub async fn release_output(
            &mut self,
            request: impl tonic::IntoRequest<super::ReleaseOutputRequest>,
        ) -> Result<tonic::Response<super::ReleaseOutputResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/ReleaseOutput");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "DeriveNextKey attempts to derive the *next* key within the key family"]
        #[doc = "(account in BIP43) specified. This method should return the next external"]
        #[doc = "child within this branch."]
        pub async fn derive_next_key(
            &mut self,
            request: impl tonic::IntoRequest<super::KeyReq>,
        ) -> Result<tonic::Response<super::super::signrpc::KeyDescriptor>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/DeriveNextKey");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "DeriveKey attempts to derive an arbitrary key specified by the passed"]
        #[doc = "KeyLocator."]
        pub async fn derive_key(
            &mut self,
            request: impl tonic::IntoRequest<super::super::signrpc::KeyLocator>,
        ) -> Result<tonic::Response<super::super::signrpc::KeyDescriptor>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/DeriveKey");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "NextAddr returns the next unused address within the wallet."]
        pub async fn next_addr(
            &mut self,
            request: impl tonic::IntoRequest<super::AddrRequest>,
        ) -> Result<tonic::Response<super::AddrResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/NextAddr");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "PublishTransaction attempts to publish the passed transaction to the"]
        #[doc = "network. Once this returns without an error, the wallet will continually"]
        #[doc = "attempt to re-broadcast the transaction on start up, until it enters the"]
        #[doc = "chain."]
        pub async fn publish_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::Transaction>,
        ) -> Result<tonic::Response<super::PublishResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/PublishTransaction");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "SendOutputs is similar to the existing sendmany call in Bitcoind, and"]
        #[doc = "allows the caller to create a transaction that sends to several outputs at"]
        #[doc = "once. This is ideal when wanting to batch create a set of transactions."]
        pub async fn send_outputs(
            &mut self,
            request: impl tonic::IntoRequest<super::SendOutputsRequest>,
        ) -> Result<tonic::Response<super::SendOutputsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/SendOutputs");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "EstimateFee attempts to query the internal fee estimator of the wallet to"]
        #[doc = "determine the fee (in sat/kw) to attach to a transaction in order to"]
        #[doc = "achieve the confirmation target."]
        pub async fn estimate_fee(
            &mut self,
            request: impl tonic::IntoRequest<super::EstimateFeeRequest>,
        ) -> Result<tonic::Response<super::EstimateFeeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/EstimateFee");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "PendingSweeps returns lists of on-chain outputs that lnd is currently"]
        #[doc = "attempting to sweep within its central batching engine. Outputs with similar"]
        #[doc = "fee rates are batched together in order to sweep them within a single"]
        #[doc = "transaction."]
        #[doc = ""]
        #[doc = "NOTE: Some of the fields within PendingSweepsRequest are not guaranteed to"]
        #[doc = "remain supported. This is an advanced API that depends on the internals of"]
        #[doc = "the UtxoSweeper, so things may change."]
        pub async fn pending_sweeps(
            &mut self,
            request: impl tonic::IntoRequest<super::PendingSweepsRequest>,
        ) -> Result<tonic::Response<super::PendingSweepsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/PendingSweeps");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "BumpFee bumps the fee of an arbitrary input within a transaction. This RPC"]
        #[doc = "takes a different approach than bitcoind's bumpfee command. lnd has a"]
        #[doc = "central batching engine in which inputs with similar fee rates are batched"]
        #[doc = "together to save on transaction fees. Due to this, we cannot rely on"]
        #[doc = "bumping the fee on a specific transaction, since transactions can change at"]
        #[doc = "any point with the addition of new inputs. The list of inputs that"]
        #[doc = "currently exist within lnd's central batching engine can be retrieved"]
        #[doc = "through the PendingSweeps RPC."]
        #[doc = ""]
        #[doc = "When bumping the fee of an input that currently exists within lnd's central"]
        #[doc = "batching engine, a higher fee transaction will be created that replaces the"]
        #[doc = "lower fee transaction through the Replace-By-Fee (RBF) policy. If it"]
        #[doc = ""]
        #[doc = "This RPC also serves useful when wanting to perform a Child-Pays-For-Parent"]
        #[doc = "(CPFP), where the child transaction pays for its parent's fee. This can be"]
        #[doc = "done by specifying an outpoint within the low fee transaction that is under"]
        #[doc = "the control of the wallet."]
        #[doc = ""]
        #[doc = "The fee preference can be expressed either as a specific fee rate or a delta"]
        #[doc = "of blocks in which the output should be swept on-chain within. If a fee"]
        #[doc = "preference is not explicitly specified, then an error is returned."]
        #[doc = ""]
        #[doc = "Note that this RPC currently doesn't perform any validation checks on the"]
        #[doc = "fee preference being provided. For now, the responsibility of ensuring that"]
        #[doc = "the new fee preference is sufficient is delegated to the user."]
        pub async fn bump_fee(
            &mut self,
            request: impl tonic::IntoRequest<super::BumpFeeRequest>,
        ) -> Result<tonic::Response<super::BumpFeeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/BumpFee");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "ListSweeps returns a list of the sweep transactions our node has produced."]
        #[doc = "Note that these sweeps may not be confirmed yet, as we record sweeps on"]
        #[doc = "broadcast, not confirmation."]
        pub async fn list_sweeps(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSweepsRequest>,
        ) -> Result<tonic::Response<super::ListSweepsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/ListSweeps");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "LabelTransaction adds a label to a transaction. If the transaction already"]
        #[doc = "has a label the call will fail unless the overwrite bool is set. This will"]
        #[doc = "overwrite the exiting transaction label. Labels must not be empty, and"]
        #[doc = "cannot exceed 500 characters."]
        pub async fn label_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::LabelTransactionRequest>,
        ) -> Result<tonic::Response<super::LabelTransactionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/LabelTransaction");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "FundPsbt creates a fully populated PSBT that contains enough inputs to fund"]
        #[doc = "the outputs specified in the template. There are two ways of specifying a"]
        #[doc = "template: Either by passing in a PSBT with at least one output declared or"]
        #[doc = "by passing in a raw TxTemplate message."]
        #[doc = ""]
        #[doc = "If there are no inputs specified in the template, coin selection is"]
        #[doc = "performed automatically. If the template does contain any inputs, it is"]
        #[doc = "assumed that full coin selection happened externally and no additional"]
        #[doc = "inputs are added. If the specified inputs aren't enough to fund the outputs"]
        #[doc = "with the given fee rate, an error is returned."]
        #[doc = ""]
        #[doc = "After either selecting or verifying the inputs, all input UTXOs are locked"]
        #[doc = "with an internal app ID."]
        #[doc = ""]
        #[doc = "NOTE: If this method returns without an error, it is the caller's"]
        #[doc = "responsibility to either spend the locked UTXOs (by finalizing and then"]
        #[doc = "publishing the transaction) or to unlock/release the locked UTXOs in case of"]
        #[doc = "an error on the caller's side."]
        pub async fn fund_psbt(
            &mut self,
            request: impl tonic::IntoRequest<super::FundPsbtRequest>,
        ) -> Result<tonic::Response<super::FundPsbtResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/FundPsbt");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "FinalizePsbt expects a partial transaction with all inputs and outputs fully"]
        #[doc = "declared and tries to sign all inputs that belong to the wallet. Lnd must be"]
        #[doc = "the last signer of the transaction. That means, if there are any unsigned"]
        #[doc = "non-witness inputs or inputs without UTXO information attached or inputs"]
        #[doc = "without witness data that do not belong to lnd's wallet, this method will"]
        #[doc = "fail. If no error is returned, the PSBT is ready to be extracted and the"]
        #[doc = "final TX within to be broadcast."]
        #[doc = ""]
        #[doc = "NOTE: This method does NOT publish the transaction once finalized. It is the"]
        #[doc = "caller's responsibility to either publish the transaction on success or"]
        #[doc = "unlock/release any locked UTXOs in case of an error in this method."]
        pub async fn finalize_psbt(
            &mut self,
            request: impl tonic::IntoRequest<super::FinalizePsbtRequest>,
        ) -> Result<tonic::Response<super::FinalizePsbtResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/FinalizePsbt");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for WalletKitClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for WalletKitClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "WalletKitClient {{ ... }}")
        }
    }
}
