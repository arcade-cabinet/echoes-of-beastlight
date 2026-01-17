import type {
	CombatState,
	CombatResult,
	CombatAction,
	CharacterStats,
	Status,
} from '@echoes-of-beastlight/game-core';

/**
 * CombatSystem - Turn-based combat logic
 * Ported from game/src/systems/combat.rs
 */
export class CombatSystem {
	private state: CombatState;
	private onStateChange: (state: CombatState) => void;

	constructor(
		playerParty: CharacterStats[],
		enemyParty: CharacterStats[],
		onStateChange: (state: CombatState) => void
	) {
		this.state = {
			playerParty: JSON.parse(JSON.stringify(playerParty)),
			enemyParty: JSON.parse(JSON.stringify(enemyParty)),
			currentTurn: 0,
			isPlayerTurn: true,
			battleLog: ['Battle started!'],
		};
		this.onStateChange = onStateChange;
		this.notifyStateChange();
	}

	/**
	 * Execute a player action
	 */
	executePlayerAction(
		action: CombatAction,
		actorIndex: number,
		targetIndex?: number
	): CombatResult | null {
		if (!this.state.isPlayerTurn) return null;

		const actor = this.state.playerParty[actorIndex];
		if (!actor || actor.hp <= 0) return null;

		if (actor.status === 'Stunned') {
			this.addBattleLog(`${this.getCombatantName(actor)} is stunned and cannot move!`);
			actor.status = 'Normal';
			this.advanceTurn();
			return null;
		}

		let result: CombatResult | null = null;

		switch (action) {
			case 'Attack':
				if (targetIndex !== undefined) {
					result = this.performAttack(actor, this.state.enemyParty, targetIndex);
				}
				break;
			case 'Defend':
				result = this.performDefend(actor);
				break;
			case 'Flee':
				result = this.attemptFlee();
				break;
			default:
				break;
		}

		if (result) {
			this.addBattleLog(result.message);
			this.applyStatusEffects();
			this.advanceTurn();
		}

		return result;
	}

	/**
	 * Execute enemy AI turn
	 */
	executeEnemyTurn(): CombatResult | null {
		if (this.state.isPlayerTurn) return null;

		// Find first alive enemy
		const enemyIndex = this.state.enemyParty.findIndex((e) => e.hp > 0);
		if (enemyIndex === -1) return null;

		const enemy = this.state.enemyParty[enemyIndex];

		if (enemy.status === 'Stunned') {
			this.addBattleLog(`Enemy is stunned and cannot move!`);
			enemy.status = 'Normal';
			this.advanceTurn();
			return null;
		}

		// Simple AI: attack random alive player
		const alivePlayers = this.state.playerParty
			.map((p, i) => ({ p, i }))
			.filter(({ p }) => p.hp > 0);

		if (alivePlayers.length === 0) return null;

		const targetData = alivePlayers[Math.floor(Math.random() * alivePlayers.length)];
		const result = this.performAttack(enemy, this.state.playerParty, targetData.i);

		this.addBattleLog(result.message);
		this.applyStatusEffects();
		this.advanceTurn();

		return result;
	}

	/**
	 * Perform an attack
	 */
	private performAttack(
		attacker: CharacterStats,
		targetParty: CharacterStats[],
		targetIndex: number
	): CombatResult {
		const target = targetParty[targetIndex];
		if (!target || target.hp <= 0) {
			return { damage: 0, isCritical: false, message: 'Invalid target!' };
		}

		// Calculate damage
		const isCritical = Math.random() < attacker.critChance;
		const baseDamage = Math.floor(Math.max(1, attacker.attack - target.defense / 2));
		const damage = Math.floor(isCritical ? baseDamage * 1.5 : baseDamage);

		// Apply damage
		target.hp = Math.max(0, target.hp - damage);

		// Check for status application
		let statusApplied: Status | undefined;
		if (Math.random() < 0.1) {
			statusApplied = Math.random() < 0.5 ? 'Poisoned' : 'Stunned';
			target.status = statusApplied;
		}

		const critText = isCritical ? ' Critical hit!' : '';
		const statusText = statusApplied ? ` ${target.status}!` : '';
		const message = `Attack deals ${damage} damage!${critText}${statusText}`;

		this.notifyStateChange();

		return { damage, isCritical, statusApplied, message };
	}

	/**
	 * Perform defend action
	 */
	private performDefend(actor: CharacterStats): CombatResult {
		// Temporarily boost defense (would need proper turn tracking)
		// For now, we assume the defense boost lasts until next turn start or is handled by a buff system
		// Since we don't have a buff system yet, we will just log it.
		// Real implementation needs temporary modifiers.
		// actor.defense = Math.floor(actor.defense * 1.5); <--- REMOVED MUTATION
		return {
			damage: 0,
			isCritical: false,
			message: 'Defending! Defense increased.',
		};
	}

	/**
	 * Attempt to flee from battle
	 */
	private attemptFlee(): CombatResult {
		const success = Math.random() < 0.3;
		return {
			damage: 0,
			isCritical: false,
			message: success ? 'Escaped successfully!' : 'Failed to escape!',
		};
	}

	/**
	 * Apply status effects at turn end
	 */
	private applyStatusEffects(): void {
		const POISON_DAMAGE_PERCENT = 0.05;

		const applyToParty = (party: CharacterStats[]) => {
			for (const char of party) {
				if (char.hp <= 0) continue;

				switch (char.status) {
					case 'Poisoned':
						const poisonDamage = Math.max(1, Math.floor(char.maxHp * POISON_DAMAGE_PERCENT));
						char.hp = Math.max(0, char.hp - poisonDamage);
						this.addBattleLog(`Poison deals ${poisonDamage} damage!`);
						break;
					case 'Stunned':
						// Skip turn logic would go here
						char.status = 'Normal'; // Stun wears off
						break;
				}
			}
		};

		applyToParty(this.state.playerParty);
		applyToParty(this.state.enemyParty);
		this.notifyStateChange();
	}

	/**
	 * Advance to next turn
	 */
	private advanceTurn(): void {
		this.state.currentTurn++;
		this.state.isPlayerTurn = !this.state.isPlayerTurn;
		this.notifyStateChange();
	}

	/**
	 * Add message to battle log
	 */
	private addBattleLog(message: string): void {
		this.state.battleLog.push(message);
		if (this.state.battleLog.length > 50) {
			this.state.battleLog.shift();
		}
	}

	/**
	 * Notify state change
	 */
	private notifyStateChange(): void {
		this.onStateChange(JSON.parse(JSON.stringify(this.state)));
	}

	private getCombatantName(actor: CharacterStats): string {
		// In a real system, we'd lookup the name from an ID or pass the name in.
		// For now, return a placeholder based on party index logic or similar
		return 'Combatant';
	}

	/**
	 * Check if battle is over
	 */
	isBattleOver(): { isOver: boolean; playerWon: boolean } {
		const allPlayersDead = this.state.playerParty.every((p) => p.hp <= 0);
		const allEnemiesDead = this.state.enemyParty.every((e) => e.hp <= 0);

		return {
			isOver: allPlayersDead || allEnemiesDead,
			playerWon: allEnemiesDead && !allPlayersDead,
		};
	}

	/**
	 * Get current state
	 */
	getState(): CombatState {
		return { ...this.state };
	}
}
