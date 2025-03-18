use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Xueqiu {
    pub detail_url: String,
    pub industry_url: String,
    pub minute_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Test {
    pub detail_url: String,
    pub industry_url: String,
    pub minute_url: String,
}

#[derive(Debug, Deserialize)]
pub struct ChromeWebDriver {
    pub endpoint: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct InitConfig {
    // pub version: String,
    pub xueqiu: Xueqiu,
    pub test: Test,
    pub chromewebdriver: ChromeWebDriver,
    pub questdb: Questdb,
}

#[derive(Debug, Deserialize)]
pub struct Questdb {
    pub endpoint: String,
}