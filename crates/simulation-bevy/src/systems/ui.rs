use crate::prelude::*;

pub fn debug_ui(
    windows: Res<Windows>,
    egui_ctx: Res<EguiContext>,
    selection: Query<(&Transform, &Velocity, &Stomach), With<Selected>>,
    simulation: Res<Simulation>,
) {
    let window = windows.get_primary().unwrap();
    let (transform, velocity, stomach) = selection.iter().next().unwrap();
    egui::Window::new("Debug")
        .vscroll(true)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Window");
            ui.label(format!("width : {}", window.width()));
            ui.label(format!("height: {}", window.height()));
            ui.heading("Simulation");
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
            ui.heading("Selection");
            ui.label(format!("x: {:.1}", transform.translation.x));
            ui.label(format!("y: {:.1}", transform.translation.y));
            ui.label(format!("linear: {:.1}", velocity.linear));
            ui.label(format!("angular: {:.1}", velocity.angular));
            ui.label(format!("satiation: {:.1}", stomach.satiation));
        });
}
