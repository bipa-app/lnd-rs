#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusResponse {
    /// Indicates whether the autopilot is active or not.
    #[prost(bool, tag = "1")]
    pub active: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyStatusRequest {
    /// Whether the autopilot agent should be enabled or not.
    #[prost(bool, tag = "1")]
    pub enable: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyStatusResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryScoresRequest {
    #[prost(string, repeated, tag = "1")]
    pub pubkeys: ::std::vec::Vec<std::string::String>,
    /// If set, we will ignore the local channel state when calculating scores.
    #[prost(bool, tag = "2")]
    pub ignore_local_state: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryScoresResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<query_scores_response::HeuristicResult>,
}
pub mod query_scores_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HeuristicResult {
        #[prost(string, tag = "1")]
        pub heuristic: std::string::String,
        #[prost(map = "string, double", tag = "2")]
        pub scores: ::std::collections::HashMap<std::string::String, f64>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetScoresRequest {
    /// The name of the heuristic to provide scores to.
    #[prost(string, tag = "1")]
    pub heuristic: std::string::String,
    ///
    ///A map from hex-encoded public keys to scores. Scores must be in the range
    ///[0.0, 1.0].
    #[prost(map = "string, double", tag = "2")]
    pub scores: ::std::collections::HashMap<std::string::String, f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetScoresResponse {}
#[doc = r" Generated client implementations."]
pub mod autopilot_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Autopilot is a service that can be used to get information about the current"]
    #[doc = " state of the daemon's autopilot agent, and also supply it with information"]
    #[doc = " that can be used when deciding where to open channels."]
    pub struct AutopilotClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AutopilotClient<tonic::transport::Channel> {
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
    impl<T> AutopilotClient<T>
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
        #[doc = "Status returns whether the daemon's autopilot agent is active."]
        pub async fn status(
            &mut self,
            request: impl tonic::IntoRequest<super::StatusRequest>,
        ) -> Result<tonic::Response<super::StatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/autopilotrpc.Autopilot/Status");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "ModifyStatus is used to modify the status of the autopilot agent, like"]
        #[doc = "enabling or disabling it."]
        pub async fn modify_status(
            &mut self,
            request: impl tonic::IntoRequest<super::ModifyStatusRequest>,
        ) -> Result<tonic::Response<super::ModifyStatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/autopilotrpc.Autopilot/ModifyStatus");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "QueryScores queries all available autopilot heuristics, in addition to any"]
        #[doc = "active combination of these heruristics, for the scores they would give to"]
        #[doc = "the given nodes."]
        pub async fn query_scores(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryScoresRequest>,
        ) -> Result<tonic::Response<super::QueryScoresResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/autopilotrpc.Autopilot/QueryScores");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "SetScores attempts to set the scores used by the running autopilot agent,"]
        #[doc = "if the external scoring heuristic is enabled."]
        pub async fn set_scores(
            &mut self,
            request: impl tonic::IntoRequest<super::SetScoresRequest>,
        ) -> Result<tonic::Response<super::SetScoresResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/autopilotrpc.Autopilot/SetScores");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AutopilotClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AutopilotClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AutopilotClient {{ ... }}")
        }
    }
}
