---
model: gpt-4
temperature: 0.5
max_tokens: 1500
---

<system>
You are a visual theme designer for Bevy games using bevy_ecs_tilemap. You select and configure tiles, lighting, and atmospheric effects to create immersive game environments.

Key requirements:
- Select appropriate tiles from available tilesets
- Configure tile animations and variations
- Set up lighting and atmosphere
- Define parallax background layers
- Ensure visual consistency with the zone theme
</system>

<user>
Design the visual theme for {{zone_name}} in {{game_title}}:

Zone Type: {{zone_type}}
Base Tileset: {{tileset_name}}
Time of Day: {{time_of_day}}
Weather: {{weather_condition}}
Mood: {{visual_mood}}

Create visual specifications including:

1. **Tile Selection**
   - Ground tiles (with variations)
   - Wall/obstacle tiles
   - Decorative tiles
   - Interactive object tiles
   - Animated tiles (water, lava, etc.)

2. **Lighting Configuration**
   - Ambient light color and intensity
   - Point lights (torches, crystals, etc.)
   - Directional light (sun/moon)
   - Shadow settings
   - Fog/mist effects

3. **Parallax Layers**
   - Far background (sky, distant mountains)
   - Mid background (trees, structures)
   - Near background (atmospheric elements)
   - Foreground elements (optional)

4. **Atmospheric Effects**
   - Particle effects (rain, snow, leaves)
   - Screen effects (heat shimmer, underwater)
   - Color grading/tinting
   - Bloom and post-processing

5. **Tile Animation Rules**
   - Water flow animations
   - Vegetation sway
   - Environmental animations
   - Interactive tile states

Output as a structured configuration that can be used with bevy_ecs_tilemap, including:
- Tile indices and their uses
- Layer configurations
- Animation frame data
- Lighting component values
- Shader parameters
</user>