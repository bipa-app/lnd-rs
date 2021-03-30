#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// A verbose description of the daemon's commit.
    #[prost(string, tag = "1")]
    pub commit: std::string::String,
    /// The SHA1 commit hash that the daemon is compiled with.
    #[prost(string, tag = "2")]
    pub commit_hash: std::string::String,
    /// The semantic version.
    #[prost(string, tag = "3")]
    pub version: std::string::String,
    /// The major application version.
    #[prost(uint32, tag = "4")]
    pub app_major: u32,
    /// The minor application version.
    #[prost(uint32, tag = "5")]
    pub app_minor: u32,
    /// The application patch number.
    #[prost(uint32, tag = "6")]
    pub app_patch: u32,
    /// The application pre-release modifier, possibly empty.
    #[prost(string, tag = "7")]
    pub app_pre_release: std::string::String,
    /// The list of build tags that were supplied during compilation.
    #[prost(string, repeated, tag = "8")]
    pub build_tags: ::std::vec::Vec<std::string::String>,
    /// The version of go that compiled the executable.
    #[prost(string, tag = "9")]
    pub go_version: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod versioner_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Versioner is a service that can be used to get information about the version"]
    #[doc = " and build information of the running daemon."]
    pub struct VersionerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VersionerClient<tonic::transport::Channel> {
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
    impl<T> VersionerClient<T>
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
        #[doc = " lncli: `version`"]
        #[doc = "GetVersion returns the current version and build information of the running"]
        #[doc = "daemon."]
        pub async fn get_version(
            &mut self,
            request: impl tonic::IntoRequest<super::VersionRequest>,
        ) -> Result<tonic::Response<super::Version>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/verrpc.Versioner/GetVersion");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for VersionerClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for VersionerClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "VersionerClient {{ ... }}")
        }
    }
}
