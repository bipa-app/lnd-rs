#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyLocator {
    /// The family of key being identified.
    #[prost(int32, tag = "1")]
    pub key_family: i32,
    /// The precise index of the key being identified.
    #[prost(int32, tag = "2")]
    pub key_index: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyDescriptor {
    ///
    ///The raw bytes of the key being identified. Either this or the KeyLocator
    ///must be specified.
    #[prost(bytes, tag = "1")]
    pub raw_key_bytes: std::vec::Vec<u8>,
    ///
    ///The key locator that identifies which key to use for signing. Either this
    ///or the raw bytes of the target key must be specified.
    #[prost(message, optional, tag = "2")]
    pub key_loc: ::std::option::Option<KeyLocator>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxOut {
    /// The value of the output being spent.
    #[prost(int64, tag = "1")]
    pub value: i64,
    /// The script of the output being spent.
    #[prost(bytes, tag = "2")]
    pub pk_script: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignDescriptor {
    ///
    ///A descriptor that precisely describes *which* key to use for signing. This
    ///may provide the raw public key directly, or require the Signer to re-derive
    ///the key according to the populated derivation path.
    ///
    ///Note that if the key descriptor was obtained through walletrpc.DeriveKey,
    ///then the key locator MUST always be provided, since the derived keys are not
    ///persisted unlike with DeriveNextKey.
    #[prost(message, optional, tag = "1")]
    pub key_desc: ::std::option::Option<KeyDescriptor>,
    ///
    ///A scalar value that will be added to the private key corresponding to the
    ///above public key to obtain the private key to be used to sign this input.
    ///This value is typically derived via the following computation:
    ///
    /// derivedKey = privkey + sha256(perCommitmentPoint || pubKey) mod N
    #[prost(bytes, tag = "2")]
    pub single_tweak: std::vec::Vec<u8>,
    ///
    ///A private key that will be used in combination with its corresponding
    ///private key to derive the private key that is to be used to sign the target
    ///input. Within the Lightning protocol, this value is typically the
    ///commitment secret from a previously revoked commitment transaction. This
    ///value is in combination with two hash values, and the original private key
    ///to derive the private key to be used when signing.
    ///
    /// k = (privKey*sha256(pubKey || tweakPub) +
    ///tweakPriv*sha256(tweakPub || pubKey)) mod N
    #[prost(bytes, tag = "3")]
    pub double_tweak: std::vec::Vec<u8>,
    ///
    ///The full script required to properly redeem the output.  This field will
    ///only be populated if a p2wsh or a p2sh output is being signed.
    #[prost(bytes, tag = "4")]
    pub witness_script: std::vec::Vec<u8>,
    ///
    ///A description of the output being spent. The value and script MUST be
    ///provided.
    #[prost(message, optional, tag = "5")]
    pub output: ::std::option::Option<TxOut>,
    ///
    ///The target sighash type that should be used when generating the final
    ///sighash, and signature.
    #[prost(uint32, tag = "7")]
    pub sighash: u32,
    ///
    ///The target input within the transaction that should be signed.
    #[prost(int32, tag = "8")]
    pub input_index: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignReq {
    /// The raw bytes of the transaction to be signed.
    #[prost(bytes, tag = "1")]
    pub raw_tx_bytes: std::vec::Vec<u8>,
    /// A set of sign descriptors, for each input to be signed.
    #[prost(message, repeated, tag = "2")]
    pub sign_descs: ::std::vec::Vec<SignDescriptor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignResp {
    ///
    ///A set of signatures realized in a fixed 64-byte format ordered in ascending
    ///input order.
    #[prost(bytes, repeated, tag = "1")]
    pub raw_sigs: ::std::vec::Vec<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputScript {
    /// The serializes witness stack for the specified input.
    #[prost(bytes, repeated, tag = "1")]
    pub witness: ::std::vec::Vec<std::vec::Vec<u8>>,
    ///
    ///The optional sig script for the specified witness that will only be set if
    ///the input specified is a nested p2sh witness program.
    #[prost(bytes, tag = "2")]
    pub sig_script: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputScriptResp {
    /// The set of fully valid input scripts requested.
    #[prost(message, repeated, tag = "1")]
    pub input_scripts: ::std::vec::Vec<InputScript>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignMessageReq {
    /// The message to be signed.
    #[prost(bytes, tag = "1")]
    pub msg: std::vec::Vec<u8>,
    /// The key locator that identifies which key to use for signing.
    #[prost(message, optional, tag = "2")]
    pub key_loc: ::std::option::Option<KeyLocator>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignMessageResp {
    ///
    ///The signature for the given message in the fixed-size LN wire format.
    #[prost(bytes, tag = "1")]
    pub signature: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMessageReq {
    /// The message over which the signature is to be verified.
    #[prost(bytes, tag = "1")]
    pub msg: std::vec::Vec<u8>,
    ///
    ///The fixed-size LN wire encoded signature to be verified over the given
    ///message.
    #[prost(bytes, tag = "2")]
    pub signature: std::vec::Vec<u8>,
    /// The public key the signature has to be valid for.
    #[prost(bytes, tag = "3")]
    pub pubkey: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMessageResp {
    /// Whether the signature was valid over the given message.
    #[prost(bool, tag = "1")]
    pub valid: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedKeyRequest {
    /// The ephemeral public key to use for the DH key derivation.
    #[prost(bytes, tag = "1")]
    pub ephemeral_pubkey: std::vec::Vec<u8>,
    ///
    ///Deprecated. The optional key locator of the local key that should be used.
    ///If this parameter is not set then the node's identity private key will be
    ///used.
    #[prost(message, optional, tag = "2")]
    pub key_loc: ::std::option::Option<KeyLocator>,
    ///
    ///A key descriptor describes the key used for performing ECDH. Either a key
    ///locator or a raw public key is expected, if neither is supplied, defaults to
    ///the node's identity private key.
    #[prost(message, optional, tag = "3")]
    pub key_desc: ::std::option::Option<KeyDescriptor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedKeyResponse {
    /// The shared public key, hashed with sha256.
    #[prost(bytes, tag = "1")]
    pub shared_key: std::vec::Vec<u8>,
}
#[doc = r" Generated client implementations."]
pub mod signer_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Signer is a service that gives access to the signing functionality of the"]
    #[doc = " daemon's wallet."]
    pub struct SignerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SignerClient<tonic::transport::Channel> {
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
    impl<T> SignerClient<T>
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
        #[doc = "SignOutputRaw is a method that can be used to generated a signature for a"]
        #[doc = "set of inputs/outputs to a transaction. Each request specifies details"]
        #[doc = "concerning how the outputs should be signed, which keys they should be"]
        #[doc = "signed with, and also any optional tweaks. The return value is a fixed"]
        #[doc = "64-byte signature (the same format as we use on the wire in Lightning)."]
        #[doc = ""]
        #[doc = "If we are  unable to sign using the specified keys, then an error will be"]
        #[doc = "returned."]
        pub async fn sign_output_raw(
            &mut self,
            request: impl tonic::IntoRequest<super::SignReq>,
        ) -> Result<tonic::Response<super::SignResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/signrpc.Signer/SignOutputRaw");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "ComputeInputScript generates a complete InputIndex for the passed"]
        #[doc = "transaction with the signature as defined within the passed SignDescriptor."]
        #[doc = "This method should be capable of generating the proper input script for"]
        #[doc = "both regular p2wkh output and p2wkh outputs nested within a regular p2sh"]
        #[doc = "output."]
        #[doc = ""]
        #[doc = "Note that when using this method to sign inputs belonging to the wallet,"]
        #[doc = "the only items of the SignDescriptor that need to be populated are pkScript"]
        #[doc = "in the TxOut field, the value in that same field, and finally the input"]
        #[doc = "index."]
        pub async fn compute_input_script(
            &mut self,
            request: impl tonic::IntoRequest<super::SignReq>,
        ) -> Result<tonic::Response<super::InputScriptResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/signrpc.Signer/ComputeInputScript");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "SignMessage signs a message with the key specified in the key locator. The"]
        #[doc = "returned signature is fixed-size LN wire format encoded."]
        #[doc = ""]
        #[doc = "The main difference to SignMessage in the main RPC is that a specific key is"]
        #[doc = "used to sign the message instead of the node identity private key."]
        pub async fn sign_message(
            &mut self,
            request: impl tonic::IntoRequest<super::SignMessageReq>,
        ) -> Result<tonic::Response<super::SignMessageResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/signrpc.Signer/SignMessage");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "VerifyMessage verifies a signature over a message using the public key"]
        #[doc = "provided. The signature must be fixed-size LN wire format encoded."]
        #[doc = ""]
        #[doc = "The main difference to VerifyMessage in the main RPC is that the public key"]
        #[doc = "used to sign the message does not have to be a node known to the network."]
        pub async fn verify_message(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyMessageReq>,
        ) -> Result<tonic::Response<super::VerifyMessageResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/signrpc.Signer/VerifyMessage");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "DeriveSharedKey returns a shared secret key by performing Diffie-Hellman key"]
        #[doc = "derivation between the ephemeral public key in the request and the node's"]
        #[doc = "key specified in the key_desc parameter. Either a key locator or a raw"]
        #[doc = "public key is expected in the key_desc, if neither is supplied, defaults to"]
        #[doc = "the node's identity private key:"]
        #[doc = "P_shared = privKeyNode * ephemeralPubkey"]
        #[doc = "The resulting shared public key is serialized in the compressed format and"]
        #[doc = "hashed with sha256, resulting in the final key length of 256bit."]
        pub async fn derive_shared_key(
            &mut self,
            request: impl tonic::IntoRequest<super::SharedKeyRequest>,
        ) -> Result<tonic::Response<super::SharedKeyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/signrpc.Signer/DeriveSharedKey");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for SignerClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SignerClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SignerClient {{ ... }}")
        }
    }
}
