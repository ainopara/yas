use anyhow::{anyhow, Result};
use std::fs;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;

use clap::{ArgMatches};
use env_logger::Builder;
use log::{error, info, LevelFilter};

use yas::artifact::internal_artifact::InternalArtifact;
use yas::lock::LockAction;
use yas::common::utils;
use yas::common::cli::get_cli;
use yas::ws::server::run_ws;

#[cfg(windows)]
use yas::capture::capture_absolute_image;
use yas::expo::genmo::GenmoFormat;
use yas::expo::good::GoodFormat;
use yas::expo::mona::MonaFormat;
use yas::scanner::config::GameType;
use yas::info::info::ScanInfo;
use yas::scanner::config::YasScannerConfig;
#[cfg(windows)]
use yas::scanner::yas_scanner::YasScanner;

fn main() {
    let matches = get_cli().get_matches();

    Builder::new()
        .filter_level(LevelFilter::Info)
        .filter_module(
            "yas",
            if matches.get_flag("verbose") {
                LevelFilter::Trace
            } else {
                LevelFilter::Info
            },
        )
        .format_timestamp_millis()
        .init();

    start(matches).unwrap_or_else(|e| error!("{:#}", e));

    info!("按 Enter 退出");
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Readline error");
}

fn start(matches: ArgMatches) -> Result<()> {
    if !utils::is_admin() {
        return Err(anyhow!("请以管理员身份运行该程序"));
    }

    if matches.get_flag("ws") {
        run_ws(matches, do_scan, do_lock)
    } else {
        run_once(matches)
    }
}

fn run_once(matches: ArgMatches) -> Result<()> {
    let output_dir = Path::new(matches.get_one::<String>("output-dir").unwrap());

    let mut lock_mode = false;
    let mut actions: Vec<LockAction> = Vec::new();

    let lock_filename = output_dir.join("lock.json");
    if lock_filename.exists() {
        print!("检测到lock文件，输入y开始加解锁，直接回车开始扫描：");
        stdout().flush()?;
        let mut s: String = String::new();
        stdin().read_line(&mut s)?;
        if s.trim() == "y" {
            let json_str = fs::read_to_string(lock_filename)?;
            actions = LockAction::from_lock_json(&json_str)?;
            lock_mode = true;
        }
    }

    if lock_mode {
        do_lock(matches, actions)
    } else {
        do_scan(matches).map(|_| ())
    }
}

#[cfg(windows)]
fn do_scan(matches: ArgMatches) -> Result<Vec<InternalArtifact>> {
    let config = YasScannerConfig::from_match(&matches)?;
    let info = get_info(&matches)?;
    let output_dir = Path::new(matches.try_get_one::<String>("output-dir")?.unwrap());

    let mut scanner = YasScanner::new(info.clone(), config)?;

    let now = SystemTime::now();
    let results = scanner.scan()?;
    let t = now.elapsed()?.as_secs_f64();
    info!("time: {}s", t);

    // Mona
    let mona = MonaFormat::new(&results);
    utils::dump_json(&mona, output_dir.join("mona.json"))?;
    // Genmo
    let genmo = GenmoFormat::new(&results);
    utils::dump_json(&genmo, output_dir.join("genmo.json"))?;
    // GOOD
    let good = GoodFormat::new(&results);
    utils::dump_json(&good, output_dir.join("good.json"))?;

    Ok(results)
}

#[cfg(not(windows))]
fn do_scan(_matches: ArgMatches) -> Result<Vec<InternalArtifact>> {
    Ok(Vec::new())
}

#[cfg(windows)]
fn do_lock(matches: ArgMatches, actions: Vec<LockAction>) -> Result<()> {
    let config = YasScannerConfig::from_match(&matches)?;
    let info = get_info(&matches)?;

    let mut scanner = YasScanner::new(info.clone(), config)?;
    scanner.lock(actions)
}

#[cfg(not(windows))]
fn do_lock(_matches: ArgMatches, _actions: Vec<LockAction>) -> Result<()> {
    Ok(())
}

#[cfg(windows)]
fn get_info(matches: &ArgMatches) -> Result<ScanInfo> {
    utils::set_dpi_awareness();

    let window_name: String = matches.get_one::<String>("window").unwrap().to_string();

    let hwnd = if window_name.is_empty() {
        let game_type = GameType::from_string(matches.get_one::<String>("game").unwrap().to_string());
        utils::find_game_window(game_type)
    } else {
        utils::find_window_by_name(&window_name)
    }
        .map_err(|_| anyhow!("未找到游戏窗口，请确认游戏已经开启"))?;

    utils::show_window_and_set_foreground(hwnd);
    utils::sleep(1000);

    let mut rect = utils::get_client_rect(hwnd)?;

    let offset_x: i32 = *matches.get_one("offset-x").unwrap();
    let offset_y: i32 = *matches.get_one("offset-y").unwrap();

    rect.left += offset_x;
    rect.top += offset_y;

    capture_absolute_image(&rect)?.save("test.png")?;

    info!(
        "left = {}, top = {}, width = {}, height = {}",
        rect.left, rect.top, rect.width, rect.height
    );

    let info: ScanInfo;
    if rect.height * 16 == rect.width * 9 {
        info =
            ScanInfo::from_16_9(rect.width as u32, rect.height as u32, rect.left, rect.top);
    } else if rect.height * 8 == rect.width * 5 {
        info = ScanInfo::from_8_5(rect.width as u32, rect.height as u32, rect.left, rect.top);
    } else if rect.height * 4 == rect.width * 3 {
        info = ScanInfo::from_4_3(rect.width as u32, rect.height as u32, rect.left, rect.top);
    } else {
        return Err(anyhow!("不支持的分辨率"));
    }

    Ok(info)
}