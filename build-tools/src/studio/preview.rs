// AI Game Generator - Procedural game generation using AI
// Copyright (C) 2024 AI Game Generator Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the MIT License as published by
// the Open Source Initiative.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

use bevy::prelude::*;
use bevy_egui::egui;

pub fn show_live_preview(ui: &mut egui::Ui) {
    ui.heading("▶️ Live Preview");
    ui.separator();

    ui.horizontal(|ui| {
        if ui.button("▶️ Play").clicked() {
            // Start game
        }
        if ui.button("⏸️ Pause").clicked() {
            // Pause game
        }
        if ui.button("🔄 Restart").clicked() {
            // Restart game
        }

        ui.separator();

        ui.label("Scale:");
        static mut SCALE: f32 = 2.0;
        unsafe {
            ui.add(egui::Slider::new(&mut SCALE, 1.0..=4.0));
        }

        ui.separator();

        if ui.button("🎮 Test Controls").clicked() {
            // Show control tester
        }
    });

    ui.separator();

    // Game render area
    ui.group(|ui| {
        ui.set_min_height(400.0);
        ui.centered_and_justified(|ui| {
            ui.heading("🎮 Game Preview");
            ui.label("The generated game will render here");
            ui.label("with live hot-reload support");

            ui.add_space(20.0);

            // Placeholder game view
            let available_size = ui.available_size();
            let game_size = egui::vec2(320.0, 240.0);
            let scale = unsafe { SCALE };
            let scaled_size = game_size * scale;

            if scaled_size.x <= available_size.x && scaled_size.y <= available_size.y {
                let (response, painter) = ui.allocate_painter(scaled_size, egui::Sense::click());

                // Draw placeholder game screen
                painter.rect_filled(
                    response.rect,
                    egui::Rounding::same(4.0),
                    egui::Color32::from_rgb(20, 20, 30),
                );

                // Draw grid pattern
                let grid_size = 16.0 * scale;
                let rect = response.rect;

                for x in 0..((rect.width() / grid_size) as i32 + 1) {
                    let x_pos = rect.left() + x as f32 * grid_size;
                    painter.line_segment(
                        [egui::pos2(x_pos, rect.top()), egui::pos2(x_pos, rect.bottom())],
                        egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20)),
                    );
                }

                for y in 0..((rect.height() / grid_size) as i32 + 1) {
                    let y_pos = rect.top() + y as f32 * grid_size;
                    painter.line_segment(
                        [egui::pos2(rect.left(), y_pos), egui::pos2(rect.right(), y_pos)],
                        egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20)),
                    );
                }

                // Draw placeholder text
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "Game Preview Area",
                    egui::FontId::proportional(20.0),
                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 100),
                );
            }
        });
    });
}

pub fn show_inspector(ui: &mut egui::Ui) {
    ui.heading("🔍 Inspector");
    ui.separator();

    // Entity selection
    ui.horizontal(|ui| {
        ui.label("Selected:");
        egui::ComboBox::from_label("")
            .selected_text("Player")
            .show_ui(ui, |ui| {
                ui.selectable_label(true, "Player");
                ui.selectable_label(false, "Enemy_1");
                ui.selectable_label(false, "NPC_Merchant");
                ui.selectable_label(false, "Tilemap");
            });
    });

    ui.separator();

    // Component inspector
    egui::ScrollArea::vertical().show(ui, |ui| {
        // Transform
        ui.collapsing("Transform", |ui| {
            ui.horizontal(|ui| {
                ui.label("Position:");
                ui.label("X:");
                ui.add(egui::DragValue::new(&mut 100.0).speed(1.0));
                ui.label("Y:");
                ui.add(egui::DragValue::new(&mut 200.0).speed(1.0));
            });

            ui.horizontal(|ui| {
                ui.label("Rotation:");
                ui.add(egui::DragValue::new(&mut 0.0).speed(1.0).suffix("°"));
            });

            ui.horizontal(|ui| {
                ui.label("Scale:");
                ui.add(egui::DragValue::new(&mut 1.0).speed(0.01).clamp_range(0.1..=10.0));
            });
        });

        // Sprite
        ui.collapsing("Sprite", |ui| {
            ui.label("Texture: player_idle.png");
            ui.horizontal(|ui| {
                ui.label("Color:");
                let mut color = [1.0, 1.0, 1.0, 1.0];
                ui.color_edit_button_rgba_unmultiplied(&mut color);
            });
            ui.checkbox(&mut true, "Visible");
        });

        // Custom Components
        ui.collapsing("Player Component", |ui| {
            ui.horizontal(|ui| {
                ui.label("Health:");
                ui.add(egui::ProgressBar::new(0.8).text("80/100"));
            });

            ui.horizontal(|ui| {
                ui.label("Speed:");
                ui.add(egui::DragValue::new(&mut 150.0).speed(1.0));
            });

            ui.horizontal(|ui| {
                ui.label("State:");
                ui.label("Idle");
            });
        });

        // Add Component button
        ui.add_space(10.0);
        if ui.button("➕ Add Component").clicked() {
            // Show component picker
        }
    });
}
