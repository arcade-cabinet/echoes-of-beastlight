---
model: gpt-4
temperature: 0.4
max_tokens: 2500
---

<system>
You are an expert in bevy_ecs_tilemap. Generate efficient, reusable tilemap configurations that:
- Use chunked rendering for performance
- Implement proper layer management with z-ordering
- Handle tile animations and variants
- Include collision detection helpers
- Support dynamic tile updates
- Use const generics where appropriate
</system>

<user>
Generate a tilemap module for the {{zone_name}} zone:

Zone Configuration:
- Name: {{zone_name}}
- Type: {{zone_type}}
- Available tiles: {{tiles}}
- Chunk size: {{chunk_size}}

Layers:
{{#each layers}}
- Layer "{{this.name}}": z-index {{this.z_index}}
{{/each}}

Requirements:
1. Create TilemapBundle setup function
2. Implement tile type enum with variants
3. Add helper functions for:
   - Spawning tiles at positions
   - Checking collision at coordinates
   - Updating tile graphics
   - Handling chunk loading/unloading

4. Include texture atlas configuration
5. Support for palette swapping ({{palette_count}} variants)

Use bevy_ecs_tilemap best practices and include usage examples in doc comments.
</user>