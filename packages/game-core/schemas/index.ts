/**
 * @echoes-of-beastlight/game-core schemas
 * Zod schemas for all game data structures
 */

// Combat system
export {
	type CharacterStats,
	CharacterStatsSchema,
	type CombatAction,
	CombatActionSchema,
	type CombatResult,
	CombatResultSchema,
	type CombatState,
	CombatStateSchema,
	type Status,
	StatusSchema,
} from './combat.js';

// Monster system
export {
	type ElementType,
	ElementTypeSchema,
	type MonsterAbility,
	MonsterAbilitySchema,
	type MonsterInstance,
	MonsterInstanceSchema,
	type MonsterParty,
	MonsterPartySchema,
	type MonsterSpecies,
	MonsterSpeciesSchema,
	type Rarity,
	RaritySchema,
} from './monster.js';
// Player system
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
} from './player.js';
// Quest system
export {
	type ObjectiveType,
	ObjectiveTypeSchema,
	type Quest,
	type QuestJournal,
	QuestJournalSchema,
	type QuestObjective,
	QuestObjectiveSchema,
	type QuestReward,
	QuestRewardSchema,
	QuestSchema,
	type QuestStatus,
	QuestStatusSchema,
} from './quest.js';
// World/Tilemap system
export {
	type Area,
	AreaSchema,
	type BiomeType,
	BiomeTypeSchema,
	type Tile,
	TileSchema,
	type TileType,
	TileTypeSchema,
	type WorldMap,
	WorldMapSchema,
} from './world.js';
