/// Module including all tonic-build generated code.
/// Each sub-module represents one proto service.
mod gen;
pub use gen::{invoicesrpc, lnrpc, routerrpc};

use gen::invoicesrpc::{invoices_client::InvoicesClient, SubscribeSingleInvoiceRequest};
use gen::lnrpc::{
    lightning_client::LightningClient, AddInvoiceResponse, ChannelBalanceRequest,
    ChannelBalanceResponse, GetInfoRequest, GetInfoResponse, Invoice, ListInvoiceRequest,
    ListInvoiceResponse, ListPaymentsRequest, ListPaymentsResponse, PayReq, PayReqString, Payment,
    PaymentHash, SendRequest, SendResponse, WalletBalanceRequest, WalletBalanceResponse,
};
use gen::routerrpc::{router_client::RouterClient, SendPaymentRequest, TrackPaymentRequest};

use hyper::client::HttpConnector;
use hyper_openssl::HttpsConnector;
use openssl::{
    error::ErrorStack,
    ssl::{SslConnector, SslMethod},
    x509::X509,
};
use std::convert::TryInto;
use tonic::Streaming;
use tonic::{
    codegen::{InterceptedService, StdError},
    metadata::{errors::InvalidMetadataValue, Ascii, MetadataValue},
    service::Interceptor,
    transport::{Channel, Endpoint},
    Response, Status,
};

#[derive(Debug, Clone)]
pub struct Lnd {
    lightning: LightningClient<InterceptedService<Channel, LndInterceptor>>,
    invoices: InvoicesClient<InterceptedService<Channel, LndInterceptor>>,
    router: RouterClient<InterceptedService<Channel, LndInterceptor>>,
}

#[derive(Debug, thiserror::Error)]
pub enum LndConnectError {
    #[error("Connector creation failed: #{0}")]
    Connector(ErrorStack),
    #[error("Interceptor creation failed: #{0}")]
    Interceptor(InvalidMetadataValue),
    #[error("Transport connection failed: #{0}")]
    Transport(tonic::transport::Error),
}

impl Lnd {
    pub async fn connect<D>(
        destination: D,
        certificate_bytes: &[u8],
    ) -> Result<Self, LndConnectError>
    where
        D: TryInto<Endpoint>,
        D::Error: Into<StdError>,
    {
        let https_connector =
            Lnd::connector(certificate_bytes).map_err(LndConnectError::Connector)?;

        let transport = tonic::transport::Endpoint::new(destination)
            .map_err(LndConnectError::Transport)?
            .connect_with_connector(https_connector)
            .await
            .map_err(LndConnectError::Transport)?;

        Ok(Lnd::build(transport, LndInterceptor::noop()))
    }

    pub async fn connect_lazy<D>(
        destination: D,
        certificate_bytes: &[u8],
    ) -> Result<Self, LndConnectError>
    where
        D: TryInto<Endpoint>,
        D::Error: Into<StdError>,
    {
        let https_connector =
            Lnd::connector(certificate_bytes).map_err(LndConnectError::Connector)?;

        let transport = tonic::transport::Endpoint::new(destination)
            .and_then(move |d| d.connect_with_connector_lazy(https_connector))
            .map_err(LndConnectError::Transport)?;

        Ok(Lnd::build(transport, LndInterceptor::noop()))
    }

    pub async fn connect_with_macaroon<D>(
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

        let interceptor =
            LndInterceptor::macaroon(macaroon_bytes).map_err(LndConnectError::Interceptor)?;

        let transport = tonic::transport::Endpoint::new(destination)
            .map_err(LndConnectError::Transport)?
            .connect_with_connector(https_connector)
            .await
            .map_err(LndConnectError::Transport)?;

        Ok(Lnd::build(transport, interceptor))
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

    fn build(channel: Channel, interceptor: LndInterceptor) -> Self {
        let lightning = LightningClient::with_interceptor(channel.clone(), interceptor.clone());
        let invoices = InvoicesClient::with_interceptor(channel.clone(), interceptor.clone());
        let router = RouterClient::with_interceptor(channel, interceptor);

        Lnd {
            lightning,
            invoices,
            router,
        }
    }
}

#[derive(Debug, Clone)]
struct LndInterceptor {
    macaroon: Option<MetadataValue<Ascii>>,
}

impl LndInterceptor {
    fn macaroon(bytes: &[u8]) -> Result<Self, InvalidMetadataValue> {
        let macaroon = Some(MetadataValue::from_str(&hex::encode(bytes))?);
        Ok(Self { macaroon })
    }

    fn noop() -> Self {
        Self { macaroon: None }
    }
}

impl Interceptor for LndInterceptor {
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        if let Some(macaroon) = self.macaroon.as_ref().cloned() {
            request.metadata_mut().insert("macaroon", macaroon);
        }

        Ok(request)
    }
}

impl Lnd {
    pub async fn get_info(&mut self) -> Result<GetInfoResponse, Status> {
        self.lightning
            .get_info(GetInfoRequest {})
            .await
            .map(Response::into_inner)
    }

    pub async fn add_invoice(&mut self, invoice: Invoice) -> Result<AddInvoiceResponse, Status> {
        self.lightning
            .add_invoice(invoice)
            .await
            .map(Response::into_inner)
    }

    pub async fn channel_balance(&mut self) -> Result<ChannelBalanceResponse, Status> {
        self.lightning
            .channel_balance(ChannelBalanceRequest {})
            .await
            .map(Response::into_inner)
    }

    pub async fn decode_pay_req(&mut self, pay_req: String) -> Result<PayReq, Status> {
        self.lightning
            .decode_pay_req(PayReqString { pay_req })
            .await
            .map(Response::into_inner)
    }

    pub async fn list_payments(
        &mut self,
        include_incomplete: bool,
        index_offset: u64,
        max_payments: u64,
        reversed: bool,
    ) -> Result<ListPaymentsResponse, Status> {
        self.lightning
            .list_payments(ListPaymentsRequest {
                include_incomplete,
                index_offset,
                max_payments,
                reversed,
            })
            .await
            .map(Response::into_inner)
    }

    pub async fn list_invoices(
        &mut self,
        pending_only: bool,
        index_offset: u64,
        num_max_invoices: u64,
        reversed: bool,
    ) -> Result<ListInvoiceResponse, Status> {
        self.lightning
            .list_invoices(ListInvoiceRequest {
                pending_only,
                index_offset,
                num_max_invoices,
                reversed,
            })
            .await
            .map(Response::into_inner)
    }

    pub async fn lookup_invoice(&mut self, r_hash: Vec<u8>) -> Result<Invoice, Status> {
        #[allow(deprecated)]
        let payment_hash = PaymentHash {
            r_hash_str: String::from(""),
            r_hash,
        };
        self.lightning
            .lookup_invoice(payment_hash)
            .await
            .map(Response::into_inner)
    }

    pub async fn send_payment_sync(
        &mut self,
        send_request: SendRequest,
    ) -> Result<SendResponse, Status> {
        self.lightning
            .send_payment_sync(send_request)
            .await
            .map(Response::into_inner)
    }

    pub async fn wallet_balance(&mut self) -> Result<WalletBalanceResponse, Status> {
        self.lightning
            .wallet_balance(WalletBalanceRequest {})
            .await
            .map(Response::into_inner)
    }
}

impl Lnd {
    pub async fn send_payment(
        &mut self,
        req: SendPaymentRequest,
    ) -> Result<Streaming<Payment>, Status> {
        self.router
            .send_payment_v2(req)
            .await
            .map(Response::into_inner)
    }

    pub async fn track_payment(
        &mut self,
        payment_hash: Vec<u8>,
        no_inflight_updates: bool,
    ) -> Result<Streaming<Payment>, Status> {
        self.router
            .track_payment_v2(TrackPaymentRequest {
                no_inflight_updates,
                payment_hash,
            })
            .await
            .map(Response::into_inner)
    }
}

impl Lnd {
    pub async fn subscribe_single_invoice(
        &mut self,
        req: SubscribeSingleInvoiceRequest,
    ) -> Result<Streaming<Invoice>, Status> {
        self.invoices
            .subscribe_single_invoice(req)
            .await
            .map(Response::into_inner)
    }
}
