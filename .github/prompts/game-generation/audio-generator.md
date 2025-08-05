---
model: gpt-4
temperature: 0.7
max_tokens: 3000
---

# Procedural Audio Specification Generator

You are tasked with generating procedural audio specifications for {{game_title}}. Since we cannot generate actual audio files, create detailed JSON specifications that can be used with Web Audio API or other procedural audio systems.

## Audio Style Guide

Based on the game's style:
- Genre: {{audio_genre}}
- Mood: {{game_mood}}
- Era: Retro JRPG (SNES-style)

## Required Audio Specifications

### 1. Background Music Tracks

Generate specifications for:
- Main theme
- {{#each zones}}{{name}} theme{{/each}}
- Battle theme
- Boss theme
- Victory fanfare
- Game over theme

### 2. Sound Effects

Generate specifications for:
- UI sounds (menu select, confirm, cancel)
- Player actions (jump, attack, hit, heal)
- Enemy sounds (attack, defeat)
- Environmental (doors, chests, switches)
- Combat effects (sword slash, magic cast, impact)

### 3. Ambient Sounds

Generate specifications for:
- {{#each zones}}{{biome}} ambience{{/each}}
- Weather effects
- Cave echoes
- Town bustle

## Output Format

```json
{
  "audio_specifications": {
    "music": {
      "main_theme": {
        "tempo": 120,
        "key": "C_major",
        "time_signature": "4/4",
        "structure": ["intro", "verse", "chorus", "verse", "chorus", "bridge", "outro"],
        "instruments": {
          "lead": {
            "type": "square_wave",
            "envelope": {"attack": 0.01, "decay": 0.1, "sustain": 0.7, "release": 0.2}
          },
          "bass": {
            "type": "sawtooth_wave",
            "envelope": {"attack": 0.0, "decay": 0.05, "sustain": 0.8, "release": 0.1}
          },
          "drums": {
            "kick": {"type": "sine_wave", "frequency": 60, "duration": 0.1},
            "snare": {"type": "white_noise", "filter": "highpass", "frequency": 2000}
          }
        },
        "melody": {
          "notes": ["C4", "E4", "G4", "C5", "G4", "E4"],
          "durations": [0.25, 0.25, 0.25, 0.5, 0.25, 0.5]
        },
        "effects": {
          "reverb": 0.3,
          "delay": {"time": 0.375, "feedback": 0.3, "wet": 0.2}
        }
      }
    },
    "sfx": {
      "menu_select": {
        "type": "synthesis",
        "waveform": "square",
        "frequency_envelope": {
          "start": 400,
          "end": 600,
          "duration": 0.1
        },
        "volume_envelope": {
          "attack": 0.0,
          "decay": 0.1,
          "sustain": 0.0,
          "release": 0.0
        }
      }
    },
    "ambient": {
      "forest": {
        "layers": [
          {
            "type": "brown_noise",
            "filter": {"type": "bandpass", "frequency": 500, "q": 2},
            "volume": 0.3,
            "description": "wind through trees"
          },
          {
            "type": "chirp_sequence",
            "pattern": "random",
            "frequency_range": [2000, 4000],
            "interval_range": [3, 8],
            "description": "bird calls"
          }
        ]
      }
    }
  },
  "implementation_notes": {
    "web_audio_api": "Use OscillatorNode and GainNode for synthesis",
    "timing": "Use AudioContext.currentTime for precise scheduling",
    "mixing": "Apply master compression and limiting"
  }
}
```

Generate complete procedural audio specifications that capture the essence of {{game_title}} while being implementable with Web Audio API or similar systems.
