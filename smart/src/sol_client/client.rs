use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use solana_client::{
    rpc_client::RpcClient,
    rpc_config,
    rpc_response::{Response, RpcLogsResponse},
};
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use solana_transaction_status::{UiTransactionEncoding, UiTransactionStatusMeta};
use std::str::FromStr;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{debug, error, info};

pub struct SolanaMonitor {
    websocket_url: String,
    rpc_client: RpcClient,
}

impl SolanaMonitor {
    pub fn new(websocket_url: &str, rpc_url: &str) -> Self {
        Self {
            websocket_url: websocket_url.to_string(),
            rpc_client: RpcClient::new(rpc_url.to_string()),
        }
    }

    pub async fn start_log_subscribe(
        &self,
        address: &str,
        sender: Sender<RpcLogsResponse>,
    ) -> Result<()> {
        // let (a, b) = PubsubClient::logs_subscribe(url, filter, config).await?;
        info!("Started monitoring address: {}", address);
        // 实现订阅日志
        let sub_msg = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "logsSubscribe",
            "params": [
                {
                    "mentions": [address]
                },
                {
                    "commitment": "confirmed"
                }
            ]
        });
        let (ws_stream, _) = connect_async(&self.websocket_url).await?;
        let (mut write, mut read) = ws_stream.split();

        //  subscribe
        write.send(Message::text(sub_msg.to_string())).await?;
        info!("Subscribe logs subcribe successfully!");

        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    let v: Value = serde_json::from_str(&text)?;
                    if let Some(params) = v.get("params") {
                        if let Some(result) = params.get("result") {
                            if let Ok(log) =
                                serde_json::from_value::<Response<RpcLogsResponse>>(result.clone())
                            {
                                if log.value.err.is_none() {
                                    if let Err(e) = sender.send(log.value.clone()).await {
                                        error!("Error sending message: {:?}", e);
                                    } else {
                                        info!(
                                            "Send message: {:?}, capital: {}",
                                            log.value,
                                            sender.capacity()
                                        );
                                    }
                                } else {
                                    error!("Error receiving message: {:?}", log.value.err);
                                }
                            } else {
                                debug!("Receive can't parse json message: {:?}", result);
                            }
                        } else {
                            debug!("Receive not result message: {:?}", params);
                        }
                    } else {
                        debug!("Receive not params message: {:?}", v);
                    }
                }
                Ok(_) => {
                    info!("Receive not text message: {:?}", msg);
                }
                Err(e) => {
                    error!("Error receiving message: {:?}", e);
                }
            }
        }

        Ok(())
    }

    pub async fn get_tx(&self, sig: &str) -> Result<UiTransactionStatusMeta> {
        // 实现获取交易信息
        let sig = Signature::from_str(sig)?;
        let tx = self.rpc_client.get_transaction_with_config(
            &sig,
            rpc_config::RpcTransactionConfig {
                encoding: Some(UiTransactionEncoding::Json),
                commitment: Some(CommitmentConfig::confirmed()),
                max_supported_transaction_version: None,
            },
        )?;
        if let Some(meta) = tx.transaction.meta {
            if let Some(e) = meta.err {
                anyhow::bail!("Transaction error: {:?}", e);
            }
            return Ok(meta);
        }
        anyhow::bail!("Transaction not found")
    }

    pub fn parse_buy_info(&self, meta: UiTransactionStatusMeta) -> Result<()> {
        Ok(())
    }
}
