#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfRequest {
    ///
    ///The transaction hash for which we should request a confirmation notification
    ///for. If set to a hash of all zeros, then the confirmation notification will
    ///be requested for the script instead.
    #[prost(bytes, tag = "1")]
    pub txid: std::vec::Vec<u8>,
    ///
    ///An output script within a transaction with the hash above which will be used
    ///by light clients to match block filters. If the transaction hash is set to a
    ///hash of all zeros, then a confirmation notification will be requested for
    ///this script instead.
    #[prost(bytes, tag = "2")]
    pub script: std::vec::Vec<u8>,
    ///
    ///The number of desired confirmations the transaction/output script should
    ///reach before dispatching a confirmation notification.
    #[prost(uint32, tag = "3")]
    pub num_confs: u32,
    ///
    ///The earliest height in the chain for which the transaction/output script
    ///could have been included in a block. This should in most cases be set to the
    ///broadcast height of the transaction/output script.
    #[prost(uint32, tag = "4")]
    pub height_hint: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfDetails {
    /// The raw bytes of the confirmed transaction.
    #[prost(bytes, tag = "1")]
    pub raw_tx: std::vec::Vec<u8>,
    /// The hash of the block in which the confirmed transaction was included in.
    #[prost(bytes, tag = "2")]
    pub block_hash: std::vec::Vec<u8>,
    /// The height of the block in which the confirmed transaction was included
    /// in.
    #[prost(uint32, tag = "3")]
    pub block_height: u32,
    /// The index of the confirmed transaction within the transaction.
    #[prost(uint32, tag = "4")]
    pub tx_index: u32,
}
/// TODO(wilmer): need to know how the client will use this first.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reorg {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfEvent {
    #[prost(oneof = "conf_event::Event", tags = "1, 2")]
    pub event: ::std::option::Option<conf_event::Event>,
}
pub mod conf_event {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        ///
        ///An event that includes the confirmation details of the request
        ///(txid/ouput script).
        #[prost(message, tag = "1")]
        Conf(super::ConfDetails),
        ///
        ///An event send when the transaction of the request is reorged out of the
        ///chain.
        #[prost(message, tag = "2")]
        Reorg(super::Reorg),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Outpoint {
    /// The hash of the transaction.
    #[prost(bytes, tag = "1")]
    pub hash: std::vec::Vec<u8>,
    /// The index of the output within the transaction.
    #[prost(uint32, tag = "2")]
    pub index: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendRequest {
    ///
    ///The outpoint for which we should request a spend notification for. If set to
    ///a zero outpoint, then the spend notification will be requested for the
    ///script instead.
    #[prost(message, optional, tag = "1")]
    pub outpoint: ::std::option::Option<Outpoint>,
    ///
    ///The output script for the outpoint above. This will be used by light clients
    ///to match block filters. If the outpoint is set to a zero outpoint, then a
    ///spend notification will be requested for this script instead.
    #[prost(bytes, tag = "2")]
    pub script: std::vec::Vec<u8>,
    ///
    ///The earliest height in the chain for which the outpoint/output script could
    ///have been spent. This should in most cases be set to the broadcast height of
    ///the outpoint/output script.
    #[prost(uint32, tag = "3")]
    pub height_hint: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendDetails {
    /// The outpoint was that spent.
    #[prost(message, optional, tag = "1")]
    pub spending_outpoint: ::std::option::Option<Outpoint>,
    /// The raw bytes of the spending transaction.
    #[prost(bytes, tag = "2")]
    pub raw_spending_tx: std::vec::Vec<u8>,
    /// The hash of the spending transaction.
    #[prost(bytes, tag = "3")]
    pub spending_tx_hash: std::vec::Vec<u8>,
    /// The input of the spending transaction that fulfilled the spend request.
    #[prost(uint32, tag = "4")]
    pub spending_input_index: u32,
    /// The height at which the spending transaction was included in a block.
    #[prost(uint32, tag = "5")]
    pub spending_height: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendEvent {
    #[prost(oneof = "spend_event::Event", tags = "1, 2")]
    pub event: ::std::option::Option<spend_event::Event>,
}
pub mod spend_event {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        ///
        ///An event that includes the details of the spending transaction of the
        ///request (outpoint/output script).
        #[prost(message, tag = "1")]
        Spend(super::SpendDetails),
        ///
        ///An event sent when the spending transaction of the request was
        ///reorged out of the chain.
        #[prost(message, tag = "2")]
        Reorg(super::Reorg),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockEpoch {
    /// The hash of the block.
    #[prost(bytes, tag = "1")]
    pub hash: std::vec::Vec<u8>,
    /// The height of the block.
    #[prost(uint32, tag = "2")]
    pub height: u32,
}
#[doc = r" Generated client implementations."]
pub mod chain_notifier_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " ChainNotifier is a service that can be used to get information about the"]
    #[doc = " chain backend by registering notifiers for chain events."]
    pub struct ChainNotifierClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ChainNotifierClient<tonic::transport::Channel> {
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
    impl<T> ChainNotifierClient<T>
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
        #[doc = "RegisterConfirmationsNtfn is a synchronous response-streaming RPC that"]
        #[doc = "registers an intent for a client to be notified once a confirmation request"]
        #[doc = "has reached its required number of confirmations on-chain."]
        #[doc = ""]
        #[doc = "A client can specify whether the confirmation request should be for a"]
        #[doc = "particular transaction by its hash or for an output script by specifying a"]
        #[doc = "zero hash."]
        pub async fn register_confirmations_ntfn(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ConfEvent>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chainrpc.ChainNotifier/RegisterConfirmationsNtfn",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = "RegisterSpendNtfn is a synchronous response-streaming RPC that registers an"]
        #[doc = "intent for a client to be notification once a spend request has been spent"]
        #[doc = "by a transaction that has confirmed on-chain."]
        #[doc = ""]
        #[doc = "A client can specify whether the spend request should be for a particular"]
        #[doc = "outpoint  or for an output script by specifying a zero outpoint."]
        pub async fn register_spend_ntfn(
            &mut self,
            request: impl tonic::IntoRequest<super::SpendRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::SpendEvent>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/chainrpc.ChainNotifier/RegisterSpendNtfn");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = "RegisterBlockEpochNtfn is a synchronous response-streaming RPC that"]
        #[doc = "registers an intent for a client to be notified of blocks in the chain. The"]
        #[doc = "stream will return a hash and height tuple of a block for each new/stale"]
        #[doc = "block in the chain. It is the client's responsibility to determine whether"]
        #[doc = "the tuple returned is for a new or stale block in the chain."]
        #[doc = ""]
        #[doc = "A client can also request a historical backlog of blocks from a particular"]
        #[doc = "point. This allows clients to be idempotent by ensuring that they do not"]
        #[doc = "missing processing a single block within the chain."]
        pub async fn register_block_epoch_ntfn(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockEpoch>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::BlockEpoch>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chainrpc.ChainNotifier/RegisterBlockEpochNtfn",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for ChainNotifierClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ChainNotifierClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ChainNotifierClient {{ ... }}")
        }
    }
}
