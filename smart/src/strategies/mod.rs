use anyhow::Result;
use serde::Deserialize;
use solana_client::rpc_response::RpcLogsResponse;
use tokio::sync::mpsc;
use tracing::{debug, error};

use crate::{config::get_global_config, sol_client::client::SolanaMonitor};

mod profit_holding;

#[derive(Clone, Debug, Deserialize)]
pub struct MonitorRule {
    pub address: String,              // 监控地址
    pub rule_type: MonitorRuleType,   // 监控规则类型
    pub conditions: MonitorCondition, // 触发条件
}

#[derive(Clone, Debug, Deserialize)]
pub enum MonitorRuleType {
    Buy,           // 买入监控
    Sell,          // 卖出监控
    ProfitHolding, // 持仓收益监控
}

#[derive(Clone, Debug, Deserialize)]
pub struct MonitorCondition {
    pub price_below: Option<f64>,        // 价格低于
    pub price_above: Option<f64>,        // 价格高于
    pub profit_percentage: Option<f64>,  // 收益百分比
    pub is_first_sell: Option<bool>,     // 是否首次卖出
    pub partial_sell: Option<bool>,      // 是否部分卖出
    pub holding_percentage: Option<f64>, // 持仓百分比
}

impl MonitorRule {
    pub async fn should_alert(&self) -> Result<()> {
        let c = get_global_config().await;
        match self.rule_type {
            MonitorRuleType::Buy => {
                todo!()
                // if let Some(price_below) = self.conditions.price_below {
                //     if rpcLogs.price < price_below {
                //         return true;
                //     }
                // }
            }
            MonitorRuleType::Sell => {
                todo!()
                // if self.conditions.is_first_sell.unwrap_or(false)
                //     && self.conditions.partial_sell.unwrap_or(false)
                // {
                //     // 检查是否是首次卖出且部分卖出
                //     return true;
                // }
            }
            MonitorRuleType::ProfitHolding => {
                let buy_flag = "Program log: Instruction: Buy".to_string();
                let data_flag = "Program data".to_string();
                let (sender, mut receiver) = mpsc::channel::<RpcLogsResponse>(1000);

                let address = self.address.clone();
                tokio::spawn(async move {
                    let solana_client = SolanaMonitor::new(&c.solana_wss_url, &c.solana_rpc_url);
                    solana_client
                        .start_log_subscribe(&address, sender)
                        .await
                        .unwrap();
                });

                while let Some(log) = receiver.recv().await {
                    debug!("log: {:?}", log);
                    if log.logs.contains(&buy_flag) {
                        if let Err(e) = self
                            .deal_profit_holding(&log.signature)
                            // .deal_profit_holding(&solana_client, &log.signature)
                            .await
                        {
                            error!(
                                "deal_profit_holding error: {:?}, address: {}",
                                e, self.address
                            );
                        }
                    }
                }
                Ok(())
            }
        }
    }
}
