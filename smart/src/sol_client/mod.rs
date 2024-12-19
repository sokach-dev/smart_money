pub mod client;

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD as base64, Engine};
use hex;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use tracing::{debug, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeEventData {
    pub mint: String,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub is_buy: bool,
    pub user: String,
    pub timestamp: i64,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeEvent {
    pub name: String,
    pub data: TradeEventData,
}

pub fn parse_program_data(program_data: &str) -> Result<Vec<TradeEvent>> {
    // Remove "Program data: " prefix
    let data = program_data
        .strip_prefix("Program data: ")
        .ok_or_else(|| anyhow!("Invalid program data format"))?;

    // Decode base64
    let decoded = base64.decode(data)?;
    /* TradeEvent 和decoded的字节的关系如下：
       event_flag: 8 byte
       mint: 32 byte
       solAmount: 8 byte
       tokenAmount: 8 byte
       isBuy: 1 byte
       user: 32 byte
       timestamp: 8 byte
       virtualSolReserves: 8 byte
       virtualTokenReserves: 8 byte
       realSolReserves: 8 byte
       realTokenReserves: 8 byte
    */

    // 8 + 32 + 8 + 8 + 1 + 32 + 8 + 8 + 8 + 8 + 8 = 129
    if decoded.len() % 129 != 0 {
        return Err(anyhow!(format!(
            "Invalid program data length: {}",
            decoded.len()
        )));
    }
    // decode
    let trade_events: Vec<TradeEvent> = decoded
        .chunks(129)
        .map(|chunk| {
            let mint_arr: [u8; 32] = chunk[8..40].try_into().unwrap();
            let mint = Pubkey::new_from_array(mint_arr).to_string();
            let sol_amount = u64::from_le_bytes(chunk[40..48].try_into().unwrap());
            let token_amount = u64::from_le_bytes(chunk[48..56].try_into().unwrap());
            let is_buy = chunk[56] != 0;
            let user_arr: [u8; 32] = chunk[57..89].try_into().unwrap();
            let user = Pubkey::new_from_array(user_arr).to_string();
            let timestamp = i64::from_le_bytes(chunk[89..97].try_into().unwrap());
            let virtual_sol_reserves = u64::from_le_bytes(chunk[97..105].try_into().unwrap());
            let virtual_token_reserves = u64::from_le_bytes(chunk[105..113].try_into().unwrap());
            let real_sol_reserves = u64::from_le_bytes(chunk[113..121].try_into().unwrap());
            let real_token_reserves = u64::from_le_bytes(chunk[121..129].try_into().unwrap());

            TradeEvent {
                name: "TradeEvent".to_string(),
                data: TradeEventData {
                    mint,
                    sol_amount,
                    token_amount,
                    is_buy,
                    user,
                    timestamp,
                    virtual_sol_reserves,
                    virtual_token_reserves,
                    real_sol_reserves,
                    real_token_reserves,
                },
            }
        })
        .collect();

    Ok(trade_events)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program_data() -> Result<()> {
        /*
        {
            "name": "TradeEvent",
            "data": {
                "mint":"7R4zU5pgHFxRQaNUhhCAPFXaSN6AWiheD6rRfkFJpump",
                "solAmount": 1253951806,
                "tokenAmount": 37809162736217,
                "isBuy":  true,
                "user":  "ASxMiMb1AJGTU4AduPNB2CGqT1TiDqWkLvy7oCUnzw5x",
                "timestamp": 1734616564,
                "virtualSolReserves": 33306996548,
                "virtualTokenReserves": 966463606623031,
                "realSolReserves": 3306996548,
                "realTokenReserves": 686563606623031,
            }
        }
         */
        let program_data = "Program data: vdt/007mYe5fUJLKQBnZyU5a25rXFCHmUq3eDeg/6m3qXr6Y4LVhXz7JvUoAAAAAWdK2IWMiAAABjF9LiRHyIjjqqF93tZIAeB6MsYzDh6xG1Oi/PnwVBw/0JWRnAAAAAERvQMEHAAAANwv2V/5uAwBEwxzFAAAAADdz4wttcAIA";

        let events = parse_program_data(program_data)?;

        println!("{:?}", events);
        assert_eq!(events.len(), 1);
        let event = events[0].clone();
        assert_eq!(event.name, "TradeEvent");
        assert_eq!(
            event.data.mint,
            "7R4zU5pgHFxRQaNUhhCAPFXaSN6AWiheD6rRfkFJpump"
        );
        assert_eq!(event.data.sol_amount, 1253951806);
        assert_eq!(event.data.token_amount, 37809162736217);
        assert_eq!(event.data.is_buy, true);
        assert_eq!(
            event.data.user,
            "ASxMiMb1AJGTU4AduPNB2CGqT1TiDqWkLvy7oCUnzw5x"
        );
        assert_eq!(event.data.timestamp, 1734616564);
        assert_eq!(event.data.virtual_sol_reserves, 33306996548);
        assert_eq!(event.data.virtual_token_reserves, 966463606623031);
        assert_eq!(event.data.real_sol_reserves, 3306996548);
        assert_eq!(event.data.real_token_reserves, 686563606623031);

        Ok(())
    }
}
