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
use bevy_egui::{EguiContexts, EguiPlugin, egui};
use bevy_inspector_egui::prelude::*;
use crossbeam_channel::{Receiver, Sender, unbounded};
use egui_dock::{DockArea, DockState, Style as DockStyle};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

mod console;
mod editor;
mod gallery;
mod generator;
mod preview;
mod theme;
mod wizard;

pub use console::*;
pub use editor::*;
pub use gallery::*;
pub use generator::*;
pub use preview::*;
pub use theme::*;
pub use wizard::*;

/// Main application plugin that orchestrates the entire game generation studio
pub struct GameGeneratorStudioPlugin;

impl Plugin for GameGeneratorStudioPlugin {
    fn build(&self, app: &mut App) {
        app
            // Core resources
            .init_resource::<StudioState>()
            .init_resource::<GeneratorState>()
            .init_resource::<ProjectDatabase>()
            .init_resource::<AssetCache>()
            .init_resource::<GenerationTasks>()
            // Communication channels
            .insert_resource(create_generation_channels())
            // Plugins
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Echoes of Beastlight - Game Studio".into(),
                    resolution: (1600., 900.).into(),
                    ..default()
                }),
                ..default()
            }))
            .add_plugins(EguiPlugin)
            .add_plugins(WorldInspectorPlugin::new())
            // Systems
            .add_systems(Startup, setup_studio)
            .add_systems(
                Update,
                (
                    studio_ui_system,
                    generation_task_processor,
                    live_preview_updater,
                    asset_hot_reload_system,
                )
                    .chain(),
            )
            // States
            .init_state::<StudioPhase>()
            .add_systems(OnEnter(StudioPhase::Setup), enter_setup_wizard)
            .add_systems(OnEnter(StudioPhase::Generation), start_generation)
            .add_systems(OnEnter(StudioPhase::LiveEdit), setup_live_editor);
    }
}

/// Studio phases representing the workflow
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum StudioPhase {
    #[default]
    Setup, // Initial wizard
    Generation, // Asset generation
    LiveEdit,   // Live game editing
}

/// Main studio state
#[derive(Resource)]
pub struct StudioState {
    pub current_project: Option<ProjectId>,
    pub dock_state: Arc<Mutex<DockState<DockTab>>>,
    pub notifications: Vec<Notification>,
    pub theme: StudioTheme,
}

impl Default for StudioState {
    fn default() -> Self {
        Self {
            current_project: None,
            dock_state: Arc::new(Mutex::new(create_default_dock_state())),
            notifications: Vec::new(),
            theme: StudioTheme::default(),
        }
    }
}

/// Dock tabs for the studio interface
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DockTab {
    ProjectWizard,
    AssetGallery,
    CodeEditor,
    LivePreview,
    Inspector,
    Console,
    Timeline,
    StyleGuide,
    Documentation,
}

/// Main studio UI system
pub fn studio_ui_system(
    mut contexts: EguiContexts,
    mut studio_state: ResMut<StudioState>,
    mut wizard_state: ResMut<WizardState>,
    current_phase: Res<State<StudioPhase>>,
    mut next_phase: ResMut<NextState<StudioPhase>>,
    generation_tx: Res<GenerationSender>,
    generation_rx: Res<GenerationReceiver>,
    mut generation_state: ResMut<GeneratorState>,
    asset_cache: Res<AssetCache>,
    console_state: ResMut<ConsoleState>,
    mut editor_state: ResMut<EditorState>,
) {
    let ctx = contexts.ctx_mut();

    // Apply custom theme
    apply_studio_theme(ctx, &studio_state.theme);

    // Top menu bar
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("🆕 New Project").clicked() {
                    next_phase.set(StudioPhase::Setup);
                    wizard_state.reset();
                }
                if ui.button("📂 Open Project").clicked() {
                    // Open project dialog
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Game Config", &["yaml"])
                        .pick_file()
                    {
                        // Load project
                        studio_state
                            .notifications
                            .push(Notification::info(format!("Loading project: {:?}", path)));
                    }
                }
                ui.separator();
                if ui.button("💾 Save Project").clicked() {
                    // Save current project
                }
                if ui.button("📦 Export Game").clicked() {
                    // Export final game
                }
                ui.separator();
                if ui.button("❌ Exit").clicked() {
                    std::process::exit(0);
                }
            });

            ui.menu_button("Edit", |ui| {
                if ui.button("↩️ Undo").clicked() {
                    // Undo last action
                }
                if ui.button("↪️ Redo").clicked() {
                    // Redo action
                }
                ui.separator();
                if ui.button("🔄 Regenerate Asset").clicked() {
                    // Open regeneration dialog
                }
                if ui.button("📋 Batch Operations").clicked() {
                    // Batch processing
                }
            });

            ui.menu_button("View", |ui| {
                if ui.button("🔧 Reset Layout").clicked() {
                    studio_state.dock_state = Arc::new(Mutex::new(create_default_dock_state()));
                }
                ui.separator();
                ui.checkbox(&mut studio_state.theme.show_fps, "Show FPS");
                ui.checkbox(&mut studio_state.theme.show_diagnostics, "Show Diagnostics");
            });

            ui.menu_button("Tools", |ui| {
                if ui.button("🎨 Style Guide Manager").clicked() {
                    // Open style guide
                }
                if ui.button("🔊 Audio Manager").clicked() {
                    // Open audio manager
                }
                if ui.button("🗺️ Map Editor").clicked() {
                    // Open map editor
                }
            });

            ui.menu_button("Help", |ui| {
                if ui.button("📚 Documentation").clicked() {
                    // Open docs
                    webbrowser::open("https://github.com/yourusername/echoes-of-beastlight/wiki")
                        .unwrap_or_default();
                }
                if ui.button("🎮 Examples").clicked() {
                    // Show examples
                }
                if ui.button("ℹ️ About").clicked() {
                    studio_state.notifications.push(Notification::info(
                        "Echoes of Beastlight Studio v0.1.0\nAI-powered game generation",
                    ));
                }
            });
        });
    });

    // Status bar
    egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            // Phase indicator
            let phase_icon = match current_phase.get() {
                StudioPhase::Setup => "🎮",
                StudioPhase::Generation => "⚙️",
                StudioPhase::LiveEdit => "✏️",
            };
            ui.label(format!("{} Phase: {:?}", phase_icon, current_phase.get()));
            ui.separator();

            // Generation progress
            if current_phase.get() == &StudioPhase::Generation {
                let progress = generation_state.overall_progress();
                ui.add(
                    egui::ProgressBar::new(progress)
                        .text(format!(
                            "{:.0}% - {}",
                            progress * 100.0,
                            generation_state.current_task
                        ))
                        .desired_width(200.0),
                );
            }

            // Memory usage
            ui.separator();
            ui.label(format!("Mem: {:.1} MB", get_memory_usage_mb()));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if studio_state.theme.show_fps {
                    // TODO: Get real FPS from Bevy diagnostics
                    ui.label(format!("FPS: {:.1}", 60.0));
                }

                // Connection status
                if generation_state.openai_connected {
                    ui.colored_label(egui::Color32::GREEN, "🟢 OpenAI Connected");
                } else {
                    ui.colored_label(egui::Color32::RED, "🔴 OpenAI Disconnected");
                }
            });
        });
    });

    // Main content area with docking
    egui::CentralPanel::default().show(ctx, |ui| {
        let mut dock_state = studio_state.dock_state.lock().unwrap();

        DockArea::new(&mut *dock_state)
            .style(DockStyle::from_egui(ui.style()))
            .show_inside(
                ui,
                &mut TabViewer {
                    wizard_state: &mut wizard_state,
                    current_phase: current_phase.get(),
                    generation_tx: &generation_tx,
                    generation_rx: &generation_rx,
                    next_phase: &mut next_phase,
                    generation_state: &mut generation_state,
                    asset_cache: &asset_cache,
                    console_state,
                    editor_state: &mut editor_state,
                    studio_state: &mut studio_state,
                },
            );
    });

    // Notifications
    show_notifications(ctx, &mut studio_state.notifications);
}

/// Tab viewer for dock system
struct TabViewer<'a> {
    wizard_state: &'a mut WizardState,
    current_phase: &'a StudioPhase,
    generation_tx: &'a GenerationSender,
    generation_rx: &'a GenerationReceiver,
    next_phase: &'a mut NextState<StudioPhase>,
    generation_state: &'a mut GeneratorState,
    asset_cache: &'a AssetCache,
    console_state: ResMut<'a, ConsoleState>,
    editor_state: &'a mut EditorState,
    studio_state: &'a mut StudioState,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = DockTab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            DockTab::ProjectWizard => "🎮 Project Setup".into(),
            DockTab::AssetGallery => "🎨 Asset Gallery".into(),
            DockTab::CodeEditor => "📝 Code Editor".into(),
            DockTab::LivePreview => "▶️ Live Preview".into(),
            DockTab::Inspector => "🔍 Inspector".into(),
            DockTab::Console => "📜 Console".into(),
            DockTab::Timeline => "⏱️ Timeline".into(),
            DockTab::StyleGuide => "🎯 Style Guide".into(),
            DockTab::Documentation => "📚 Documentation".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            DockTab::ProjectWizard => wizard::show_project_wizard(
                ui,
                self.wizard_state,
                self.current_phase,
                self.next_phase,
                self.generation_tx,
            ),
            DockTab::AssetGallery => {
                gallery::show_asset_gallery(ui, self.asset_cache, self.generation_tx)
            }
            DockTab::CodeEditor => editor::show_code_editor(ui, self.editor_state),
            DockTab::LivePreview => preview::show_live_preview(ui),
            DockTab::Inspector => preview::show_inspector(ui),
            DockTab::Console => console::show_console(ui, self.console_state),
            DockTab::Timeline => show_timeline(ui),
            DockTab::StyleGuide => show_style_guide(ui, self.generation_state),
            DockTab::Documentation => show_documentation(ui),
        }
    }
}

/// Create default dock layout
fn create_default_dock_state() -> DockState<DockTab> {
    let mut state = DockState::new(vec![DockTab::ProjectWizard]);

    // Create default layout
    let tree = state.main_surface_mut();

    let [left, right] = tree.split_left(
        egui_dock::NodeIndex::root(),
        0.2,
        vec![DockTab::AssetGallery],
    );
    let [center, bottom] = tree.split_below(right, 0.7, vec![DockTab::Console]);
    let [preview, inspector] = tree.split_right(center, 0.7, vec![DockTab::Inspector]);

    tree.set_focused_node(preview);
    tree.push_to_focused_leaf(DockTab::LivePreview);
    tree.push_to_focused_leaf(DockTab::CodeEditor);

    state
}

/// Timeline view for animation and sequencing
fn show_timeline(ui: &mut egui::Ui) {
    ui.heading("⏱️ Timeline");
    ui.separator();

    ui.label("Animation timeline will appear here");

    // TODO: Implement timeline with keyframes
}

/// Style guide manager
fn show_style_guide(ui: &mut egui::Ui, generation_state: &GeneratorState) {
    ui.heading("🎯 Style Guide");
    ui.separator();

    ui.label("Maintaining consistent visual style across all assets");

    ui.add_space(10.0);

    // Color palette
    ui.group(|ui| {
        ui.heading("Color Palette");
        ui.horizontal_wrapped(|ui| {
            for (name, color) in &generation_state.style_guide.colors {
                ui.vertical(|ui| {
                    let (r, g, b, _) = color.to_tuple();
                    ui.colored_label(
                        egui::Color32::from_rgb(
                            (r * 255.0) as u8,
                            (g * 255.0) as u8,
                            (b * 255.0) as u8,
                        ),
                        "████",
                    );
                    ui.small(name);
                });
            }
        });
    });

    ui.add_space(10.0);

    // Reference sprites
    ui.group(|ui| {
        ui.heading("Reference Sprites");
        ui.horizontal_wrapped(|ui| {
            for reference in &generation_state.style_guide.references {
                ui.group(|ui| {
                    ui.set_min_size(egui::vec2(64.0, 80.0));
                    ui.vertical_centered(|ui| {
                        ui.colored_label(egui::Color32::GRAY, "🖼️");
                        ui.small(&reference.name);
                    });
                });
            }
        });
    });

    ui.add_space(10.0);

    if ui.button("🔄 Regenerate Style Guide").clicked() {
        // Regenerate style guide
    }
}

/// Documentation viewer
fn show_documentation(ui: &mut egui::Ui) {
    ui.heading("📚 Documentation");
    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.collapsing("Getting Started", |ui| {
            ui.label("1. Create a new project using the Project Wizard");
            ui.label("2. Configure your game settings");
            ui.label("3. Generate initial assets");
            ui.label("4. Use the live editor to refine");
        });

        ui.collapsing("Keyboard Shortcuts", |ui| {
            ui.label("Ctrl+N - New Project");
            ui.label("Ctrl+O - Open Project");
            ui.label("Ctrl+S - Save Project");
            ui.label("F5 - Regenerate Current Asset");
        });

        ui.collapsing("API Reference", |ui| {
            ui.label("See online documentation for API details");
        });
    });
}

/// System implementations
fn setup_studio(mut commands: Commands) {
    // Setup default camera for UI
    commands.spawn(Camera2dBundle::default());

    info!("Game Generator Studio initialized");
}

fn generation_task_processor(
    mut generation_state: ResMut<GeneratorState>,
    generation_rx: Res<GenerationReceiver>,
    mut console: ResMut<ConsoleState>,
) {
    // Process generation results
    while let Ok(result) = generation_rx.0.try_recv() {
        match result {
            GenerationResult::Success { task_id, output } => {
                console.log(ConsoleLevel::Info, format!("✓ Task {} completed", task_id));
                generation_state.complete_task(task_id, output);
            }
            GenerationResult::Error { task_id, error } => {
                console.log(
                    ConsoleLevel::Error,
                    format!("✗ Task {} failed: {}", task_id, error),
                );
                generation_state.fail_task(task_id, error);
            }
            GenerationResult::Progress { task_id, progress } => {
                generation_state.update_progress(task_id, progress);
            }
        }
    }
}

fn live_preview_updater() {
    // Update live preview
}

fn asset_hot_reload_system() {
    // Watch for asset changes and reload
}

fn enter_setup_wizard() {
    info!("Entering setup wizard");
}

fn start_generation() {
    info!("Starting generation phase");
}

fn setup_live_editor() {
    info!("Setting up live editor");
}

/// Communication types
#[derive(Resource)]
pub struct GenerationSender(pub Sender<GenerationRequest>);

#[derive(Resource)]
pub struct GenerationReceiver(pub Receiver<GenerationResult>);

fn create_generation_channels() -> (GenerationSender, GenerationReceiver) {
    let (tx, rx) = unbounded();
    let (result_tx, result_rx) = unbounded();

    // Spawn async runtime for generation tasks
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            generator::run_generation_loop(rx, result_tx).await;
        });
    });

    (GenerationSender(tx), GenerationReceiver(result_rx))
}

// Core types
pub type ProjectId = Uuid;

#[derive(Debug, Clone)]
pub struct Notification {
    pub level: NotificationLevel,
    pub message: String,
    pub timestamp: std::time::Instant,
}

#[derive(Debug, Clone, Copy)]
pub enum NotificationLevel {
    Info,
    Warning,
    Error,
    Success,
}

impl Notification {
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            level: NotificationLevel::Info,
            message: message.into(),
            timestamp: std::time::Instant::now(),
        }
    }
}

fn show_notifications(ctx: &egui::Context, notifications: &mut Vec<Notification>) {
    let mut to_remove = Vec::new();

    for (i, notif) in notifications.iter().enumerate() {
        let elapsed = notif.timestamp.elapsed().as_secs_f32();
        if elapsed > 5.0 {
            to_remove.push(i);
            continue;
        }

        let opacity = if elapsed > 4.0 {
            1.0 - (elapsed - 4.0)
        } else {
            1.0
        };

        egui::Window::new("notification")
            .id(egui::Id::new(i))
            .fixed_pos(egui::pos2(
                ctx.screen_rect().width() - 320.0,
                50.0 + i as f32 * 80.0,
            ))
            .fixed_size(egui::vec2(300.0, 60.0))
            .collapsible(false)
            .title_bar(false)
            .show(ctx, |ui| {
                ui.set_opacity(opacity);

                let color = match notif.level {
                    NotificationLevel::Info => egui::Color32::LIGHT_BLUE,
                    NotificationLevel::Warning => egui::Color32::YELLOW,
                    NotificationLevel::Error => egui::Color32::RED,
                    NotificationLevel::Success => egui::Color32::GREEN,
                };

                ui.horizontal(|ui| {
                    ui.colored_label(
                        color,
                        match notif.level {
                            NotificationLevel::Info => "ℹ️",
                            NotificationLevel::Warning => "⚠️",
                            NotificationLevel::Error => "❌",
                            NotificationLevel::Success => "✅",
                        },
                    );
                    ui.label(&notif.message);
                });
            });
    }

    for i in to_remove.into_iter().rev() {
        notifications.remove(i);
    }
}

fn get_memory_usage_mb() -> f32 {
    // Placeholder - would use actual memory tracking
    128.5
}
