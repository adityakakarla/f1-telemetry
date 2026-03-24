mod f1;

use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints, Points};
use f1::hamilton::{CarData, fetch_car_data};

struct MyApp {
    car_data: Vec<CarData>,
}

impl MyApp {
    fn new() -> Self {
        let car_data = fetch_car_data().unwrap_or_else(|e| {
            eprintln!("Error fetching car data: {e:?}");
            vec![]
        });
        println!("{}", car_data.len());
        Self { car_data }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Lewis Hamilton");

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
