use clap::{arg, Command, value_parser};

pub fn get_cli() -> Command {
    Command::new("YAS-lock - 《原神》&《崩坏：星穹铁道》圣遗物导出&加解锁")
        .version("v2.0.0")
        .author("wormtql <584130248@qq.com>")
        .arg(arg!(--"dump" "输出模型预测结果、二值化图像和灰度图像，debug专用"))
        .arg(arg!(--"capture-only" "只保存截图，不进行扫描，debug专用"))
        .arg(arg!(--"mark" "保存标记后的截图，debug专用"))
        .arg(arg!(--"output-dir" <DIR> "输出目录").default_value("."))
        .arg(arg!(--"verbose" "显示详细信息"))
        .arg(arg!(--"no-check" "不检测是否已打开背包等"))
        .arg(arg!(--"dxgcap" "使用dxgcap捕获屏幕"))
        .arg(
            arg!(--"listen" <HOST> "开启 Websocket 服务")
                .default_value("127.0.0.1:2022"),
        )
        .arg(
            arg!(--"max-row" <ROW> "最大扫描行数")
                .default_value("1000")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            arg!(--"min-star" <STAR> "最小星级")
                .default_value("5")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            arg!(--"min-level" <LEVEL> "最小等级")
                .default_value("0")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            arg!(--"speed" <SPEED> "速度（共1-5档，如提示大量重复尝试降低速度）")
                .default_value("5")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            arg!(--"number" <NUM> "指定圣遗物/遗器数量（在自动识别数量不准确时使用）")
                .default_value("0")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            arg!(--"default-stop" <TIME> "等待动画、鼠标点击等操作的默认停顿时间(ms)")
                .default_value("500")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            arg!(--"scroll-stop" <TIME> "页面滚动停顿时间(ms)")
                .default_value("100")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            arg!(--"lock-stop" <TIME> "加解锁停顿时间(ms)")
                .default_value("100")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            arg!(--"max-wait-switch-artifact" <TIME> "切换圣遗物/遗器最大等待时间(ms)")
                .default_value("800")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            arg!(--"max-wait-scroll" <TIME> "翻页的最大等待时间(ms)（翻页不正确可以考虑加大该选项）")
                .default_value("0")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            arg!(--"max-wait-lock" <TIME> "加解锁的最大等待时间(ms)（加解锁不正确可以考虑加大该选项）")
                .default_value("0")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            arg!(--"offset-x" <OFFSET> "人为指定横坐标偏移（截图有偏移时可用该选项校正）")
                .default_value("0")
                .value_parser(value_parser!(i32)),
        )
        .arg(
            arg!(--"offset-y" <OFFSET> "人为指定纵坐标偏移（截图有偏移时可用该选项校正）")
                .default_value("0")
                .value_parser(value_parser!(i32)),
        )
        .arg(
            arg!(--"window" <NAME> "游戏窗口名")
                .default_value(""),
        )
        .arg(
            arg!(--"scroll-speed" <SPEED> "滚轮速度（单位：像素，仅在云原神模式下生效）")
                .default_value("15.0")
                .value_parser(value_parser!(f64)),
        )
}