use bevy_egui::egui::{Color32, FontId, Id, Pos2, RichText};

use crate::prelude::*;

// All panels must be declared in the same system as the order of panel creation is important
pub fn ui_status_bar(
    mut contexts: EguiContexts,
    simulation: Res<Simulation>,
    mut action_state: ResMut<ActionState<SimulationSpeedAction>>,
    diagnostics: Res<Diagnostics>,
) {
    //
    // Bottom status bar
    //
    // Always present and span the whole bottom

    egui::TopBottomPanel::bottom("status_bar")
        .frame(egui::Frame::default().fill(egui::Color32::from_rgb(30, 30, 30)))
        .resizable(false)
        .default_height(UI_STATUS_BAR_HEIGHT)
        .show(contexts.ctx_mut(), |ui| {
            let half_width = ui.available_width() / 2.0;
            let mut spacing = ui.spacing_mut();
            spacing.button_padding = egui::Vec2::new(2.0, 2.0);
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add_space(30.0);
                ui.heading(format!("Steps: {:6}", simulation.steps));
                ui.add_space(ui.available_width() - half_width - 30.0 * 3.0);
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
                    action_state.press(SimulationSpeedAction::PauseUnpause);
                }
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("⬇").color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text("Decrease speed")
                    .clicked()
                {
                    action_state.press(SimulationSpeedAction::Decelerate);
                }
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("⬆").color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text("Increase speed")
                    .clicked()
                {
                    action_state.press(SimulationSpeedAction::Accelerate);
                }
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("⏭").color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text("Fastest")
                    .clicked()
                {
                    action_state.press(SimulationSpeedAction::Fastest);
                }
                ui.add_space(ui.available_width() - 60.0);
                let mut fps_s = "N/A".to_string();
                if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                    if let Some(average) = fps.average() {
                        fps_s = format!("{average:.1}");
                    }
                }
                ui.label(RichText::new(format!("fps: {fps_s}")).font(FontId::proportional(10.0)));
            });
        });
}
pub fn ui_simulation(
    mut contexts: EguiContexts,
    ecosystem_config: ResMut<EcosystemConfig>,
    ecosystem_statistics: Res<EcosystemStatistics>,
    mut ui_state: ResMut<UiState>,
) {
    egui::popup::show_tooltip_at(
        contexts.ctx_mut(),
        Id::new("Simulation"),
        Some(Pos2 { x: 10.0, y: 10.0 }),
        |ui| {
            ui.set_width(400.0);
            if ui.label("[1] Simulation").clicked() {
                ui_state.ui_simulation_open = !ui_state.ui_simulation_open;
            }
            if ui_state.ui_simulation_open {
                let mut plot_lines = vec![];
                for (name, stats) in ecosystem_statistics.organisms.iter() {
                    if let Some(stat) = stats.last_stored() {
                        let hue = ecosystem_config.organisms_per_name[name].visualization.hue;
                        let [r, g, b, _] = Color::hsl(hue, 1.0, 0.7).as_rgba_f32();
                        let color = Color32::from_rgb(
                            (r * 256.0) as u8,
                            (g * 256.0) as u8,
                            (b * 256.0) as u8,
                        );
                        ui.label(RichText::new(format!("{name} {}", stat.size)).color(color));
                        plot_lines.push(
                            egui::plot::Line::new(
                                stats
                                    .accumulation
                                    .iter()
                                    .map(|(step, stat)| [*step as f64, stat.size as f64])
                                    .collect::<Vec<_>>(),
                            )
                            .color(color),
                        );
                    }
                }
                let plot = egui::plot::Plot::new("Population")
                    .height(80.0)
                    .show_x(false)
                    .show_y(true)
                    .center_x_axis(false)
                    .center_y_axis(false)
                    .allow_zoom(false)
                    .allow_drag(false);
                plot.show(ui, |plot_ui| {
                    for plot_line in plot_lines {
                        plot_ui.line(plot_line);
                    }
                });
            }
        },
    );
}
