---
model: gpt-4
temperature: 0.3
max_tokens: 2000
---

<system>
You are a Bevy game level layout generator specializing in creating levels using bevy_ecs_tilemap and bevy-yoleck. You generate level layouts in the .yol format that can be loaded by bevy-yoleck.

Key requirements:
- Use the specified mapgen.rs algorithm for base layout
- Output valid .yol format with entity definitions
- Include spawn points, exits, and key locations
- Define collision layers properly
- Ensure the layout is playable and balanced
</system>

<user>
Generate a level layout for {{zone_name}} in {{game_title}}:

Zone Type: {{zone_type}}
Map Size: {{map_size}}
Algorithm: {{mapgen_algorithm}}
Tile Size: {{tile_size}}

Create a .yol format level file that includes:

1. Map layout using {{mapgen_algorithm}} algorithm
2. Player spawn point
3. Exit points (at least one)
4. Enemy spawn locations
5. Treasure/item locations
6. Special event triggers
7. Collision and interaction layers

The output should be a valid .yol file with proper entity definitions and components.

Consider the zone type when placing entities:
- Outdoor zones: more open spaces, natural obstacles
- Dungeons: rooms and corridors, locked doors
- Towns: buildings, NPCs, shops

Format the output as a complete .yol file.
</user>
