use std::error::Error;
use std::f32::consts::E;
use std::{thread, time};
use std::time::{Duration, UNIX_EPOCH};

use chrono::TimeZone;
use questdb::ingress::{Buffer, Sender, TimestampMicros, TimestampNanos};
use thirtyfour::error::WebDriverError;
use thirtyfour::stringmatch::StringMatchLength;
use thirtyfour::By;
use thirtyfour::{error::WebDriverResult, WebDriver};
use tokio::time::sleep;
use crate::elements::Xueqiu;
use questdb::ingress::Timestamp;
use crate::tools::symbol::*;
use crate::CONFIG;

use super::stock::{Detail, Item, Minute, Quote, TranMinute};


// code example: SH600000
// if code is None, then get all stock data
pub async  fn exec(driver: &WebDriver, xueqiu: Xueqiu,code:Option<String>) -> WebDriverResult<()> {
    
    let symbols = match code {
        Some(c) => vec![c],
        None => generate_all_shanghai_stock_symbol(true)
    };
    for symbol in symbols {
        println!("\n-----------------------\n请求对象：{:?}\n",symbol);
        //访问详情页
        driver.goto(format!("{}&symbol={}",&xueqiu.detail_url,&symbol)).await?;
        let resp = driver.find_element(By::Tag("pre")).await?.text().await?;
        let detail_resp: Detail =  match serde_json::from_str::<Detail>(&resp) {
            Ok(r) => r,
            Err(e) => {
                if resp.contains("{\"market\":null,\"quote\":null,\"others\":null,\"tags\":null}") {
                    println!("代码不存在:{}\n",&symbol);
                    continue;
                    
                }
                println!("detail响应解析错误:{}\n{}",e,resp);
                continue;
            }
        };
        
        write_quote_to_questdb(&detail_resp.data.quote).await.unwrap();
        
        thread::sleep(Duration::from_secs(2));
        //访问行业页
        // driver.goto(format!("{}&symbol={}",&xueqiu.industry_url,&symbol)).await?;

        //访问分钟数据
        driver.goto(format!("{}?symbol={}&period=1d",&xueqiu.minute_url,symbol)).await?;
        let resp = driver.find_element(By::Tag("pre")).await?.text().await?;
        let minute_resp:Minute =  match serde_json::from_str::<Minute>(&resp) {
            Ok(r) => r,
            Err(e) => {
                println!("分钟数据序列化错误:{}\n{}",e,&resp);
                continue;
            }
        };
        for item in minute_resp.data.items.iter() {
            let tran_minute = TranMinute::new(item,&symbol);
            write_tran_minute_to_questdb(&tran_minute).await.unwrap();
        }
        println!("写入minute成功\n");
        thread::sleep(Duration::from_secs(2));
    }

    Ok(())
    
}

pub async fn write_quote_to_questdb(quote:&Quote) -> Result<(),Box<dyn Error>> {

    let mut sender = Sender::from_conf(format!("{}",&CONFIG.questdb.endpoint))?;
    let mut buffer = Buffer::new();

    buffer
        .table("Quote")?
        .column_i64("time",quote.timestamp)?
        .column_f64("current_ext", option_to_f64(&quote.current_ext))?
        .column_str("symbol", quote.symbol.clone())?
        .column_f64("volume_ext", option_to_f64(&quote.volume_ext))?
        .column_f64("high52w", option_to_f64(&quote.high52w))?
        .column_i64("delayed",option_to_i64(&quote.delayed))?
        .column_i64("type",option_to_i64(&quote.r#type))?
        .column_f64("tick_size", option_to_f64(&quote.tick_size))?
        .column_f64("float_shares", option_to_f64(&quote.float_shares))?
        .column_f64("limit_down", option_to_f64(&quote.limit_down))?
        .column_str("no_profit", option_to_string(&quote.no_profit))?
        .column_f64("high", option_to_f64(&quote.high))?
        .column_f64("float_market_capital", option_to_f64(&quote.float_market_capital))?
        .column_str("timestamp_ext", option_to_string(&quote.timestamp_ext))?
        .column_i64("lot_size",option_to_i64(&quote.lot_size))?
        .column_i64("lock_set", option_to_i64(&quote.lock_set))?
        .column_str("weighted_voting_rights", option_to_string(&quote.weighted_voting_rights))?
        .column_f64("chg", option_to_f64(&quote.chg))?
        .column_f64("eps", option_to_f64(&quote.eps))?
        .column_f64("last_close", option_to_f64(&quote.last_close))?
        .column_f64("profit_four", option_to_f64(&quote.profit_four))?
        .column_i64("volume",option_to_i64(&quote.volume))?
        .column_f64("volume_ratio", option_to_f64(&quote.volume_ratio))?
        .column_f64("profit_forecast", option_to_f64(&quote.profit_forecast))?
        .column_f64("turnover_rate", option_to_f64(&quote.turnover_rate))?
        .column_f64("low52w", option_to_f64(&quote.low52w))?
        .column_str("name", option_to_string(&quote.name))?
        .column_str("exchange", option_to_string(&quote.exchange))?
        .column_f64("pe_forecast", option_to_f64(&quote.pe_forecast))?
        .column_f64("total_shares", option_to_f64(&quote.total_shares))?
        .column_i64("status",option_to_i64(&quote.status))?
        .column_str("is_vie_desc", option_to_string(&quote.is_vie_desc))?
        .column_str("security_status", option_to_string(&quote.security_status))?
        .column_str("code", option_to_string(&quote.code))?
        .column_f64("goodwill_in_net_assets", option_to_f64(&quote.goodwill_in_net_assets))?
        .column_f64("avg_price", option_to_f64(&quote.avg_price))?
        .column_f64("percent", option_to_f64(&quote.percent))?
        .column_str("weighted_voting_rights_desc", option_to_string(&quote.weighted_voting_rights_desc))?
        .column_f64("amplitude", option_to_f64(&quote.amplitude))?
        .column_f64("current", option_to_f64(&quote.current))?
        .column_str("is_vie", option_to_string(&quote.is_vie))?
        .column_f64("current_year_percent", option_to_f64(&quote.current_year_percent))?
        .column_i64("issue_date",option_to_i64(&quote.issue_date))?
        .column_str("sub_type", option_to_string(&quote.sub_type))?
        .column_f64("low", option_to_f64(&quote.low))?
        .column_str("is_registration_desc", option_to_string(&quote.is_registration_desc))?
        .column_str("no_profit_desc", option_to_string(&quote.no_profit_desc))?
        .column_f64("market_capital", option_to_f64(&quote.market_capital))?
        .column_f64("dividend", option_to_f64(&quote.dividend))?
        .column_f64("dividend_yield", option_to_f64(&quote.dividend_yield))?
        .column_str("currency", option_to_string(&quote.currency))?
        .column_f64("navps", option_to_f64(&quote.navps))?
        .column_f64("profit", option_to_f64(&quote.profit))?
        .column_f64("pe_lyr", option_to_f64(&quote.pe_lyr))?
        .column_f64("amount", option_to_f64(&quote.amount))?
        .column_f64("pledge_ratio", option_to_f64(&quote.pledge_ratio))?
        .column_str("traded_amount_ext", option_to_string(&quote.traded_amount_ext))?
        .column_str("is_registration", option_to_string(&quote.is_registration))?
        .column_f64("pb", option_to_f64(&quote.pb))?
        .column_f64("limit_up", option_to_f64(&quote.limit_up))?
        .column_f64("pe_ttm", option_to_f64(&quote.pe_ttm))?
        .column_i64("time",quote.time)?
        .column_f64("open", option_to_f64(&quote.open))?
        // .at(TimestampNanos::new(quote.timestamp.clone()*1000*1000*1000))?;
        .at(TimestampNanos::new(quote.timestamp.clone()*1000000))?;
    
    sender.flush(&mut buffer)?;
    println!("写入quote成功\n");
    Ok(())
}

pub async fn write_tran_minute_to_questdb(tran_minute:&TranMinute) -> Result<(),Box<dyn Error>> {
    // println!("写入minute:{:?}\n",tran_minute);
    let mut sender = Sender::from_conf(format!("{}",&CONFIG.questdb.endpoint))?;
    let mut buffer = Buffer::new();
    // insert tran_minute into questdb

    buffer
        .table("Minute")?
        .column_str("symbol", tran_minute.symbol.clone())?
        .column_f64("current", option_to_f64(&tran_minute.current))?
        .column_f64("volume", option_to_f64(&tran_minute.volume))?
        .column_f64("avg_price", option_to_f64(&tran_minute.avg_price))?
        .column_f64("chg", option_to_f64(&tran_minute.chg))?
        .column_f64("percent", option_to_f64(&tran_minute.percent))?
        .column_i64("time", tran_minute.timestamp.clone())?
        .column_f64("amount", option_to_f64(&tran_minute.amount))?
        .column_f64("high", option_to_f64(&tran_minute.high))?
        .column_f64("low", option_to_f64(&tran_minute.low))?
        .column_f64("amount_total", option_to_f64(&tran_minute.amount_total))?
        .column_f64("volume_total", option_to_f64(&tran_minute.volume_total))?
        .column_f64("macd", option_to_f64(&tran_minute.macd))?
        .column_f64("kdj", option_to_f64(&tran_minute.kdj))?
        .column_f64("ratio", option_to_f64(&tran_minute.ratio))?
        .column_f64("volume_compare_volume_sum", option_to_f64(&tran_minute.volume_compare_volume_sum))?
        .column_f64("volume_compare_volume_sum_last", option_to_f64(&tran_minute.volume_compare_volume_sum_last))?
        .at(TimestampNanos::new(tran_minute.timestamp.clone()*1000000))?;
        
        sender.flush(&mut buffer)?;
        // println!("写入minute成功\n");
        Ok(())
}

fn option_to_string(option:&Option<String>) -> String {
    match option {
        Some(s) => s.clone(),
        None => "".to_string()
    } 
}
fn option_to_f64(option:&Option<f64>) -> f64 {
    match option {
        Some(s) => s.clone(),
        None => 0.0
    } 
}
fn option_to_i64(option:&Option<i64>) -> i64 {
    match option {
        Some(s) => s.clone(),
        None => 0
    } 
}

fn epoch_to_timestamp_microseconds(timestamp: i64) -> TimestampMicros {
    // Create a SystemTime from the Unix epoch plus the given milliseconds
    let system_time = UNIX_EPOCH + Duration::from_millis(timestamp as u64);

    // Get the duration since the Unix epoch
    let duration_since_epoch = system_time.duration_since(UNIX_EPOCH).expect("transfor timestamp failed");
    let timestamp_micros_value = duration_since_epoch.as_micros() as i64;
    TimestampMicros::new(timestamp_micros_value)
}