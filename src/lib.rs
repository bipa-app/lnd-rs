/// Module including all tonic-build generated code.
/// Each sub-module represents one proto service.
pub mod lnrpc;

use std::convert::TryInto;

use lnrpc::lnrpc::{
    lightning_client::LightningClient, ChannelBalanceRequest, ChannelBalanceResponse,
    WalletBalanceRequest, WalletBalanceResponse,
};
use tonic::{
    codegen::StdError,
    transport::{Channel, Endpoint},
    Response, Status,
};

pub struct Lnd {
    lightning_client: LightningClient<Channel>,
}

impl Lnd {
    pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
        D: TryInto<Endpoint>,
        D::Error: Into<StdError>,
    {
        let lightning_client = LightningClient::connect(dst).await?;
        Ok(Lnd { lightning_client })
    }

    pub async fn wallet_balance(&mut self) -> Result<Response<WalletBalanceResponse>, Status> {
        self.lightning_client
            .wallet_balance(WalletBalanceRequest {})
            .await
    }

    pub async fn channel_balance(&mut self) -> Result<Response<ChannelBalanceResponse>, Status> {
        self.lightning_client
            .channel_balance(ChannelBalanceRequest {})
            .await
    }
}
