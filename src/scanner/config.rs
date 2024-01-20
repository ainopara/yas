use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct YasScannerConfig {
    max_row: u32,
    capture_only: bool,
    min_star: u32,
    min_level: u32,
    max_wait_switch_artifact: u32,
    scroll_stop: u32,
    number: u32,
    dump_mode: bool,
    speed: u32,
    no_check: bool,
    max_wait_scroll: u32,
    mark: bool,
    dxgcap: bool,
    default_stop: u32,
    yun: bool,
    scroll_speed: f64,
    lock_stop: u32,
    max_wait_lock: u32,
}

impl YasScannerConfig {
    pub fn from_match(matches: &ArgMatches) -> Result<YasScannerConfig> {
        Ok(YasScannerConfig {
            max_row: *matches.get_one("max-row").unwrap(),
            capture_only: matches.get_flag("capture-only"),
            dump_mode: matches.get_flag("dump"),
            mark: matches.get_flag("mark"),
            min_star: *matches.get_one("min-star").unwrap(),
            min_level: *matches.get_one("min-level").unwrap(),
            max_wait_switch_artifact: *matches.get_one("max-wait-switch-artifact").unwrap(),
            scroll_stop: *matches.get_one("scroll-stop").unwrap(),
            number: *matches.get_one("number").unwrap(),
            speed: *matches.get_one("speed").unwrap(),
            no_check: matches.get_flag("no-check"),
            max_wait_scroll: *matches.get_one("max-wait-scroll").unwrap(),
            dxgcap: matches.get_flag("dxgcap"),
            default_stop: *matches.get_one("default-stop").unwrap(),
            yun: matches.get_one::<String>("window").unwrap().to_string() != String::from("原神"),
            scroll_speed: *matches.get_one("scroll-speed").unwrap(),
            lock_stop: *matches.get_one("lock-stop").unwrap(),
            max_wait_lock: *matches.get_one("max-wait-lock").unwrap(),
        })
    }
}