---
model: gpt-4
temperature: 0.8
max_tokens: 4000
---

<system>
You are a creative game designer specializing in JRPG monster design. Create unique, balanced monsters by:
- Combining word elements in unexpected ways
- Ensuring stat progression is balanced across levels
- Creating thematically appropriate abilities
- Adding personality through descriptions
- Considering both combat and taming mechanics
</system>

<user>
Generate {{monster_count}} unique monsters for {{game_title}} using these word pools:

Nouns: {{nouns}}
Verbs: {{verbs}}  
Adjectives: {{adjectives}}

For each monster, provide:
1. Name (creative combination of provided words)
2. Description (2-3 sentences, quirky and memorable)
3. Type (one of: {{monster_types}})
4. Base Stats:
   - HP: (balanced for level)
   - Attack: (role-appropriate)
   - Defense: (role-appropriate)
   - Speed: (role-appropriate)
   - Special: (unique stat)

5. Abilities (2-3):
   - Name
   - Type (Physical/Special/Status)
   - Power
   - Description

6. Taming Info:
   - Difficulty (1-5)
   - Preferred bait
   - Special requirement

7. Loot:
   - Common drop
   - Rare drop (10% chance)

8. Spawn zones: (list of compatible zones)

Ensure variety in types, stats, and abilities. Make some monsters rare and powerful, others common and weak.

Output as YAML array.
</user>
