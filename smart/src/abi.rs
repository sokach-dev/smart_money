pub struct TransactionInfo {
    pub price: f64,                     // 交易价格
    pub current_profit_percentage: f64, // 当前收益百分比
    pub logs: Vec<String>,              // 交易日志
    pub signature: String,              // 交易签名
}
