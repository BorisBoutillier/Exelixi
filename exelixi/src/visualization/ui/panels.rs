use crate::prelude::*;

// All panels must be declared in the same system as the order of panel creation is important
pub fn panels_ui(
    mut contexts: EguiContexts,
    simulation: Res<Simulation>,
    mut ui_state: ResMut<UiState>,
    mut action_state: ResMut<ActionState<SimulationSpeedAction>>,
) {
    //
    // Bottom status bar
    //
    // Always present and first to span the whole bottom

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
                ui.heading(format!("Generation: {:4}", simulation.generation));
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
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("S").color(egui::Color32::from_rgb(211, 211, 211)),
                    ))
                    .on_hover_text("Stat panel")
                    .clicked()
                {
                    ui_state.stat_panel = !ui_state.stat_panel;
                }
            });
        });
    //
    // Left side panel for Statistics
    //
    if ui_state.stat_panel {
        egui::SidePanel::left("left_panel")
            .frame(
                egui::Frame::default()
                    .fill(egui::Color32::from_rgb(30, 30, 30))
                    .inner_margin(egui::Vec2::new(10.0, 10.0)),
            )
            .resizable(false)
            .min_width(UI_LEFT_PANEL_WIDTH)
            .max_width(UI_LEFT_PANEL_WIDTH)
            .show(contexts.ctx_mut(), |ui| {
                egui::CollapsingHeader::new("Simulation")
                    .default_open(true)
                    .show(ui, |ui| {
                        let size_color = egui::Color32::from_rgb(100, 100, 255);
                        let dead_color = egui::Color32::from_rgb(230, 25, 25);
                        let avg_color = egui::Color32::from_rgb(25, 180, 25);
                        let population_size_line = egui::plot::Line::new(
                            simulation
                                .statistics
                                .generations
                                .iter()
                                .enumerate()
                                .map(|(i, s)| [i as f64, s.start_size as f64])
                                .collect::<Vec<_>>(),
                        )
                        .color(size_color);
                        let population_dead_line = egui::plot::Line::new(
                            simulation
                                .statistics
                                .generations
                                .iter()
                                .enumerate()
                                .map(|(i, s)| [i as f64, (s.start_size - s.end_size) as f64])
                                .collect::<Vec<_>>(),
                        )
                        .color(dead_color);
                        let plot_bottom = egui::plot::Line::new(
                            simulation
                                .statistics
                                .generations
                                .iter()
                                .enumerate()
                                .map(|(i, s)| [i as f64, s.food_decay as f64])
                                .collect::<Vec<_>>(),
                        )
                        .color(avg_color);
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(format!(
                                    "Population: {:4}",
                                    simulation.statistics.latest_start_size()
                                ))
                                .color(size_color),
                            )
                            .on_hover_text("Population at the start");
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new(format!(
                                    "Deaths: {:4}",
                                    simulation.statistics.latest_dead()
                                ))
                                .color(dead_color),
                            )
                            .on_hover_text("Deaths at end");
                        });
                        let plot = egui::plot::Plot::new("population_plot")
                            .height(80.0)
                            .show_x(false)
                            .show_y(true)
                            .center_x_axis(false)
                            .center_y_axis(false)
                            .allow_zoom(false)
                            .allow_drag(false);
                        plot.show(ui, |plot_ui| {
                            plot_ui.line(population_dead_line);
                            plot_ui.line(population_size_line);
                        });
                        ui.label(
                            egui::RichText::new(format!(
                                "Uneaten food: {:5}",
                                simulation.statistics.latest_food_decay()
                            ))
                            .color(avg_color),
                        )
                        .on_hover_text("Number food that have decayed or have been eaten at end");
                        let plot = egui::plot::Plot::new("Food decay plot")
                            .height(80.0)
                            .show_x(false)
                            .show_y(true)
                            .center_x_axis(false)
                            .center_y_axis(false)
                            .allow_zoom(false)
                            .allow_drag(false);
                        plot.show(ui, |plot_ui| {
                            plot_ui.line(plot_bottom);
                        });
                    });
                egui::CollapsingHeader::new("Population genetic")
                    .default_open(true)
                    .show(ui, |ui| {
                        if !simulation.statistics.population.fov_angle.is_empty() {
                            let chart = egui::plot::BarChart::new(
                                simulation
                                    .statistics
                                    .population
                                    .fov_angle
                                    .iter()
                                    .map(|(r, c)| {
                                        egui::plot::Bar::new((r.start as f64) / 100.0, *c as f64)
                                            .width((r.end - r.start) as f64 / 200.0)
                                    })
                                    .collect(),
                            )
                            .color(egui::Color32::from_rgb(100, 100, 255))
                            .name("FOV _angle");
                            egui::plot::Plot::new("FOV angle")
                                .height(80.0)
                                .legend(egui::plot::Legend::default())
                                .show(ui, |plot_ui| plot_ui.bar_chart(chart));
                        }
                        if !simulation.statistics.population.fov_range.is_empty() {
                            let chart = egui::plot::BarChart::new(
                                simulation
                                    .statistics
                                    .population
                                    .fov_range
                                    .iter()
                                    .map(|(r, c)| {
                                        egui::plot::Bar::new(r.start as f64, *c as f64)
                                            .width((r.end - r.start) as f64 / 2.0)
                                    })
                                    .collect(),
                            )
                            .color(egui::Color32::from_rgb(50, 50, 255))
                            .name("FOV range");
                            egui::plot::Plot::new("FOV range")
                                .height(80.0)
                                .legend(egui::plot::Legend::default())
                                .show(ui, |plot_ui| plot_ui.bar_chart(chart));
                        }
                    });
            });
    }
}
