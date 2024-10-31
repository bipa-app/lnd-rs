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
pub use gen::{chainrpc, invoicesrpc, lnrpc, routerrpc};
use gen::{
    chainrpc::{chain_kit_client::ChainKitClient, GetBestBlockRequest, GetBestBlockResponse},
    invoicesrpc::{
        invoices_client::InvoicesClient, lookup_invoice_msg::InvoiceRef, AddHoldInvoiceRequest,
        AddHoldInvoiceResp, CancelInvoiceMsg, CancelInvoiceResp, LookupInvoiceMsg, LookupModifier,
        SettleInvoiceMsg, SettleInvoiceResp, SubscribeSingleInvoiceRequest,
    },
    lnrpc::{
        lightning_client::LightningClient, AddInvoiceResponse, BatchOpenChannelRequest,
        BatchOpenChannelResponse, ChannelAcceptRequest, ChannelAcceptResponse,
        ChannelBalanceRequest, ChannelBalanceResponse, CloseChannelRequest, CloseStatusUpdate,
        ClosedChannelsRequest, ClosedChannelsResponse, ConnectPeerRequest, ConnectPeerResponse,
        ForwardingHistoryRequest, ForwardingHistoryResponse, GetInfoRequest, GetInfoResponse,
        GetTransactionsRequest, Invoice, ListChannelsRequest, ListChannelsResponse,
        ListInvoiceRequest, ListInvoiceResponse, ListPaymentsRequest, ListPaymentsResponse,
        NewAddressRequest, NewAddressResponse, PayReq, PayReqString, Payment,
        PendingChannelsRequest, PendingChannelsResponse, SendCoinsRequest, SendCoinsResponse,
        SendRequest, SendResponse, TransactionDetails, WalletBalanceRequest, WalletBalanceResponse,
    },
    routerrpc::{router_client::RouterClient, SendPaymentRequest, TrackPaymentRequest},
};
use opentelemetry::trace::{FutureExt, TraceContextExt};
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
    chainkit: ChainKitClient<InterceptedService<Channel, LndInterceptor>>,
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
        let router = RouterClient::with_interceptor(channel.clone(), interceptor.clone());
        let chainkit = ChainKitClient::with_interceptor(channel, interceptor);

        Lnd {
            lightning,
            invoices,
            router,
            chainkit,
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
    pub async fn get_info(&self) -> Result<GetInfoResponse, Status> {
        let span = span!(self.tracer => "lnrpc". "Lightning" / "GetInfo");

        self.clone()
            .lightning
            .get_info(GetInfoRequest {})
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn add_invoice(&self, invoice: Invoice) -> Result<AddInvoiceResponse, Status> {
        let span = span!(self.tracer => "lnrpc". "Lightning" / "AddInvoice");

        self.clone()
            .lightning
            .add_invoice(invoice)
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn add_hold_invoice(
        &self,
        hash: Vec<u8>,
        value: i64,
    ) -> Result<AddHoldInvoiceResp, Status> {
        let span = span!(self.tracer => "lnrpc". "Invoices" / "AddHoldInvoice");

        self.invoices
            .clone()
            .add_hold_invoice(AddHoldInvoiceRequest {
                hash,
                value,
                ..AddHoldInvoiceRequest::default()
            })
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn cancel_invoice(&self, payment_hash: Vec<u8>) -> Result<CancelInvoiceResp, Status> {
        let span = span!(self.tracer => "lnrpc". "Invoices" / "CancelInvoice");

        self.invoices
            .clone()
            .cancel_invoice(CancelInvoiceMsg { payment_hash })
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn settle_invoice(&self, preimage: Vec<u8>) -> Result<SettleInvoiceResp, Status> {
        let span = span!(self.tracer => "lnrpc". "Invoices" / "SettleInvoice");

        self.invoices
            .clone()
            .settle_invoice(SettleInvoiceMsg { preimage })
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn channel_balance(&self) -> Result<ChannelBalanceResponse, Status> {
        let span = span!(self.tracer => "lnrpc". "Lightning" / "ChannelBalance");

        self.lightning
            .clone()
            .channel_balance(ChannelBalanceRequest {})
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn list_payments(
        &self,
        include_incomplete: bool,
        index_offset: u64,
        max_payments: u64,
        reversed: bool,
    ) -> Result<ListPaymentsResponse, Status> {
        let span = span!(self.tracer => "lnrpc". "Lightning" / "ListPayments");

        self.lightning
            .clone()
            .list_payments(ListPaymentsRequest {
                include_incomplete,
                index_offset,
                max_payments,
                reversed,
                ..ListPaymentsRequest::default()
            })
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn list_invoices(
        &self,
        pending_only: bool,
        index_offset: u64,
        num_max_invoices: u64,
        reversed: bool,
    ) -> Result<ListInvoiceResponse, Status> {
        let span = span!(self.tracer => "lnrpc". "Lightning" / "ListInvoices");

        self.lightning
            .clone()
            .list_invoices(ListInvoiceRequest {
                pending_only,
                index_offset,
                num_max_invoices,
                reversed,
                ..ListInvoiceRequest::default()
            })
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn send_payment_sync(
        &self,
        send_request: SendRequest,
    ) -> Result<SendResponse, Status> {
        let span = span!(self.tracer => "lnrpc". "Lightning" / "SendPaymentSync");

        self.lightning
            .clone()
            .send_payment_sync(send_request)
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn wallet_balance(&self) -> Result<WalletBalanceResponse, Status> {
        let span = span!(self.tracer => "lnrpc". "Lightning" / "WalletBalance");

        self.lightning
            .clone()
            .wallet_balance(WalletBalanceRequest::default())
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn forwarding_history(
        &self,
        req: ForwardingHistoryRequest,
    ) -> Result<ForwardingHistoryResponse, Status> {
        let span = span!(self.tracer => "lnrpc". "Lightning" / "ForwardingHistory");

        self.lightning
            .clone()
            .forwarding_history(req)
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn list_channels(
        &self,
        req: ListChannelsRequest,
    ) -> Result<ListChannelsResponse, Status> {
        let span = span!(self.tracer => "lnrpc". "Lightning" / "ListChannels");

        self.lightning
            .clone()
            .list_channels(req)
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn closed_channels(
        &self,
        req: ClosedChannelsRequest,
    ) -> Result<ClosedChannelsResponse, Status> {
        let span = span!(self.tracer => "lnrpc". "Lightning" / "ClosedChannels");

        self.lightning
            .clone()
            .closed_channels(req)
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn new_address(&self, req: NewAddressRequest) -> Result<NewAddressResponse, Status> {
        let span = span!(self.tracer => "lnrpc". "Lightning" / "NewAddress");

        self.lightning
            .clone()
            .new_address(req)
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn pending_channels(&self) -> Result<PendingChannelsResponse, Status> {
        let span = span!(self.tracer => "lnrpc". "Lightning" / "PendingChannels");

        self.lightning
            .clone()
            .pending_channels(PendingChannelsRequest {})
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn channel_acceptor(
        &self,
        req: impl tonic::IntoStreamingRequest<Message = ChannelAcceptResponse>,
    ) -> Result<Streaming<ChannelAcceptRequest>, Status> {
        let span = span!(self.tracer => "lnrpc". "Lightning" / "ChannelAcceptor");

        self.lightning
            .clone()
            .channel_acceptor(req)
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn close_channel(
        &self,
        req: CloseChannelRequest,
    ) -> Result<Streaming<CloseStatusUpdate>, Status> {
        let span = span!(self.tracer => "lnrpc". "Lighting" / "CloseChannel");

        self.lightning
            .clone()
            .close_channel(req)
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn send_coins(&self, req: SendCoinsRequest) -> Result<SendCoinsResponse, Status> {
        let span = span!(self.tracer => "lnrpc". "Lightning" / "SendCoins");

        self.lightning
            .clone()
            .send_coins(req)
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn get_transactions(
        &self,
        req: GetTransactionsRequest,
    ) -> Result<TransactionDetails, Status> {
        let span = span!(self.tracer => "lnrpc". "Lightning" / "GetTransactions");

        self.lightning
            .clone()
            .get_transactions(req)
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn get_best_block(&self) -> Result<GetBestBlockResponse, Status> {
        let span = span!(self.tracer => "lnrpc". "ChainKit" / "GetBestBlock");

        self.chainkit
            .clone()
            .get_best_block(GetBestBlockRequest::default())
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn connect_to_peer(
        &self,
        req: ConnectPeerRequest,
    ) -> Result<ConnectPeerResponse, Status> {
        let span = span!(self.tracer => "lnrpc". "Lightning" / "ConnectPeer");

        self.lightning
            .clone()
            .connect_peer(req)
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn batch_open_channel(
        &self,
        req: BatchOpenChannelRequest,
    ) -> Result<BatchOpenChannelResponse, Status> {
        let span = span!(self.tracer => "lnrpc". "Lightning" / "BatchOpenChannel");

        self.lightning
            .clone()
            .batch_open_channel(req)
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }
}

impl Lnd {
    pub async fn send_payment(
        &self,
        req: SendPaymentRequest,
    ) -> Result<Streaming<Payment>, Status> {
        let span = span!(self.tracer => "lnrpc". "Router" / "SendPaymentV2");

        self.router
            .clone()
            .send_payment_v2(req)
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }

    pub async fn track_payment(
        &self,
        payment_hash: Vec<u8>,
        no_inflight_updates: bool,
    ) -> Result<Streaming<Payment>, Status> {
        let span = span!(self.tracer => "lnrpc". "Router" / "TrackPaymentV2");

        self.router
            .clone()
            .track_payment_v2(TrackPaymentRequest {
                no_inflight_updates,
                payment_hash,
            })
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }
}

pub trait LookupInvoice {
    fn lookup_invoice(
        &self,
        payment_hash: Vec<u8>,
    ) -> impl std::future::Future<Output = Result<Invoice, Status>> + Send;
}

impl LookupInvoice for Lnd {
    async fn lookup_invoice(&self, payment_hash: Vec<u8>) -> Result<Invoice, Status> {
        let span = span!(self.tracer => "lnrpc". "Invoices" / "LookupInvoiceV2");

        self.invoices
            .clone()
            .lookup_invoice_v2(LookupInvoiceMsg {
                lookup_modifier: LookupModifier::Default as i32,
                invoice_ref: Some(InvoiceRef::PaymentHash(payment_hash)),
            })
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }
}

pub trait SubscribeSingleInvoice {
    fn subscribe_single_invoice(
        &self,
        r_hash: Vec<u8>,
    ) -> impl std::future::Future<Output = Result<Streaming<Invoice>, Status>> + Send;
}

impl SubscribeSingleInvoice for Lnd {
    async fn subscribe_single_invoice(
        &self,
        r_hash: Vec<u8>,
    ) -> Result<Streaming<Invoice>, Status> {
        let span = span!(self.tracer => "lnrpc". "Invoices" / "SubscribeSingleInvoice");

        self.invoices
            .clone()
            .subscribe_single_invoice(SubscribeSingleInvoiceRequest { r_hash })
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }
}

pub trait DecodePayReq {
    fn decode_pay_req(
        &self,
        pay_req: String,
    ) -> impl std::future::Future<Output = Result<PayReq, Status>> + Send;
}

impl DecodePayReq for Lnd {
    async fn decode_pay_req(&self, pay_req: String) -> Result<PayReq, Status> {
        let span = span!(self.tracer => "lnrpc". "Lightning" / "DecodePayReq");

        self.lightning
            .clone()
            .decode_pay_req(PayReqString { pay_req })
            .with_context(opentelemetry::Context::current_with_span(span))
            .await
            .map(Response::into_inner)
    }
}
