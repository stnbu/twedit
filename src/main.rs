use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    eframe::run::<MyApp>()
}

struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        const RIGHT_MARGIN_WIDTH: f32 = 100.0;
        const TOP_MARGIN_HEIGHT: f32 = 50.0;
        const TEXT_AREA_MIN_WIDTH: f32 = 50.0;

        egui::TopPanel::top("top_margin")
            .fixed_height(TOP_MARGIN_HEIGHT)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Top Margin Content Here");
                    ui.add_space(RIGHT_MARGIN_WIDTH);
                });
            });

        egui::SidePanel::right("right_margin")
            .fixed_width(RIGHT_MARGIN_WIDTH)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(TOP_MARGIN_HEIGHT);
                    ui.label("Right Margin Content Here");
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (_, main_area) = ui.split_vertically(ui.available_size().y - TOP_MARGIN_HEIGHT);
            main_area.split(|ui| {
                ui.group(|ui| {
                    ui.set_min_width(TEXT_AREA_MIN_WIDTH);
                    ui.label("Widget W1");
                });

                ui.group(|ui| {
                    ui.set_min_width(TEXT_AREA_MIN_WIDTH);
                    ui.label("Widget W2");
                });
            });
        });
    }
}
