use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use eframe::egui;
use eframe::egui::RichText;

const N: usize = 280; // Twitter's character limit

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native("twedit", options, Box::new(|_cc| Box::<App>::default()))
}

struct App {
    input: String,
    tweets: Vec<String>,
    clicked_buttons: Vec<bool>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input: String::new(),
            tweets: Vec::new(),
            clicked_buttons: Vec::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Main textarea for user input
            ui.add(egui::TextEdit::multiline(&mut self.input).desired_width(200.0));

            // Update tweets list based on input
            self.tweets = split_into_tweets(&self.input);
            self.clicked_buttons.resize(self.tweets.len(), false);

            // Side column for copy buttons
            egui::SidePanel::right("side_panel").show(ctx, |ui| {
                for (index, tweet) in self.tweets.iter().enumerate() {
                    if ui
                        .button(format!("{}/{}", index + 1, self.tweets.len()))
                        .clicked()
                    {
                        copy_to_clipboard(tweet);
                        self.clicked_buttons[index] = true;
                    }
                    //ui.label(RichText::new("Red text").color(Color32::RED));
                    if self.clicked_buttons[index] {
                        // For demonstration, change the background color to show it has been clicked.
                        //ui.label("Copied").color(egui::Color32::from_rgb(0, 255, 0));
                        //                            .set_bg_color(egui::Color32::from_black_alpha(192));
                        //ui.label(RichText("Copied").color(egui::Color32::from_rgb(0, 255, 0)));
                        ui.label(
                            RichText::new("Copied").color(egui::Color32::from_black_alpha(192)),
                        );
                    }
                }
            });

            // Preview area
            egui::ScrollArea::vertical()
                .max_width(200.0)
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if self.tweets.len() > 1 {
                        for tweet in &self.tweets {
                            ui.group(|ui| {
                                ui.label(tweet);
                                ui.horizontal(|ui| {
                                    ui.separator();
                                });
                            });
                        }
                    } else {
                        ui.label("Preview:");
                    }
                });
        });
    }
}

/// Split the input text into tweets
fn split_into_tweets(input: &str) -> Vec<String> {
    let chars: Vec<_> = input.chars().collect();
    let mut tweets = Vec::new();

    for chunk in chars.chunks(N) {
        tweets.push(chunk.iter().collect());
    }

    // Add the tweet numbering
    let tweets_len = tweets.len();
    tweets.iter_mut().enumerate().for_each(|(i, tweet)| {
        *tweet = format!("{}/{}> {}", i + 1, tweets_len, tweet);
    });

    tweets
}

/// Copies the provided text to the system clipboard.
fn copy_to_clipboard(text: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(text.to_owned()).unwrap();
}
