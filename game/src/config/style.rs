use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Complete style configuration for Echoes of Beastlight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleConfig {
    pub visual: VisualStyle,
    pub audio: AudioStyle,
    pub ui: UIStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualStyle {
    pub art_style: ArtStyle,
    pub palette: ColorPalette,
    pub sprites: SpriteConfig,
    pub effects: EffectsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtStyle {
    pub name: &'static str,
    pub description: &'static str,
    pub perspective: &'static str,
    pub tile_size: u32,
    pub pixel_scale: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    // Core world colors - corrupted light theme
    pub primary_dark: Color,   // Deep shadow
    pub primary_mid: Color,    // Twilight
    pub primary_light: Color,  // Fading light
    pub primary_bright: Color, // Pure light (rare)

    // Corruption colors
    pub corruption_dark: Color, // Deep corruption
    pub corruption_mid: Color,  // Active corruption
    pub corruption_glow: Color, // Corruption energy

    // Nature colors (being corrupted)
    pub nature_earth: Color, // Earth tones
    pub nature_water: Color, // Water (tainted)
    pub nature_plant: Color, // Vegetation (dying)

    // Magic/Energy colors
    pub magic_pure: Color,    // Uncorrupted magic
    pub magic_tainted: Color, // Corrupted magic
    pub energy_heal: Color,   // Healing energy
    pub energy_harm: Color,   // Harmful energy

    // UI Semantic colors
    pub ui_health: Color,
    pub ui_mana: Color,
    pub ui_stamina: Color,
    pub ui_experience: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteConfig {
    pub character_size: UVec2,
    pub monster_sizes: MonsterSizes,
    pub tile_size: u32,
    pub outline_width: f32,
    pub outline_color: Color,
    pub animation_fps: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterSizes {
    pub tiny: UVec2,   // 16x16
    pub small: UVec2,  // 32x32
    pub medium: UVec2, // 48x48
    pub large: UVec2,  // 64x64
    pub huge: UVec2,   // 96x96
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectsConfig {
    pub particle_style: &'static str,
    pub glow_intensity: f32,
    pub corruption_animation: &'static str,
    pub light_bloom: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioStyle {
    pub music_theme: &'static str,
    pub ambient_style: &'static str,
    pub effect_style: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIStyle {
    pub theme: &'static str,
    pub font_family: &'static str,
    pub base_font_size: f32,
    pub panel_style: &'static str,
    pub button_style: &'static str,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            visual: VisualStyle {
                art_style: ArtStyle {
                    name: "Corrupted Light Pixel Art",
                    description: "Dark fantasy pixel art with light/shadow contrast",
                    perspective: "Top-down 3/4 view",
                    tile_size: 32,
                    pixel_scale: 2,
                },
                palette: ColorPalette {
                    // Dark to light progression
                    primary_dark: Color::hex("#0a0a0f").unwrap(),
                    primary_mid: Color::hex("#2a2a3e").unwrap(),
                    primary_light: Color::hex("#6a6a8e").unwrap(),
                    primary_bright: Color::hex("#ffffff").unwrap(),

                    // Purple corruption theme
                    corruption_dark: Color::hex("#1a0a2e").unwrap(),
                    corruption_mid: Color::hex("#53354a").unwrap(),
                    corruption_glow: Color::hex("#903749").unwrap(),

                    // Muted nature colors
                    nature_earth: Color::hex("#3e2723").unwrap(),
                    nature_water: Color::hex("#263238").unwrap(),
                    nature_plant: Color::hex("#1b5e20").unwrap(),

                    // Magic colors
                    magic_pure: Color::hex("#64b5f6").unwrap(),
                    magic_tainted: Color::hex("#7b1fa2").unwrap(),
                    energy_heal: Color::hex("#81c784").unwrap(),
                    energy_harm: Color::hex("#e57373").unwrap(),

                    // UI colors
                    ui_health: Color::hex("#d32f2f").unwrap(),
                    ui_mana: Color::hex("#1976d2").unwrap(),
                    ui_stamina: Color::hex("#388e3c").unwrap(),
                    ui_experience: Color::hex("#fbc02d").unwrap(),
                },
                sprites: SpriteConfig {
                    character_size: UVec2::new(32, 32),
                    monster_sizes: MonsterSizes {
                        tiny: UVec2::new(16, 16),
                        small: UVec2::new(32, 32),
                        medium: UVec2::new(48, 48),
                        large: UVec2::new(64, 64),
                        huge: UVec2::new(96, 96),
                    },
                    tile_size: 32,
                    outline_width: 1.0,
                    outline_color: Color::BLACK,
                    animation_fps: 8.0,
                },
                effects: EffectsConfig {
                    particle_style: "pixel_glow",
                    glow_intensity: 0.8,
                    corruption_animation: "pulsing_veins",
                    light_bloom: true,
                },
            },
            audio: AudioStyle {
                music_theme: "Dark orchestral with ethereal elements",
                ambient_style: "Atmospheric with distant echoes",
                effect_style: "Crisp 16-bit inspired with modern processing",
            },
            ui: UIStyle {
                theme: "Dark fantasy with light accents",
                font_family: "pixel_gothic",
                base_font_size: 16.0,
                panel_style: "stone_frame_corrupted",
                button_style: "glowing_rune",
            },
        }
    }
}
