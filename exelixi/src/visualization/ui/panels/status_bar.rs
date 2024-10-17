use crate::prelude::*;

use bevy::diagnostic::DiagnosticsStore;
use bevy_egui::egui::{FontId, RichText};

pub fn ui_status_bar(
    mut contexts: EguiContexts,
    simulation: Res<Simulation>,
    ecosystem: Res<EcosystemRuntime>,
    mut action_state: ResMut<ActionState<SimulationAction>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    egui::TopBottomPanel::bottom("status_bar")
        .frame(egui::Frame::default().fill(egui::Color32::from_rgb(30, 30, 30)))
        .resizable(false)
        .default_height(UI_STATUS_BAR_HEIGHT)
        .show(contexts.ctx_mut(), |ui| {
            let half_width = ui.available_width() / 2.0;
            let spacing = ui.spacing_mut();
            spacing.button_padding = egui::Vec2::new(2.0, 2.0);
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add_space(30.0);
                ui.heading(format!("Steps: {:6}", ecosystem.steps));
                ui.add_space(ui.available_width() - half_width - 30.0 * 3.5);
                let (text, hover_text) =
                    if simulation.control.state == SimulationControlState::Paused {
                        ("▶", "Run")
                    } else {
                        ("⏸", "Pause")
                    };
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new(text).color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text(hover_text)
                    .clicked()
                {
                    action_state.press(&SimulationAction::PauseUnpause);
                }
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("⬇").color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text("Decrease speed")
                    .clicked()
                {
                    action_state.press(&SimulationAction::Decelerate);
                }
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("⬆").color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text("Increase speed")
                    .clicked()
                {
                    action_state.press(&SimulationAction::Accelerate);
                }
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("⏭").color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text("Fastest")
                    .clicked()
                {
                    action_state.press(&SimulationAction::Fastest);
                }
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("💾").color(egui::Color32::from_rgb(200, 255, 200)),
                    ))
                    .on_hover_text("Save")
                    .clicked()
                {
                    action_state.press(&SimulationAction::Save);
                }
                ui.add_space(ui.available_width() - 60.0);
                let mut fps_s = "N/A".to_string();
                if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                    if let Some(average) = fps.average() {
                        fps_s = format!("{average:.1}");
                    }
                }
                ui.label(RichText::new(format!("fps: {fps_s}")).font(FontId::proportional(10.0)));
            });
        });
}
