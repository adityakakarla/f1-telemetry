mod f1;

use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints, Points};
use f1::car_data::{CarData, fetch_car_data};

struct Driver {
    name: &'static str,
    number: u32,
}

const DRIVERS: &[Driver] = &[
    Driver { name: "Max Verstappen", number: 1 },
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
    Driver { name: "Franco Colapinto", number: 43 },
    Driver { name: "Lewis Hamilton", number: 44 },
    Driver { name: "Carlos Sainz", number: 55 },
    Driver { name: "George Russell", number: 63 },
    Driver { name: "Valtteri Bottas", number: 77 },
    Driver { name: "Lando Norris", number: 81 },
    Driver { name: "Oliver Bearman", number: 87 },
    Driver { name: "Arvid Lindblad", number: 41 },
    Driver { name: "Oscar Piastri", number: 4 },
];

struct MyApp {
    car_data: Vec<CarData>,
    selected_driver_idx: usize,
    loading: bool,
}

impl MyApp {
    fn new() -> Self {
        // Default to Lewis Hamilton (index of driver #44)
        let default_idx = DRIVERS.iter().position(|d| d.number == 44).unwrap_or(0);
        let car_data = fetch_car_data(DRIVERS[default_idx].number).unwrap_or_else(|e| {
            eprintln!("Error fetching car data: {e:?}");
            vec![]
        });
        Self {
            car_data,
            selected_driver_idx: default_idx,
            loading: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("F1 Telemetry");

                ui.add_space(16.0);

                let current_label = format!(
                    "{} ({})",
                    DRIVERS[self.selected_driver_idx].name,
                    DRIVERS[self.selected_driver_idx].number
                );

                egui::ComboBox::from_label("Driver")
                    .selected_text(current_label)
                    .show_ui(ui, |ui| {
                        for (i, driver) in DRIVERS.iter().enumerate() {
                            let label = format!("{} ({})", driver.name, driver.number);
                            if ui.selectable_value(&mut self.selected_driver_idx, i, label).clicked() {
                                self.loading = true;
                            }
                        }
                    });
            });

            if self.loading {
                self.loading = false;
                self.car_data = fetch_car_data(DRIVERS[self.selected_driver_idx].number)
                    .unwrap_or_else(|e| {
                        eprintln!("Error fetching car data: {e:?}");
                        vec![]
                    });
            }

            let speed_data: Vec<[f64; 2]> = self
                .car_data
                .iter()
                .enumerate()
                .filter_map(|(i, d)| d.speed.map(|s| [i as f64, s as f64]))
                .collect();

            let speed_line = Line::new("Speed (km/h)", PlotPoints::new(speed_data.clone()));
            let speed_points =
                Points::new("Sample Points", PlotPoints::new(speed_data)).radius(4.0);

            let plot = Plot::new("Telemetry")
                .legend(Legend::default().title("Channels"))
                .show_axes(true)
                .show_grid(true);

            plot.show(ui, |plot_ui| {
                plot_ui.line(speed_line);
                plot_ui.points(speed_points);
            });
        });
    }
}

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "F1 Telemetry",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    )
}
