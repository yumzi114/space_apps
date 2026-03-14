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
       egui::CentralPanel::default().show(ctx, |ui| {
           
           
           ui.heading("Hello World!");
            let user = self.chat_ctx.db.user();
            ui.label(format!("{:?}",user.count()));
           let mut messages = self.chat_ctx.db.message().iter().collect::<Vec<_>>();
            messages.sort_by_key(|m| m.sent);
            egui::ScrollArea::vertical().max_height(200.).show(ui, |ui| {
               for message in messages {
                    ui.label(format!("{}:{}", message.text,message.sent));
                }
            });
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

