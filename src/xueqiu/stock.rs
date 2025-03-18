use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Xueqiu {
    pub detail_url: String,
    pub industry_url: String,
    pub minute_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Detail {
    pub data: Data,
    pub error_code: i64,
    pub error_description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub market: Market,
    pub quote: Quote,
    pub others: Others,
    pub tags: Vec<Tag>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Market {
    pub status_id: i64,
    pub region: String,
    pub status: String,
    pub time_zone: String,
    pub time_zone_desc: Option<String>,
    pub delay_tag: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    pub current_ext: Option<f64>,
    pub symbol: String,
    pub volume_ext: Option<f64>,
    pub high52w: Option<f64>,
    pub delayed: Option<i64>,
    pub r#type: Option<i64>,
    pub tick_size: Option<f64>,
    pub float_shares: Option<f64>,
    pub limit_down: Option<f64>,
    pub no_profit: Option<String>,
    pub high:Option<f64>,
    pub float_market_capital: Option<f64>,
    pub timestamp_ext: Option<String>,
    pub lot_size: Option<i64>,
    pub lock_set: Option<i64>,
    pub weighted_voting_rights: Option<String>,
    pub chg: Option<f64>,
    pub eps: Option<f64>,
    pub last_close: Option<f64>,
    pub profit_four: Option<f64>,
    pub volume: Option<i64>,
    pub volume_ratio: Option<f64>,
    pub profit_forecast: Option<f64>,
    pub turnover_rate: Option<f64>,
    pub low52w: Option<f64>,
    pub name: Option<String>,
    pub exchange: Option<String>,
    pub pe_forecast: Option<f64>,
    pub total_shares: Option<f64>,
    pub status: Option<i64>,
    pub is_vie_desc: Option<String>,
    pub security_status: Option<String>,
    pub code: Option<String>,
    pub goodwill_in_net_assets: Option<f64>,
    pub avg_price: Option<f64>,
    pub percent: Option<f64>,
    pub weighted_voting_rights_desc: Option<String>,
    pub amplitude: Option<f64>,
    pub current: Option<f64>,
    pub is_vie: Option<String>,
    pub current_year_percent: Option<f64>,
    pub issue_date: Option<i64>,
    pub sub_type: Option<String>,
    pub low: Option<f64>,
    pub is_registration_desc:Option<String>,
    pub no_profit_desc: Option<String>,
    pub market_capital: Option<f64>,
    pub dividend: Option<f64>,
    pub dividend_yield: Option<f64>,
    pub currency: Option<String>,
    pub navps: Option<f64>,
    pub profit: Option<f64>,
    pub timestamp: i64,
    pub pe_lyr: Option<f64>,
    pub amount: Option<f64>,
    pub pledge_ratio: Option<f64>,
    pub traded_amount_ext: Option<String>,
    pub is_registration: Option<String>,
    pub pb: Option<f64>,
    pub limit_up: Option<f64>,
    pub pe_ttm: Option<f64>,
    pub time: i64,
    pub open: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Others {
    pub pankou_ratio: f64,
    pub cyb_switch: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub description: String,
    pub value: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Minute {
    pub data: MinuteData,
    pub error_code: i64,
    pub error_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinuteData {
    // #[serde(rename = "last_close")]
    pub last_close: f64,
    pub after: Vec<serde_json::Value>, // 空数组，使用通用类型处理
    pub items: Vec<Item>,
    pub items_size: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub current: Option<f64>,
    pub volume: Option<f64>,
    pub avg_price: Option<f64>,
    pub chg: Option<f64>,
    pub percent: Option<f64>,
    pub timestamp: i64,
    pub amount: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub amount_total: Option<f64>,
    pub volume_total: Option<f64>,
    pub macd: Option<f64>,  // 示例中为null，使用Option
    pub kdj: Option<f64>,   // 示例中为null，使用Option
    pub ratio: Option<f64>, // 示例中为null，使用Option
    pub capital: Option<Capital>,
    pub volume_compare: Option<VolumeCompare>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Capital {
    pub small: f64,
    pub medium: f64,
    pub large: f64,
    pub xlarge: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeCompare {
    pub volume_sum: f64,
    pub volume_sum_last: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranMinute {
    pub current: Option<f64>,
    pub volume: Option<f64>,
    pub avg_price: Option<f64>,
    pub chg: Option<f64>,
    pub percent: Option<f64>,
    pub timestamp: i64,
    pub amount: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub amount_total: Option<f64>,
    pub volume_total: Option<f64>,
    pub macd: Option<f64>,
    pub kdj: Option<f64>,
    pub ratio: Option<f64>,
    pub capital_small: Option<f64>,
    pub capital_medium: Option<f64>,
    pub capital_large: Option<f64>,
    pub capital_xlarge: Option<f64>,
    pub volume_compare_volume_sum: Option<f64>,
    pub volume_compare_volume_sum_last: Option<f64>,
    pub symbol: String,
}
impl TranMinute {
    pub fn new(minute: &Item, name: &String) -> Self {
        TranMinute {
            current: minute.current,
            volume: minute.volume,
            avg_price: minute.avg_price,
            chg: minute.chg,
            percent: minute.percent,
            timestamp: minute.timestamp,
            amount: minute.amount,
            high: minute.high,
            low: minute.low,
            amount_total: minute.amount_total,
            volume_total: minute.volume_total,
            macd: minute.macd,
            kdj: minute.kdj,
            ratio: minute.ratio,
            capital_small: minute.capital.as_ref().map(|c| c.small),
            capital_medium: minute.capital.as_ref().map(|c| c.medium),
            capital_large: minute.capital.as_ref().map(|c| c.large),
            capital_xlarge: minute.capital.as_ref().map(|c| c.xlarge),
            volume_compare_volume_sum: minute.volume_compare.as_ref().map(|c| c.volume_sum),
            volume_compare_volume_sum_last: minute.volume_compare.as_ref().map(|c| c.volume_sum_last),
            symbol: name.clone(),
        }
    }
}
