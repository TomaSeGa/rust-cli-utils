use eframe::egui;

fn main() -> eframe::Result<()> 
{
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "TODO List App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

#[derive(Default)]
struct MyApp
{
    text: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) 
    {
        egui::CentralPanel::default().show(ctx, |ui| 
        {
            ui.heading("TODO list start");
            ui.text_edit_multiline(&mut self.text);
        });
    }
}