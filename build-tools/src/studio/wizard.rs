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

use crate::studio::{GenerationRequest, GenerationSender, StudioPhase};
use bevy::prelude::*;
use bevy_egui::egui;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Project setup wizard state
#[derive(Resource, Default)]
pub struct WizardState {
    pub current_step: WizardStep,
    pub game_config: GameConfiguration,
    pub validation_errors: Vec<String>,
}

impl WizardState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn validate_current_step(&mut self) -> bool {
        self.validation_errors.clear();

        match self.current_step {
            WizardStep::BasicInfo => {
                if self.game_config.name.is_empty() {
                    self.validation_errors.push("Game name is required".into());
                }
                if self.game_config.tagline.is_empty() {
                    self.validation_errors.push("Tagline is required".into());
                }
            }
            WizardStep::GameplayDesign => {
                if self.game_config.core_mechanics.is_empty() {
                    self.validation_errors
                        .push("Select at least one core mechanic".into());
                }
                if self.game_config.gameplay_loop.len() < 20 {
                    self.validation_errors
                        .push("Please provide a more detailed gameplay loop description".into());
                }
            }
            WizardStep::VisualStyle => {
                if self.game_config.art_references.is_empty() {
                    self.validation_errors
                        .push("Select at least one art reference".into());
                }
            }
            WizardStep::Features => {
                // Features are optional
            }
            WizardStep::TechnicalSettings => {
                if self.game_config.platforms.is_empty() {
                    self.validation_errors
                        .push("Select at least one target platform".into());
                }
            }
            WizardStep::Review => {
                // Final validation
                return self.validate_configuration();
            }
        }

        self.validation_errors.is_empty()
    }

    pub fn validate_configuration(&mut self) -> bool {
        self.validation_errors.clear();

        // Comprehensive validation
        if self.game_config.name.is_empty() {
            self.validation_errors.push("Game name is required".into());
        }

        if self.game_config.core_mechanics.is_empty() {
            self.validation_errors
                .push("At least one core mechanic is required".into());
        }

        if self.game_config.platforms.is_empty() {
            self.validation_errors
                .push("At least one platform must be selected".into());
        }

        self.validation_errors.is_empty()
    }

    pub fn start_generation(&self, tx: &GenerationSender) {
        let request = GenerationRequest::FullGame {
            config: self.game_config.clone(),
        };

        let _ = tx.0.send(request);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WizardStep {
    #[default]
    BasicInfo,
    GameplayDesign,
    VisualStyle,
    Features,
    TechnicalSettings,
    Review,
}

impl WizardStep {
    pub fn next(&self) -> Self {
        match self {
            WizardStep::BasicInfo => WizardStep::GameplayDesign,
            WizardStep::GameplayDesign => WizardStep::VisualStyle,
            WizardStep::VisualStyle => WizardStep::Features,
            WizardStep::Features => WizardStep::TechnicalSettings,
            WizardStep::TechnicalSettings => WizardStep::Review,
            WizardStep::Review => WizardStep::Review,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            WizardStep::BasicInfo => WizardStep::BasicInfo,
            WizardStep::GameplayDesign => WizardStep::BasicInfo,
            WizardStep::VisualStyle => WizardStep::GameplayDesign,
            WizardStep::Features => WizardStep::VisualStyle,
            WizardStep::TechnicalSettings => WizardStep::Features,
            WizardStep::Review => WizardStep::TechnicalSettings,
        }
    }
}

/// Complete game configuration built from wizard
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameConfiguration {
    // Basic Info
    pub name: String,
    pub genre: GameGenre,
    pub tagline: String,
    pub target_audience: TargetAudience,

    // Gameplay Design
    pub core_mechanics: Vec<CoreMechanic>,
    pub gameplay_loop: String,
    pub progression_system: ProgressionType,
    pub difficulty_curve: DifficultyCurve,

    // Visual Style
    pub art_references: Vec<String>,
    pub color_mood: ColorMood,
    pub sprite_style: SpriteStyle,
    pub animation_complexity: AnimationComplexity,

    // Features
    pub features: IndexMap<GameFeature, FeatureConfig>,
    pub platforms: Vec<Platform>,

    // Technical
    pub map_size: MapSize,
    pub performance_target: PerformanceTarget,
    pub multiplayer_support: MultiplayerConfig,
}

pub fn show_project_wizard(
    ui: &mut egui::Ui,
    wizard_state: &mut WizardState,
    current_phase: &StudioPhase,
    next_phase: &mut NextState<StudioPhase>,
    generation_tx: &GenerationSender,
) {
    ui.heading("🎮 Game Project Setup Wizard");
    ui.separator();

    // Progress indicator
    show_progress_indicator(ui, wizard_state.current_step);
    ui.separator();

    // Current step content
    egui::ScrollArea::vertical().show(ui, |ui| match wizard_state.current_step {
        WizardStep::BasicInfo => show_basic_info_step(ui, wizard_state),
        WizardStep::GameplayDesign => show_gameplay_design_step(ui, wizard_state),
        WizardStep::VisualStyle => show_visual_style_step(ui, wizard_state),
        WizardStep::Features => show_features_step(ui, wizard_state),
        WizardStep::TechnicalSettings => show_technical_settings_step(ui, wizard_state),
        WizardStep::Review => show_review_step(ui, wizard_state),
    });

    ui.separator();

    // Show validation errors
    if !wizard_state.validation_errors.is_empty() {
        ui.colored_label(egui::Color32::RED, "Please fix the following issues:");
        for error in &wizard_state.validation_errors {
            ui.label(format!("• {}", error));
        }
        ui.separator();
    }

    // Navigation buttons
    show_navigation_buttons(ui, wizard_state, next_phase, generation_tx);
}

fn show_progress_indicator(ui: &mut egui::Ui, current_step: WizardStep) {
    ui.horizontal(|ui| {
        let steps = [
            ("📝", WizardStep::BasicInfo, "Basic Info"),
            ("🎮", WizardStep::GameplayDesign, "Gameplay"),
            ("🎨", WizardStep::VisualStyle, "Visual Style"),
            ("⚙️", WizardStep::Features, "Features"),
            ("🔧", WizardStep::TechnicalSettings, "Technical"),
            ("✅", WizardStep::Review, "Review"),
        ];

        for (i, (icon, step, label)) in steps.iter().enumerate() {
            let is_current = *step == current_step;
            let is_complete = i < steps
                .iter()
                .position(|(_, s, _)| *s == current_step)
                .unwrap_or(0);

            ui.vertical(|ui| {
                ui.set_min_width(80.0);

                let response = if is_current {
                    ui.strong(format!("{} {}", icon, label))
                } else if is_complete {
                    ui.colored_label(egui::Color32::GREEN, format!("✓ {}", label))
                } else {
                    ui.weak(format!("{} {}", icon, label))
                };

                if is_current {
                    ui.add(
                        egui::widgets::ProgressBar::new(1.0)
                            .desired_height(2.0)
                            .fill(egui::Color32::from_rgb(0, 150, 255)),
                    );
                }
            });

            if i < steps.len() - 1 {
                ui.label("→");
            }
        }
    });
}

fn show_basic_info_step(ui: &mut egui::Ui, wizard_state: &mut WizardState) {
    ui.heading("Basic Information");
    ui.add_space(10.0);

    egui::Grid::new("basic_info_grid")
        .num_columns(2)
        .spacing([40.0, 10.0])
        .show(ui, |ui| {
            ui.label("Game Name:");
            ui.add(
                egui::TextEdit::singleline(&mut wizard_state.game_config.name)
                    .desired_width(300.0)
                    .hint_text("Enter your game's name"),
            );
            ui.end_row();

            ui.label("Genre:");
            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", wizard_state.game_config.genre))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut wizard_state.game_config.genre,
                        GameGenre::Action,
                        "Action",
                    );
                    ui.selectable_value(&mut wizard_state.game_config.genre, GameGenre::RPG, "RPG");
                    ui.selectable_value(
                        &mut wizard_state.game_config.genre,
                        GameGenre::Puzzle,
                        "Puzzle",
                    );
                    ui.selectable_value(
                        &mut wizard_state.game_config.genre,
                        GameGenre::Adventure,
                        "Adventure",
                    );
                    ui.selectable_value(
                        &mut wizard_state.game_config.genre,
                        GameGenre::Simulation,
                        "Simulation",
                    );
                    ui.selectable_value(
                        &mut wizard_state.game_config.genre,
                        GameGenre::Strategy,
                        "Strategy",
                    );
                });
            ui.end_row();

            ui.label("Tagline:");
            ui.add(
                egui::TextEdit::singleline(&mut wizard_state.game_config.tagline)
                    .desired_width(300.0)
                    .hint_text("A brief, catchy description"),
            );
            ui.end_row();

            ui.label("Target Audience:");
            ui.horizontal(|ui| {
                ui.radio_value(
                    &mut wizard_state.game_config.target_audience,
                    TargetAudience::Casual,
                    "Casual",
                );
                ui.radio_value(
                    &mut wizard_state.game_config.target_audience,
                    TargetAudience::Core,
                    "Core",
                );
                ui.radio_value(
                    &mut wizard_state.game_config.target_audience,
                    TargetAudience::Hardcore,
                    "Hardcore",
                );
            });
            ui.end_row();
        });

    ui.add_space(20.0);
    ui.collapsing("💡 Tips", |ui| {
        ui.label("• Choose a memorable name that reflects your game's theme");
        ui.label("• Your tagline should capture the essence of the gameplay");
        ui.label("• Consider your target audience when making design decisions");
    });
}

fn show_gameplay_design_step(ui: &mut egui::Ui, wizard_state: &mut WizardState) {
    ui.heading("Gameplay Design");
    ui.add_space(10.0);

    ui.label("Core Mechanics (select all that apply):");
    ui.indent("mechanics", |ui| {
        let mechanics = [
            (CoreMechanic::Combat, "⚔️ Combat", "Fighting enemies"),
            (
                CoreMechanic::Exploration,
                "🗺️ Exploration",
                "Discovering new areas",
            ),
            (
                CoreMechanic::Puzzle,
                "🧩 Puzzle Solving",
                "Mental challenges",
            ),
            (
                CoreMechanic::Collection,
                "💎 Collection",
                "Gathering items/resources",
            ),
            (
                CoreMechanic::Building,
                "🏗️ Building",
                "Construction mechanics",
            ),
            (CoreMechanic::Social, "👥 Social", "NPC interactions"),
            (CoreMechanic::Stealth, "🥷 Stealth", "Avoiding detection"),
            (CoreMechanic::Racing, "🏁 Racing", "Speed challenges"),
        ];

        ui.columns(2, |columns| {
            for (i, (mechanic, label, desc)) in mechanics.iter().enumerate() {
                let col = i / 4;
                columns[col].horizontal(|ui| {
                    let mut has_mechanic =
                        wizard_state.game_config.core_mechanics.contains(mechanic);
                    if ui.checkbox(&mut has_mechanic, label).changed() {
                        if has_mechanic {
                            wizard_state.game_config.core_mechanics.push(*mechanic);
                        } else {
                            wizard_state
                                .game_config
                                .core_mechanics
                                .retain(|m| m != mechanic);
                        }
                    }
                    ui.weak(desc);
                });
            }
        });
    });

    ui.add_space(10.0);
    ui.label("Gameplay Loop Description:");
    ui.add(
        egui::TextEdit::multiline(&mut wizard_state.game_config.gameplay_loop)
            .desired_width(f32::INFINITY)
            .desired_rows(4)
            .hint_text("Describe what players do repeatedly in your game..."),
    );

    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.label("Progression System:");
        egui::ComboBox::from_id_source("progression")
            .selected_text(format!("{:?}", wizard_state.game_config.progression_system))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut wizard_state.game_config.progression_system,
                    ProgressionType::Linear,
                    "Linear",
                );
                ui.selectable_value(
                    &mut wizard_state.game_config.progression_system,
                    ProgressionType::Branching,
                    "Branching",
                );
                ui.selectable_value(
                    &mut wizard_state.game_config.progression_system,
                    ProgressionType::Open,
                    "Open World",
                );
                ui.selectable_value(
                    &mut wizard_state.game_config.progression_system,
                    ProgressionType::Metroidvania,
                    "Metroidvania",
                );
            });
    });

    ui.add_space(10.0);
    ui.collapsing("Difficulty Settings", |ui| {
        ui.add(
            egui::Slider::new(
                &mut wizard_state
                    .game_config
                    .difficulty_curve
                    .starting_difficulty,
                0.0..=1.0,
            )
            .text("Starting Difficulty")
            .show_value(true),
        );
        ui.add(
            egui::Slider::new(
                &mut wizard_state.game_config.difficulty_curve.ramp_speed,
                0.0..=1.0,
            )
            .text("Difficulty Ramp Speed")
            .show_value(true),
        );
    });
}

fn show_visual_style_step(ui: &mut egui::Ui, wizard_state: &mut WizardState) {
    ui.heading("Visual Style");
    ui.add_space(10.0);

    ui.label("Reference Games (for inspiration):");
    ui.group(|ui| {
        let references = [
            ("Secret of Mana", "🌳"),
            ("Chrono Trigger", "⏰"),
            ("Final Fantasy VI", "⚔️"),
            ("Zelda: Link to the Past", "🗡️"),
            ("Super Metroid", "🚀"),
            ("Earthbound", "🌍"),
            ("Stardew Valley", "🌾"),
            ("Hollow Knight", "🦋"),
        ];

        ui.columns(2, |columns| {
            for (i, (reference, icon)) in references.iter().enumerate() {
                let col = i / 4;
                let mut selected = wizard_state
                    .game_config
                    .art_references
                    .contains(&reference.to_string());
                if columns[col]
                    .checkbox(&mut selected, format!("{} {}", icon, reference))
                    .changed()
                {
                    if selected {
                        wizard_state
                            .game_config
                            .art_references
                            .push(reference.to_string());
                    } else {
                        wizard_state
                            .game_config
                            .art_references
                            .retain(|r| r != reference);
                    }
                }
            }
        });
    });

    ui.add_space(10.0);
    ui.label("Color Mood:");
    ui.horizontal(|ui| {
        for mood in [
            ColorMood::Vibrant,
            ColorMood::Pastel,
            ColorMood::Dark,
            ColorMood::Earthy,
            ColorMood::Neon,
        ] {
            let color = match mood {
                ColorMood::Vibrant => egui::Color32::from_rgb(255, 100, 100),
                ColorMood::Pastel => egui::Color32::from_rgb(255, 200, 200),
                ColorMood::Dark => egui::Color32::from_rgb(50, 50, 50),
                ColorMood::Earthy => egui::Color32::from_rgb(150, 100, 50),
                ColorMood::Neon => egui::Color32::from_rgb(0, 255, 255),
            };

            if ui
                .add(
                    egui::RadioButton::new(
                        wizard_state.game_config.color_mood == mood,
                        format!("{:?}", mood),
                    )
                    .fill(color),
                )
                .clicked()
            {
                wizard_state.game_config.color_mood = mood;
            }
        }
    });

    ui.add_space(10.0);
    ui.group(|ui| {
        ui.label("Sprite Style:");

        ui.horizontal(|ui| {
            ui.label("Detail Level:");
            ui.radio_value(
                &mut wizard_state.game_config.sprite_style.detail_level,
                DetailLevel::Minimal,
                "Minimal",
            );
            ui.radio_value(
                &mut wizard_state.game_config.sprite_style.detail_level,
                DetailLevel::Moderate,
                "Moderate",
            );
            ui.radio_value(
                &mut wizard_state.game_config.sprite_style.detail_level,
                DetailLevel::Detailed,
                "Detailed",
            );
        });

        ui.checkbox(
            &mut wizard_state.game_config.sprite_style.use_outline,
            "Use black outline",
        );
        ui.checkbox(
            &mut wizard_state.game_config.sprite_style.pixel_perfect,
            "Pixel perfect rendering",
        );

        ui.horizontal(|ui| {
            ui.label("Animation Complexity:");
            ui.radio_value(
                &mut wizard_state.game_config.animation_complexity,
                AnimationComplexity::Simple,
                "Simple",
            );
            ui.radio_value(
                &mut wizard_state.game_config.animation_complexity,
                AnimationComplexity::Moderate,
                "Moderate",
            );
            ui.radio_value(
                &mut wizard_state.game_config.animation_complexity,
                AnimationComplexity::Complex,
                "Complex",
            );
        });
    });

    // Visual preview area
    ui.add_space(20.0);
    ui.group(|ui| {
        ui.set_min_height(200.0);
        ui.label("Style Preview");
        ui.centered_and_justified(|ui| {
            ui.colored_label(
                egui::Color32::GRAY,
                "Style preview will be generated after configuration...",
            );
        });
    });
}

fn show_features_step(ui: &mut egui::Ui, wizard_state: &mut WizardState) {
    ui.heading("Game Features");
    ui.add_space(10.0);

    let features = [
        (
            GameFeature::CombatSystem,
            "⚔️ Combat System",
            "Real-time or turn-based combat",
        ),
        (
            GameFeature::Inventory,
            "🎒 Inventory System",
            "Item management",
        ),
        (
            GameFeature::Dialogue,
            "💬 Dialogue System",
            "NPC conversations",
        ),
        (
            GameFeature::Crafting,
            "🔨 Crafting",
            "Create items from resources",
        ),
        (
            GameFeature::SaveLoad,
            "💾 Save/Load",
            "Game state persistence",
        ),
        (
            GameFeature::DayNight,
            "🌅 Day/Night Cycle",
            "Time progression",
        ),
        (
            GameFeature::Weather,
            "🌦️ Weather",
            "Dynamic weather effects",
        ),
        (
            GameFeature::Quests,
            "📜 Quest System",
            "Objectives and missions",
        ),
        (GameFeature::Minimap, "🗺️ Minimap", "Navigation aid"),
        (
            GameFeature::Achievements,
            "🏆 Achievements",
            "Player accomplishments",
        ),
    ];

    egui::ScrollArea::vertical()
        .max_height(400.0)
        .show(ui, |ui| {
            for (feature, name, description) in features {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        let mut enabled = wizard_state.game_config.features.contains_key(&feature);
                        if ui.checkbox(&mut enabled, name).changed() {
                            if enabled {
                                wizard_state
                                    .game_config
                                    .features
                                    .insert(feature, FeatureConfig::default_for_feature(feature));
                            } else {
                                wizard_state.game_config.features.remove(&feature);
                            }
                        }
                        ui.weak(description);
                    });

                    // Feature-specific configuration
                    if let Some(config) = wizard_state.game_config.features.get_mut(&feature) {
                        ui.indent("feature_config", |ui| match feature {
                            GameFeature::CombatSystem => {
                                ui.horizontal(|ui| {
                                    ui.label("Type:");
                                    ui.radio_value(
                                        &mut config.combat_type,
                                        CombatType::RealTime,
                                        "Real-time",
                                    );
                                    ui.radio_value(
                                        &mut config.combat_type,
                                        CombatType::TurnBased,
                                        "Turn-based",
                                    );
                                    ui.radio_value(
                                        &mut config.combat_type,
                                        CombatType::Tactical,
                                        "Tactical",
                                    );
                                });
                            }
                            GameFeature::Inventory => {
                                ui.horizontal(|ui| {
                                    ui.label("Size:");
                                    ui.add(
                                        egui::DragValue::new(&mut config.inventory_slots)
                                            .clamp_range(10..=100)
                                            .suffix(" slots"),
                                    );
                                });
                            }
                            GameFeature::Dialogue => {
                                ui.checkbox(
                                    &mut config.dialogue_choices,
                                    "Multiple choice responses",
                                );
                                ui.checkbox(
                                    &mut config.dialogue_branching,
                                    "Branching conversations",
                                );
                            }
                            GameFeature::SaveLoad => {
                                ui.horizontal(|ui| {
                                    ui.label("Save slots:");
                                    ui.add(
                                        egui::DragValue::new(&mut config.save_slots)
                                            .clamp_range(1..=10),
                                    );
                                });
                                ui.checkbox(&mut config.autosave, "Autosave");
                            }
                            _ => {}
                        });
                    }
                });
            }
        });
}

fn show_technical_settings_step(ui: &mut egui::Ui, wizard_state: &mut WizardState) {
    ui.heading("Technical Settings");
    ui.add_space(10.0);

    ui.group(|ui| {
        ui.label("World Size:");
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut wizard_state.game_config.map_size,
                MapSize::Small,
                "Small (10-20 screens)",
            );
            ui.selectable_value(
                &mut wizard_state.game_config.map_size,
                MapSize::Medium,
                "Medium (50-100 screens)",
            );
            ui.selectable_value(
                &mut wizard_state.game_config.map_size,
                MapSize::Large,
                "Large (100+ screens)",
            );
            ui.selectable_value(
                &mut wizard_state.game_config.map_size,
                MapSize::Massive,
                "Massive (500+ screens)",
            );
        });
    });

    ui.add_space(10.0);
    ui.group(|ui| {
        ui.label("Performance Target:");
        ui.vertical(|ui| {
            ui.radio_value(
                &mut wizard_state.game_config.performance_target,
                PerformanceTarget::Low,
                "Low-end devices (integrated graphics, mobile)",
            );
            ui.radio_value(
                &mut wizard_state.game_config.performance_target,
                PerformanceTarget::Medium,
                "Medium devices (entry gaming, modern laptops)",
            );
            ui.radio_value(
                &mut wizard_state.game_config.performance_target,
                PerformanceTarget::High,
                "High-end devices (dedicated GPU, gaming rigs)",
            );
        });
    });

    ui.add_space(10.0);
    ui.group(|ui| {
        ui.label("Target Platforms:");
        ui.horizontal_wrapped(|ui| {
            for platform in [
                Platform::Windows,
                Platform::Mac,
                Platform::Linux,
                Platform::Web,
                Platform::Steam,
                Platform::Itch,
            ] {
                let icon = match platform {
                    Platform::Windows => "🪟",
                    Platform::Mac => "🍎",
                    Platform::Linux => "🐧",
                    Platform::Web => "🌐",
                    Platform::Steam => "♨️",
                    Platform::Itch => "🎮",
                };

                let mut selected = wizard_state.game_config.platforms.contains(&platform);
                if ui
                    .checkbox(&mut selected, format!("{} {:?}", icon, platform))
                    .changed()
                {
                    if selected {
                        wizard_state.game_config.platforms.push(platform);
                    } else {
                        wizard_state
                            .game_config
                            .platforms
                            .retain(|p| p != &platform);
                    }
                }
            }
        });
    });

    ui.add_space(10.0);
    ui.collapsing("Advanced Settings", |ui| {
        ui.checkbox(
            &mut wizard_state.game_config.multiplayer_support.enabled,
            "Multiplayer Support",
        );
        if wizard_state.game_config.multiplayer_support.enabled {
            ui.indent("mp_settings", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Max Players:");
                    ui.add(
                        egui::DragValue::new(
                            &mut wizard_state.game_config.multiplayer_support.max_players,
                        )
                        .clamp_range(2..=32),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Network Type:");
                    ui.radio_value(
                        &mut wizard_state.game_config.multiplayer_support.network_type,
                        NetworkType::Local,
                        "Local",
                    );
                    ui.radio_value(
                        &mut wizard_state.game_config.multiplayer_support.network_type,
                        NetworkType::Online,
                        "Online",
                    );
                });
            });
        }

        ui.separator();

        ui.checkbox(&mut wizard_state.game_config.mod_support, "Mod Support");
        ui.checkbox(
            &mut wizard_state.game_config.controller_support,
            "Controller Support",
        );
    });
}

fn show_review_step(ui: &mut egui::Ui, wizard_state: &mut WizardState) {
    ui.heading("Review Configuration");
    ui.add_space(10.0);

    // Show configuration summary
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.group(|ui| {
            ui.heading("📝 Basic Info");
            ui.label(format!("Name: {}", wizard_state.game_config.name));
            ui.label(format!("Genre: {:?}", wizard_state.game_config.genre));
            ui.label(format!("Tagline: {}", wizard_state.game_config.tagline));
            ui.label(format!(
                "Audience: {:?}",
                wizard_state.game_config.target_audience
            ));
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.heading("🎮 Gameplay");
            ui.label(format!(
                "Core Mechanics: {}",
                wizard_state
                    .game_config
                    .core_mechanics
                    .iter()
                    .map(|m| format!("{:?}", m))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            ui.label(format!(
                "Progression: {:?}",
                wizard_state.game_config.progression_system
            ));
            ui.label("Gameplay Loop:");
            ui.indent("loop", |ui| {
                ui.weak(&wizard_state.game_config.gameplay_loop);
            });
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.heading("🎨 Visual Style");
            ui.label(format!(
                "References: {}",
                wizard_state.game_config.art_references.join(", ")
            ));
            ui.label(format!(
                "Color Mood: {:?}",
                wizard_state.game_config.color_mood
            ));
            ui.label(format!(
                "Sprite Detail: {:?}",
                wizard_state.game_config.sprite_style.detail_level
            ));
            ui.label(format!(
                "Animation: {:?}",
                wizard_state.game_config.animation_complexity
            ));
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.heading("⚙️ Features");
            for (feature, _) in &wizard_state.game_config.features {
                ui.label(format!("✓ {:?}", feature));
            }
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.heading("🔧 Technical");
            ui.label(format!(
                "World Size: {:?}",
                wizard_state.game_config.map_size
            ));
            ui.label(format!(
                "Performance: {:?}",
                wizard_state.game_config.performance_target
            ));
            ui.label(format!(
                "Platforms: {}",
                wizard_state
                    .game_config
                    .platforms
                    .iter()
                    .map(|p| format!("{:?}", p))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            if wizard_state.game_config.multiplayer_support.enabled {
                ui.label(format!(
                    "Multiplayer: Up to {} players",
                    wizard_state.game_config.multiplayer_support.max_players
                ));
            }
        });
    });
}

fn show_navigation_buttons(
    ui: &mut egui::Ui,
    wizard_state: &mut WizardState,
    next_phase: &mut NextState<StudioPhase>,
    generation_tx: &GenerationSender,
) {
    ui.horizontal(|ui| {
        if wizard_state.current_step != WizardStep::BasicInfo {
            if ui.button("← Previous").clicked() {
                wizard_state.current_step = wizard_state.current_step.previous();
            }
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if wizard_state.current_step == WizardStep::Review {
                let ready = wizard_state.validation_errors.is_empty();
                ui.add_enabled_ui(ready, |ui| {
                    if ui.button("🚀 Generate Game").clicked() {
                        if wizard_state.validate_configuration() {
                            wizard_state.start_generation(generation_tx);
                            next_phase.set(StudioPhase::Generation);
                        }
                    }
                });
            } else {
                if ui.button("Next →").clicked() {
                    if wizard_state.validate_current_step() {
                        wizard_state.current_step = wizard_state.current_step.next();
                    }
                }
            }

            if ui.button("Skip to Review").clicked() {
                wizard_state.current_step = WizardStep::Review;
            }
        });
    });
}

// Type definitions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum GameGenre {
    #[default]
    Action,
    RPG,
    Puzzle,
    Adventure,
    Simulation,
    Strategy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TargetAudience {
    Casual,
    #[default]
    Core,
    Hardcore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoreMechanic {
    Combat,
    Exploration,
    Puzzle,
    Collection,
    Building,
    Social,
    Stealth,
    Racing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ProgressionType {
    #[default]
    Linear,
    Branching,
    Open,
    Metroidvania,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DifficultyCurve {
    pub starting_difficulty: f32,
    pub ramp_speed: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ColorMood {
    #[default]
    Vibrant,
    Pastel,
    Dark,
    Earthy,
    Neon,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpriteStyle {
    pub detail_level: DetailLevel,
    pub use_outline: bool,
    pub pixel_perfect: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DetailLevel {
    Minimal,
    #[default]
    Moderate,
    Detailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AnimationComplexity {
    Simple,
    #[default]
    Moderate,
    Complex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameFeature {
    CombatSystem,
    Inventory,
    Dialogue,
    Crafting,
    SaveLoad,
    DayNight,
    Weather,
    Quests,
    Minimap,
    Achievements,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureConfig {
    pub combat_type: CombatType,
    pub inventory_slots: u32,
    pub dialogue_choices: bool,
    pub dialogue_branching: bool,
    pub save_slots: u32,
    pub autosave: bool,
}

impl FeatureConfig {
    pub fn default_for_feature(feature: GameFeature) -> Self {
        let mut config = Self::default();
        match feature {
            GameFeature::Inventory => config.inventory_slots = 30,
            GameFeature::SaveLoad => {
                config.save_slots = 3;
                config.autosave = true;
            }
            GameFeature::Dialogue => {
                config.dialogue_choices = true;
            }
            _ => {}
        }
        config
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CombatType {
    #[default]
    RealTime,
    TurnBased,
    Tactical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    Windows,
    Mac,
    Linux,
    Web,
    Steam,
    Itch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MapSize {
    Small,
    #[default]
    Medium,
    Large,
    Massive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PerformanceTarget {
    Low,
    #[default]
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MultiplayerConfig {
    pub enabled: bool,
    pub max_players: u32,
    pub network_type: NetworkType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum NetworkType {
    #[default]
    Local,
    Online,
}
