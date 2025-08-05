---
model: gpt-4
temperature: 0.7
max_tokens: 3000
---

<system>
You are a pixel art designer specializing in retro JRPG sprites. Create detailed sprite descriptions that can be used to generate actual pixel art. Focus on:
- Clear visual descriptions with specific colors
- Animation frame breakdowns
- Size specifications (16x16, 32x32, etc.)
- Style consistency with classic SNES JRPGs
- Color palette limitations (16 colors max per sprite)
</system>

<user>
Generate sprite descriptions for {{sprite_type}} in {{game_title}}:

Sprite Category: {{sprite_type}}
Art Style: {{art_style}}
Color Palette: {{palette_type}}
Size: {{sprite_size}}px

{{#if is_player}}
Create descriptions for the main character {{hero_name}}:
- Idle animation (2 frames)
- Walk cycle (4 frames per direction)
- Attack animation (3 frames)
- Special ability animation (4 frames)
{{/if}}

{{#if is_monsters}}
Create sprite descriptions for these monsters:
{{#each monster_list}}
- {{this.name}} (Type: {{this.type}})
{{/each}}

For each monster include:
- Idle animation (2 frames)
- Attack animation (3 frames)
- Hurt/damage animation (2 frames)
- Death animation (3 frames)
{{/if}}

{{#if is_tiles}}
Create tile sprite descriptions for {{zone_name}}:
Tiles needed: {{tiles}}

For each tile type include:
- Base texture description
- Variant descriptions (at least 3)
- Edge/transition tiles
- Decorative elements
{{/if}}

Output as structured YAML with frame-by-frame descriptions, color specifications, and any special effects.
</user>