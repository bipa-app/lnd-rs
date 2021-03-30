/// Module including all tonic-build generated code.
/// Each sub-module represents one proto service.
pub mod lnrpc;

use hyper::client::HttpConnector;
use hyper_openssl::HttpsConnector;
use lnrpc::lnrpc::{
    lightning_client::LightningClient, ChannelBalanceRequest, ChannelBalanceResponse,
    WalletBalanceRequest, WalletBalanceResponse,
};
use openssl::{
    error::ErrorStack,
    ssl::{SslConnector, SslMethod},
    x509::X509,
};
use std::convert::TryInto;
use tonic::{
    codegen::StdError,
    metadata::{errors::InvalidMetadataValue, MetadataValue},
    transport::{Channel, Endpoint},
    Interceptor, Response, Status,
};

pub struct Lnd {
    lightning_client: LightningClient<Channel>,
}

pub enum LndConnectError {
    Connector(ErrorStack),
    Interceptor(InvalidMetadataValue),
    Transport(tonic::transport::Error),
}

impl Lnd {
    pub async fn connect<D>(
        destination: D,
        certificate_bytes: &[u8],
        macaroon_bytes: &[u8],
    ) -> Result<Self, LndConnectError>
    where
        D: TryInto<Endpoint>,
        D::Error: Into<StdError>,
    {
        let https_connector =
            Lnd::connector(certificate_bytes).map_err(LndConnectError::Connector)?;

        let interceptor = Lnd::interceptor(macaroon_bytes).map_err(LndConnectError::Interceptor)?;

        let transport = tonic::transport::Endpoint::new(destination)
            .map_err(LndConnectError::Transport)?
            .connect_with_connector(https_connector)
            .await
            .map_err(LndConnectError::Transport)?;

        let lightning_client = LightningClient::with_interceptor(transport, interceptor);

        Ok(Lnd { lightning_client })
    }

    fn connector(certificate_bytes: &[u8]) -> Result<HttpsConnector<HttpConnector>, ErrorStack> {
        let mut connector = SslConnector::builder(SslMethod::tls())?;
        let ca = X509::from_pem(&certificate_bytes).unwrap();

        connector.cert_store_mut().add_cert(ca)?;
        connector.set_alpn_protos(b"\x02h2")?;

        let mut http = HttpConnector::new();
        http.enforce_http(false);

        HttpsConnector::with_connector(http, connector)
    }

    fn interceptor(macaroon_bytes: &[u8]) -> Result<Interceptor, InvalidMetadataValue> {
        let metadata = MetadataValue::from_str(&hex::encode(macaroon_bytes))?;

        Ok(Interceptor::new(move |mut request| {
            request.metadata_mut().insert("macaroon", metadata.clone());
            Ok(request)
        }))
    }
}

impl Lnd {
    pub async fn wallet_balance(&mut self) -> Result<WalletBalanceResponse, Status> {
        self.lightning_client
            .wallet_balance(WalletBalanceRequest {})
            .await
            .map(Response::into_inner)
    }

    pub async fn channel_balance(&mut self) -> Result<ChannelBalanceResponse, Status> {
        self.lightning_client
            .channel_balance(ChannelBalanceRequest {})
            .await
            .map(Response::into_inner)
    }
}
