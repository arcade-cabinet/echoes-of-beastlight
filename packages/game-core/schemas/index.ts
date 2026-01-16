/**
 * @echoes-of-beastlight/game-core schemas
 * Zod schemas for all game data structures
 */

// Combat system
export {
	StatusSchema,
	CharacterStatsSchema,
	CombatActionSchema,
	CombatResultSchema,
	CombatStateSchema,
	type Status,
	type CharacterStats,
	type CombatAction,
	type CombatResult,
	type CombatState,
} from './combat.js';

// Monster system
export {
	ElementTypeSchema,
	RaritySchema,
	MonsterAbilitySchema,
	MonsterSpeciesSchema,
	MonsterInstanceSchema,
	MonsterPartySchema,
	type ElementType,
	type Rarity,
	type MonsterAbility,
	type MonsterSpecies,
	type MonsterInstance,
	type MonsterParty,
} from './monster.js';

// Quest system
export {
	ObjectiveTypeSchema,
	QuestStatusSchema,
	QuestObjectiveSchema,
	QuestRewardSchema,
	QuestSchema,
	QuestJournalSchema,
	type ObjectiveType,
	type QuestStatus,
	type QuestObjective,
	type QuestReward,
	type Quest,
	type QuestJournal,
} from './quest.js';

// World/Tilemap system
export {
	BiomeTypeSchema,
	TileTypeSchema,
	TileSchema,
	AreaSchema,
	WorldMapSchema,
	type BiomeType,
	type TileType,
	type Tile,
	type Area,
	type WorldMap,
} from './world.js';

// Player system
export {
	InventoryItemSchema,
	InventorySchema,
	PlayerPositionSchema,
	PlayerSettingsSchema,
	PlayerSaveSchema,
	createNewPlayerSave,
	type InventoryItem,
	type Inventory,
	type PlayerPosition,
	type PlayerSettings,
	type PlayerSave,
} from './player.js';
