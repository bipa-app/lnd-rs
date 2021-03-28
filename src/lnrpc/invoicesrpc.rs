#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelInvoiceMsg {
    /// Hash corresponding to the (hold) invoice to cancel.
    #[prost(bytes, tag = "1")]
    pub payment_hash: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelInvoiceResp {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddHoldInvoiceRequest {
    ///
    ///An optional memo to attach along with the invoice. Used for record keeping
    ///purposes for the invoice's creator, and will also be set in the description
    ///field of the encoded payment request if the description_hash field is not
    ///being used.
    #[prost(string, tag = "1")]
    pub memo: std::string::String,
    /// The hash of the preimage
    #[prost(bytes, tag = "2")]
    pub hash: std::vec::Vec<u8>,
    ///
    ///The value of this invoice in satoshis
    ///
    ///The fields value and value_msat are mutually exclusive.
    #[prost(int64, tag = "3")]
    pub value: i64,
    ///
    ///The value of this invoice in millisatoshis
    ///
    ///The fields value and value_msat are mutually exclusive.
    #[prost(int64, tag = "10")]
    pub value_msat: i64,
    ///
    ///Hash (SHA-256) of a description of the payment. Used if the description of
    ///payment (memo) is too long to naturally fit within the description field
    ///of an encoded payment request.
    #[prost(bytes, tag = "4")]
    pub description_hash: std::vec::Vec<u8>,
    /// Payment request expiry time in seconds. Default is 3600 (1 hour).
    #[prost(int64, tag = "5")]
    pub expiry: i64,
    /// Fallback on-chain address.
    #[prost(string, tag = "6")]
    pub fallback_addr: std::string::String,
    /// Delta to use for the time-lock of the CLTV extended to the final hop.
    #[prost(uint64, tag = "7")]
    pub cltv_expiry: u64,
    ///
    ///Route hints that can each be individually used to assist in reaching the
    ///invoice's destination.
    #[prost(message, repeated, tag = "8")]
    pub route_hints: ::std::vec::Vec<super::lnrpc::RouteHint>,
    /// Whether this invoice should include routing hints for private channels.
    #[prost(bool, tag = "9")]
    pub private: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddHoldInvoiceResp {
    ///
    ///A bare-bones invoice for a payment within the Lightning Network.  With the
    ///details of the invoice, the sender has all the data necessary to send a
    ///payment to the recipient.
    #[prost(string, tag = "1")]
    pub payment_request: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SettleInvoiceMsg {
    /// Externally discovered pre-image that should be used to settle the hold
    /// invoice.
    #[prost(bytes, tag = "1")]
    pub preimage: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SettleInvoiceResp {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeSingleInvoiceRequest {
    /// Hash corresponding to the (hold) invoice to subscribe to.
    #[prost(bytes, tag = "2")]
    pub r_hash: std::vec::Vec<u8>,
}
#[doc = r" Generated client implementations."]
pub mod invoices_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Invoices is a service that can be used to create, accept, settle and cancel"]
    #[doc = " invoices."]
    pub struct InvoicesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InvoicesClient<tonic::transport::Channel> {
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
    impl<T> InvoicesClient<T>
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
        #[doc = "SubscribeSingleInvoice returns a uni-directional stream (server -> client)"]
        #[doc = "to notify the client of state transitions of the specified invoice."]
        #[doc = "Initially the current invoice state is always sent out."]
        pub async fn subscribe_single_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeSingleInvoiceRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::lnrpc::Invoice>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/invoicesrpc.Invoices/SubscribeSingleInvoice",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = "CancelInvoice cancels a currently open invoice. If the invoice is already"]
        #[doc = "canceled, this call will succeed. If the invoice is already settled, it will"]
        #[doc = "fail."]
        pub async fn cancel_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelInvoiceMsg>,
        ) -> Result<tonic::Response<super::CancelInvoiceResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/invoicesrpc.Invoices/CancelInvoice");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "AddHoldInvoice creates a hold invoice. It ties the invoice to the hash"]
        #[doc = "supplied in the request."]
        pub async fn add_hold_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::AddHoldInvoiceRequest>,
        ) -> Result<tonic::Response<super::AddHoldInvoiceResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/invoicesrpc.Invoices/AddHoldInvoice");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "SettleInvoice settles an accepted invoice. If the invoice is already"]
        #[doc = "settled, this call will succeed."]
        pub async fn settle_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::SettleInvoiceMsg>,
        ) -> Result<tonic::Response<super::SettleInvoiceResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/invoicesrpc.Invoices/SettleInvoice");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for InvoicesClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for InvoicesClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "InvoicesClient {{ ... }}")
        }
    }
}
