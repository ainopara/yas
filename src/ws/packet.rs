use crate::scanner::config::YasScannerConfig;
use anyhow::Result;
use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ConfigNotifyData {
    pub config: YasScannerConfig,
}

impl ConfigNotifyData {
    pub fn new(matches: &ArgMatches) -> Result<ConfigNotifyData> {
        Ok(ConfigNotifyData {
            config: YasScannerConfig::from_match(&matches)?,
        })
    }
    pub fn packet(matches: &ArgMatches) -> Result<Packet> {
        Ok(Packet::ConfigNotify(Self::new(matches)?))
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ScanReqData {
    pub argv: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ScanRspData {
    pub success: bool,
    pub message: String,
    pub json: String,
}

impl ScanRspData {
    pub fn new(result: Result<String>) -> Result<ScanRspData> {
        Ok(match result {
            Ok(arts) => ScanRspData {
                success: true,
                message: String::from(""),
                json: arts,
            },
            Err(e) => ScanRspData {
                success: false,
                message: e.to_string(),
                json: String::from(""),
            },
        })
    }
    pub fn packet(result: Result<String>) -> Result<Packet> {
        Ok(Packet::ScanRsp(Self::new(result)?))
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LockReqData {
    pub argv: Vec<String>,
    pub indices: Option<Vec<u32>>,
    pub lock_json: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LockRspData {
    pub success: bool,
    pub message: String,
}

impl LockRspData {
    pub fn new(result: Result<()>) -> Result<LockRspData> {
        Ok(match result {
            Ok(()) => LockRspData {
                success: true,
                message: String::from(""),
            },
            Err(e) => LockRspData {
                success: false,
                message: e.to_string(),
            },
        })
    }
    pub fn packet(result: Result<()>) -> Result<Packet> {
        Ok(Packet::LockRsp(Self::new(result)?))
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "cmd", content = "data")]
pub enum Packet {
    ConfigNotify(ConfigNotifyData),
    ScanReq(ScanReqData),
    ScanRsp(ScanRspData),
    LockReq(LockReqData),
    LockRsp(LockRspData),
}

impl Packet {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        to_string(&self)
    }
    pub fn name(&self) -> &str {
        match self {
            Self::ConfigNotify(_) => "ConfigNotify",
            Self::ScanReq(_) => "ScanReq",
            Self::ScanRsp(_) => "ScanRsp",
            Self::LockReq(_) => "LockReq",
            Self::LockRsp(_) => "LockRsp",
        }
    }
}
