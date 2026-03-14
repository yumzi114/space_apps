use eframe::egui::{self, Context};
use spacetimedb_sdk::Table;

use crate::{MyEguiApp, module_bindings::{MessageTableAccess, UserTableAccess, send_message}, settings_app::format_timestamp_hms};



pub fn live_view(
    app:&mut MyEguiApp,
    ctx:&Context,
    ui:&mut egui::Ui, 
){
    
    ui.heading("LIVE VIEW ");
    let user = app.chat_ctx.db.user();
    ui.label(format!("연결된 사용자 : {:?}",user.count()));
    let mut messages = app.chat_ctx.db.message().iter().collect::<Vec<_>>();
    messages.sort_by_key(|m| m.sent);
    // let asd = ui.re
    ui.vertical(|ui| {
    // 헤더 고정
    let fill = if ui.ctx().style().visuals.dark_mode {
        egui::Color32::from_rgb(117, 88, 88)
    } else {
        egui::Color32::from_rgb(113, 117, 114)
    };
    egui::Frame::NONE
        .fill(fill)
        .inner_margin(egui::Margin::same(6))
        .show(ui, |ui| {
            ui.set_height(20.);

            ui.horizontal(|ui| {
                ui.add_sized([50.0, 20.0], egui::Label::new(egui::RichText::new("순번").strong()));
                ui.add_sized([100.0, 20.0], egui::Label::new(egui::RichText::new("시간(H:M:S.MS)").strong()));
                ui.add_sized([ui.available_width(), 20.0], egui::Label::new(egui::RichText::new("메세지").strong()));
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
                            ui.add_sized([100.0, 18.0], egui::Label::new(format_timestamp_hms(message.sent)));
                            ui.add_sized([ui.available_width() - 8.0, 18.0], egui::Label::new(message.text.to_string()));
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
    egui::TopBottomPanel::bottom("bottom").show(ctx, |ui|{
        if ui.button("atoms").clicked(){
            app.chat_ctx.reducers.send_message("text".to_string()).unwrap();
        }
    });
    
}