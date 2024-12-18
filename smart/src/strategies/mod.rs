use anyhow::Result;
use serde::Deserialize;
use std::sync::Arc;
use tokio::{
    fs,
    sync::{OnceCell, RwLock},
    time::{interval, Duration},
};
use validator::Validate;

use crate::config::get_global_config;

#[derive(Clone, Debug, Validate, Deserialize)]
pub struct MonitorRule {
    #[validate(length(min = 1))]
    pub address: String, // 要监控的地址
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
    pub fn should_alert(&self, transaction_info: &TransactionInfo) -> bool {
        match self.rule_type {
            MonitorRuleType::Buy => {
                if let Some(price_below) = self.conditions.price_below {
                    if transaction_info.price < price_below {
                        return true;
                    }
                }
            }
            MonitorRuleType::Sell => {
                if self.conditions.is_first_sell.unwrap_or(false)
                    && self.conditions.partial_sell.unwrap_or(false)
                {
                    // 检查是否是首次卖出且部分卖出
                    return true;
                }
            }
            MonitorRuleType::ProfitHolding => {
                if let Some(profit_percentage) = self.conditions.profit_percentage {
                    if transaction_info.current_profit_percentage > profit_percentage {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Strategy {
    // 你的策略配置结构
    pub rules: Vec<MonitorRule>,
}

pub struct StrategyManager {
    strategies: Arc<RwLock<Strategy>>,
    file_path: String,
}

impl StrategyManager {
    pub async fn new(file_path: String) -> Result<Self> {
        // 初始加载策略文件
        let content = fs::read_to_string(&file_path).await?;
        let strategies = toml::from_str(&content)?;

        Ok(Self {
            strategies: Arc::new(RwLock::new(strategies)),
            file_path,
        })
    }

    pub async fn start_auto_reload(self: &Arc<Self>, interval_secs: u32) {
        let manager = self.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(interval_secs as u64));
            loop {
                interval.tick().await;
                if let Err(e) = manager.reload().await {
                    tracing::error!("Failed to reload strategies: {}", e);
                }
            }
        });
    }

    async fn reload(&self) -> Result<()> {
        let content = fs::read_to_string(&self.file_path).await?;
        let new_strategies: Strategy = toml::from_str(&content)?;

        let mut strategies = self.strategies.write().await;
        *strategies = new_strategies;
        Ok(())
    }

    pub async fn get_strategies(&self) -> Arc<Strategy> {
        Arc::new(self.strategies.read().await.clone())
    }
}

// 全局单例
static STRATEGY_MANAGER: OnceCell<Arc<StrategyManager>> = OnceCell::const_new();

pub async fn init_strategy_manager() -> Result<()> {
    let config = get_global_config().await;
    let manager = Arc::new(StrategyManager::new(config.strategies_file_path.clone()).await?);

    // 启动自动重载
    manager
        .start_auto_reload(config.upload_strategy_file_interval)
        .await;

    STRATEGY_MANAGER
        .set(manager)
        .map_err(|_| anyhow::anyhow!("StrategyManager already initialized"))?;
    Ok(())
}

pub async fn get_global_strategies() -> &'static Arc<StrategyManager> {
    STRATEGY_MANAGER
        .get()
        .expect("StrategyManager not initialized")
}

/*
#[tokio::main]
async fn main() -> Result<()> {
    // 初始化策略管理器
    init_strategy_manager().await?;

    // 在需要使用策略的地方
    let manager = get_global_strategies().await;
    let strategies = manager.get_strategies().await;

    // 使用策略
    for rule in &strategies.rules {
        // ... 处理规则
    }

    Ok(())
}
*/
