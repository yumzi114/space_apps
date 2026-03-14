use eframe::egui;
use chrono::{DateTime, Local, Utc};
use chrono_tz::Asia::Seoul;
use spacetimedb_sdk::Timestamp;
pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            // "../files/Pilseung_Gothic.ttf"
            "../../../assets/PyeojinGothic-Bold.ttf"
        )).into(),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());
    ctx.set_fonts(fonts);
}

pub fn format_timestamp_hms(ts: Timestamp) -> String {
    let micros = ts.to_micros_since_unix_epoch();
    let secs = (micros / 1_000_000) as i64;
    let sub_micros = (micros % 1_000_000) as u32;

    match DateTime::<Utc>::from_timestamp(secs, sub_micros * 1_000) {
        Some(dt) => {
            let local = dt.with_timezone(&Seoul);
            local.format("%H:%M:%S%.3f").to_string()
        }
        None => "invalid-time".to_string(),
    }
}