---
model: gpt-4
temperature: 0.7
max_tokens: 3000
---

# Sprite Generation Prompt

You are tasked with generating sprite descriptions for {{game_title}}.

## Style Guide Compliance

**CRITICAL**: All sprites MUST comply with the established style guide:
- Use ONLY colors from `color-palette.json`
- Maintain {{sprite_size}}x{{sprite_size}} pixel dimensions
- Follow the outline style defined in the style guide
- Match the shading technique (flat/gradient/dithered)
- Ensure consistency with reference sprites

## Character Sprites

For the hero character ({{hero_name}}):
- **Base Design**: {{hero_description}}
- **Sprite Requirements**:
  - Idle animation (4 frames)
  - Walk cycle (8 frames, 4 directions)
  - Attack animation (6 frames)
  - Hit/damage animation (3 frames)
  - Color palette: Use primary character colors from style guide

## Enemy Sprites

Generate enemy sprites based on zone types:
{{#each zones}}
- **{{name}} Enemies**:
  - Design 2-3 enemy types fitting the {{biome}} theme
  - Each enemy needs: idle (2 frames), move (4 frames), attack (4 frames)
  - Use zone-specific color variations from palette
{{/each}}

## Environmental Sprites

### Tilesets

- **Base Tiles** (16x16 each, combine into {{sprite_size}}x{{sprite_size}}):
  - Grass variations (4 tiles)
  - Stone/rock variations (4 tiles)
  - Water with animation (4 frames)
  - Path/dirt variations (4 tiles)

### Interactive Objects

- Treasure chests (closed/open states)
- Doors (locked/unlocked states)
- Switches/levers (on/off states)
- Collectibles (coins, hearts, potions)

## Effects Sprites

### Combat Effects

- Slash effect (5 frames, directional)
- Magic projectiles (3 types, 4 frames each)
- Impact effects (small/medium/large)
- Status effect overlays

### Environmental Effects

- Weather particles (rain, snow, leaves)
- Ambient animations (grass sway, water ripple)
- Lighting effects (torch fire, magic glow)

## Sprite Sheet Organization

Organize all sprites into efficient sprite sheets:

```
characters.png:
- Hero animations (grid layout)
- NPCs (if any)

enemies.png:
- All enemy types and animations
- Organized by zone

tileset.png:
- All environment tiles
- Transition tiles
- Decorative elements

effects.png:
- All VFX sprites
- Organized by type

ui.png:
- HUD elements
- Menu components
- Icons
```

## Technical Requirements

1. **Format**: PNG with transparency
2. **Color Mode**: Indexed color using style palette
3. **Optimization**: Remove unused space, pack efficiently
4. **Naming**: `[category]_[name]_[state]_[frame].png`

## Validation Output

For each sprite, provide:

```json
{
  "sprite_name": "hero_idle",
  "dimensions": [32, 32],
  "frame_count": 4,
  "colors_used": ["#1a1c2c", "#5d275d", "#b13e53"],
  "style_compliance": true,
  "animation_fps": 8
}
```

Remember: The goal is cohesive, consistent pixel art that tells the story of {{game_title}} through its visual style.
