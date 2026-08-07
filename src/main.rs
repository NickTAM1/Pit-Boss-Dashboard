use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Pit Boss Operations"),
        ..Default::default()
    };

    eframe::run_native(
        "Pit Boss Operations",
        options,
        Box::new(|_cc| Ok(Box::<PitBossApp>::default())),
    )
}

#[derive(Default)]
struct PitBossApp;

impl eframe::App for PitBossApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::right("vip_panel")
            .resizable(false)
            .exact_size(300.0)
            .show(ui, |ui| {
                ui.add_space(10.0);
                ui.heading("VIP Activity");
                ui.separator();

                ui.add_space(10.0);
                ui.label(egui::RichText::new("System Online").color(egui::Color32::GREEN));
                ui.label("Awaiting WebSocket connection...");

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    if ui.button("Simulate Alert").clicked() {
                        println!("Simulate button clicked! Later, this will inject mock data.");
                    }
                });
            });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.add_space(10.0);
            ui.heading("Live Floor Heatmap");
            ui.separator();

            ui.centered_and_justified(|ui| {
                ui.label(
                    egui::RichText::new(
                        "Map rendering area initialized.\nReady for 2D grid setup.",
                    )
                    .size(24.0)
                    .color(egui::Color32::DARK_GRAY),
                );
            });
        });
    }
}
