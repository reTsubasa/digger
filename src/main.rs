
use std::{fs::File, io::Read};
use lazy_static::lazy_static;
use thirtyfour::{common::print, cookie, prelude::*};
use serde::{Deserialize, Serialize};
mod elements;
mod xueqiu;
mod tools;


use elements::{InitConfig};

// 定义全局配置变量
lazy_static! {
    pub static ref CONFIG: InitConfig = load_config();
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // init config
    let init_config = load_config();

    // Create a new WebDriver session.
    let mut caps = DesiredCapabilities::chrome();
        caps.add_arg("--headless");
        caps.add_arg("--no-sandbox");
        caps.add_arg("--disable-gpu");
    let driver = WebDriver::new(format!("{}:{}", init_config.chromewebdriver.endpoint, init_config.chromewebdriver.port.to_string()), caps).await?;
    // init cookies
    driver.goto("https://xueqiu.com").await?;
    // 获取cookies
    let cookies = driver.get_all_cookies().await?;
    for cookie in cookies {
        driver.add_cookie(cookie).await?;
    }
    // exec xueqiu
    xueqiu::connect::exec(&driver,init_config.xueqiu,None).await?;

    

    // //访问接口
    // driver.goto("https://stock.xueqiu.com/v5/stock/quote.json?symbol=SH601398&extend=detail").await?;
    // let resp = driver.find_element(By::Tag("pre")).await?.text().await?;

    // let res:Resp= serde_json::from_str(&resp).unwrap();

    // print!("{:?}",res.data.quote.name);

    // //访问板块
    // driver.goto("https://xueqiu.com/stock/industry/stockList.json?code=SH601398&type=1&size=100").await?;
    // let resp = driver.find_element(By::Tag("pre")).await?.text().await?;
    // println!("{:?}",resp);

    // // 访问分钟数据
    // driver.goto("https://stock.xueqiu.com/v5/stock/chart/minute.json?symbol=SH601398&period=1d").await?;

    
    // // explicitly close the browser.
    // driver.quit().await?;

    Ok(())
}

fn load_config() -> InitConfig {
    let mut file = File::open("/Users/hyc/snake-game/digger/src/config.toml").expect("config.toml not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("cannot read config.toml");
    let config: InitConfig = toml::from_str(&contents).expect("cannot parse config.toml");
    return config
}