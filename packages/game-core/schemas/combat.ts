import { z } from 'zod';

/**
 * Combat Status Effects
 * Ported from game/src/systems/combat.rs
 */
export const StatusSchema = z.enum(['Normal', 'Poisoned', 'Stunned']);
export type Status = z.infer<typeof StatusSchema>;

/**
 * Character stats for combat
 * Ported from game/src/systems/combat.rs
 */
export const CharacterStatsSchema = z.object({
	hp: z.number().int().min(0),
	maxHp: z.number().int().min(1),
	attack: z.number().int().min(0),
	defense: z.number().int().min(0),
	critChance: z.number().min(0).max(1),
	status: StatusSchema,
});
export type CharacterStats = z.infer<typeof CharacterStatsSchema>;

/**
 * Combat action types
 */
export const CombatActionSchema = z.enum(['Attack', 'Defend', 'UseItem', 'Flee', 'UseAbility']);
export type CombatAction = z.infer<typeof CombatActionSchema>;

/**
 * Result of a combat action
 */
export const CombatResultSchema = z.object({
	damage: z.number().int().min(0),
	isCritical: z.boolean(),
	statusApplied: StatusSchema.optional(),
	message: z.string(),
});
export type CombatResult = z.infer<typeof CombatResultSchema>;

/**
 * Combat state for turn-based battles
 */
export const CombatStateSchema = z.object({
	playerParty: z.array(CharacterStatsSchema),
	enemyParty: z.array(CharacterStatsSchema),
	currentTurn: z.number().int().min(0),
	isPlayerTurn: z.boolean(),
	battleLog: z.array(z.string()),
});
export type CombatState = z.infer<typeof CombatStateSchema>;
