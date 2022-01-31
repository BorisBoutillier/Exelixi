use crate::prelude::*;

// All panels must be declared in the same system as the order of panel creation is important
pub fn panels_ui(
    egui_ctx: Res<EguiContext>,
    mut simulation: ResMut<Simulation>,
    mut ui_state: ResMut<UiState>,
) {
    //
    // Bottom status bar
    //
    // Always present and first to span the whole bottom

    egui::TopBottomPanel::bottom("status_bar")
        .frame(egui::Frame::default().fill(egui::Color32::from_rgb(30, 30, 30)))
        .resizable(false)
        .default_height(UI_STATUS_BAR_HEIGHT)
        .show(egui_ctx.ctx(), |ui| {
            let half_width = ui.available_width() / 2.0;
            let mut spacing = ui.spacing_mut();
            spacing.button_padding = egui::Vec2::new(2.0, 2.0);
            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                ui.add_space(30.0);
                ui.heading(format!("Generation: {:4}", simulation.generation));
                ui.add_space(ui.available_width() - half_width - 30.0 * 3.0);
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("⏹").color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text("Stop")
                    .clicked()
                {
                    simulation.control.speed_factor = 1;
                    simulation.control.state = SimulationControlState::Paused;
                }
                let (text, hover_text, control) =
                    if simulation.control.state == SimulationControlState::Paused {
                        ("▶", "Run", SimulationControlState::Run)
                    } else {
                        ("⏸", "Pause", SimulationControlState::Paused)
                    };
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new(text).color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text(hover_text)
                    .clicked()
                {
                    simulation.control.state = control;
                }
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("⬇").color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text("Decrease speed")
                    .clicked()
                {
                    simulation.control.speed_factor = (simulation.control.speed_factor / 2).max(1);
                }
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("⬆").color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text("Increase speed")
                    .clicked()
                {
                    simulation.control.speed_factor *= 2;
                }
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("⏭").color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text("Fastest")
                    .clicked()
                {
                    simulation.control.state = SimulationControlState::Fastest;
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
        let size_color = egui::Color32::from_rgb(100, 100, 255);
        let dead_color = egui::Color32::from_rgb(230, 25, 25);
        let avg_color = egui::Color32::from_rgb(25, 180, 25);
        let population_size_line = egui::plot::Line::new(egui::plot::Values::from_values_iter(
            simulation
                .statistics
                .generations
                .iter()
                .enumerate()
                .map(|(i, s)| egui::plot::Value::new(i as f64, s.start_size as f64)),
        ))
        .color(size_color);
        let population_dead_line = egui::plot::Line::new(egui::plot::Values::from_values_iter(
            simulation
                .statistics
                .generations
                .iter()
                .enumerate()
                .map(|(i, s)| egui::plot::Value::new(i as f64, (s.start_size - s.end_size) as f64)),
        ))
        .color(dead_color);
        let plot_bottom = egui::plot::Line::new(egui::plot::Values::from_values_iter(
            simulation
                .statistics
                .generations
                .iter()
                .enumerate()
                .map(|(i, s)| egui::plot::Value::new(i as f64, s.food_decay as f64)),
        ))
        .color(avg_color);
        egui::SidePanel::left("left_panel")
            .frame(
                egui::Frame::default()
                    .fill(egui::Color32::from_rgb(30, 30, 30))
                    .margin(egui::Vec2::new(10.0, 10.0)),
            )
            .resizable(false)
            .min_width(UI_LEFT_PANEL_WIDTH)
            .max_width(UI_LEFT_PANEL_WIDTH)
            .show(egui_ctx.ctx(), |ui| {
                egui::CollapsingHeader::new("Simulation")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(format!(
                                    "Population: {:4}",
                                    simulation.statistics.latest_start_size()
                                ))
                                .color(size_color),
                            )
                            .on_hover_text("Animal population at the start");
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new(format!(
                                    "Deaths: {:4}",
                                    simulation.statistics.latest_dead()
                                ))
                                .color(dead_color),
                            )
                            .on_hover_text("Animal deaths at end");
                        });
                        let plot = egui::plot::Plot::new("population_plot")
                            .height(50.0)
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
                            .height(50.0)
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
            });
    }
}