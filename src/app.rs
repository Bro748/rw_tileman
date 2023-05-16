use egui::CollapsingHeader;

use crate::{
    lingo_de::{self},
    TileInfo, TileInit,
};
#[derive(Debug)]
pub enum AppError {
    IOError(std::io::Error),
    Todo,
}
pub struct TilemanApp {
    path: String,
    selected_tile: Option<TileInfo>,
    all_tiles: TileInit,
    dumped_errors: bool,
}

impl TilemanApp {
    pub fn new(_cc: &eframe::CreationContext) -> Result<Self, AppError> {
        let path = String::from("testfiles");
        Ok(Self {
            path: path.clone(),
            selected_tile: Default::default(),
            all_tiles: lingo_de::parse_multiple_tile_info(
                &std::fs::read_to_string(path.as_str()).unwrap(),
            )?,
            dumped_errors: false,
        })
    }
}

impl Default for TilemanApp {
    fn default() -> Self {
        Self {
            path: Default::default(),
            selected_tile: Default::default(),
            all_tiles: Default::default(),
            dumped_errors: false,
        }
    }
}

impl eframe::App for TilemanApp {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_close_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {}

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("button1").clicked() {};
                // ui.button("button2");
                // ui.button("button3");
                // ui.button("button4");
            })
        });
        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("tiles");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for category in self.all_tiles.categories.iter_mut() {
                    CollapsingHeader::new(category.name.as_str()).show(ui, |ui| {
                        for item in category.tiles.iter_mut() {
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut item.active, "");
                                if ui.button(item.name.as_str()).clicked() {
                                    println!("{}", item.name);
                                    self.selected_tile = Some(item.clone());
                                    //self.selected_tile = Some(Box::new(*item));
                                };
                            });

                            // if ui.button(item.name.as_str()).clicked() {
                            //     println!("{}", item.name)
                            // };
                        }
                    });
                }
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Path to init");
            ui.text_edit_singleline(&mut self.path);
            ui.label(format!("{:?}", self.selected_tile));
        });

        if !self.dumped_errors {
            std::fs::write(
                "mass_out.txt",
                format!("{:#?}", self.all_tiles.errored_lines),
            )
            .expect("could not write results");
            self.dumped_errors = true;
        }
    }
}
