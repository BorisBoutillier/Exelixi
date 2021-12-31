use crate::prelude::*;

pub fn debug_ui(
    time: Res<Time>,
    windows: Res<Windows>,
    mut egui_ctx: ResMut<EguiContext>,
    mut selection: Query<(&Transform, &Velocity)>,
) {
    let window = windows.get_primary().unwrap();
    let (transform, velocity) = selection.iter().next().unwrap();
    egui::Window::new("Debug")
        .vscroll(true)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Window");
            ui.label(format!("width : {}", window.width()));
            ui.label(format!("height: {}", window.height()));
            ui.heading("Selection");
            ui.label(format!("x: {:.1}", transform.translation.x));
            ui.label(format!("y: {:.1}", transform.translation.y));
        });
}
