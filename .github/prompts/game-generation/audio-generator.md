---
model: gpt-4
temperature: 0.6
max_tokens: 2500
---

<system>
You are a retro game audio designer specializing in chiptune and SNES-style music. Create detailed audio specifications that can be used to generate actual game audio. Focus on:
- Instrument specifications (square waves, triangle waves, noise channels)
- Tempo and time signatures
- Melodic patterns and chord progressions
- Sound effect parameters (pitch, duration, envelope)
- Looping points for background music
</system>

<user>
Generate audio specifications for {{audio_type}} in {{game_title}}:

Audio Category: {{audio_type}}
Style: {{music_style}}
Mood: {{mood}}

{{#if is_music}}
Create specifications for these tracks:
{{#each track_list}}
- {{this.name}} ({{this.usage}})
{{/each}}

For each track include:
- BPM and time signature
- Key signature
- Instrument layers (lead, bass, drums, harmony)
- Loop structure (intro, A section, B section, outro)
- Dynamic variations
{{/if}}

{{#if is_sfx}}
Create sound effect specifications for:
{{#each sfx_list}}
- {{this.name}} ({{this.category}})
{{/each}}

For each sound effect include:
- Duration (ms)
- Base frequency
- Waveform type
- Envelope (ADSR)
- Any modulation or effects
{{/if}}

{{#if is_ambient}}
Create ambient sound specifications for:
{{#each ambient_list}}
- {{this.zone}} environment
{{/each}}

Include:
- Layered elements
- Loop duration
- Crossfade parameters
- Environmental effects (reverb, echo)
{{/if}}

Output as structured YAML with technical parameters suitable for audio synthesis.
</user>