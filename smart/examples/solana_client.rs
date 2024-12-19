// use start_monitoring

use std::env;

use anyhow::Result;
use smart::{
    sol_client::client::SolanaMonitor,
    strategies::{MonitorCondition, MonitorRule, MonitorRuleType},
};
use solana_client::rpc_response::RpcLogsResponse;
use tracing::{debug, error, info};
use utils::log::init_tracing;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    init_tracing();
    let mr = MonitorRule {
        address: "ASxMiMb1AJGTU4AduPNB2CGqT1TiDqWkLvy7oCUnzw5x".to_string(),
        rule_type: MonitorRuleType::Buy,
        conditions: MonitorCondition {
            price_below: Some(1.0),
            price_above: Some(2.0),
            profit_percentage: Some(3.0),
            is_first_sell: Some(true),
            partial_sell: Some(true),
            holding_percentage: Some(4.0),
        },
    };
    let wss = env::var("WSS_SOLANA_URL")?;
    let rpc = env::var("RPC_SOLANA_URL")?;

    let (sender, mut receiver) = tokio::sync::mpsc::channel::<RpcLogsResponse>(1000);

    // start monitoring in a new task
    tokio::spawn(async move {
        let sm = SolanaMonitor::new(&wss, &rpc);
        sm.start_log_subscribe(&mr.address, sender).await.unwrap();
    });

    // receive logs
    while let Some(log) = receiver.recv().await {
        info!("Received log: {:?}", log);
    }
    Ok(())
}
