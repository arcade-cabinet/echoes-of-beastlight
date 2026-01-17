import type {
	BiomeType,
	ElementType,
	MonsterInstance,
	MonsterSpecies,
	Rarity,
} from '../schemas/index.js';

/**
 * Monster generation parameters
 */
export interface MonsterGenParams {
	biome: BiomeType;
	minLevel: number;
	maxLevel: number;
	rarityWeights?: Partial<Record<Rarity, number>>;
}

/**
 * Default rarity weights for monster generation
 */
const DEFAULT_RARITY_WEIGHTS: Record<Rarity, number> = {
	Common: 0.5,
	Uncommon: 0.3,
	Rare: 0.15,
	Epic: 0.04,
	Legendary: 0.01,
};

/**
 * Biome to element type mapping
 */
const BIOME_ELEMENTS: Record<BiomeType, ElementType[]> = {
	Forest: ['Nature', 'Earth', 'Air'],
	Desert: ['Fire', 'Earth', 'Light'],
	Tundra: ['Water', 'Air', 'Light'],
	Swamp: ['Water', 'Nature', 'Shadow'],
	Mountains: ['Earth', 'Air', 'Electric'],
	Plains: ['Nature', 'Air', 'Light'],
	Volcanic: ['Fire', 'Earth', 'Shadow'],
	Ocean: ['Water', 'Electric', 'Shadow'],
	Cave: ['Earth', 'Shadow', 'Fire'],
	Ruins: ['Shadow', 'Light', 'Electric'],
};

/**
 * Select a random element from array
 */
function randomChoice<T>(arr: T[]): T {
	return arr[Math.floor(Math.random() * arr.length)];
}

/**
 * Select rarity based on weights
 */
function selectRarity(weights: Record<Rarity, number>): Rarity {
	const total = Object.values(weights).reduce((a, b) => a + b, 0);
	let rand = Math.random() * total;

	for (const [rarity, weight] of Object.entries(weights) as [Rarity, number][]) {
		rand -= weight;
		if (rand <= 0) return rarity;
	}

	return 'Common';
}

/**
 * Calculate base stats for a rarity
 */
function getBaseStatsForRarity(rarity: Rarity): MonsterSpecies['baseStats'] {
	const multipliers: Record<Rarity, number> = {
		Common: 1.0,
		Uncommon: 1.2,
		Rare: 1.5,
		Epic: 1.8,
		Legendary: 2.2,
	};

	const mult = multipliers[rarity];
	return {
		hp: Math.floor(50 * mult),
		attack: Math.floor(10 * mult),
		defense: Math.floor(8 * mult),
		critChance: Math.min(0.05 * mult, 0.25),
	};
}

/**
 * Generate a random monster species for a biome
 */
export function generateMonsterSpecies(biome: BiomeType, rarity?: Rarity): MonsterSpecies {
	const selectedRarity = rarity ?? selectRarity(DEFAULT_RARITY_WEIGHTS);
	const element = randomChoice(BIOME_ELEMENTS[biome]);
	const baseStats = getBaseStatsForRarity(selectedRarity);

	// Generate unique ID
	const id = crypto.randomUUID();

	return {
		id,
		name: `${element} ${biome} Creature`, // Placeholder - would use name generator
		description: `A ${selectedRarity.toLowerCase()} ${element.toLowerCase()}-type creature found in ${biome.toLowerCase()} regions.`,
		element,
		rarity: selectedRarity,
		baseStats,
		abilities: [], // Would populate from ability pool
		spriteKey: `monster-${biome.toLowerCase()}-${element.toLowerCase()}`,
		biomes: [biome],
	};
}

/**
 * Create a monster instance from a species
 */
export function createMonsterInstance(
	species: MonsterSpecies,
	level: number,
	isTamed = false,
): MonsterInstance {
	// Scale stats by level
	const levelMult = 1 + (level - 1) * 0.1;

	return {
		id: crypto.randomUUID(),
		speciesId: species.id,
		level,
		experience: 0,
		stats: {
			hp: Math.floor(species.baseStats.hp * levelMult),
			maxHp: Math.floor(species.baseStats.hp * levelMult),
			attack: Math.floor(species.baseStats.attack * levelMult),
			defense: Math.floor(species.baseStats.defense * levelMult),
			critChance: Math.min(species.baseStats.critChance + level * 0.002, 0.5),
			status: 'Normal',
		},
		learnedAbilities: species.abilities.slice(0, Math.min(level, 4)),
		isTamed,
	};
}

/**
 * Generate a wild monster encounter
 */
export function generateWildEncounter(params: MonsterGenParams): MonsterInstance {
	const weights = { ...DEFAULT_RARITY_WEIGHTS, ...params.rarityWeights };
	const rarity = selectRarity(weights);
	const species = generateMonsterSpecies(params.biome, rarity);
	const level =
		params.minLevel + Math.floor(Math.random() * (params.maxLevel - params.minLevel + 1));

	return createMonsterInstance(species, level, false);
}
