use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints, Points};

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Lewis Hamilton");

            // Sample F1 speed telemetry over a lap (time in seconds, speed in km/h)
            let speed_data: Vec<[f64; 2]> = vec![
                [0.0, 0.0],
                [2.0, 120.0],
                [5.0, 280.0],
                [10.0, 310.0],
                [15.0, 295.0],
                [20.0, 80.0], // braking zone
                [22.0, 60.0], // corner
                [25.0, 200.0],
                [30.0, 320.0],
                [35.0, 315.0],
                [40.0, 70.0], // braking zone
                [43.0, 55.0], // corner
                [47.0, 240.0],
                [52.0, 330.0],
            ];

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
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
