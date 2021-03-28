#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoResponse {
    /// The public key of the watchtower.
    #[prost(bytes, tag = "1")]
    pub pubkey: std::vec::Vec<u8>,
    /// The listening addresses of the watchtower.
    #[prost(string, repeated, tag = "2")]
    pub listeners: ::std::vec::Vec<std::string::String>,
    /// The URIs of the watchtower.
    #[prost(string, repeated, tag = "3")]
    pub uris: ::std::vec::Vec<std::string::String>,
}
#[doc = r" Generated client implementations."]
pub mod watchtower_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Watchtower is a service that grants access to the watchtower server"]
    #[doc = " functionality of the daemon."]
    pub struct WatchtowerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WatchtowerClient<tonic::transport::Channel> {
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
    impl<T> WatchtowerClient<T>
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
        #[doc = " lncli: tower info"]
        #[doc = "GetInfo returns general information concerning the companion watchtower"]
        #[doc = "including its public key and URIs where the server is currently"]
        #[doc = "listening for clients."]
        pub async fn get_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInfoRequest>,
        ) -> Result<tonic::Response<super::GetInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/watchtowerrpc.Watchtower/GetInfo");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for WatchtowerClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for WatchtowerClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "WatchtowerClient {{ ... }}")
        }
    }
}
