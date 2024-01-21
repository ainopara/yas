use crate::common::PixelRect;
use crate::info::info::ScanInfo;

pub struct Rect(f64, f64, f64, f64); // top, right, bottom, left

pub struct WindowInfo {
    pub width: f64,
    pub height: f64,

    pub title_pos: Rect,
    pub main_stat_name_pos: Rect,
    pub main_stat_value_pos: Rect,
    pub level_pos: Rect,
    pub panel_pos: Rect,

    pub sub_stat1_pos: Rect,
    pub sub_stat2_pos: Rect,
    pub sub_stat3_pos: Rect,
    pub sub_stat4_pos: Rect,

    pub sub_stat1_name_pos: Rect,
    pub sub_stat1_value_pos: Rect,
    pub sub_stat2_name_pos: Rect,
    pub sub_stat2_value_pos: Rect,
    pub sub_stat3_name_pos: Rect,
    pub sub_stat3_value_pos: Rect,
    pub sub_stat4_name_pos: Rect,
    pub sub_stat4_value_pos: Rect,

    pub equip_pos: Rect,
    pub art_count_pos: Rect,

    pub art_width: f64,
    pub art_height: f64,
    pub art_gap_x: f64,
    pub art_gap_y: f64,

    pub art_row: usize,
    pub art_col: usize,

    pub left_margin: f64,
    pub top_margin: f64,

    pub flag_x: f64,
    pub flag_y: f64,

    pub star_x: f64,
    pub star_y: f64,

    pub lock_x: f64,
    pub lock_y: f64,

    pub art_lock_x: f64,
    pub art_lock_y: f64,

    pub ruler_left: f64,
    pub ruler_top: f64,
    pub ruler_height: f64,

    pub menu_x: f64,
    pub menu_y: f64,

    pub scrollbar_left: f64,
    pub scrollbar_top: f64,
    pub scrollbar_height: f64,

    pub art_shift_x: f64,
    pub art_shift_y: f64,
}

impl WindowInfo {
    pub fn to_scan_info(&self, h: f64, w: f64, left: i32, top: i32) -> ScanInfo {
        let convert_rect = |rect: &Rect| {
            let top = rect.0 / self.height * h;
            let right = rect.1 / self.width * w;
            let bottom = rect.2 / self.height * h;
            let left = rect.3 / self.width * w;

            PixelRect {
                left: left.round() as i32,
                top: top.round() as i32,
                width: (right - left).round() as i32,
                height: (bottom - top).round() as i32,
            }
        };

        let convert_x = |x: f64| x / self.width * w;

        let convert_y = |y: f64| y / self.height * h;

        let p = convert_rect(&self.panel_pos);
        let to_rel = |rect: PixelRect| PixelRect {
            left: rect.left - p.left,
            top: rect.top - p.top,
            width: rect.width,
            height: rect.height,
        };

        ScanInfo {
            panel_position: convert_rect(&self.panel_pos),
            title_position: to_rel(convert_rect(&self.title_pos)),
            main_stat_name_position: to_rel(convert_rect(&self.main_stat_name_pos)),
            main_stat_value_position: to_rel(convert_rect(&self.main_stat_value_pos)),
            level_position: to_rel(convert_rect(&self.level_pos)),
            sub_stat1_position: to_rel(convert_rect(&self.sub_stat1_pos)),
            sub_stat2_position: to_rel(convert_rect(&self.sub_stat2_pos)),
            sub_stat3_position: to_rel(convert_rect(&self.sub_stat3_pos)),
            sub_stat4_position: to_rel(convert_rect(&self.sub_stat4_pos)),
            sub_stat1_name_pos: to_rel(convert_rect(&self.sub_stat1_name_pos)),
            sub_stat1_value_pos: to_rel(convert_rect(&self.sub_stat1_value_pos)),
            sub_stat2_name_pos: to_rel(convert_rect(&self.sub_stat2_name_pos)),
            sub_stat2_value_pos: to_rel(convert_rect(&self.sub_stat2_value_pos)),
            sub_stat3_name_pos: to_rel(convert_rect(&self.sub_stat3_name_pos)),
            sub_stat3_value_pos: to_rel(convert_rect(&self.sub_stat3_value_pos)),
            sub_stat4_name_pos: to_rel(convert_rect(&self.sub_stat4_name_pos)),
            sub_stat4_value_pos: to_rel(convert_rect(&self.sub_stat4_value_pos)),
            equip_position: to_rel(convert_rect(&self.equip_pos)),
            art_count_position: convert_rect(&self.art_count_pos).to_bound(),
            art_width: convert_x(self.art_width) as u32,
            art_height: convert_y(self.art_height) as u32,
            art_gap_x: convert_x(self.art_gap_x) as u32,
            art_gap_y: convert_y(self.art_gap_y) as u32,
            art_row: self.art_row as u32,
            art_col: self.art_col as u32,
            left_margin: convert_x(self.left_margin),
            top_margin: convert_y(self.top_margin),
            width: w as u32,
            height: h as u32,
            left,
            top,
            flag_x: convert_x(self.flag_x) as u32,
            flag_y: convert_y(self.flag_y) as u32,
            star_x: convert_x(self.star_x) as u32,
            star_y: convert_y(self.star_y) as u32,
            lock_x: convert_x(self.lock_x) as u32,
            lock_y: convert_y(self.lock_y) as u32,
            art_lock_x: convert_x(self.art_lock_x),
            art_lock_y: convert_y(self.art_lock_y),
            ruler_left: convert_x(self.ruler_left) as u32,
            ruler_top: convert_y(self.ruler_top) as u32,
            ruler_height: convert_y(self.ruler_height) as u32,
            menu_x: convert_x(self.menu_x) as u32,
            menu_y: convert_y(self.menu_y) as u32,
            scrollbar_left: convert_x(self.scrollbar_left) as u32,
            scrollbar_top: convert_y(self.scrollbar_top) as u32,
            scrollbar_height: convert_y(self.scrollbar_height) as u32,
            art_shift_x: convert_x(self.art_shift_x),
            art_shift_y: convert_y(self.art_shift_y),
        }
    }
}

pub const WINDOW_16_9: WindowInfo = WindowInfo {
    width: 1600.0,
    height: 900.0,

    title_pos: Rect(106.6, 1417.7, 139.6, 1111.8),
    main_stat_name_pos: Rect(224.3, 1253.9, 248.0, 1110.0),
    main_stat_value_pos: Rect(248.4, 1246.8, 286.8, 1110.0),
    level_pos: Rect(360.0, 1160.0, 378.0, 1117.0),
    panel_pos: Rect(100.0, 1500.0, 800.0, 1090.0),

    sub_stat1_pos: Rect(398.1, 1343.0, 427.3, 1130.2),
    sub_stat2_pos: Rect(427.3, 1343.0, 458.2, 1130.2),
    sub_stat3_pos: Rect(458.2, 1343.0, 490.9, 1130.2),
    sub_stat4_pos: Rect(490.9, 1343.0, 523.0, 1130.2),

    sub_stat1_name_pos: Rect(0.0, 0.0, 0.0, 0.0),
    sub_stat1_value_pos: Rect(0.0, 0.0, 0.0, 0.0),
    sub_stat2_name_pos: Rect(0.0, 0.0, 0.0, 0.0),
    sub_stat2_value_pos: Rect(0.0, 0.0, 0.0, 0.0),
    sub_stat3_name_pos: Rect(0.0, 0.0, 0.0, 0.0),
    sub_stat3_value_pos: Rect(0.0, 0.0, 0.0, 0.0),
    sub_stat4_name_pos: Rect(0.0, 0.0, 0.0, 0.0),
    sub_stat4_value_pos: Rect(0.0, 0.0, 0.0, 0.0),

    equip_pos: Rect(762.6, 1389.4, 787.8, 1154.9),
    art_count_pos: Rect(27.1, 1504.7, 52.9, 1314.9),

    art_width: 1055.0 - 953.0,
    art_height: 373.0 - 247.0,
    art_gap_x: 953.0 - 933.0,
    art_gap_y: 247.0 - 227.0,

    art_row: 5,
    art_col: 8,

    left_margin: 98.0,
    top_margin: 100.0,

    flag_x: 271.1,
    flag_y: 89.8,

    star_x: 379.4,
    star_y: 23.9,

    lock_x: 1450.0,
    lock_y: 357.0,

    art_lock_x: 12.0,
    art_lock_y: 14.0,

    ruler_left: 272.0,
    ruler_top: 102.0,
    ruler_height: 123.0,

    menu_x: 540.0,
    menu_y: 50.0,

    scrollbar_left: 1074.0,
    scrollbar_top: 108.0,
    scrollbar_height: 668.0,

    art_shift_x: 122.0,
    art_shift_y: 146.0,
};

pub const WINDOW_16_9_STARRAIL: WindowInfo = WindowInfo {
    width: 1600.0,
    height: 900.0,

    title_pos: Rect(111.0, 1400.0, 132.0, 1169.0),
    main_stat_name_pos: Rect(335.0, 1407.0, 355.0, 1207.0),
    main_stat_value_pos: Rect(335.0, 1535.0, 355.0, 1465.0),
    level_pos: Rect(258.0, 1240.0, 285.0, 1170.0),
    panel_pos: Rect(100.0, 1550.0, 780.0, 1150.0),

    sub_stat1_pos: Rect(0.0, 0.0, 0.0, 0.0),
    sub_stat2_pos: Rect(0.0, 0.0, 0.0, 0.0),
    sub_stat3_pos: Rect(0.0, 0.0, 0.0, 0.0),
    sub_stat4_pos: Rect(0.0, 0.0, 0.0, 0.0),

    sub_stat1_name_pos: Rect(370.0, 1369.0, 392.0, 1204.0),
    sub_stat1_value_pos: Rect(370.0, 1534.0, 392.0, 1380.0),
    sub_stat2_name_pos: Rect(402.0, 1369.0, 425.0, 1204.0),
    sub_stat2_value_pos: Rect(402.0, 1534.0, 425.0, 1380.0),
    sub_stat3_name_pos: Rect(435.0, 1369.0, 458.0, 1204.0),
    sub_stat3_value_pos: Rect(435.0, 1534.0, 458.0, 1380.0),
    sub_stat4_name_pos: Rect(467.0, 1369.0, 489.0, 1204.0),
    sub_stat4_value_pos: Rect(467.0, 1534.0, 489.0, 1380.0),

    equip_pos: Rect(741.0, 1340.0, 775.0, 1305.0),
    art_count_pos: Rect(813.0, 960.0, 836.0, 753.0),

    art_width: 96.0,
    art_height: 112.0,
    art_gap_x: 8.0,
    art_gap_y: 12.0,

    art_row: 5,
    art_col: 9,

    left_margin: 108.0,
    top_margin: 166.0,

    flag_x: 270.0,
    flag_y: 158.0,

    star_x: 383.0,
    star_y: 38.0,

    lock_x: 1510.0,
    lock_y: 271.0,

    art_lock_x: 89.0,
    art_lock_y: 72.0,

    ruler_left: 270.0,
    ruler_top: 166.0,
    ruler_height: 112.0,

    menu_x: 690.0,
    menu_y: 50.0,

    scrollbar_left: 1127.0,
    scrollbar_top: 166.0,
    scrollbar_height: 610.0,

    art_shift_x: 104.0,
    art_shift_y: 124.0,
};
