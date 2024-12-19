use anyhow::Result;
use solana_transaction_status::option_serializer::OptionSerializer;
use tracing::info;
use super::MonitorRule;

impl MonitorRule {
    pub async fn deal_profit_holding(&self, sig: &str) -> Result<()> {
        todo!()
        // 1. get tx use sig
        // let meta = solana_client.get_tx(&sig).await?;
        // if let OptionSerializer::Some(post_token_balances) = meta.post_token_balances {
        //     for post_token_balance in post_token_balances {
        //         if post_token_balance.owner == OptionSerializer::Some(self.address.clone()) {
        //             let token_address = post_token_balance.mint;
        //             let buy_amount = post_token_balance.ui_token_amount;
        //             info!(
        //                 "deal_profit_holding: token_address: {}, buy_amount: {}",
        //                 token_address, buy_amount.ui_amount_string
        //             );
        //         }
        //     }
        // }
        // Ok(())

        // if let Some(profit_percentage) = self.conditions.profit_percentage {
        //     if rpcLogs.current_profit_percentage > profit_percentage {
        //         return true;
        //     }
        // }
    }
}
