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

use bevy_egui::egui;

#[derive(Default)]
pub struct StudioTheme {
    pub show_fps: bool,
    pub show_diagnostics: bool,
}

pub fn apply_studio_theme(ctx: &egui::Context, theme: &StudioTheme) {
    let mut style = (*ctx.style()).clone();

    // Dark theme optimized for game development
    style.visuals = egui::Visuals::dark();
    style.visuals.window_rounding = egui::Rounding::same(4.0);
    style.visuals.menu_rounding = egui::Rounding::same(4.0);
    style.visuals.button_frame = true;

    // Custom colors
    style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(30, 30, 40);
    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(40, 40, 50);
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(50, 50, 70);
    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(60, 60, 90);

    style.visuals.selection.bg_fill = egui::Color32::from_rgb(100, 100, 200);
    style.visuals.hyperlink_color = egui::Color32::from_rgb(90, 170, 255);

    // Spacing
    style.spacing.item_spacing = egui::vec2(8.0, 4.0);
    style.spacing.button_padding = egui::vec2(8.0, 4.0);
    style.spacing.indent = 20.0;

    ctx.set_style(style);
}
