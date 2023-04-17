use super::*;
use crate::prelude::*;

use bevy_egui::egui::{CollapsingHeader, Color32, RichText};

const HONEY: Color32 = Color32::from_rgb(0xEC, 0x97, 0x06);
pub fn ui_simulation(
    mut contexts: EguiContexts,
    ecosystem_config: ResMut<EcosystemConfig>,
    ecosystem_statistics: Res<EcosystemStatistics>,
    mut ui_state: ResMut<UiState>,
) {
    // egui::popup::show_tooltip_at(
    //     contexts.ctx_mut(),
    //     Id::new("Simulation"),
    //     Some(Pos2 { x: 10.0, y: 10.0 }),
    egui::containers::Window::new("Simulation")
        .fixed_pos((10.0, 10.0))
        .resizable(false)
        .title_bar(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.set_width(400.0);
            if create_header(ui, "[1] Simulation").clicked() {
                ui_state.simulation_open = !ui_state.simulation_open;
            }
            if ui_state.simulation_open {
                CollapsingHeader::new("Population")
                    .default_open(true)
                    .show(ui, |ui| {
                        let mut plot_lines = vec![];
                        for (species, stats) in ecosystem_statistics.organisms.iter() {
                            if let Some(stat) = stats.last_stored() {
                                let color = ecosystem_config.get_egui_color(species, 1.0, 0.7);
                                let checked = ui_state
                                    .simulation_energy_checked
                                    .entry(*species)
                                    .or_insert(true);
                                ui.checkbox(
                                    checked,
                                    RichText::new(format!("{} {}", stats.name, stat.size))
                                        .color(color),
                                );
                                if *checked {
                                    plot_lines.push(
                                        egui::plot::Line::new(
                                            stats
                                                .accumulation
                                                .iter()
                                                .map(|(step, stat)| {
                                                    [*step as f64, stat.size as f64]
                                                })
                                                .collect::<Vec<_>>(),
                                        )
                                        .color(color),
                                    );
                                }
                            }
                        }
                        if plot_lines.is_empty() {
                            ui.label(RichText::new("No data yet").color(HONEY));
                        } else {
                            let plot = egui::plot::Plot::new("Population").height(80.0);
                            plot.show(ui, |plot_ui| {
                                for plot_line in plot_lines {
                                    plot_ui.line(plot_line);
                                }
                            });
                        }
                    });
                CollapsingHeader::new("Energy avg")
                    .default_open(true)
                    .show(ui, |ui| {
                        let mut plot_lines = vec![];
                        for (species, stats) in ecosystem_statistics.organisms.iter() {
                            if let Some(stat) = stats.last_stored() {
                                let color = ecosystem_config.get_egui_color(species, 1.0, 0.7);
                                let checked = ui_state
                                    .simulation_population_checked
                                    .entry(*species)
                                    .or_insert(true);
                                ui.checkbox(
                                    checked,
                                    RichText::new(format!("{} {:.0}", stats.name, stat.energy_avg))
                                        .color(color),
                                );
                                if *checked {
                                    plot_lines.push(
                                        egui::plot::Line::new(
                                            stats
                                                .accumulation
                                                .iter()
                                                .map(|(step, stat)| {
                                                    [*step as f64, stat.energy_avg as f64]
                                                })
                                                .collect::<Vec<_>>(),
                                        )
                                        .color(color),
                                    );
                                }
                            }
                        }
                        if plot_lines.is_empty() {
                            ui.label(RichText::new("No data yet").color(HONEY));
                        } else {
                            let plot = egui::plot::Plot::new("Energy avg").height(80.0);
                            plot.show(ui, |plot_ui| {
                                for plot_line in plot_lines {
                                    plot_ui.line(plot_line);
                                }
                            });
                        }
                    });
                CollapsingHeader::new("Generation")
                    .default_open(true)
                    .show(ui, |ui| {
                        for (species, stats) in ecosystem_statistics.organisms.iter() {
                            if let Some(stat) = stats.last_stored() {
                                let color = ecosystem_config.get_egui_color(species, 1.0, 0.7);
                                let value = if let Some(generation) = stat.generation {
                                    generation.to_string()
                                } else {
                                    "n/a".to_string()
                                };
                                ui.label(
                                    RichText::new(format!("{} {}", stats.name, value)).color(color),
                                );
                            }
                        }
                    });
            }
        });
}
