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
use serde_json::to_string;

use yas::lock::LockAction;
use yas::common::{PixelRect, RawCaptureImage, utils};
use yas::common::cli::get_cli;
use yas::ws::server::run_ws;

#[cfg(windows)]
use yas::capture::capture_absolute_image;
use yas::expo::genmo::GenmoFormat;
use yas::expo::good::GoodFormat;
use yas::expo::mona::MonaFormat;
use yas::expo::march7th::March7thFormat;
use yas::expo::hood::HoodFormat;
use yas::scanner::config::GameType;
use yas::info::info::ScanInfo;
use yas::scanner::config::YasScannerConfig;
#[cfg(windows)]
use yas::scanner::yas_scanner::YasScanner;

fn main() {
    let matches = get_cli().get_matches();

    let test = false;
    if test {
        let _ = test_mark();
        return
    }

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
fn do_scan(matches: ArgMatches) -> Result<String> {
    let config = YasScannerConfig::from_match(&matches)?;
    let info = get_info(&matches)?;
    let output_dir = Path::new(matches.try_get_one::<String>("output-dir")?.unwrap());

    let game = config.game;
    let mut scanner = YasScanner::new(info.clone(), config)?;

    match game {
        GameType::Genshin => {
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

            Ok(to_string(&GoodFormat::new(&results))?)
        }
        GameType::Starrail => {
            let now = SystemTime::now();
            let results = scanner.scan_starrail()?;
            let t = now.elapsed()?.as_secs_f64();
            info!("time: {}s", t);

            let output_filename = output_dir.join("march7th.json");
            let march7th = March7thFormat::new(&results);
            march7th.save(String::from(output_filename.to_str().unwrap()));

            let output_filename = output_dir.join("hood.json");
            let hood = HoodFormat::new(&results);
            hood.save(String::from(output_filename.to_str().unwrap()));

            Ok(to_string(&HoodFormat::new(&results))?)
        }
    }





}

#[cfg(not(windows))]
fn do_scan(_matches: ArgMatches) -> Result<String> {
    Ok(String::from(""))
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

    let game_type = GameType::from_string(matches.get_one::<String>("game").unwrap().to_string());
    let window_name: String = matches.get_one::<String>("window").unwrap().to_string();
    
    let hwnd = if window_name.is_empty() {
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

    let info = ScanInfo::from_rect(&rect, game_type).unwrap();
    Ok(info)
}

fn test_mark() -> Result<()> {
    // let rect = PixelRect { left: 0, top: 0, width: 1600, height: 900 };
    let rect = PixelRect { left: 0, top: 0, width: 1920, height: 1080 };
    let info = ScanInfo::from_rect(&rect, GameType::Genshin).unwrap();
    let config = YasScannerConfig {
        max_row: 1000,
        capture_only: false,
        min_star: 5,
        min_level: 0,
        max_wait_switch_artifact: 0,
        scroll_stop: 0,
        number: 0,
        dump_mode: false,
        speed: 1,
        no_check: false,
        max_wait_scroll: 0,
        mark: true,
        dxgcap: false,
        default_stop: 0,
        yun: false,
        scroll_speed: 0.0,
        lock_stop: 0,
        max_wait_lock: 0,
        game: GameType::Genshin
    };
    let mut scanner = YasScanner::new(info, config)?;
    let mut img = RawCaptureImage::load("test_genshin.png")?;
    let _ = scanner.mark(&mut img);
    let _ = img.save("text_genshin_marked.png");
    Ok(())
}