/**
 * @echoes-of-beastlight/game-core
 * Shared game logic, schemas, and utilities for Echoes of Beastlight
 *
 * This package is framework-agnostic and can be used by:
 * - React Native mobile app (apps/mobile)
 * - Web test app (apps/web)
 * - Server-side validation
 */

// Core game systems (full implementations)
export {
	type Ability,
	AbilitySchema,
	type ActionResult,
	type ActionType,
	ActionTypeSchema,
	type CombatAction,
	CombatActionSchema,
	type Combatant,
	CombatantSchema,
	// Combat engine
	CombatEngine,
	type CombatState,
	createTestCombatant,
	type Element,
	ElementSchema,
	type Status,
	// Types and schemas
	StatusSchema,
	type TurnEntry,
} from './combat/index.js';
// Re-export generation utilities
export * from './generation/index.js';
export {
	addExperience,
	type BiomeType,
	BiomeTypeSchema,
	calculateStats,
	calculateTamingChance,
	createMonsterInstance,
	type EvolutionPath,
	EvolutionPathSchema,
	type EvolutionRequirement,
	EvolutionRequirementSchema,
	type EvolutionRequirementType,
	EvolutionRequirementTypeSchema,
	evolveMonster,
	getExpForLevel,
	getSpeciesById,
	getSpeciesForBiome,
	type MonsterInstance,
	// Monster system
	MonsterParty,
	type MonsterSpecies,
	monsterToCombatant,
	NATURES,
	type NatureModifier,
	type Rarity,
	// Types and schemas
	RaritySchema,
	STARTER_SPECIES,
} from './monsters/index.js';
export {
	generateQuest,
	generateQuestsForRegion,
	MAIN_STORY_QUESTS,
	type ObjectiveType,
	// Types and schemas
	ObjectiveTypeSchema,
	type Quest,
	type QuestDifficulty,
	QuestDifficultySchema,
	type QuestGeneratorConfig,
	type QuestJournal,
	// Quest system
	QuestManager,
	type QuestObjective,
	type QuestReward,
	type QuestStatus,
	QuestStatusSchema,
} from './quests/index.js';

// Combat schemas
export {
	type CharacterStats,
	CharacterStatsSchema,
	type CombatResult,
	CombatResultSchema,
} from './schemas/combat.js';
// Re-export schemas (for backward compatibility - use specific exports above for types)
export * as schemas from './schemas/index.js';
// Player schemas and utilities
export {
	createNewPlayerSave,
	type Inventory,
	type InventoryItem,
	InventoryItemSchema,
	InventorySchema,
	type PlayerPosition,
	PlayerPositionSchema,
	type PlayerSave,
	PlayerSaveSchema,
	type PlayerSettings,
	PlayerSettingsSchema,
} from './schemas/player.js';
// World schemas
export {
	type Area,
	AreaSchema,
	type Tile,
	TileSchema,
	type TileType,
	TileTypeSchema,
	type WorldMap,
	WorldMapSchema,
} from './schemas/world.js';

// Utilities
export * from './utils.js';

// Version info
export const VERSION = '1.0.0';
