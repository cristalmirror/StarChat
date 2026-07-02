use eframe::egui;

fn main() -> eframe::Result<()> {
    let opciones = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([250.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Mi App",
        opciones,
        Box::new(|_cc| Ok(Box::new(MiApp::default()))),
    )
}

#[derive(Default)]
struct MiApp {
    texto: String,
}

impl eframe::App for MiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("panel_inferior")
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    let input = ui.add(
                        egui::TextEdit::singleline(&mut self.texto)
                            .hint_text("Escribe algo...")
                            .desired_width(ui.available_width() - 60.0),
                    );
                    let boton = ui.button("Enviar");

                    if boton.clicked()
                        || (input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                    {
                        println!("Text0: {}", self.texto);
                        self.texto.clear();
                    }
                });
                ui.add_space(6.0);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hola Mundo!!");
        });
    }
}
