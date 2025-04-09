// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[warn(unused_imports)]
mod common;
mod config;
use config::*;
mod trans;
use crate::trans::transport::{Msg, TransMode, Transport};
use eframe::egui;
use egui::{ComboBox, Context, Id, Modal, ProgressBar, Ui, Widget, Window};

 

fn main() {
    // 自定义格式（包含时间、日志级别、目标模块）
    tracing_subscriber::fmt()
        .with_target(true) // 显示模块路径
        .with_timer(tracing_subscriber::fmt::time::uptime()) // 显示时间
        .init();
    let native_options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([640.0, 240.0]) // wide enough for the drag-drop overlay text
        .with_drag_and_drop(true),
        ..Default::default()
    };
    let _ = eframe::run_native("ComicView", native_options, Box::new(|cc| {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Ok(Box::<MyApp>::default())}));
}

 
struct MyApp {
    id: String,
    token: String,
    pwd: String,
    login_modal_open: bool,
    save_progress: Option<f32>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            id: "Arthur".to_owned(),
            token: "Aloha".to_owned(),
            pwd: "".to_owned(),
            login_modal_open: false,
            save_progress: None,
        }
    }
}

 




impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.id)
                    .labelled_by(name_label.id);
            });
            let modal = Modal::new( Id::new("Modal A"));
            let is_show = self.login_modal_open.clone();
            if is_show{
                // What goes inside the modal
                modal.show(ui.ctx(),|ui| {
                    ui.set_width(250.0);
                    ui.heading("Edit User");
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut self.id);

                    ui.text_edit_singleline(&mut self.token);

                    ui.separator();

                    egui::Sides::new().show(
                        ui,
                        |_ui| {},
                        |ui| {
                            if ui.button("Save").clicked() {
                                self.save_progress= Some(0.1);
                                // self.login_modal_open=false;
                            }
                            if ui.button("Cancel").clicked() {
                                self.login_modal_open=false;
                            }
                        },
                    );
                });
            }

            if let Some(progress) = self.save_progress {
                Modal::new(Id::new("Modal D")).show(ui.ctx(), |ui| {
                    ui.set_width(70.0);
                    ui.heading("Loading…");
    
                    ProgressBar::new(progress).ui(ui);
    
                    if progress >= 1.0 {
                        self.save_progress = None;
                    } else {
                        self.save_progress = Some(progress + 0.003);
                        ui.ctx().request_repaint();
                    }
                });
            }


            if ui.button("Increment").clicked() {
                self.login_modal_open=true;
            }
            ui.label(format!("Hello '{}', token {}", self.token, self.token));

        });
    }
}

 