#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionResponse {
    /// The version information for lncli.
    #[prost(message, optional, tag = "1")]
    pub lncli: ::std::option::Option<super::verrpc::Version>,
    /// The version information for lnd.
    #[prost(message, optional, tag = "2")]
    pub lnd: ::std::option::Option<super::verrpc::Version>,
}
