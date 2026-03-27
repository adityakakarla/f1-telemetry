mod f1;

use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};
use f1::car_data::{CarData, fetch_car_data};
use f1::sessions::{Session, fetch_sessions};
use std::sync::mpsc::{self, Receiver};
use std::thread;

struct Driver {
    name: &'static str,
    number: u32,
}

const DRIVERS: &[Driver] = &[
    Driver { name: "Max Verstappen", number: 1 },
    Driver { name: "Oscar Piastri", number: 4 },
    Driver { name: "Gabriel Bortoleto", number: 5 },
    Driver { name: "Isack Hadjar", number: 6 },
    Driver { name: "Pierre Gasly", number: 10 },
    Driver { name: "Sergio Pérez", number: 11 },
    Driver { name: "Andrea Kimi Antonelli", number: 12 },
    Driver { name: "Fernando Alonso", number: 14 },
    Driver { name: "Charles Leclerc", number: 16 },
    Driver { name: "Lance Stroll", number: 18 },
    Driver { name: "Alexander Albon", number: 23 },
    Driver { name: "Nico Hülkenberg", number: 27 },
    Driver { name: "Liam Lawson", number: 30 },
    Driver { name: "Esteban Ocon", number: 31 },
    Driver { name: "Arvid Lindblad", number: 41 },
    Driver { name: "Franco Colapinto", number: 43 },
    Driver { name: "Lewis Hamilton", number: 44 },
    Driver { name: "Carlos Sainz", number: 55 },
    Driver { name: "George Russell", number: 63 },
    Driver { name: "Valtteri Bottas", number: 77 },
    Driver { name: "Lando Norris", number: 81 },
    Driver { name: "Oliver Bearman", number: 87 },
];

fn driver_color(number: u32) -> egui::Color32 {
    match number {
        81 | 4 => egui::Color32::from_rgb(255, 128, 0),    // McLaren orange
        44 | 16 => egui::Color32::from_rgb(220, 0, 0),     // Ferrari red
        1 | 6 => egui::Color32::from_rgb(30, 65, 255),     // Red Bull blue
        63 | 12 => egui::Color32::from_rgb(0, 210, 190),   // Mercedes teal
        55 | 23 => egui::Color32::from_rgb(0, 130, 250),   // Williams blue
        30 | 41 => egui::Color32::from_rgb(100, 145, 255), // Racing Bulls
        14 | 18 => egui::Color32::from_rgb(0, 160, 100),   // Aston Martin green
        87 | 31 => egui::Color32::from_rgb(180, 180, 180), // Haas gray
        27 | 5 => egui::Color32::from_rgb(150, 20, 20),    // Audi dark red
        10 | 43 => egui::Color32::from_rgb(255, 0, 145),   // Alpine pink
        11 | 77 => egui::Color32::from_rgb(200, 200, 255), // Cadillac
        _ => egui::Color32::WHITE,
    }
}

fn session_label(session: &Session) -> String {
    let date = if session.date_start.len() >= 10 {
        &session.date_start[..10]
    } else {
        session.date_start.as_str()
    };
    format!("{} — {} ({})", session.location, session.session_name, date)
}

struct ChannelToggles {
    speed: bool,
    throttle: bool,
    brake: bool,
    rpm: bool,
    gear: bool,
}

struct MyApp {
    year: u32,
    sessions: Vec<Session>,
    selected_session_idx: usize,
    sessions_loading: bool,
    sessions_rx: Option<Receiver<Vec<Session>>>,

    selected_driver_idx: usize,

    car_data: Vec<CarData>,
    data_loading: bool,
    data_rx: Option<Receiver<Vec<CarData>>>,

    toggles: ChannelToggles,
}

impl MyApp {
    fn new() -> Self {
        let mut app = Self {
            year: 2025,
            sessions: vec![],
            selected_session_idx: 0,
            sessions_loading: false,
            sessions_rx: None,
            selected_driver_idx: DRIVERS.iter().position(|d| d.number == 44).unwrap_or(0),
            car_data: vec![],
            data_loading: false,
            data_rx: None,
            toggles: ChannelToggles {
                speed: true,
                throttle: false,
                brake: false,
                rpm: false,
                gear: false,
            },
        };
        app.trigger_sessions_fetch();
        app
    }

    fn trigger_sessions_fetch(&mut self) {
        self.sessions = vec![];
        self.car_data = vec![];
        self.sessions_loading = true;
        self.sessions_rx = None;
        let year = self.year;
        let (tx, rx) = mpsc::channel();
        self.sessions_rx = Some(rx);
        thread::spawn(move || {
            let _ = tx.send(fetch_sessions(year));
        });
    }

    fn trigger_data_fetch(&mut self) {
        if self.sessions.is_empty() {
            return;
        }
        let session_key = self.sessions[self.selected_session_idx].session_key;
        let driver_number = DRIVERS[self.selected_driver_idx].number;
        self.car_data = vec![];
        self.data_loading = true;
        self.data_rx = None;
        let (tx, rx) = mpsc::channel();
        self.data_rx = Some(rx);
        thread::spawn(move || {
            let data = fetch_car_data(driver_number, session_key).unwrap_or_default();
            let _ = tx.send(data);
        });
    }

    fn poll_sessions(&mut self, ctx: &egui::Context) {
        if !self.sessions_loading {
            return;
        }
        ctx.request_repaint();
        if let Some(rx) = &self.sessions_rx {
            if let Ok(sessions) = rx.try_recv() {
                self.sessions = sessions;
                self.sessions_loading = false;
                self.sessions_rx = None;
                if !self.sessions.is_empty() {
                    self.selected_session_idx = self.sessions.len() - 1;
                    self.trigger_data_fetch();
                }
            }
        }
    }

    fn poll_data(&mut self, ctx: &egui::Context) {
        if !self.data_loading {
            return;
        }
        ctx.request_repaint();
        if let Some(rx) = &self.data_rx {
            if let Ok(data) = rx.try_recv() {
                self.car_data = data;
                self.data_loading = false;
                self.data_rx = None;
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_sessions(ctx);
        self.poll_data(ctx);

        // Flags set inside closures, acted on after all borrows are released
        let mut reload_sessions = false;
        let mut reload_data = false;

        egui::CentralPanel::default().show(ctx, |ui| {
            // ── Controls row ──────────────────────────────────────────────────
            ui.horizontal(|ui| {
                ui.heading("F1 Telemetry");
                ui.add_space(16.0);

                // Year selector
                let prev_year = self.year;
                egui::ComboBox::from_id_salt("year_select")
                    .selected_text(self.year.to_string())
                    .show_ui(ui, |ui| {
                        for year in [2023u32, 2024, 2025, 2026] {
                            ui.selectable_value(&mut self.year, year, year.to_string());
                        }
                    });
                if self.year != prev_year {
                    reload_sessions = true;
                }

                // Session selector
                if self.sessions_loading {
                    ui.spinner();
                    ui.label("Loading sessions…");
                } else if self.sessions.is_empty() {
                    ui.label("No sessions found");
                } else {
                    let label = session_label(&self.sessions[self.selected_session_idx]);
                    let prev_session = self.selected_session_idx;
                    egui::ComboBox::from_id_salt("session_select")
                        .selected_text(label)
                        .width(280.0)
                        .show_ui(ui, |ui| {
                            for (i, session) in self.sessions.iter().enumerate() {
                                ui.selectable_value(
                                    &mut self.selected_session_idx,
                                    i,
                                    session_label(session),
                                );
                            }
                        });
                    if self.selected_session_idx != prev_session {
                        reload_data = true;
                    }
                }

                // Driver selector
                let driver_label = format!(
                    "{} ({})",
                    DRIVERS[self.selected_driver_idx].name,
                    DRIVERS[self.selected_driver_idx].number,
                );
                let prev_driver = self.selected_driver_idx;
                egui::ComboBox::from_id_salt("driver_select")
                    .selected_text(driver_label)
                    .show_ui(ui, |ui| {
                        for (i, driver) in DRIVERS.iter().enumerate() {
                            ui.selectable_value(
                                &mut self.selected_driver_idx,
                                i,
                                format!("{} ({})", driver.name, driver.number),
                            );
                        }
                    });
                if self.selected_driver_idx != prev_driver {
                    reload_data = true;
                }
            });

            // ── Channel toggles ───────────────────────────────────────────────
            ui.horizontal(|ui| {
                ui.label("Channels:");
                ui.checkbox(&mut self.toggles.speed, "Speed");
                ui.checkbox(&mut self.toggles.throttle, "Throttle");
                ui.checkbox(&mut self.toggles.brake, "Brake");
                ui.checkbox(&mut self.toggles.rpm, "RPM");
                ui.checkbox(&mut self.toggles.gear, "Gear");
            });

            ui.separator();

            // ── Main content ──────────────────────────────────────────────────
            if self.data_loading {
                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Loading telemetry…");
                });
            } else if self.car_data.is_empty() {
                ui.add_space(20.0);
                ui.label("No data available for this driver/session.");
            } else {
                let color = driver_color(DRIVERS[self.selected_driver_idx].number);

                Plot::new("Telemetry")
                    .legend(Legend::default().title("Channels"))
                    .show_axes(true)
                    .show_grid(true)
                    .show(ui, |plot_ui| {
                        if self.toggles.speed {
                            let pts: Vec<[f64; 2]> = self
                                .car_data
                                .iter()
                                .enumerate()
                                .filter_map(|(i, d)| d.speed.map(|v| [i as f64, v as f64]))
                                .collect();
                            plot_ui.line(
                                Line::new("Speed (km/h)", PlotPoints::new(pts)).color(color),
                            );
                        }
                        if self.toggles.throttle {
                            let pts: Vec<[f64; 2]> = self
                                .car_data
                                .iter()
                                .enumerate()
                                .filter_map(|(i, d)| d.throttle.map(|v| [i as f64, v as f64]))
                                .collect();
                            plot_ui.line(
                                Line::new("Throttle (%)", PlotPoints::new(pts))
                                    .color(egui::Color32::from_rgb(0, 200, 80)),
                            );
                        }
                        if self.toggles.brake {
                            let pts: Vec<[f64; 2]> = self
                                .car_data
                                .iter()
                                .enumerate()
                                .filter_map(|(i, d)| d.brake.map(|v| [i as f64, v as f64]))
                                .collect();
                            plot_ui.line(
                                Line::new("Brake (%)", PlotPoints::new(pts))
                                    .color(egui::Color32::from_rgb(255, 60, 60)),
                            );
                        }
                        if self.toggles.rpm {
                            let pts: Vec<[f64; 2]> = self
                                .car_data
                                .iter()
                                .enumerate()
                                .filter_map(|(i, d)| d.rpm.map(|v| [i as f64, v as f64]))
                                .collect();
                            plot_ui.line(
                                Line::new("RPM", PlotPoints::new(pts))
                                    .color(egui::Color32::from_rgb(180, 100, 255)),
                            );
                        }
                        if self.toggles.gear {
                            let pts: Vec<[f64; 2]> = self
                                .car_data
                                .iter()
                                .enumerate()
                                .filter_map(|(i, d)| d.n_gear.map(|v| [i as f64, v as f64]))
                                .collect();
                            plot_ui.line(
                                Line::new("Gear", PlotPoints::new(pts))
                                    .color(egui::Color32::from_rgb(255, 210, 0)),
                            );
                        }
                    });
            }
        });

        // Trigger fetches after closures have released their borrows
        if reload_sessions {
            self.trigger_sessions_fetch();
        } else if reload_data {
            self.trigger_data_fetch();
        }
    }
}

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "F1 Telemetry",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    )
}
