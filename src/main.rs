
use parking_lot::RwLock;

// TODO: 用prelude来消除警告
use crate::commands::*;
use crate::config::Config;
use crate::download_manager::DownloadManager;
use crate::events::prelude::*;
use crate::jm_client::JmClient;
use tokio::runtime::Runtime;

mod commands;
mod config;
mod download_manager;
mod errors;
mod events;
mod extensions;
mod jm_client;
mod responses;
mod save_archive;
mod types;
mod utils;
mod state;

use eframe::egui;
use egui::{ComboBox, Id, Modal, ProgressBar, Ui, Widget, Window};
use std::sync::Arc;
use crate::state::StateManager;
use crate::responses::GetUserProfileRespData;
use crate::types::SearchSort;

fn main() {
    // 自定义格式（包含时间、日志级别、目标模块）
    tracing_subscriber::fmt()
        .with_target(true) // 显示模块路径
        .with_timer(tracing_subscriber::fmt::time::uptime()) // 显示时间
        .init();
    let native_options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([640.0, 300.0]) // wide enough for the drag-drop overlay text
        .with_drag_and_drop(true),
        ..Default::default()
    };
    
    


    let _ = eframe::run_native("ComicView", native_options, Box::new(|cc| {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Ok(Box::<MyApp>::default())}));
}

#[derive(Clone)]
struct MyApp {
    id: String,
    token: String,
    pwd: String,
    login_modal_open: bool,
    imgpath: Option<String>,
    save_progress: Option<f32>,
    img:egui::widgets::ImageSource<'static>,
    state: Arc<StateManager>,
    user_profile:GetUserProfileRespData,
    keyword:String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            id: "Arthur".to_owned(),
            token: "Aloha".to_owned(),
            pwd: "".to_owned(),
            login_modal_open: false,
            save_progress: None,
            imgpath: None,
            img:egui::include_image!("ferris.svg"),
            state: Arc::new(StateManager::new()),
            user_profile:GetUserProfileRespData::default(),
            keyword:"".to_owned(),
        }
    }

}
use crate::state::State;
impl MyApp{
    fn state<T>(&self) -> State<'_, T>
    where
        T: Send + Sync + 'static,
      {
        self.state
          .try_get()
          .expect("state() called before manage() for given type")
      }

}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let config = RwLock::new({
                Config::new(&self).expect("读取配置失败")
            });
            self.state.set(config);
            let jm_client = JmClient::new(self.clone());
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
                                let rt: Runtime = Runtime::new().unwrap();

                                let user_profile = rt.block_on(jm_client.login(&self.id, &self.token)).unwrap();
                                println!("登录接口返回:{:?}",user_profile);
                                // self.imgpath = Some("file://C:\\tmp\\1.jpg".to_string());
                                self.user_profile = user_profile;;
                                self.login_modal_open=false;
                            }
                            if ui.button("Cancel").clicked() {
                                self.login_modal_open=false;
                            }
                        },
                    );
                });
            }

            if let Some(imgpath) = &self.imgpath {
                // ui.image(imgpath);
            }else{
                // ui.image(egui::include_image!("C:\\tmp\\3.jpg")).on_hover_text_at_pointer("Svg");

            }
            
            


            if ui.button("Increment").clicked() {
                self.login_modal_open=true;
            }
            ui.label(format!("Hello '{}', token {}", self.token, self.token));

            if ui.button("Search").clicked() {
                let rt: Runtime = Runtime::new().unwrap();
                let keyword = "性转";
                let search_resp = rt.block_on(
                    jm_client.search(&keyword, 1, SearchSort::Latest)
                ).unwrap();
                println!("搜索结果{:?}",search_resp);
            }
            
            if ui.button("类别").clicked(){
                ui.menu_button("Popups can have submenus", |ui| {
                    ui.menu_button("SubMenu1", |ui| {
                        
                        let _ = ui.button("Item1");
                        let _ = ui.button("Item2");
                    });
                    ui.menu_button("SubMenu2", |ui| {
                        let _ = ui.button("Item3");
                    });
                    let _ = ui.button("Item4");
                });
            
            }


            
        });
    }
}


