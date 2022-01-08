use crate::prelude::*;

pub fn debug_ui(
    egui_ctx: Res<EguiContext>,
    selection: Query<(&Transform, &Velocity, &Stomach), With<Selected>>,
    mut simulation: ResMut<Simulation>,
    config: Res<SimulationConfig>,
    diagnostics: Res<Diagnostics>,
) {
    let (transform, velocity, stomach) = selection.iter().next().unwrap();
    let mut new_speed = None;
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
            let sts = ((simulation.generation * config.generation_length) + simulation.age) as f64
                / simulation.duration.as_secs_f64();
            ui.label(format!("sps: {:.0}", sts));
        });
    let mut fonts = egui::FontDefinitions::default();
    // Large button text:
    fonts.family_and_size.insert(
        egui::TextStyle::Body,
        (egui::FontFamily::Proportional, 20.0),
    );
    fonts.family_and_size.insert(
        egui::TextStyle::Button,
        (egui::FontFamily::Proportional, 30.0),
    );
    egui_ctx.ctx().set_fonts(fonts);
    egui::TopBottomPanel::bottom("bottom_panel")
        .frame(
            egui::Frame::default()
                .fill(egui::Color32::from_rgb(0, 0, 30))
                .margin(egui::Vec2::new(10.0, 10.0)),
        )
        .show(egui_ctx.ctx(), |ui| {
            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                ui.add_space(30.0);
                ui.label(format!("generation : {:3}", simulation.generation));
                ui.label(format!("age: {:4}", simulation.age));
                ui.add_space(ui.available_size().x * 0.4);
                let text;
                let hover_text;
                if simulation.speed == SimulationSpeed::Paused {
                    text = "▶";
                    hover_text = "Normal Speed";
                } else {
                    text = "⏸";
                    hover_text = "Pause"
                };
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new(text).color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text(hover_text)
                    .clicked()
                {
                    new_speed = if simulation.speed != SimulationSpeed::Paused {
                        Some(SimulationSpeed::Paused)
                    } else {
                        Some(SimulationSpeed::Normal)
                    };
                }
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("⏩").color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text("Fast Speed")
                    .clicked()
                {
                    new_speed = Some(SimulationSpeed::Fast);
                }
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("⏭").color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text("Maximum Speed")
                    .clicked()
                {
                    new_speed = Some(SimulationSpeed::Fastest);
                }
            });
        });
    if let Some(new_speed) = new_speed {
        simulation.speed = new_speed;
    }
}
