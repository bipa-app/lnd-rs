#![deny(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

/// Module including all tonic-build generated code.
/// Each sub-module represents one proto service.
mod gen;
pub use gen::{invoicesrpc, lnrpc, routerrpc};
use gen::{
    invoicesrpc::{
        invoices_client::InvoicesClient, lookup_invoice_msg::InvoiceRef, AddHoldInvoiceRequest,
        AddHoldInvoiceResp, CancelInvoiceMsg, CancelInvoiceResp, LookupInvoiceMsg, LookupModifier,
        SettleInvoiceMsg, SettleInvoiceResp, SubscribeSingleInvoiceRequest,
    },
    lnrpc::{
        lightning_client::LightningClient, AddInvoiceResponse, ChannelAcceptRequest,
        ChannelAcceptResponse, ChannelBalanceRequest, ChannelBalanceResponse, CloseChannelRequest,
        CloseStatusUpdate, ClosedChannelsRequest, ClosedChannelsResponse, ForwardingHistoryRequest,
        ForwardingHistoryResponse, GetInfoRequest, GetInfoResponse, Invoice, ListChannelsRequest,
        ListChannelsResponse, ListInvoiceRequest, ListInvoiceResponse, ListPaymentsRequest,
        ListPaymentsResponse, NewAddressRequest, NewAddressResponse, PayReq, PayReqString, Payment,
        PendingChannelsRequest, PendingChannelsResponse, SendCoinsRequest, SendCoinsResponse,
        SendRequest, SendResponse, WalletBalanceRequest, WalletBalanceResponse,
    },
    routerrpc::{router_client::RouterClient, SendPaymentRequest, TrackPaymentRequest},
};
use opentelemetry::trace::FutureExt;
use std::convert::TryInto;
use tonic::{
    codegen::{InterceptedService, StdError},
    metadata::{errors::InvalidMetadataValue, Ascii, MetadataValue},
    service::Interceptor,
    transport::{Certificate, ClientTlsConfig},
    transport::{Channel, Endpoint},
    Response, Status, Streaming,
};

#[derive(Debug, Clone)]
pub struct Lnd {
    lightning: LightningClient<InterceptedService<Channel, LndInterceptor>>,
    invoices: InvoicesClient<InterceptedService<Channel, LndInterceptor>>,
    router: RouterClient<InterceptedService<Channel, LndInterceptor>>,
    tracer: std::sync::Arc<opentelemetry::global::BoxedTracer>,
}

#[derive(Debug, thiserror::Error)]
pub enum LndConnectError {
    #[error("Interceptor creation failed: #{0}")]
    Interceptor(InvalidMetadataValue),
    #[error("Transport connection failed: #{0}")]
    Transport(tonic::transport::Error),
}

macro_rules! span {
    ($tracer:expr => $package:literal. $service:literal / $method:literal) => {
        opentelemetry::trace::SpanBuilder::from_name(concat!($package, ".", $service, "/", $method))
            .with_kind(opentelemetry::trace::SpanKind::Client)
            .with_attributes([
                opentelemetry::KeyValue::new("service.name", "lnd"),
                opentelemetry::KeyValue::new("rpc.system", "grpc"),
                opentelemetry::KeyValue::new("rpc.service", concat!($package, ".", $service)),
                opentelemetry::KeyValue::new("rpc.method", $method),
            ])
            .start($tracer.as_ref())
    };
}

impl Lnd {
    pub async fn connect<D>(destination: D, cert_pem_bytes: &[u8]) -> Result<Self, LndConnectError>
    where
        D: TryInto<Endpoint>,
        D::Error: Into<StdError>,
    {
        let cert = Certificate::from_pem(cert_pem_bytes);
        let transport = tonic::transport::Endpoint::new(destination)
            .and_then(|d| d.tls_config(ClientTlsConfig::new().ca_certificate(cert)))
            .map_err(LndConnectError::Transport)?
            .connect()
            .await
            .map_err(LndConnectError::Transport)?;

        Ok(Lnd::build(transport, LndInterceptor::noop()))
    }

    pub async fn connect_lazy<D>(
        destination: D,
        cert_pem_bytes: &[u8],
    ) -> Result<Self, LndConnectError>
    where
        D: TryInto<Endpoint>,
        D::Error: Into<StdError>,
    {
        let cert = Certificate::from_pem(cert_pem_bytes);
        let transport = tonic::transport::Endpoint::new(destination)
            .and_then(|d| d.tls_config(ClientTlsConfig::new().ca_certificate(cert)))
            .map(move |d| d.connect_lazy())
            .map_err(LndConnectError::Transport)?;

        Ok(Lnd::build(transport, LndInterceptor::noop()))
    }

    pub async fn connect_with_macaroon<D>(
        destination: D,
        cert_pem_bytes: &[u8],
        macaroon_bytes: &[u8],
    ) -> Result<Self, LndConnectError>
    where
        D: TryInto<Endpoint>,
        D::Error: Into<StdError>,
    {
        let interceptor =
            LndInterceptor::macaroon(macaroon_bytes).map_err(LndConnectError::Interceptor)?;

        let cert = Certificate::from_pem(cert_pem_bytes);
        let transport = tonic::transport::Endpoint::new(destination)
            .and_then(|d| d.tls_config(ClientTlsConfig::new().ca_certificate(cert)))
            .map_err(LndConnectError::Transport)?
            .connect()
            .await
            .map_err(LndConnectError::Transport)?;

        Ok(Lnd::build(transport, interceptor))
    }

    pub async fn connect_with_macaroon_lazy<D>(
        destination: D,
        cert_pem_bytes: &[u8],
        macaroon_bytes: &[u8],
    ) -> Result<Self, LndConnectError>
    where
        D: TryInto<Endpoint>,
        D::Error: Into<StdError>,
    {
        let interceptor =
            LndInterceptor::macaroon(macaroon_bytes).map_err(LndConnectError::Interceptor)?;

        let cert = Certificate::from_pem(cert_pem_bytes);
        let transport = tonic::transport::Endpoint::new(destination)
            .and_then(|d| d.tls_config(ClientTlsConfig::new().ca_certificate(cert)))
            .map(move |d| d.connect_lazy())
            .map_err(LndConnectError::Transport)?;

        Ok(Lnd::build(transport, interceptor))
    }

    fn build(channel: Channel, interceptor: LndInterceptor) -> Self {
        let tracer = std::sync::Arc::new(opentelemetry::global::tracer("lnd"));
        let lightning = LightningClient::with_interceptor(channel.clone(), interceptor.clone());
        let invoices = InvoicesClient::with_interceptor(channel.clone(), interceptor.clone());
        let router = RouterClient::with_interceptor(channel, interceptor);

        Lnd {
            lightning,
            invoices,
            router,
            tracer,
        }
    }
}

#[derive(Debug, Clone)]
struct LndInterceptor {
    macaroon: Option<MetadataValue<Ascii>>,
}

impl LndInterceptor {
    fn macaroon(bytes: &[u8]) -> Result<Self, InvalidMetadataValue> {
        let macaroon = Some(MetadataValue::try_from(&hex::encode(bytes))?);
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
        span!(self.tracer => "lnrpc". "Lightning" / "GetInfo");

        self.lightning
            .get_info(GetInfoRequest {})
            .with_current_context()
            .await
            .map(Response::into_inner)
    }

    pub async fn add_invoice(&mut self, invoice: Invoice) -> Result<AddInvoiceResponse, Status> {
        span!(self.tracer => "lnrpc". "Lightning" / "AddInvoice");

        self.lightning
            .add_invoice(invoice)
            .with_current_context()
            .await
            .map(Response::into_inner)
    }

    pub async fn add_hold_invoice(
        &mut self,
        hash: Vec<u8>,
        value: i64,
    ) -> Result<AddHoldInvoiceResp, Status> {
        span!(self.tracer => "lnrpc". "Invoices" / "AddHoldInvoice");

        self.invoices
            .add_hold_invoice(AddHoldInvoiceRequest {
                hash,
                value,
                ..AddHoldInvoiceRequest::default()
            })
            .with_current_context()
            .await
            .map(Response::into_inner)
    }

    pub async fn cancel_invoice(
        &mut self,
        payment_hash: Vec<u8>,
    ) -> Result<CancelInvoiceResp, Status> {
        span!(self.tracer => "lnrpc". "Invoices" / "CancelInvoice");

        self.invoices
            .cancel_invoice(CancelInvoiceMsg { payment_hash })
            .with_current_context()
            .await
            .map(Response::into_inner)
    }

    pub async fn settle_invoice(&mut self, preimage: Vec<u8>) -> Result<SettleInvoiceResp, Status> {
        span!(self.tracer => "lnrpc". "Invoices" / "SettleInvoice");

        self.invoices
            .settle_invoice(SettleInvoiceMsg { preimage })
            .with_current_context()
            .await
            .map(Response::into_inner)
    }

    pub async fn channel_balance(&mut self) -> Result<ChannelBalanceResponse, Status> {
        span!(self.tracer => "lnrpc". "Lightning" / "ChannelBalance");

        self.lightning
            .channel_balance(ChannelBalanceRequest {})
            .with_current_context()
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
        span!(self.tracer => "lnrpc". "Lightning" / "ListPayments");

        self.lightning
            .list_payments(ListPaymentsRequest {
                include_incomplete,
                index_offset,
                max_payments,
                reversed,
                ..ListPaymentsRequest::default()
            })
            .with_current_context()
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
        span!(self.tracer => "lnrpc". "Lightning" / "ListInvoices");

        self.lightning
            .list_invoices(ListInvoiceRequest {
                pending_only,
                index_offset,
                num_max_invoices,
                reversed,
                ..ListInvoiceRequest::default()
            })
            .with_current_context()
            .await
            .map(Response::into_inner)
    }

    pub async fn send_payment_sync(
        &mut self,
        send_request: SendRequest,
    ) -> Result<SendResponse, Status> {
        span!(self.tracer => "lnrpc". "Lightning" / "SendPaymentSync");

        self.lightning
            .send_payment_sync(send_request)
            .with_current_context()
            .await
            .map(Response::into_inner)
    }

    pub async fn wallet_balance(&mut self) -> Result<WalletBalanceResponse, Status> {
        span!(self.tracer => "lnrpc". "Lightning" / "WalletBalance");

        self.lightning
            .wallet_balance(WalletBalanceRequest::default())
            .with_current_context()
            .await
            .map(Response::into_inner)
    }

    pub async fn forwarding_history(
        &mut self,
        req: ForwardingHistoryRequest,
    ) -> Result<ForwardingHistoryResponse, Status> {
        span!(self.tracer => "lnrpc". "Lightning" / "ForwardingHistory");

        self.lightning
            .forwarding_history(req)
            .with_current_context()
            .await
            .map(Response::into_inner)
    }

    pub async fn list_channels(
        &mut self,
        req: ListChannelsRequest,
    ) -> Result<ListChannelsResponse, Status> {
        span!(self.tracer => "lnrpc". "Lightning" / "ListChannels");

        self.lightning
            .list_channels(req)
            .with_current_context()
            .await
            .map(Response::into_inner)
    }

    pub async fn closed_channels(
        &mut self,
        req: ClosedChannelsRequest,
    ) -> Result<ClosedChannelsResponse, Status> {
        span!(self.tracer => "lnrpc". "Lightning" / "ClosedChannels");

        self.lightning
            .closed_channels(req)
            .with_current_context()
            .await
            .map(Response::into_inner)
    }

    pub async fn new_address(
        &mut self,
        req: NewAddressRequest,
    ) -> Result<NewAddressResponse, Status> {
        span!(self.tracer => "lnrpc". "Lightning" / "NewAddress");

        self.lightning
            .new_address(req)
            .with_current_context()
            .await
            .map(Response::into_inner)
    }

    pub async fn pending_channels(&mut self) -> Result<PendingChannelsResponse, Status> {
        span!(self.tracer => "lnrpc". "Lightning" / "PendingChannels");

        self.lightning
            .pending_channels(PendingChannelsRequest {})
            .with_current_context()
            .await
            .map(Response::into_inner)
    }

    pub async fn channel_acceptor(
        &mut self,
        req: impl tonic::IntoStreamingRequest<Message = ChannelAcceptResponse>,
    ) -> Result<Streaming<ChannelAcceptRequest>, Status> {
        span!(self.tracer => "lnrpc". "Lightning" / "ChannelAcceptor");

        self.lightning
            .channel_acceptor(req)
            .with_current_context()
            .await
            .map(Response::into_inner)
    }

    pub async fn close_channel(
        &mut self,
        req: CloseChannelRequest,
    ) -> Result<Streaming<CloseStatusUpdate>, Status> {
        span!(self.tracer => "lnrpc". "Lighting" / "CloseChannel");

        self.lightning
            .close_channel(req)
            .with_current_context()
            .await
            .map(Response::into_inner)
    }

    pub async fn send_coins(&mut self, req: SendCoinsRequest) -> Result<SendCoinsResponse, Status> {
        span!(self.tracer => "lnrpc". "Lightning" / "SendCoins");

        self.lightning
            .send_coins(req)
            .with_current_context()
            .await
            .map(Response::into_inner)
    }
}

impl Lnd {
    pub async fn send_payment(
        &mut self,
        req: SendPaymentRequest,
    ) -> Result<Streaming<Payment>, Status> {
        span!(self.tracer => "lnrpc". "Router" / "SendPaymentV2");

        self.router
            .send_payment_v2(req)
            .with_current_context()
            .await
            .map(Response::into_inner)
    }

    pub async fn track_payment(
        &mut self,
        payment_hash: Vec<u8>,
        no_inflight_updates: bool,
    ) -> Result<Streaming<Payment>, Status> {
        span!(self.tracer => "lnrpc". "Router" / "TrackPaymentV2");

        self.router
            .track_payment_v2(TrackPaymentRequest {
                no_inflight_updates,
                payment_hash,
            })
            .with_current_context()
            .await
            .map(Response::into_inner)
    }
}

#[async_trait::async_trait]
pub trait LookupInvoice {
    async fn lookup_invoice(&mut self, payment_hash: Vec<u8>) -> Result<Invoice, Status>;
}

#[async_trait::async_trait]
impl LookupInvoice for Lnd {
    async fn lookup_invoice(&mut self, payment_hash: Vec<u8>) -> Result<Invoice, Status> {
        span!(self.tracer => "lnrpc". "Invoices" / "LookupInvoiceV2");

        self.invoices
            .lookup_invoice_v2(LookupInvoiceMsg {
                lookup_modifier: LookupModifier::Default as i32,
                invoice_ref: Some(InvoiceRef::PaymentHash(payment_hash)),
            })
            .with_current_context()
            .await
            .map(Response::into_inner)
    }
}

#[async_trait::async_trait]
pub trait SubscribeSingleInvoice {
    async fn subscribe_single_invoice(
        &mut self,
        r_hash: Vec<u8>,
    ) -> Result<Streaming<Invoice>, Status>;
}

#[async_trait::async_trait]
impl SubscribeSingleInvoice for Lnd {
    async fn subscribe_single_invoice(
        &mut self,
        r_hash: Vec<u8>,
    ) -> Result<Streaming<Invoice>, Status> {
        span!(self.tracer => "lnrpc". "Invoices" / "SubscribeSingleInvoice");

        self.invoices
            .subscribe_single_invoice(SubscribeSingleInvoiceRequest { r_hash })
            .with_current_context()
            .await
            .map(Response::into_inner)
    }
}

#[async_trait::async_trait]
pub trait DecodePayReq {
    async fn decode_pay_req(&mut self, pay_req: String) -> Result<PayReq, Status>;
}

#[async_trait::async_trait]
impl DecodePayReq for Lnd {
    async fn decode_pay_req(&mut self, pay_req: String) -> Result<PayReq, Status> {
        span!(self.tracer => "lnrpc". "Lightning" / "DecodePayReq");

        self.lightning
            .decode_pay_req(PayReqString { pay_req })
            .with_current_context()
            .await
            .map(Response::into_inner)
    }
}
