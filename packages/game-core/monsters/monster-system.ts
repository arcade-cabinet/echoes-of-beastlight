import { z } from 'zod';
import type { Ability, Combatant, Element } from '../combat/combat-engine';

/**
 * Monster Rarity
 */
export const RaritySchema = z.enum(['Common', 'Uncommon', 'Rare', 'Epic', 'Legendary']);
export type Rarity = z.infer<typeof RaritySchema>;

/**
 * Evolution requirement types
 */
export const EvolutionRequirementTypeSchema = z.enum([
	'Level',
	'Item',
	'Friendship',
	'Location',
	'TimeOfDay',
	'Combat',
]);
export type EvolutionRequirementType = z.infer<typeof EvolutionRequirementTypeSchema>;

/**
 * Evolution requirement
 */
export const EvolutionRequirementSchema = z.object({
	type: EvolutionRequirementTypeSchema,
	value: z.union([z.string(), z.number()]),
	description: z.string(),
});
export type EvolutionRequirement = z.infer<typeof EvolutionRequirementSchema>;

/**
 * Evolution path
 */
export const EvolutionPathSchema = z.object({
	targetSpeciesId: z.string(),
	requirements: z.array(EvolutionRequirementSchema),
});
export type EvolutionPath = z.infer<typeof EvolutionPathSchema>;

/**
 * Biome type for spawn locations
 * Re-exported from schemas for consistency
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
 * Monster species definition (template for monsters)
 */
export interface MonsterSpecies {
	id: string;
	name: string;
	description: string;
	element: Element;
	rarity: Rarity;
	baseStats: {
		hp: number;
		mp: number;
		attack: number;
		defense: number;
		speed: number;
		critChance: number;
	};
	statGrowth: {
		hp: number;
		mp: number;
		attack: number;
		defense: number;
		speed: number;
	};
	learnableAbilities: Array<{
		ability: Ability;
		learnLevel: number;
	}>;
	evolutionPaths: EvolutionPath[];
	spawnBiomes: BiomeType[];
	spawnChance: number;
	tamingDifficulty: number; // 0.0 - 1.0, higher = harder
	baseExpYield: number;
}

/**
 * Monster instance (actual monster owned by player or encountered)
 */
export interface MonsterInstance {
	id: string;
	speciesId: string;
	nickname?: string;
	level: number;
	experience: number;
	expToNextLevel: number;
	stats: {
		hp: number;
		maxHp: number;
		mp: number;
		maxMp: number;
		attack: number;
		defense: number;
		speed: number;
		critChance: number;
	};
	knownAbilities: Ability[];
	friendship: number; // 0-255
	capturedAt: string; // ISO date
	originalTrainer: string;
	isShiny: boolean; // Rare variant
	natureModifier: NatureModifier;
}

/**
 * Nature affects stat growth
 */
export interface NatureModifier {
	name: string;
	increasedStat: 'attack' | 'defense' | 'speed' | 'hp' | 'mp' | null;
	decreasedStat: 'attack' | 'defense' | 'speed' | 'hp' | 'mp' | null;
}

/**
 * Available natures
 */
export const NATURES: NatureModifier[] = [
	{ name: 'Hardy', increasedStat: null, decreasedStat: null },
	{ name: 'Brave', increasedStat: 'attack', decreasedStat: 'speed' },
	{ name: 'Bold', increasedStat: 'defense', decreasedStat: 'attack' },
	{ name: 'Timid', increasedStat: 'speed', decreasedStat: 'attack' },
	{ name: 'Modest', increasedStat: 'mp', decreasedStat: 'attack' },
	{ name: 'Careful', increasedStat: 'defense', decreasedStat: 'mp' },
	{ name: 'Adamant', increasedStat: 'attack', decreasedStat: 'mp' },
	{ name: 'Jolly', increasedStat: 'speed', decreasedStat: 'mp' },
	{ name: 'Quiet', increasedStat: 'mp', decreasedStat: 'speed' },
	{ name: 'Lax', increasedStat: 'defense', decreasedStat: 'speed' },
];

/**
 * Experience required for each level
 */
export function getExpForLevel(level: number): number {
	// Cubic growth formula
	return Math.floor((4 * level ** 3) / 5);
}

/**
 * Calculate stats for a monster at a given level
 */
export function calculateStats(
	species: MonsterSpecies,
	level: number,
	nature: NatureModifier,
): MonsterInstance['stats'] {
	const getNatureMultiplier = (stat: 'attack' | 'defense' | 'speed' | 'hp' | 'mp'): number => {
		if (nature.increasedStat === stat) return 1.1;
		if (nature.decreasedStat === stat) return 0.9;
		return 1.0;
	};

	const hp = Math.floor(
		(species.baseStats.hp + species.statGrowth.hp * level) * getNatureMultiplier('hp'),
	);
	const mp = Math.floor(
		(species.baseStats.mp + species.statGrowth.mp * level) * getNatureMultiplier('mp'),
	);
	const attack = Math.floor(
		(species.baseStats.attack + species.statGrowth.attack * level) * getNatureMultiplier('attack'),
	);
	const defense = Math.floor(
		(species.baseStats.defense + species.statGrowth.defense * level) *
			getNatureMultiplier('defense'),
	);
	const speed = Math.floor(
		(species.baseStats.speed + species.statGrowth.speed * level) * getNatureMultiplier('speed'),
	);

	return {
		hp,
		maxHp: hp,
		mp,
		maxMp: mp,
		attack,
		defense,
		speed,
		critChance: species.baseStats.critChance + level * 0.002,
	};
}

/**
 * Create a new monster instance from species
 */
export function createMonsterInstance(
	species: MonsterSpecies,
	level: number,
	trainer: string,
	isWild: boolean = true,
): MonsterInstance {
	const nature = NATURES[Math.floor(Math.random() * NATURES.length)];
	const stats = calculateStats(species, level, nature);

	// Determine which abilities the monster knows based on level
	const knownAbilities = species.learnableAbilities
		.filter((la) => la.learnLevel <= level)
		.map((la) => la.ability)
		.slice(-4); // Max 4 abilities

	// Shiny chance (1/512)
	const isShiny = Math.random() < 1 / 512;

	return {
		id: `monster_${Date.now()}_${Math.random().toString(36).slice(2, 9)}`,
		speciesId: species.id,
		level,
		experience: getExpForLevel(level),
		expToNextLevel: getExpForLevel(level + 1),
		stats,
		knownAbilities,
		friendship: isWild ? 50 : 70, // Wild monsters start with lower friendship
		capturedAt: new Date().toISOString(),
		originalTrainer: trainer,
		isShiny,
		natureModifier: nature,
	};
}

/**
 * Add experience to a monster and handle leveling
 */
export function addExperience(
	monster: MonsterInstance,
	species: MonsterSpecies,
	exp: number,
): { leveledUp: boolean; newAbilities: Ability[]; canEvolve: boolean } {
	const newAbilities: Ability[] = [];
	let leveledUp = false;

	monster.experience += exp;

	// Level up loop
	while (monster.experience >= monster.expToNextLevel && monster.level < 100) {
		monster.level++;
		leveledUp = true;

		// Recalculate stats
		const newStats = calculateStats(species, monster.level, monster.natureModifier);
		const hpDiff = newStats.maxHp - monster.stats.maxHp;
		const mpDiff = newStats.maxMp - monster.stats.maxMp;

		monster.stats = {
			...newStats,
			hp: monster.stats.hp + hpDiff, // Preserve current HP ratio
			mp: monster.stats.mp + mpDiff,
		};

		// Learn new abilities
		const newAbility = species.learnableAbilities.find((la) => la.learnLevel === monster.level);
		if (newAbility) {
			if (monster.knownAbilities.length < 4) {
				monster.knownAbilities.push(newAbility.ability);
			}
			newAbilities.push(newAbility.ability);
		}

		monster.expToNextLevel = getExpForLevel(monster.level + 1);
	}

	// Check evolution conditions
	const canEvolve = species.evolutionPaths.some((path) =>
		path.requirements.every((req) => {
			if (req.type === 'Level') {
				return monster.level >= (req.value as number);
			}
			if (req.type === 'Friendship') {
				return monster.friendship >= (req.value as number);
			}
			// Other requirements need external state
			return false;
		}),
	);

	return { leveledUp, newAbilities, canEvolve };
}

/**
 * Evolve a monster
 */
export function evolveMonster(
	monster: MonsterInstance,
	_currentSpecies: MonsterSpecies,
	targetSpecies: MonsterSpecies,
): MonsterInstance {
	const evolvedStats = calculateStats(targetSpecies, monster.level, monster.natureModifier);

	// Transfer HP/MP ratios
	const hpRatio = monster.stats.hp / monster.stats.maxHp;
	const mpRatio = monster.stats.mp / monster.stats.maxMp;

	// Keep abilities but add any new ones from evolution
	const newAbilities = targetSpecies.learnableAbilities
		.filter((la) => la.learnLevel <= monster.level)
		.map((la) => la.ability);

	// Merge abilities, keeping old ones and adding new
	const combinedAbilities = [...monster.knownAbilities];
	for (const ability of newAbilities) {
		if (!combinedAbilities.find((a) => a.id === ability.id)) {
			combinedAbilities.push(ability);
		}
	}

	return {
		...monster,
		speciesId: targetSpecies.id,
		stats: {
			...evolvedStats,
			hp: Math.floor(evolvedStats.maxHp * hpRatio),
			mp: Math.floor(evolvedStats.maxMp * mpRatio),
		},
		knownAbilities: combinedAbilities.slice(-4),
		friendship: Math.min(255, monster.friendship + 20), // Evolution boosts friendship
	};
}

/**
 * Calculate taming success chance
 * Based on game/src/systems/taming.rs
 */
export function calculateTamingChance(
	monster: MonsterInstance,
	species: MonsterSpecies,
	playerLevel: number,
	baitBonus: number = 0,
): number {
	// Lower HP = higher catch rate
	const hpRatio = 1 - monster.stats.hp / monster.stats.maxHp;

	// Level difference modifier
	const levelMod = Math.min(1, playerLevel / monster.level);

	// Base formula from Rust
	const baseChance = hpRatio * levelMod * (1 - species.tamingDifficulty * 0.5);

	// Apply bait bonus
	const finalChance = Math.min(0.95, baseChance + baitBonus);

	return Math.max(0.05, finalChance); // Minimum 5% chance, max 95%
}

/**
 * Convert monster instance to combat-ready Combatant
 */
export function monsterToCombatant(
	monster: MonsterInstance,
	species: MonsterSpecies,
	isPlayerOwned: boolean,
): Combatant {
	return {
		id: monster.id,
		name: monster.nickname || species.name,
		level: monster.level,
		hp: monster.stats.hp,
		maxHp: monster.stats.maxHp,
		mp: monster.stats.mp,
		maxMp: monster.stats.maxMp,
		attack: monster.stats.attack,
		defense: monster.stats.defense,
		speed: monster.stats.speed,
		critChance: monster.stats.critChance,
		critMultiplier: monster.isShiny ? 1.75 : 1.5, // Shiny bonus
		status: 'Normal',
		statusDuration: 0,
		element: species.element,
		abilities: monster.knownAbilities,
		isPlayerOwned,
	};
}

/**
 * Monster party management
 */
export class MonsterParty {
	private members: MonsterInstance[] = [];
	readonly maxSize = 6;

	constructor(initialMembers: MonsterInstance[] = []) {
		this.members = initialMembers.slice(0, this.maxSize);
	}

	get size(): number {
		return this.members.length;
	}

	get isFull(): boolean {
		return this.members.length >= this.maxSize;
	}

	getMembers(): readonly MonsterInstance[] {
		return this.members;
	}

	getActiveMember(): MonsterInstance | null {
		return this.members.find((m) => m.stats.hp > 0) || null;
	}

	add(monster: MonsterInstance): boolean {
		if (this.isFull) return false;
		this.members.push(monster);
		return true;
	}

	remove(monsterId: string): MonsterInstance | null {
		const idx = this.members.findIndex((m) => m.id === monsterId);
		if (idx < 0) return null;
		return this.members.splice(idx, 1)[0];
	}

	swap(index1: number, index2: number): boolean {
		if (
			index1 < 0 ||
			index1 >= this.members.length ||
			index2 < 0 ||
			index2 >= this.members.length
		) {
			return false;
		}
		[this.members[index1], this.members[index2]] = [this.members[index2], this.members[index1]];
		return true;
	}

	healAll(): void {
		for (const monster of this.members) {
			monster.stats.hp = monster.stats.maxHp;
			monster.stats.mp = monster.stats.maxMp;
		}
	}

	hasAliveMembers(): boolean {
		return this.members.some((m) => m.stats.hp > 0);
	}
}

// ========================================
// STARTER MONSTER SPECIES DEFINITIONS
// ========================================

export const STARTER_SPECIES: MonsterSpecies[] = [
	{
		id: 'emberpup',
		name: 'Emberpup',
		description: 'A fiery canine with glowing embers in its fur. Loyal and eager to fight.',
		element: 'Fire',
		rarity: 'Uncommon',
		baseStats: {
			hp: 45,
			mp: 25,
			attack: 15,
			defense: 10,
			speed: 14,
			critChance: 0.1,
		},
		statGrowth: { hp: 8, mp: 4, attack: 3, defense: 2, speed: 3 },
		learnableAbilities: [
			{
				ability: {
					id: 'ember_bite',
					name: 'Ember Bite',
					element: 'Fire',
					power: 25,
					cost: 5,
					accuracy: 0.95,
					statusEffect: 'Burned',
					statusChance: 0.1,
					description: 'A fiery bite that may burn the target.',
				},
				learnLevel: 1,
			},
			{
				ability: {
					id: 'flame_dash',
					name: 'Flame Dash',
					element: 'Fire',
					power: 40,
					cost: 10,
					accuracy: 0.9,
					description: 'Charges at the enemy in a blaze of fire.',
				},
				learnLevel: 8,
			},
			{
				ability: {
					id: 'inferno_howl',
					name: 'Inferno Howl',
					element: 'Fire',
					power: 65,
					cost: 20,
					accuracy: 0.85,
					statusEffect: 'Burned',
					statusChance: 0.3,
					description: 'A devastating howl that engulfs enemies in flames.',
				},
				learnLevel: 16,
			},
		],
		evolutionPaths: [
			{
				targetSpeciesId: 'blazewolf',
				requirements: [{ type: 'Level', value: 20, description: 'Reach level 20' }],
			},
		],
		spawnBiomes: ['Forest', 'Mountains'],
		spawnChance: 0.15,
		tamingDifficulty: 0.3,
		baseExpYield: 65,
	},
	{
		id: 'aquafin',
		name: 'Aquafin',
		description: 'A playful water sprite that glides through rivers and streams.',
		element: 'Ice',
		rarity: 'Uncommon',
		baseStats: {
			hp: 50,
			mp: 30,
			attack: 12,
			defense: 12,
			speed: 12,
			critChance: 0.08,
		},
		statGrowth: { hp: 9, mp: 5, attack: 2, defense: 3, speed: 2 },
		learnableAbilities: [
			{
				ability: {
					id: 'frost_spray',
					name: 'Frost Spray',
					element: 'Ice',
					power: 22,
					cost: 5,
					accuracy: 1.0,
					description: 'A gentle spray of icy water.',
				},
				learnLevel: 1,
			},
			{
				ability: {
					id: 'ice_shard',
					name: 'Ice Shard',
					element: 'Ice',
					power: 35,
					cost: 8,
					accuracy: 0.95,
					statusEffect: 'Frozen',
					statusChance: 0.15,
					description: 'Hurls sharp ice crystals at the enemy.',
				},
				learnLevel: 7,
			},
			{
				ability: {
					id: 'blizzard_wave',
					name: 'Blizzard Wave',
					element: 'Ice',
					power: 60,
					cost: 18,
					accuracy: 0.85,
					statusEffect: 'Frozen',
					statusChance: 0.25,
					description: 'Summons a freezing wave of snow and ice.',
				},
				learnLevel: 15,
			},
		],
		evolutionPaths: [
			{
				targetSpeciesId: 'glacialord',
				requirements: [{ type: 'Level', value: 22, description: 'Reach level 22' }],
			},
		],
		spawnBiomes: ['Tundra', 'Ocean', 'Cave'],
		spawnChance: 0.12,
		tamingDifficulty: 0.35,
		baseExpYield: 70,
	},
	{
		id: 'voltkit',
		name: 'Voltkit',
		description: 'A mischievous electric creature that crackles with static energy.',
		element: 'Lightning',
		rarity: 'Uncommon',
		baseStats: {
			hp: 40,
			mp: 28,
			attack: 14,
			defense: 8,
			speed: 18,
			critChance: 0.12,
		},
		statGrowth: { hp: 7, mp: 5, attack: 3, defense: 1, speed: 4 },
		learnableAbilities: [
			{
				ability: {
					id: 'spark_touch',
					name: 'Spark Touch',
					element: 'Lightning',
					power: 20,
					cost: 4,
					accuracy: 1.0,
					statusEffect: 'Paralyzed',
					statusChance: 0.1,
					description: 'A quick zap that may paralyze.',
				},
				learnLevel: 1,
			},
			{
				ability: {
					id: 'thunder_bolt',
					name: 'Thunder Bolt',
					element: 'Lightning',
					power: 45,
					cost: 12,
					accuracy: 0.9,
					statusEffect: 'Paralyzed',
					statusChance: 0.2,
					description: 'Calls down lightning from above.',
				},
				learnLevel: 10,
			},
			{
				ability: {
					id: 'storm_surge',
					name: 'Storm Surge',
					element: 'Lightning',
					power: 70,
					cost: 22,
					accuracy: 0.8,
					statusEffect: 'Paralyzed',
					statusChance: 0.35,
					description: 'Unleashes the fury of a thunderstorm.',
				},
				learnLevel: 18,
			},
		],
		evolutionPaths: [
			{
				targetSpeciesId: 'tempestfox',
				requirements: [{ type: 'Level', value: 24, description: 'Reach level 24' }],
			},
		],
		spawnBiomes: ['Plains', 'Mountains'],
		spawnChance: 0.1,
		tamingDifficulty: 0.4,
		baseExpYield: 75,
	},
];

/**
 * Get species by ID
 */
export function getSpeciesById(id: string): MonsterSpecies | undefined {
	return STARTER_SPECIES.find((s) => s.id === id);
}

/**
 * Get species that spawn in a biome
 */
export function getSpeciesForBiome(biome: BiomeType): MonsterSpecies[] {
	return STARTER_SPECIES.filter((s) => s.spawnBiomes.includes(biome));
}
