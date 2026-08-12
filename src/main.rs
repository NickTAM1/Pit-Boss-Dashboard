use eframe::egui;
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};
use tungstenite::{Message, connect};

const BACKEND_URL: &str = "ws://127.0.0.1:3000/ws";

enum NetworkEvent {
    Connected,
    Message(String),
    Error(String),
    Disconnected,
}

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

struct PitBossApp {
    event_tx: Sender<NetworkEvent>,
    event_rx: Receiver<NetworkEvent>,
    connection_started: bool,
    status: String,
    last_message: String,
    error_message: String,
}

impl Default for PitBossApp {
    fn default() -> Self {
        let (event_tx, event_rx) = mpsc::channel();

        Self {
            event_tx,
            event_rx,
            connection_started: false,
            status: "Connecting...".to_string(),
            last_message: String::new(),
            error_message: String::new(),
        }
    }
}

impl PitBossApp {
    fn start_connection(&mut self) {
        if self.connection_started {
            return;
        }

        self.connection_started = true;
        self.status = "Connecting...".to_string();

        let event_tx = self.event_tx.clone();
        thread::spawn(move || {
            let result = connect(BACKEND_URL);

            match result {
                Ok((mut socket, _response)) => {
                    let _ = event_tx.send(NetworkEvent::Connected);

                    if let Err(error) = socket.send(Message::Text(
                        r#"{"type":"desktop","message":"dashboard-online"}"#.into(),
                    )) {
                        let _ = event_tx.send(NetworkEvent::Error(error.to_string()));
                        return;
                    }

                    loop {
                        match socket.read() {
                            Ok(Message::Text(message)) => {
                                let _ = event_tx.send(NetworkEvent::Message(message.to_string()));
                            }
                            Ok(Message::Close(_)) => {
                                let _ = event_tx.send(NetworkEvent::Disconnected);
                                break;
                            }
                            Ok(_) => {}
                            Err(error) => {
                                let _ = event_tx.send(NetworkEvent::Error(error.to_string()));
                                break;
                            }
                        }
                    }
                }
                Err(error) => {
                    let _ = event_tx.send(NetworkEvent::Error(error.to_string()));
                }
            }
        });
    }

    fn process_network_events(&mut self) {
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                NetworkEvent::Connected => {
                    self.status = "Connected".to_string();
                    self.error_message.clear();
                }
                NetworkEvent::Message(message) => {
                    self.last_message = message;
                }
                NetworkEvent::Error(error) => {
                    self.status = "Connection error".to_string();
                    self.error_message = error;
                    self.connection_started = false;
                }
                NetworkEvent::Disconnected => {
                    self.status = "Disconnected".to_string();
                    self.connection_started = false;
                }
            }
        }
    }
}

impl eframe::App for PitBossApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.start_connection();
        self.process_network_events();
        ui.ctx().request_repaint_after(Duration::from_millis(100));

        egui::Panel::right("vip_panel")
            .resizable(false)
            .exact_size(300.0)
            .show(ui, |ui| {
                ui.add_space(10.0);
                ui.heading("VIP Activity");
                ui.separator();

                ui.add_space(10.0);
                let status_color = if self.status == "Connected" {
                    egui::Color32::GREEN
                } else if self.status == "Connection error" {
                    egui::Color32::RED
                } else {
                    egui::Color32::YELLOW
                };

                ui.label(egui::RichText::new(&self.status).color(status_color));

                if !self.last_message.is_empty() {
                    ui.separator();
                    ui.label("Last server message:");
                    ui.label(&self.last_message);
                }

                if !self.error_message.is_empty() {
                    ui.separator();
                    ui.colored_label(egui::Color32::RED, &self.error_message);
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    if ui.button("Simulate Alert").clicked() {
                        println!(
                            "Simulate button clicked. The desktop WebSocket is read-only for now."
                        );
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
