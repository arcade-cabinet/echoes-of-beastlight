import { z } from 'zod';

/**
 * Biome types for procedural world generation
 * 32x32 pixel art with limited color palettes per biome
 */
export const BiomeTypeSchema = z.enum([
	'Forest',
	'Desert',
	'Tundra',
	'Swamp',
	'Mountains',
	'Plains',
	'Volcanic',
	'Ocean',
	'Cave',
	'Ruins',
]);
export type BiomeType = z.infer<typeof BiomeTypeSchema>;

/**
 * Tile types for tilemap system
 */
export const TileTypeSchema = z.enum([
	'Ground',
	'Water',
	'Wall',
	'Bridge',
	'Door',
	'Stairs',
	'Chest',
	'NPC',
	'Spawn',
	'Exit',
]);
export type TileType = z.infer<typeof TileTypeSchema>;

/**
 * Single tile in the world
 */
export const TileSchema = z.object({
	x: z.number().int(),
	y: z.number().int(),
	type: TileTypeSchema,
	biome: BiomeTypeSchema,
	isWalkable: z.boolean(),
	isInteractable: z.boolean(),
	spriteKey: z.string(),
	metadata: z.record(z.unknown()).optional(),
});
export type Tile = z.infer<typeof TileSchema>;

/**
 * Area/Zone definition
 */
export const AreaSchema = z.object({
	id: z.string(),
	name: z.string(),
	description: z.string(),
	biome: BiomeTypeSchema,
	width: z.number().int().min(1),
	height: z.number().int().min(1),
	tiles: z.array(TileSchema),
	monsterSpawnTable: z.array(
		z.object({
			speciesId: z.string(),
			weight: z.number().min(0).max(1),
			minLevel: z.number().int().min(1),
			maxLevel: z.number().int().min(1),
		})
	),
	connections: z.array(
		z.object({
			targetAreaId: z.string(),
			exitTileX: z.number().int(),
			exitTileY: z.number().int(),
			entryTileX: z.number().int(),
			entryTileY: z.number().int(),
		})
	),
	npcs: z.array(z.string()), // NPC IDs
	isUnlocked: z.boolean().default(false),
});
export type Area = z.infer<typeof AreaSchema>;

/**
 * World map containing all areas
 */
export const WorldMapSchema = z.object({
	areas: z.record(AreaSchema),
	currentAreaId: z.string(),
	discoveredAreas: z.array(z.string()),
});
export type WorldMap = z.infer<typeof WorldMapSchema>;
