import { z } from 'zod';
import { CharacterStatsSchema } from './combat.js';
import { MonsterPartySchema } from './monster.js';
import { QuestJournalSchema } from './quest.js';

/**
 * Player inventory item
 */
export const InventoryItemSchema = z.object({
	itemId: z.string(),
	quantity: z.number().int().min(1),
});
export type InventoryItem = z.infer<typeof InventoryItemSchema>;

/**
 * Player inventory
 */
export const InventorySchema = z.object({
	items: z.array(InventoryItemSchema),
	maxSlots: z.number().int().min(1).default(50),
	gold: z.number().int().min(0).default(0),
});
export type Inventory = z.infer<typeof InventorySchema>;

/**
 * Player position in world
 */
export const PlayerPositionSchema = z.object({
	areaId: z.string(),
	tileX: z.number().int(),
	tileY: z.number().int(),
	facing: z.enum(['up', 'down', 'left', 'right']),
});
export type PlayerPosition = z.infer<typeof PlayerPositionSchema>;

/**
 * Player settings/preferences
 */
export const PlayerSettingsSchema = z.object({
	musicVolume: z.number().min(0).max(1).default(0.7),
	sfxVolume: z.number().min(0).max(1).default(0.8),
	textSpeed: z.enum(['slow', 'normal', 'fast']).default('normal'),
	battleAnimations: z.boolean().default(true),
});
export type PlayerSettings = z.infer<typeof PlayerSettingsSchema>;

/**
 * Complete player save data
 */
export const PlayerSaveSchema = z.object({
	id: z.string().uuid(),
	name: z.string().min(1).max(20),
	stats: CharacterStatsSchema,
	level: z.number().int().min(1).max(100).default(1),
	experience: z.number().int().min(0).default(0),
	position: PlayerPositionSchema,
	monsters: MonsterPartySchema,
	inventory: InventorySchema,
	quests: QuestJournalSchema,
	settings: PlayerSettingsSchema,
	playtime: z.number().int().min(0).default(0), // seconds
	createdAt: z.string().datetime(),
	updatedAt: z.string().datetime(),
});
export type PlayerSave = z.infer<typeof PlayerSaveSchema>;

/**
 * New game starting state
 */
export const createNewPlayerSave = (name: string): PlayerSave => {
	const now = new Date().toISOString();
	return {
		id: crypto.randomUUID(),
		name,
		stats: {
			hp: 100,
			maxHp: 100,
			attack: 10,
			defense: 10,
			critChance: 0.05,
			status: 'Normal',
		},
		level: 1,
		experience: 0,
		position: {
			areaId: 'starting-village',
			tileX: 5,
			tileY: 5,
			facing: 'down',
		},
		monsters: {
			active: [],
			storage: [],
		},
		inventory: {
			items: [],
			maxSlots: 50,
			gold: 100,
		},
		quests: {
			active: [],
			completed: [],
			failed: [],
		},
		settings: {
			musicVolume: 0.7,
			sfxVolume: 0.8,
			textSpeed: 'normal',
			battleAnimations: true,
		},
		playtime: 0,
		createdAt: now,
		updatedAt: now,
	};
};
