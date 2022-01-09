use crate::prelude::*;

pub fn debug_ui(
    egui_ctx: Res<EguiContext>,
    selection: Query<(&Transform, &Velocity, &Stomach), With<Selected>>,
    simulation: ResMut<Simulation>,
    config: Res<SimulationConfig>,
    diagnostics: Res<Diagnostics>,
) {
    let (transform, velocity, stomach) = selection.iter().next().unwrap();
    egui::Window::new("Debug")
        .vscroll(true)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Simulation");
            ui.label(format!("width : {}", config.environment_size.width));
            ui.label(format!("height: {}", config.environment_size.height));
            ui.label(simulation.speed.to_string());
            ui.label("fitness".to_string());
            ui.label(format!(
                "    min: {:.2}",
                simulation.statistics.min_fitness()
            ));
            ui.label(format!(
                "    max: {:.2}",
                simulation.statistics.max_fitness()
            ));
            ui.label(format!(
                "    avg: {:.2}",
                simulation.statistics.avg_fitness()
            ));
            ui.heading("Selection");
            ui.label(format!("x: {:.1}", transform.translation.x));
            ui.label(format!("y: {:.1}", transform.translation.y));
            ui.label(format!("linear: {:.1}", velocity.linear));
            ui.label(format!("angular: {:.1}", velocity.angular));
            ui.label(format!("satiation: {:.1}", stomach.satiation));
            ui.separator();

            let mut fps_s = "N/A".to_string();
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(average) = fps.average() {
                    fps_s = format!("{:.1}", average);
                }
            }
            ui.label(format!("fps: {}", fps_s));
        });
}
pub fn status_bar_ui(
    egui_ctx: Res<EguiContext>,
    mut simulation: ResMut<Simulation>,
    mut orthos: Query<&mut OrthographicProjection>,
) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.family_and_size.insert(
        egui::TextStyle::Body,
        (egui::FontFamily::Proportional, 20.0),
    );
    fonts.family_and_size.insert(
        egui::TextStyle::Monospace,
        (egui::FontFamily::Proportional, 20.0),
    );
    fonts.family_and_size.insert(
        egui::TextStyle::Button,
        (egui::FontFamily::Proportional, 30.0),
    );
    egui_ctx.ctx().set_fonts(fonts);

    let mut new_speed = None;
    egui::TopBottomPanel::bottom("bottom_panel")
        .frame(
            egui::Frame::default()
                .fill(egui::Color32::from_rgb(0, 0, 0))
                .margin(egui::Vec2::new(10.0, 10.0)),
        )
        .show(egui_ctx.ctx(), |ui| {
            let half_width = ui.available_width() / 2.0;
            let mut spacing = ui.spacing_mut();
            spacing.button_padding = egui::Vec2::new(2.0, 2.0);
            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                ui.add_space(30.0);
                ui.label(
                    egui::RichText::new(format!("generation : {:3}", simulation.generation))
                        .text_style(egui::TextStyle::Monospace),
                );
                ui.label(
                    egui::RichText::new(format!("step : {:4}", simulation.age))
                        .text_style(egui::TextStyle::Monospace),
                );
                ui.add_space(ui.available_width() - half_width - 30.0 * 2.0);
                for (text, hover_text, speed) in &[
                    ("⏸", "Pause", SimulationSpeed::Paused),
                    ("▶", "Normal Speed", SimulationSpeed::Normal),
                    ("⏩", "Fast Speed", SimulationSpeed::Fast),
                    ("⏭", "Maximum Speed", SimulationSpeed::Fastest),
                ] {
                    if ui
                        .add(egui::Button::new(
                            egui::RichText::new(*text).color(egui::Color32::from_rgb(0, 255, 0)),
                        ))
                        .on_hover_text(*hover_text)
                        .clicked()
                    {
                        new_speed = Some(*speed);
                    }
                }
            });
        });
    if let Some(new_speed) = new_speed {
        simulation.speed = new_speed;
        for mut ortho in orthos.iter_mut() {
            ortho.scale *= 2.0;
        }
    }
}
