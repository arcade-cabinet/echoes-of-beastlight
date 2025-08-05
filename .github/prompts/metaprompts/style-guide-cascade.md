# Style Guide Generation Cascade

Generate a comprehensive visual style guide for {{game_title}} that will ensure consistency across all generated assets.

## Core Style Definition

First, establish the fundamental visual identity:

1. **Color Palette Definition**
   - Primary colors (3-4 main colors with hex values)
   - Secondary colors (2-3 accent colors)
   - UI colors (background, text, highlights)
   - Semantic colors (health, mana, damage, etc.)
   - Color relationships and harmony rules

2. **Art Style Parameters**
   - Pixel density: {{sprite_size}}x{{sprite_size}}
   - Outline style: (none/1px black/colored)
   - Shading technique: (flat/simple gradient/dithered)
   - Perspective: {{perspective}}
   - Animation style: (snappy/smooth/bouncy)

3. **Visual References**
   - Reference games: {{reference_games}}
   - Art movement influences
   - Mood/atmosphere keywords

## Cascade Generation

Generate the following prompts that will create the style guide assets:

### 1. Base Style Reference Image

```
Create a style guide reference image showing:
- Color swatches with hex codes
- Example character in idle pose
- Sample environment tiles (grass, stone, water)
- UI element examples
- Lighting/shadow examples
All in {{sprite_size}}x{{sprite_size}} pixel art style
```

### 2. Character Style Sheet

```
Design a character style template showing:
- Body proportions grid
- Head variations (3-4 examples)
- Outfit color schemes using palette
- Animation keyframe positions
- Emotion expression samples
```

### 3. Environment Tileset Template

```
Create environment tile templates:
- Base terrain types with palette
- Transition tiles between terrains
- Decorative elements (trees, rocks)
- Interactive object styles
- Lighting overlay examples
```

### 4. Effects and VFX Guide

```
Design visual effects templates:
- Particle effects (fire, magic, water)
- Impact effects using palette
- Status effect indicators
- Environmental effects (weather)
- UI feedback animations
```

### 5. Audio Style Guide (Procedural)

```json
{
  "audio_style": {
    "genre": "retro_jrpg",
    "mood": "{{game_mood}}",
    "instrumentation": {
      "lead": ["square_wave", "triangle_wave"],
      "bass": ["sawtooth_wave"],
      "drums": ["white_noise", "sine_wave"]
    },
    "tempo_ranges": {
      "exploration": [80, 100],
      "combat": [120, 140],
      "boss": [140, 160]
    },
    "key_signatures": ["C_major", "A_minor", "F_major"],
    "effects": {
      "reverb": 0.3,
      "delay": 0.1,
      "bit_crush": true
    }
  }
}
```

## Style Enforcement Rules

All subsequent asset generation must:
1. Sample colors ONLY from the defined palette
2. Match the established pixel density
3. Use consistent outline/shading techniques
4. Follow proportion guidelines
5. Maintain atmospheric consistency

## Validation Checklist

Each generated asset should be validated against:
- [ ] Uses only approved colors
- [ ] Matches pixel grid exactly
- [ ] Consistent outline treatment
- [ ] Appropriate detail level
- [ ] Cohesive with other assets

## Output Format

The style guide should output:
1. `style-guide.png` - Visual reference sheet
2. `color-palette.json` - Programmatic color values
3. `style-rules.md` - Written style documentation
4. `validation-matrix.json` - Automated checking rules
