---
model: gpt-4
temperature: 0.3
max_tokens: 2500
---

<system>
You are a gameplay systems designer for Bevy games. You create level-specific systems, event triggers, and AI behaviors that make each level unique and engaging.

Key requirements:
- Define level-specific gameplay systems
- Create event triggers and responses
- Configure AI behaviors for the zone
- Implement special mechanics unique to the level
- Ensure all systems work within Bevy's ECS architecture
</system>

<user>
Design gameplay logic for {{zone_name}} in {{game_title}}:

Zone Type: {{zone_type}}
Core Mechanics: {{core_mechanics}}
Special Features: {{zone_special_features}}
Difficulty: {{difficulty_level}}

Create level-specific systems including:

1. **Event Triggers**
   - Entry events (cutscenes, dialogue)
   - Combat triggers (ambushes, boss fights)
   - Environmental triggers (traps, puzzles)
   - Progress gates (keys, switches, objectives)
   - Exit conditions

2. **AI Behaviors**
   - Monster patrol patterns
   - Aggro/detection ranges
   - Combat behaviors (melee, ranged, special)
   - Group tactics (if applicable)
   - Retreat/reinforcement logic

3. **Special Mechanics**
   - Zone-specific hazards (lava, ice, poison)
   - Environmental interactions
   - Puzzle mechanisms
   - Timed challenges
   - Secret areas/rewards

4. **Level State Management**
   - Checkpoint system
   - Respawn rules
   - Persistent changes (doors, switches)
   - Dynamic difficulty adjustment
   - Achievement/challenge tracking

5. **Resource Systems**
   - Health/mana regeneration zones
   - Resource nodes (if applicable)
   - Shop/upgrade points
   - Fast travel unlocks

Output as Rust code snippets using Bevy's ECS that can be integrated into the level module:
- System functions with proper Bevy queries
- Event definitions
- Component structures
- Resource definitions
- System scheduling/ordering

Include comments explaining the gameplay purpose of each system.
</user>