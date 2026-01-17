import { z } from 'zod';
import { CharacterStatsSchema } from './combat.js';

/**
 * Monster elemental types
 * Based on vision docs - biome-specific creatures
 */
export const ElementTypeSchema = z.enum([
	'Fire',
	'Water',
	'Earth',
	'Air',
	'Light',
	'Shadow',
	'Nature',
	'Electric',
]);
export type ElementType = z.infer<typeof ElementTypeSchema>;

/**
 * Monster rarity tiers
 */
export const RaritySchema = z.enum(['Common', 'Uncommon', 'Rare', 'Epic', 'Legendary']);
export type Rarity = z.infer<typeof RaritySchema>;

/**
 * Monster ability definition
 */
export const MonsterAbilitySchema = z.object({
	id: z.string(),
	name: z.string(),
	description: z.string(),
	element: ElementTypeSchema,
	power: z.number().int().min(0),
	accuracy: z.number().min(0).max(1),
	cooldown: z.number().int().min(0),
});
export type MonsterAbility = z.infer<typeof MonsterAbilitySchema>;

/**
 * Monster species definition (template)
 * Used by procedural generator
 */
export const MonsterSpeciesSchema = z.object({
	id: z.string().uuid(),
	name: z.string(),
	description: z.string(),
	element: ElementTypeSchema,
	rarity: RaritySchema,
	baseStats: z.object({
		hp: z.number().int().min(1),
		attack: z.number().int().min(0),
		defense: z.number().int().min(0),
		critChance: z.number().min(0).max(1),
	}),
	abilities: z.array(z.string()), // ability IDs
	spriteKey: z.string(),
	biomes: z.array(z.string()),
});
export type MonsterSpecies = z.infer<typeof MonsterSpeciesSchema>;

/**
 * Monster instance (tamed/encountered)
 */
export const MonsterInstanceSchema = z.object({
	id: z.string().uuid(),
	speciesId: z.string(),
	nickname: z.string().optional(),
	level: z.number().int().min(1).max(100),
	experience: z.number().int().min(0),
	stats: CharacterStatsSchema,
	learnedAbilities: z.array(z.string()),
	isTamed: z.boolean(),
});
export type MonsterInstance = z.infer<typeof MonsterInstanceSchema>;

/**
 * Player's monster party
 */
export const MonsterPartySchema = z.object({
	active: z.array(MonsterInstanceSchema).max(6),
	storage: z.array(MonsterInstanceSchema),
});
export type MonsterParty = z.infer<typeof MonsterPartySchema>;
