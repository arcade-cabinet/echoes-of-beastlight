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

#[derive(Resource, Default)]
pub struct EditorState {
    pub current_file: Option<String>,
    pub file_content: String,
    pub modified: bool,
    pub syntax_highlight: bool,
}

pub fn show_code_editor(ui: &mut egui::Ui, editor_state: &mut EditorState) {
    ui.heading("📝 Code Editor");
    ui.separator();

    // File tabs
    ui.horizontal(|ui| {
        if ui
            .selectable_label(
                editor_state.current_file == Some("main.rs".into()),
                "main.rs",
            )
            .clicked()
        {
            editor_state.current_file = Some("main.rs".into());
            editor_state.file_content = get_sample_code("main.rs");
        }
        if ui
            .selectable_label(
                editor_state.current_file == Some("player.rs".into()),
                "player.rs",
            )
            .clicked()
        {
            editor_state.current_file = Some("player.rs".into());
            editor_state.file_content = get_sample_code("player.rs");
        }
        if ui
            .selectable_label(
                editor_state.current_file == Some("combat.rs".into()),
                "combat.rs",
            )
            .clicked()
        {
            editor_state.current_file = Some("combat.rs".into());
            editor_state.file_content = get_sample_code("combat.rs");
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.checkbox(&mut editor_state.syntax_highlight, "Syntax Highlight");
        });
    });

    ui.separator();

    // Editor toolbar
    ui.horizontal(|ui| {
        if ui.button("💾 Save").clicked() {
            // Save file
            editor_state.modified = false;
        }
        if ui.button("↩️ Undo").clicked() {
            // Undo
        }
        if ui.button("↪️ Redo").clicked() {
            // Redo
        }
        ui.separator();
        if ui.button("🔍 Find").clicked() {
            // Open find dialog
        }
        if ui.button("🔄 Replace").clicked() {
            // Open replace dialog
        }

        if editor_state.modified {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.colored_label(egui::Color32::YELLOW, "● Modified");
            });
        }
    });

    ui.separator();

    // Code editor area
    egui::ScrollArea::both()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            if editor_state.syntax_highlight {
                // Simple syntax highlighting
                show_highlighted_code(ui, &editor_state.file_content);
            } else {
                let response = ui.add(
                    egui::TextEdit::multiline(&mut editor_state.file_content)
                        .code_editor()
                        .desired_width(f32::INFINITY)
                        .desired_rows(30),
                );

                if response.changed() {
                    editor_state.modified = true;
                }
            }
        });
}

fn show_highlighted_code(ui: &mut egui::Ui, code: &str) {
    use egui::{text::LayoutJob, Color32, FontId, TextFormat};

    let mut job = LayoutJob::default();

    // Simple Rust syntax highlighting
    let keywords = vec![
        "use", "mod", "fn", "let", "mut", "const", "struct", "impl", "pub", "if", "else", "for",
        "while", "loop", "match", "return", "self", "Self", "true", "false",
    ];

    let lines = code.lines();
    for line in lines {
        let mut remaining = line;
        let indent = line.len() - line.trim_start().len();

        // Add indentation
        if indent > 0 {
            job.append(&" ".repeat(indent), 0.0, TextFormat::default());
        }

        remaining = remaining.trim_start();

        // Check for comments
        if remaining.starts_with("//") {
            job.append(
                remaining,
                0.0,
                TextFormat {
                    color: Color32::from_rgb(100, 200, 100),
                    ..Default::default()
                },
            );
        } else {
            // Tokenize and highlight
            let words: Vec<&str> = remaining.split_whitespace().collect();
            for (i, word) in words.iter().enumerate() {
                let color =
                    if keywords.contains(&word.trim_end_matches(|c: char| !c.is_alphanumeric())) {
                        Color32::from_rgb(200, 100, 200) // Keywords in purple
                    } else if word.starts_with('"') || word.starts_with('\'') {
                        Color32::from_rgb(200, 150, 100) // Strings in orange
                    } else if word.chars().all(|c| c.is_numeric() || c == '.') {
                        Color32::from_rgb(100, 200, 200) // Numbers in cyan
                    } else {
                        Color32::from_gray(200) // Default text
                    };

                job.append(
                    word,
                    0.0,
                    TextFormat {
                        color,
                        ..Default::default()
                    },
                );

                if i < words.len() - 1 {
                    job.append(" ", 0.0, TextFormat::default());
                }
            }
        }

        job.append("\n", 0.0, TextFormat::default());
    }

    ui.label(job);
}

fn get_sample_code(filename: &str) -> String {
    match filename {
        "main.rs" => r#"use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    // Spawn player
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player { speed: 150.0 },
    ));
}"#
        .to_string(),

        "player.rs" => r#"use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

pub fn player_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in &mut query {
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            transform.translation += direction * 150.0 * time.delta_seconds();
        }
    }
}"#
        .to_string(),

        _ => "// Generated code will appear here".to_string(),
    }
}
