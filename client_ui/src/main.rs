mod module_bindings;
use module_bindings::*;
use eframe::egui::{self, ScrollArea};

use serde::{Deserialize, Serialize};
use spacetimedb_sdk::{credentials, DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey};

use crate::{app_threads::tt_thread, chat_fn::{connect_to_db, register_callbacks, subscribe_to_tables}, pages::live_view, settings_app::{format_timestamp_hms, setup_custom_fonts}};


mod chat_fn;
mod app_threads;
mod pages;
mod settings_app;
#[derive(Default, Debug, Serialize, Deserialize)]
struct AppConfig {
    feild_font_size: f32,
    value_font_size: f32,
}
enum App_Mode {
    LIVE,
    USERS,
    DATA
}
fn load_or_create_config() -> AppConfig {
    match confy::load("app-config", None) {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("config load failed: {err}");

            let cfg = AppConfig::default();

            if let Err(store_err) = confy::store("app-config", None, &cfg) {
                eprintln!("config store failed: {store_err}");
            }

            cfg
        }
    }
}
// #[derive(Default)]
// #[derive(Serialize, Deserialize)]
struct MyEguiApp {
    chat_ctx: DbConnection,
    rt: tokio::runtime::Runtime,
    app_config:AppConfig,
    app_mode:App_Mode
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        let chat_ctx: DbConnection = connect_to_db();
        let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
        let app_config = load_or_create_config();

        Self{
            rt,
            chat_ctx,
            app_config,
            app_mode:App_Mode::LIVE,
        }
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
       egui::CentralPanel::default().show(ctx, |ui| {
           egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                ui.menu_button(" 시스템 ", |ui| {
                    ui.menu_button("메뉴선택", |ui| {
                        if ui.button(" 실시간 ").clicked() {
                            self.app_mode=App_Mode::LIVE;
                        }
                        if ui.button(" 사용자 ").clicked() {
                            self.app_mode=App_Mode::USERS;
                        }
                        if ui.button(" 데이터 ").clicked() {
                            self.app_mode=App_Mode::DATA;
                        }
                    });
                    if ui.button(" 종 료 ").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
                ui.menu_button(" 보기설정 ", |ui| {
                    ui.menu_button("배경색상", |ui| {
                        egui::widgets::global_theme_preference_buttons(ui);
                    });
                    ui.menu_button("글자크기", |ui| {
                        let f_resp=ui.add(egui::Slider::new(&mut self.app_config.feild_font_size, 8.0..=40.0).text("제목"));
                        if f_resp.changed() {
                            let _ = confy::store("app-config", None, &self.app_config);
                        }
                        let v_resp=ui.add(egui::Slider::new(&mut self.app_config.value_font_size, 8.0..=40.0).text("값"));
                        if v_resp.changed() {
                            let _ = confy::store("app-config", None, &self.app_config);
                        }
                        // egui::widgets::global_theme_preference_buttons(ui);
                    });
                });
                ui.add_space(16.0);
            });
            match self.app_mode {
                App_Mode::LIVE=>{
                    live_view(self, ctx, ui);
                },
                App_Mode::USERS=>{

                }
                App_Mode::DATA=>{

                }
            }
       });
   }
}



#[tokio::main]
async fn main() {
    // let ctx: DbConnection = connect_to_db();
    // ctx.run_threaded();
    let native_options = eframe::NativeOptions::default();
    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
    eframe::run_native(
        "My egui App", 
        native_options, 
        Box::new(
            |cc| {
                
                let app = MyEguiApp::new(cc);
                register_callbacks(&app.chat_ctx);
                subscribe_to_tables(&app.chat_ctx);
                app.chat_ctx.run_threaded();
                // tt_thread(&app.rt.handle());
                Ok(Box::new(app))
            }
        ));

}

