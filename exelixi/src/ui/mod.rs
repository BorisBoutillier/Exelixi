use crate::prelude::*;

mod eye_viewer;
mod panels;
mod selection;
mod user_selection;

pub use eye_viewer::*;
pub use panels::*;
pub use selection::*;
pub use user_selection::*;

pub const UI_STATUS_BAR_HEIGHT: f32 = 40.0;
pub const UI_LEFT_PANEL_WIDTH: f32 = 400.0;

#[derive(Resource)]
pub struct UiState {
    // Does the Left panel for statistics is visible
    pub stat_panel: bool,
}
//
pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiState { stat_panel: true })
            .add_plugin(EyeViewerPlugin)
            //.add_system(_debug_ui)
            .add_system(panels_ui)
            .add_system(user_selection)
            .add_system_to_stage(CoreStage::PostUpdate, selection_changed);
    }
}

pub fn _debug_ui(
    mut egui_ctx: ResMut<EguiContext>,
    selection: Query<(&Locomotion, &Body), With<Selected>>,
    simulation: Res<Simulation>,
    config: Res<SimulationConfig>,
    diagnostics: Res<Diagnostics>,
) {
    egui::Window::new("Debug")
        .vscroll(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Simulation");
            ui.label(format!("width : {}", config.environment.width));
            ui.label(format!("height: {}", config.environment.height));
            if let Ok((locomotion, body)) = selection.get_single() {
                ui.heading("Selection");
                ui.label(format!("linear: {:.1}", locomotion.linear));
                ui.label(format!("angular: {:.1}", locomotion.angular));
                ui.label(format!("energy: {:.1}", body.energy()));
            }
            ui.separator();

            let mut fps_s = "N/A".to_string();
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(average) = fps.average() {
                    fps_s = format!("{average:.1}");
                }
            }
            ui.label(format!("fps: {fps_s}"));
            ui.label(format!("sps: {:.2}", simulation.sps(&config)));
        });
}
