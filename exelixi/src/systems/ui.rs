use crate::prelude::*;

pub fn debug_ui(
    egui_ctx: Res<EguiContext>,
    selection: Query<(&Transform, &Velocity, &Stomach), With<Selected>>,
    mut simulation: ResMut<Simulation>,
    config: Res<SimulationConfig>,
    diagnostics: Res<Diagnostics>,
) {
    let (transform, velocity, stomach) = selection.iter().next().unwrap();
    let mut invert_running = false;
    egui::Window::new("Debug")
        .vscroll(true)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Simulation");
            ui.label(format!("width : {}", config.environment_size.width));
            ui.label(format!("height: {}", config.environment_size.height));
            ui.label(format!("generation : {}", simulation.generation));
            ui.label(format!("age: {}", simulation.age));
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
            let running_button_s = if simulation.running { "Pause" } else { "Run" };
            invert_running = ui.button(running_button_s).clicked();
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
    if invert_running {
        simulation.running = !simulation.running;
    }
}
