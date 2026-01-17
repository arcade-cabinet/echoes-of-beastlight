import { z } from 'zod';

/**
 * Combat Status Effects
 * Ported from game/src/systems/combat.rs
 */
export const StatusSchema = z.enum([
	'Normal',
	'Poisoned',
	'Stunned',
	'Burned',
	'Frozen',
	'Paralyzed',
]);
export type Status = z.infer<typeof StatusSchema>;

/**
 * Element types for abilities and damage
 */
export const ElementSchema = z.enum(['Physical', 'Fire', 'Ice', 'Lightning', 'Shadow', 'Light']);
export type Element = z.infer<typeof ElementSchema>;

/**
 * Ability definition
 */
export const AbilitySchema = z.object({
	id: z.string(),
	name: z.string(),
	element: ElementSchema,
	power: z.number().min(0),
	cost: z.number().min(0),
	accuracy: z.number().min(0).max(1),
	statusEffect: StatusSchema.optional(),
	statusChance: z.number().min(0).max(1).optional(),
	description: z.string(),
});
export type Ability = z.infer<typeof AbilitySchema>;

/**
 * Combatant - a unit in battle (player monster or enemy)
 */
export const CombatantSchema = z.object({
	id: z.string(),
	name: z.string(),
	level: z.number().int().min(1),
	hp: z.number().int(),
	maxHp: z.number().int().min(1),
	mp: z.number().int().min(0),
	maxMp: z.number().int().min(0),
	attack: z.number().int().min(0),
	defense: z.number().int().min(0),
	speed: z.number().int().min(0),
	critChance: z.number().min(0).max(1),
	critMultiplier: z.number().min(1),
	status: StatusSchema,
	statusDuration: z.number().int().min(0),
	element: ElementSchema,
	abilities: z.array(AbilitySchema),
	isPlayerOwned: z.boolean(),
});
export type Combatant = z.infer<typeof CombatantSchema>;

/**
 * Combat action types
 */
export const ActionTypeSchema = z.enum([
	'Attack',
	'Ability',
	'Defend',
	'Item',
	'Flee',
	'Switch',
	'Capture',
]);
export type ActionType = z.infer<typeof ActionTypeSchema>;

/**
 * Combat action
 */
export const CombatActionSchema = z.object({
	type: ActionTypeSchema,
	actorId: z.string(),
	targetId: z.string().optional(),
	abilityId: z.string().optional(),
	itemId: z.string().optional(),
});
export type CombatAction = z.infer<typeof CombatActionSchema>;

/**
 * Result of a single combat action
 */
export interface ActionResult {
	actorId: string;
	targetId?: string;
	damage: number;
	healing: number;
	isCritical: boolean;
	isMiss: boolean;
	statusApplied?: Status;
	statusRemoved?: Status;
	message: string;
}

/**
 * Turn order entry
 */
export interface TurnEntry {
	combatantId: string;
	speed: number;
	isPlayerOwned: boolean;
}

/**
 * Combat state
 */
export interface CombatState {
	playerParty: Combatant[];
	enemyParty: Combatant[];
	turnOrder: TurnEntry[];
	currentTurnIndex: number;
	roundNumber: number;
	battleLog: ActionResult[];
	isPlayerTurn: boolean;
	isBattleOver: boolean;
	winner: 'player' | 'enemy' | null;
}

/**
 * Element effectiveness chart
 * Returns damage multiplier
 */
const ELEMENT_EFFECTIVENESS: Record<Element, Record<Element, number>> = {
	Physical: {
		Physical: 1.0,
		Fire: 1.0,
		Ice: 1.0,
		Lightning: 1.0,
		Shadow: 1.0,
		Light: 1.0,
	},
	Fire: {
		Physical: 1.0,
		Fire: 0.5,
		Ice: 2.0,
		Lightning: 1.0,
		Shadow: 1.5,
		Light: 0.75,
	},
	Ice: {
		Physical: 1.0,
		Fire: 0.5,
		Ice: 0.5,
		Lightning: 1.0,
		Shadow: 0.75,
		Light: 1.5,
	},
	Lightning: {
		Physical: 1.0,
		Fire: 1.0,
		Ice: 1.5,
		Lightning: 0.5,
		Shadow: 1.5,
		Light: 0.75,
	},
	Shadow: {
		Physical: 1.0,
		Fire: 0.75,
		Ice: 1.5,
		Lightning: 0.75,
		Shadow: 0.5,
		Light: 2.0,
	},
	Light: {
		Physical: 1.0,
		Fire: 1.5,
		Ice: 0.75,
		Lightning: 1.5,
		Shadow: 2.0,
		Light: 0.5,
	},
};

/**
 * Status effect damage per turn
 */
const STATUS_DAMAGE: Partial<Record<Status, number>> = {
	Poisoned: 5,
	Burned: 8,
};

/**
 * Status effect combat modifiers
 */
const STATUS_MODIFIERS: Partial<
	Record<Status, { attackMod: number; defenseMod: number; speedMod: number }>
> = {
	Stunned: { attackMod: 0, defenseMod: 0.5, speedMod: 0 },
	Paralyzed: { attackMod: 0.75, defenseMod: 1.0, speedMod: 0.5 },
	Frozen: { attackMod: 0.5, defenseMod: 1.5, speedMod: 0.25 },
};

/**
 * Helper: Create a miss/fail action result
 */
function createMissResult(actorId: string, message: string, targetId?: string): ActionResult {
	return {
		actorId,
		targetId,
		damage: 0,
		healing: 0,
		isCritical: false,
		isMiss: true,
		message,
	};
}

/**
 * Helper: Calculate ability damage
 */
function calculateAbilityDamage(
	ability: Ability,
	actor: Combatant,
	target: Combatant,
): { damage: number; isCritical: boolean; effectiveness: number } {
	const effectiveness = ELEMENT_EFFECTIVENESS[ability.element][target.element];
	const attackMod = STATUS_MODIFIERS[actor.status]?.attackMod ?? 1.0;

	let baseDamage = ability.power + actor.attack * 0.5 * attackMod;
	baseDamage = baseDamage * effectiveness - target.defense * 0.3;
	if (baseDamage < 1) baseDamage = 1;

	const variance = 1 + Math.random() * 0.2;
	let damage = Math.floor(baseDamage * variance);

	const isCritical = Math.random() < actor.critChance;
	if (isCritical) {
		damage = Math.floor(damage * actor.critMultiplier);
	}

	return { damage, isCritical, effectiveness };
}

/**
 * Combat Engine - manages turn-based battles
 */
export class CombatEngine {
	private state: CombatState;

	constructor(playerParty: Combatant[], enemyParty: Combatant[]) {
		this.state = {
			playerParty: JSON.parse(JSON.stringify(playerParty)),
			enemyParty: JSON.parse(JSON.stringify(enemyParty)),
			turnOrder: [],
			currentTurnIndex: 0,
			roundNumber: 1,
			battleLog: [],
			isPlayerTurn: true,
			isBattleOver: false,
			winner: null,
		};

		this.calculateTurnOrder();
	}

	/**
	 * Calculate turn order based on speed
	 */
	private calculateTurnOrder(): void {
		const entries: TurnEntry[] = [];

		// Add all alive combatants
		for (const c of this.state.playerParty) {
			if (c.hp > 0) {
				const speedMod = STATUS_MODIFIERS[c.status]?.speedMod ?? 1.0;
				entries.push({
					combatantId: c.id,
					speed: c.speed * speedMod,
					isPlayerOwned: true,
				});
			}
		}

		for (const c of this.state.enemyParty) {
			if (c.hp > 0) {
				const speedMod = STATUS_MODIFIERS[c.status]?.speedMod ?? 1.0;
				entries.push({
					combatantId: c.id,
					speed: c.speed * speedMod,
					isPlayerOwned: false,
				});
			}
		}

		// Sort by speed descending (faster goes first)
		entries.sort((a, b) => b.speed - a.speed);

		this.state.turnOrder = entries;
		this.state.currentTurnIndex = 0;

		if (entries.length > 0) {
			this.state.isPlayerTurn = entries[0].isPlayerOwned;
		}
	}

	/**
	 * Get current combat state
	 */
	getState(): Readonly<CombatState> {
		return this.state;
	}

	/**
	 * Get the combatant whose turn it is
	 */
	getCurrentCombatant(): Combatant | null {
		if (this.state.isBattleOver || this.state.turnOrder.length === 0) {
			return null;
		}

		const entry = this.state.turnOrder[this.state.currentTurnIndex];
		return this.findCombatant(entry.combatantId);
	}

	/**
	 * Find a combatant by ID
	 */
	private findCombatant(id: string): Combatant | null {
		return (
			this.state.playerParty.find((c) => c.id === id) ||
			this.state.enemyParty.find((c) => c.id === id) ||
			null
		);
	}

	/**
	 * Execute a combat action
	 */
	executeAction(action: CombatAction): ActionResult {
		const actor = this.findCombatant(action.actorId);
		if (!actor || actor.hp <= 0) {
			return {
				actorId: action.actorId,
				damage: 0,
				healing: 0,
				isCritical: false,
				isMiss: true,
				message: 'Invalid actor',
			};
		}

		let result: ActionResult;

		switch (action.type) {
			case 'Attack':
				result = this.executeBasicAttack(actor, action.targetId);
				break;
			case 'Ability':
				result = this.executeAbility(actor, action.targetId, action.abilityId);
				break;
			case 'Defend':
				result = this.executeDefend(actor);
				break;
			case 'Flee':
				result = this.executeFlee(actor);
				break;
			case 'Capture':
				result = this.executeCapture(actor, action.targetId);
				break;
			default:
				result = {
					actorId: actor.id,
					damage: 0,
					healing: 0,
					isCritical: false,
					isMiss: false,
					message: `${actor.name} does nothing.`,
				};
		}

		this.state.battleLog.push(result);
		this.advanceTurn();
		this.checkBattleEnd();

		return result;
	}

	/**
	 * Execute basic attack
	 * Damage formula: (Attack - Defense/2) * (1 + random 0-20%) * critMultiplier * statusMod
	 */
	private executeBasicAttack(actor: Combatant, targetId?: string): ActionResult {
		const target = targetId ? this.findCombatant(targetId) : null;
		if (!target || target.hp <= 0) {
			return {
				actorId: actor.id,
				damage: 0,
				healing: 0,
				isCritical: false,
				isMiss: true,
				message: `${actor.name}'s attack missed!`,
			};
		}

		// Check if stunned (can't act)
		if (actor.status === 'Stunned') {
			return {
				actorId: actor.id,
				targetId: target.id,
				damage: 0,
				healing: 0,
				isCritical: false,
				isMiss: true,
				message: `${actor.name} is stunned and cannot act!`,
			};
		}

		// Get attack modifier from status
		const attackMod = STATUS_MODIFIERS[actor.status]?.attackMod ?? 1.0;

		// Calculate base damage
		let baseDamage = actor.attack * attackMod - target.defense / 2;
		if (baseDamage < 1) baseDamage = 1;

		// Random variance (0-20%)
		const variance = 1 + Math.random() * 0.2;
		let damage = Math.floor(baseDamage * variance);

		// Check for critical hit
		const isCritical = Math.random() < actor.critChance;
		if (isCritical) {
			damage = Math.floor(damage * actor.critMultiplier);
		}

		// Status bonus damage (from original Rust code)
		const statusDamage = STATUS_DAMAGE[target.status] ?? 0;
		damage += statusDamage;

		// Apply damage
		target.hp = Math.max(0, target.hp - damage);

		const critText = isCritical ? ' Critical hit!' : '';
		const statusText = statusDamage > 0 ? ` (${target.status} adds ${statusDamage} damage)` : '';

		return {
			actorId: actor.id,
			targetId: target.id,
			damage,
			healing: 0,
			isCritical,
			isMiss: false,
			message: `${actor.name} attacks ${target.name} for ${damage} damage!${critText}${statusText}`,
		};
	}

	/**
	 * Execute ability
	 */
	private executeAbility(actor: Combatant, targetId?: string, abilityId?: string): ActionResult {
		// Validate ability and get it
		const validation = this.validateAndGetAbility(actor, abilityId);
		if ('error' in validation) return validation.error;

		const ability = validation.ability;

		// Validate target
		const target = targetId ? this.findCombatant(targetId) : null;
		if (!target || target.hp <= 0) {
			return createMissResult(actor.id, `${ability.name} missed!`);
		}

		// Consume MP and check accuracy
		actor.mp -= ability.cost;
		if (Math.random() > ability.accuracy) {
			return createMissResult(actor.id, `${actor.name}'s ${ability.name} missed!`, target.id);
		}

		// Calculate and apply damage
		const { damage, isCritical, effectiveness } = calculateAbilityDamage(ability, actor, target);
		target.hp = Math.max(0, target.hp - damage);

		// Apply status effect
		const statusApplied = this.tryApplyStatus(ability, target);

		// Build result message
		const message = this.buildAbilityMessage(
			actor,
			target,
			ability,
			damage,
			isCritical,
			effectiveness,
			statusApplied,
		);

		return {
			actorId: actor.id,
			targetId: target.id,
			damage,
			healing: 0,
			isCritical,
			isMiss: false,
			statusApplied,
			message,
		};
	}

	/**
	 * Validate ability can be used and return it
	 */
	private validateAndGetAbility(
		actor: Combatant,
		abilityId?: string,
	): { ability: Ability } | { error: ActionResult } {
		if (!abilityId) return { error: createMissResult(actor.id, 'No ability specified') };

		const ability = actor.abilities.find((a) => a.id === abilityId);
		if (!ability) return { error: createMissResult(actor.id, 'Ability not found') };
		if (actor.mp < ability.cost) {
			return {
				error: createMissResult(
					actor.id,
					`${actor.name} doesn't have enough MP for ${ability.name}!`,
				),
			};
		}
		if (actor.status === 'Stunned') {
			return { error: createMissResult(actor.id, `${actor.name} is stunned and cannot act!`) };
		}
		return { ability };
	}

	/**
	 * Try to apply status effect from ability
	 */
	private tryApplyStatus(ability: Ability, target: Combatant): Status | undefined {
		if (ability.statusEffect && ability.statusChance && Math.random() < ability.statusChance) {
			target.status = ability.statusEffect;
			target.statusDuration = 3;
			return ability.statusEffect;
		}
		return undefined;
	}

	/**
	 * Build ability result message
	 */
	private buildAbilityMessage(
		actor: Combatant,
		target: Combatant,
		ability: Ability,
		damage: number,
		isCritical: boolean,
		effectiveness: number,
		statusApplied?: Status,
	): string {
		const effectivenessText =
			effectiveness > 1
				? " It's super effective!"
				: effectiveness < 1
					? " It's not very effective..."
					: '';
		const critText = isCritical ? ' Critical hit!' : '';
		const statusText = statusApplied ? ` ${target.name} is now ${statusApplied}!` : '';
		return `${actor.name} uses ${ability.name}! ${damage} damage to ${target.name}!${effectivenessText}${critText}${statusText}`;
	}

	/**
	 * Execute defend action
	 */
	private executeDefend(actor: Combatant): ActionResult {
		// Defending temporarily boosts defense (handled in damage calculation)
		// For now, just restore some HP
		const healing = Math.floor(actor.maxHp * 0.05);
		actor.hp = Math.min(actor.maxHp, actor.hp + healing);

		return {
			actorId: actor.id,
			damage: 0,
			healing,
			isCritical: false,
			isMiss: false,
			message: `${actor.name} defends and recovers ${healing} HP!`,
		};
	}

	/**
	 * Execute flee attempt
	 */
	private executeFlee(actor: Combatant): ActionResult {
		// Flee chance based on speed comparison
		const playerSpeed = this.state.playerParty.reduce((sum, c) => sum + c.speed, 0);
		const enemySpeed = this.state.enemyParty.reduce((sum, c) => sum + c.speed, 0);

		const fleeChance = 0.3 + (playerSpeed - enemySpeed) * 0.01;
		const success = Math.random() < fleeChance;

		if (success) {
			this.state.isBattleOver = true;
			this.state.winner = null; // Draw/escaped
			return {
				actorId: actor.id,
				damage: 0,
				healing: 0,
				isCritical: false,
				isMiss: false,
				message: 'Got away safely!',
			};
		}

		return {
			actorId: actor.id,
			damage: 0,
			healing: 0,
			isCritical: false,
			isMiss: true,
			message: "Couldn't escape!",
		};
	}

	/**
	 * Execute capture attempt (taming)
	 * Based on game/src/systems/taming.rs
	 */
	private executeCapture(actor: Combatant, targetId?: string): ActionResult {
		const target = targetId ? this.findCombatant(targetId) : null;
		if (!target || target.hp <= 0 || target.isPlayerOwned) {
			return {
				actorId: actor.id,
				damage: 0,
				healing: 0,
				isCritical: false,
				isMiss: true,
				message: 'Invalid capture target!',
			};
		}

		// Taming formula from Rust:
		// taming_chance = (health / max_health) * (level / 100)
		// Lower health = higher capture chance (inverted from original)
		const healthRatio = 1 - target.hp / target.maxHp; // Lower HP = higher chance
		const levelBonus = actor.level / 100;
		const baseChance = 0.1 + healthRatio * 0.5 + levelBonus * 0.2;

		const success = Math.random() < baseChance;

		if (success) {
			// Remove from enemy party
			const idx = this.state.enemyParty.findIndex((c) => c.id === target.id);
			if (idx >= 0) {
				const captured = this.state.enemyParty.splice(idx, 1)[0];
				captured.isPlayerOwned = true;
				// Don't add to player party mid-battle, just mark as captured
			}

			this.checkBattleEnd();

			return {
				actorId: actor.id,
				targetId: target.id,
				damage: 0,
				healing: 0,
				isCritical: false,
				isMiss: false,
				message: `${target.name} was captured! It joins your party!`,
			};
		}

		return {
			actorId: actor.id,
			targetId: target.id,
			damage: 0,
			healing: 0,
			isCritical: false,
			isMiss: true,
			message: `${target.name} broke free from the capture attempt!`,
		};
	}

	/**
	 * Advance to the next turn
	 */
	private advanceTurn(): void {
		if (this.state.isBattleOver) return;

		// Process status effects for current combatant
		const current = this.getCurrentCombatant();
		if (current && current.hp > 0) {
			this.processStatusEffects(current);
		}

		// Move to next turn
		this.state.currentTurnIndex++;

		// If we've gone through all combatants, start new round
		if (this.state.currentTurnIndex >= this.state.turnOrder.length) {
			this.state.roundNumber++;
			this.calculateTurnOrder();
		}

		// Skip dead combatants
		while (this.state.currentTurnIndex < this.state.turnOrder.length && !this.state.isBattleOver) {
			const entry = this.state.turnOrder[this.state.currentTurnIndex];
			const combatant = this.findCombatant(entry.combatantId);
			if (combatant && combatant.hp > 0) {
				this.state.isPlayerTurn = entry.isPlayerOwned;
				break;
			}
			this.state.currentTurnIndex++;
		}
	}

	/**
	 * Process status effects at end of turn
	 */
	private processStatusEffects(combatant: Combatant): void {
		if (combatant.status === 'Normal' || combatant.statusDuration <= 0) {
			return;
		}

		// Apply damage from status
		const damage = STATUS_DAMAGE[combatant.status];
		if (damage) {
			combatant.hp = Math.max(0, combatant.hp - damage);
			this.state.battleLog.push({
				actorId: combatant.id,
				damage,
				healing: 0,
				isCritical: false,
				isMiss: false,
				message: `${combatant.name} takes ${damage} damage from ${combatant.status}!`,
			});
		}

		// Reduce duration
		combatant.statusDuration--;
		if (combatant.statusDuration <= 0) {
			const oldStatus = combatant.status;
			combatant.status = 'Normal';
			this.state.battleLog.push({
				actorId: combatant.id,
				damage: 0,
				healing: 0,
				isCritical: false,
				isMiss: false,
				statusRemoved: oldStatus,
				message: `${combatant.name} recovered from ${oldStatus}!`,
			});
		}
	}

	/**
	 * Check if battle has ended
	 */
	private checkBattleEnd(): void {
		const playerAlive = this.state.playerParty.some((c) => c.hp > 0);
		const enemyAlive = this.state.enemyParty.some((c) => c.hp > 0);

		if (!playerAlive) {
			this.state.isBattleOver = true;
			this.state.winner = 'enemy';
		} else if (!enemyAlive) {
			this.state.isBattleOver = true;
			this.state.winner = 'player';
		}
	}

	/**
	 * Get AI action for enemy turn
	 */
	getAIAction(): CombatAction {
		const current = this.getCurrentCombatant();
		if (!current || current.isPlayerOwned) {
			throw new Error('Not an AI turn');
		}

		// Simple AI: attack random player party member
		const aliveTargets = this.state.playerParty.filter((c) => c.hp > 0);
		if (aliveTargets.length === 0) {
			return {
				type: 'Defend',
				actorId: current.id,
			};
		}

		// 30% chance to use ability if available and has MP
		const usableAbilities = current.abilities.filter((a) => current.mp >= a.cost);
		if (usableAbilities.length > 0 && Math.random() < 0.3) {
			const ability = usableAbilities[Math.floor(Math.random() * usableAbilities.length)];
			const target = aliveTargets[Math.floor(Math.random() * aliveTargets.length)];
			return {
				type: 'Ability',
				actorId: current.id,
				targetId: target.id,
				abilityId: ability.id,
			};
		}

		// Otherwise basic attack
		const target = aliveTargets[Math.floor(Math.random() * aliveTargets.length)];
		return {
			type: 'Attack',
			actorId: current.id,
			targetId: target.id,
		};
	}
}

/**
 * Create a test combatant for debugging
 */
export function createTestCombatant(
	id: string,
	name: string,
	level: number,
	isPlayerOwned: boolean,
): Combatant {
	const baseHp = 50 + level * 10;
	const baseMp = 20 + level * 5;

	return {
		id,
		name,
		level,
		hp: baseHp,
		maxHp: baseHp,
		mp: baseMp,
		maxMp: baseMp,
		attack: 10 + level * 2,
		defense: 8 + Math.floor(level * 1.5),
		speed: 10 + Math.floor(Math.random() * 5),
		critChance: 0.1 + level * 0.005,
		critMultiplier: 1.5,
		status: 'Normal',
		statusDuration: 0,
		element: 'Physical',
		abilities: [
			{
				id: `${id}_tackle`,
				name: 'Tackle',
				element: 'Physical',
				power: 20 + level * 2,
				cost: 5,
				accuracy: 0.95,
				description: 'A basic physical attack',
			},
		],
		isPlayerOwned,
	};
}
