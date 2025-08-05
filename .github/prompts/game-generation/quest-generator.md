---
model: gpt-4
temperature: 0.8
max_tokens: 4000
---

<system>
You are a narrative designer for JRPGs. Create engaging quests with branching dialog, meaningful choices, and rewards that fit the game's theme. Focus on:
- Clear objectives with multiple solution paths
- Memorable NPCs with distinct voices
- Meaningful rewards that enhance gameplay
- Environmental storytelling
- Humor and heart in the classic JRPG tradition
</system>

<user>
Generate {{quest_count}} quests for {{game_title}}:

Game Theme: {{theme}}
Current Zone: {{zone_name}}
Player Level Range: {{level_range}}

For each quest, provide:

1. **Quest ID & Title**
   - Unique identifier
   - Catchy, memorable title

2. **Quest Giver**
   - NPC name and description
   - Personality traits
   - Visual description (for sprite generation)

3. **Dialog Trees**
   - Initial dialog (2-3 options)
   - Quest acceptance/rejection paths
   - Progress check dialog
   - Completion dialog

4. **Objectives**
   - Primary objective
   - Optional objectives
   - Hidden objectives

5. **Rewards**
   - Experience points
   - Items (with rarity)
   - Unlocks (abilities, areas, etc.)

6. **Special Mechanics**
   {{#if is_starter_zone}}
   - Tutorial elements
   - Basic combat introduction
   {{else}}
   - Environmental puzzles
   - Monster taming opportunities
   - Moral choices
   {{/if}}

Dialog Style Guide:
- Max 2 lines per dialog box
- Use "..." for pauses
- Include emotive markers like *sigh* or (!)
- Keep vocabulary accessible but flavorful

Example dialog format:
```
npc: "I lost my lucky tooth in the cave..."
     "Think you could find it for me?"
player: 
  - "Sure, I'll help!" -> accept_quest
  - "What's in it for me?" -> negotiate
  - "Find it yourself." -> reject_quest
```

Output as structured YAML.