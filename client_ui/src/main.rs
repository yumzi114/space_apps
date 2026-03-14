mod module_bindings;
use module_bindings::*;
use eframe::egui::{self, ScrollArea};

use spacetimedb_sdk::{credentials, DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey};

use crate::chat_fn::{connect_to_db, register_callbacks, subscribe_to_tables, user_input_loop};


mod chat_fn;

// #[derive(Default)]
struct MyEguiApp {
    chat_ctx: DbConnection
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let chat_ctx: DbConnection = connect_to_db();
        Self{
            chat_ctx
        }
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
       egui::CentralPanel::default().show(ctx, |ui| {
           
           egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
                ui.menu_button("Weget", |ui| {
                    if ui.button("Quit").clicked() {
                        // ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
                ui.menu_button("Sys color", |ui| {
                    egui::widgets::global_theme_preference_buttons(ui);
                });
            });
           ui.heading("SpacetimeDB TEST ");
            let user = self.chat_ctx.db.user();
            ui.label(format!("{:?}",user.count()));
           let mut messages = self.chat_ctx.db.message().iter().collect::<Vec<_>>();
            messages.sort_by_key(|m| m.sent);
            // let asd = ui.re
            ui.vertical(|ui| {
            // 헤더 고정
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(30, 30, 30))
                .inner_margin(egui::Margin::same(6))
                .show(ui, |ui| {
                    ui.set_height(20.);

                    ui.horizontal(|ui| {
                        ui.add_sized([50.0, 20.0], egui::Label::new(egui::RichText::new("No").strong()));
                        ui.add_sized([80.0, 20.0], egui::Label::new(egui::RichText::new("MSG").strong()));
                        ui.add_sized([ui.available_width(), 20.0], egui::Label::new(egui::RichText::new("Time").strong()));
                    });
                });

            ui.separator();

            // 바디만 스크롤
            egui::ScrollArea::vertical()
                // .max_height(20.)
                .stick_to_bottom(true)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for (idx,message) in messages.iter().enumerate() {
                        egui::Frame::NONE
                            .inner_margin(egui::Margin::symmetric(6, 4))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.add_sized([50.0, 18.0], egui::Label::new(idx.to_string()));
                                    ui.add_sized([80.0, 18.0], egui::Label::new(message.text.to_string()));
                                    ui.add_sized([ui.available_width() - 8.0, 18.0], egui::Label::new(format!("{}",message.sent)));
                                });
                            });

                        ui.separator();
                    }
                });
            });
            // egui::ScrollArea::vertical()
            // // .max_height(ui.available_height() * 0.5)
            // .show(ui, |ui| {
            //     egui::Frame::NONE
            //         .inner_margin(egui::Margin::symmetric(0, 0))
            //         .show(ui, |ui| {
            //             ui.set_width(ui.available_width() - 20.0);

            //             for (idx,message) in messages.iter().enumerate() {
                            
            //                 ui.label(format!("{} : {} : {}", idx,message.text,message.sent));
            //                 // ui.label(row.created_at.to_string());
            //             }
            //         });
            // });
            // egui::ScrollArea::vertical().show(ui, |ui| {
            //    for message in messages {
            //         ui.set_width(ui.available_width() - 16.0);
            //         ui.label(format!("{}:{}", message.text,message.sent));
            //     }
            // });
            
           
       });
       egui::TopBottomPanel::bottom("bottom").show(ctx, |ui|{
            if ui.button("atoms").clicked(){
                self.chat_ctx.reducers.send_message("text".to_string()).unwrap();
            }
        });
   }
}



fn main() {
    // let ctx: DbConnection = connect_to_db();
    // ctx.run_threaded();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App", 
        native_options, 
        Box::new(
            |cc| {
                let app = MyEguiApp::new(cc);
                register_callbacks(&app.chat_ctx);
                subscribe_to_tables(&app.chat_ctx);
                app.chat_ctx.run_threaded();
                Ok(Box::new(app))
            }
            ));

}

