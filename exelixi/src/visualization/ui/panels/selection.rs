use super::*;
use crate::prelude::*;

use bevy_egui::egui::{Align2, CollapsingHeader};

pub fn ui_selection(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    selection: Query<(Entity, &Organism, &Body, Option<&Eye>), With<Selected>>,
    ecosystem_config: ResMut<EcosystemConfig>,
) {
    egui::containers::Window::new("Selection")
        .anchor(Align2::RIGHT_TOP, (-10.0, 10.0))
        .resizable(false)
        .title_bar(false)
        .show(contexts.ctx_mut(), |ui| {
            if create_header(ui, "[2] Selection").clicked() {
                ui_state.selection_open = !ui_state.selection_open;
            }
            if ui_state.selection_open {
                if let Ok((entity, organism, body, eye)) = selection.get_single() {
                    let species_color =
                        ecosystem_config.get_egui_color(&organism.species(), 1.0, 0.7);
                    let species_name = ecosystem_config.get_species_name(&organism.species());
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(species_name).color(species_color));
                        ui.label(
                            RichText::new(format!("[{:?}]", entity))
                                .small()
                                .color(species_color),
                        );
                    });
                    CollapsingHeader::new("Body")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("Energy: {}", body.energy()));
                                ui.small(format!("[{:2.0}%]", body.energy_pct() * 100.0));
                            });
                        });
                    if let Some(eye) = eye {
                        CollapsingHeader::new("Eye")
                            .default_open(true)
                            .show(ui, |ui| {
                                ui.label(format!(
                                    "Fov: {:.0} x {:.1} rad",
                                    eye.fov_range, eye.fov_angle
                                ));
                                CollapsingHeader::new("Sensors:").default_open(false).show(
                                    ui,
                                    |ui| {
                                        egui::Grid::new("Eye sensors").spacing((0.0, 0.0)).show(
                                            ui,
                                            |ui| {
                                                let eye_sensors = eye.get_sensors();
                                                for cx in 0..eye.cell_sensors.n_sensors() {
                                                    for x in 0..eye.n_cells {
                                                        let value = eye_sensors
                                                            [x * eye.cell_sensors.n_sensors() + cx];
                                                        ui.label(get_text_for_neuron(value));
                                                    }
                                                    ui.end_row()
                                                }
                                            },
                                        )
                                    },
                                );
                            });
                    }
                } else {
                    ui.label(RichText::new("<Select an organism>").color(HONEY));
                }
            }
        });
}
