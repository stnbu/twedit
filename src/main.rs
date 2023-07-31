use eframe::egui;

const DELIMITER: &str = "XXXXXXX";
const N: usize = 3;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native("twedit", options, Box::new(|_cc| Box::<App>::default()))
}

struct App {
    input: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let result_text = construct_display_text(&self.input);

            // Render the text area
            ui.add(egui::TextEdit::multiline(&mut self.input).desired_width(f32::INFINITY));

            // Render the result below it
            ui.group(|ui| {
                ui.label("Result:");
                ui.monospace(result_text);
            });
        });
    }
}

/// Construct the display text with delimiters.
fn construct_display_text(input: &str) -> String {
    let chars: Vec<_> = input.chars().collect();
    let mut result = String::new();
    let mut count = 0;
    let mut delimiter_count = 0;

    for c in &chars {
        if count == 0 {
            result.push_str(&format_delimiter(delimiter_count, chars.len() / N));
            delimiter_count += 1;
        }
        result.push(*c);
        count += 1;
        if count == N {
            count = 0;
        }
    }
    if count != 0 || chars.len() == 0 {
        result.push_str(&format_delimiter(delimiter_count, chars.len() / N + 1));
    }
    result
}

/// Format the delimiter with the current and total count.
fn format_delimiter(current: usize, total: usize) -> String {
    format!("\n{:02}/{:02}>", current + 1, total)
}
