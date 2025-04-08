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
mod modal;
 

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
impl modal::Demo for MyApp {
    fn name(&self) -> &'static str {
        "🗖 Modals"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        use modal::View as _;
        Window::new(self.name())
            .open(open)
            .vscroll(false)
            .resizable(false)
            .show(ctx, |ui| self.ui(ui));
    }
}


impl modal::View for MyApp {
    fn ui(&mut self, ui: &mut Ui) {
        let Self {
            id,
            token,
            pwd,
            login_modal_open,
            save_progress,
        } = self;

        ui.horizontal(|ui| {
            if ui.button("Open User Modal").clicked() {
                *login_modal_open = true;
            }
        });


        if *login_modal_open {
            let modal = Modal::new(Id::new("Modal A")).show(ui.ctx(), |ui| {
                ui.set_width(250.0);

                ui.heading("Edit User");

                ui.label("Name:");
                ui.text_edit_singleline(id);

                ui.text_edit_singleline(pwd);

                ui.separator();

                egui::Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        if ui.button("Save").clicked() {
                            *save_progress = Some(0.0);
                        }
                        if ui.button("Cancel").clicked() {
                            // You can call `ui.close()` to close the modal.
                            // (This causes the current modals `should_close` to return true)
                            *login_modal_open = false;
                        }
                    },
                );
            });

            if modal.should_close() {
                *login_modal_open = false;
            }
        }

         

        if let Some(progress) = *save_progress {
            Modal::new(Id::new("Modal C")).show(ui.ctx(), |ui| {
                ui.set_width(70.0);
                ui.heading("Saving…");

                ProgressBar::new(progress).ui(ui);

                if progress >= 1.0 {
                    *save_progress = None;
                    *login_modal_open = false;
                } else {
                    *save_progress = Some(progress + 0.003);
                    ui.ctx().request_repaint();
                }
            });
        }

        ui.vertical_centered(|ui| {
            ui.add(egui_github_link_file!());
        });
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
            if ui.button("Increment").clicked() {
                self.login_modal_open = true;
            }
            ui.label(format!("Hello '{}', token {}", self.token, self.token));

        });
    }
}


/// Create a [`Hyperlink`](egui::Hyperlink) to this egui source code file on github.
#[macro_export]
macro_rules! egui_github_link_file {
    () => {
        $crate::egui_github_link_file!("(source code)")
    };
    ($label: expr) => {
        egui::github_link_file!(
            "https://github.com/emilk/egui/blob/master/",
            egui::RichText::new($label).small()
        )
    };
}