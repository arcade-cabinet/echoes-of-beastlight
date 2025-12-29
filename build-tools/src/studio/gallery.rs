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

use crate::studio::{AssetCache, GenerationRequest, GenerationSender};
use bevy::prelude::*;
use bevy_egui::egui;
use std::collections::HashMap;

pub fn show_asset_gallery(
    ui: &mut egui::Ui,
    asset_cache: &AssetCache,
    generation_tx: &GenerationSender,
) {
    ui.heading("🎨 Asset Gallery");
    ui.separator();

    // Filter controls
    ui.horizontal(|ui| {
        ui.label("Filter:");
        static mut FILTER: String = String::new();
        unsafe {
            ui.text_edit_singleline(&mut FILTER);
        }

        ui.separator();

        ui.label("Type:");
        if ui.selectable_label(true, "All").clicked() {}
        if ui.selectable_label(false, "Sprites").clicked() {}
        if ui.selectable_label(false, "Tiles").clicked() {}
        if ui.selectable_label(false, "UI").clicked() {}
        if ui.selectable_label(false, "Audio").clicked() {}
    });

    ui.separator();

    // Asset grid
    egui::ScrollArea::both().show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            // Display cached assets
            for entry in asset_cache.cached_assets.iter() {
                let (id, asset) = entry.pair();

                ui.group(|ui| {
                    ui.set_min_size(egui::vec2(128.0, 160.0));
                    ui.vertical_centered(|ui| {
                        // Asset thumbnail
                        if let Some(thumbnail) = &asset.thumbnail {
                            // Show actual thumbnail
                            ui.colored_label(egui::Color32::GRAY, "🖼️");
                        } else {
                            // Placeholder
                            ui.colored_label(egui::Color32::GRAY, "Asset Thumbnail");
                        }

                        ui.label(&asset.id);
                        ui.small(format!("Type: {}", asset.asset_type));

                        ui.horizontal(|ui| {
                            if ui.small_button("✏️").on_hover_text("Edit").clicked() {
                                // Open editor
                            }
                            if ui.small_button("🔄").on_hover_text("Regenerate").clicked() {
                                // Send regeneration request
                                let request = GenerationRequest::RegenerateAsset {
                                    asset_id: id.clone(),
                                    modifications: HashMap::new(),
                                };
                                let _ = generation_tx.0.send(request);
                            }
                            if ui.small_button("📋").on_hover_text("Copy ID").clicked() {
                                ui.output_mut(|o| o.copied_text = id.clone());
                            }
                        });
                    });
                });
            }

            // Add new asset button
            ui.group(|ui| {
                ui.set_min_size(egui::vec2(128.0, 160.0));
                ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    if ui.button("➕ Generate New").clicked() {
                        // Open generation dialog
                    }
                });
            });
        });
    });
}
