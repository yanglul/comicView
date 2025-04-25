
use egui::menu::{SubMenu,SubMenuButton};
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
        Ok(Box::<MyApp>::new(MyApp::new(cc)))
    }));
}


use crate::types::Subclass;




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
    category:Vec<Subclass>,
    selected_item:usize,
    selected_key: usize,
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
            selected_item:0 ,
            selected_key: 0,
            category:vec![
            Subclass {
                name: "一类别".to_string(),
                item: vec!["东城区".to_string(), "西城区".to_string(), "朝阳区".to_string()],
            },
            Subclass {
                name: "二类别".to_string(),
                item: vec!["黄浦区".to_string(), "徐汇区".to_string(), "浦东新区".to_string()],
            },
            Subclass {
                name: "三类别".to_string(),
                item: vec!["广州市".to_string(), "深圳市".to_string(), "珠海市".to_string()],
            },
            ]
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

    fn new(cc: &eframe::CreationContext<'_>) -> Self{
        //加载全局字体
        load_global_font(& cc.egui_ctx);
        MyApp::default()
    }

}

///全局加载支持中文的字体
pub fn load_global_font(ctx: &egui::Context){
    let mut fonts = eframe::egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert("msyh".to_owned(),
                           Arc::new(eframe::egui::FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\msyh.ttc")))); // .ttf and .otf supported

    // Put my font first (highest priority):
    fonts.families.get_mut(&eframe::egui::FontFamily::Proportional).unwrap()
        .insert(0, "msyh".to_owned());

    // Put my font as last fallback for monospace:
    fonts.families.get_mut(&eframe::egui::FontFamily::Monospace).unwrap()
        .push("msyh".to_owned());

    // let mut ctx = egui::CtxRef::default();
    ctx.set_fonts(fonts);
}
 



impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let config = RwLock::new({
            Config::new(&self).expect("读取配置失败")
        });
        self.state.set(config);
        let jm_client = JmClient::new(self.clone());

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("top_panel")
            .resizable(true)
            .min_height(32.0)
            .show_inside(ui, |ui| {
                ui.heading("Top Panel");
                if ui.button("Increment").clicked() {
                    self.login_modal_open=true;
                }
                ui.label(format!("Hello '{}', token {}, keyword {}", self.token, self.token, self.keyword));
    
                if ui.button("Search").clicked() {
                    let rt: Runtime = Runtime::new().unwrap();
                    let keyword = "性转";
                    let search_resp = rt.block_on(
                        jm_client.search(&keyword, 1, SearchSort::Latest)
                    ).unwrap();
                    println!("搜索结果{:?}",search_resp);
                }
                
            });

            egui::SidePanel::left("left_panel")
                .resizable(true)
                .default_width(150.0)
                .width_range(80.0..=200.0)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Left Panel");
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

                        egui::ComboBox::from_id_salt("选择大类")
                        .selected_text(&self.keyword)
                        .show_ui(ui, |ui| {
                            for (i, subc) in self.category.iter().enumerate() {
                                ui.menu_button(subc.name.clone(), |ui| {
                                    for (j,item) in self.category[i].item.iter().enumerate(){
                                        if ui.button(item).clicked(){
                                        }
                                    }

                                });
                            }
                            
                        });


                        

                    }
                
                
                );
            });

            egui::ScrollArea::both()
            .show(ui,|ui|{
                ui.columns(5, |columns| {
                    for i in  0..10 {
                        columns[i % 5].image(egui::include_image!("C:\\tmp\\1.jpg"));
                    }
                });
            });
 
            


        });
    }
}



 
