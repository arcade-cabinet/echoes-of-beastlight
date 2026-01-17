import { z } from 'zod';
import type { BiomeType } from '../monsters/monster-system';

/**
 * Quest objective types
 */
export const ObjectiveTypeSchema = z.enum([
	'DefeatMonsters',
	'CaptureMonster',
	'CollectItems',
	'ExploreArea',
	'TalkToNPC',
	'DeliverItem',
	'ReachLevel',
	'WinBattles',
	'EvolveMaster',
	'DiscoverSpecies',
]);
export type ObjectiveType = z.infer<typeof ObjectiveTypeSchema>;

/**
 * Quest difficulty
 */
export const QuestDifficultySchema = z.enum(['Easy', 'Normal', 'Hard', 'Expert', 'Legendary']);
export type QuestDifficulty = z.infer<typeof QuestDifficultySchema>;

/**
 * Quest status
 */
export const QuestStatusSchema = z.enum(['Available', 'Active', 'Completed', 'Failed', 'Expired']);
export type QuestStatus = z.infer<typeof QuestStatusSchema>;

/**
 * Reward types
 */
export interface QuestReward {
	type: 'Gold' | 'Experience' | 'Item' | 'Monster' | 'Badge';
	amount?: number;
	itemId?: string;
	monsterId?: string;
}

/**
 * Quest objective
 */
export interface QuestObjective {
	id: string;
	type: ObjectiveType;
	description: string;
	target: string; // monster species ID, item ID, area ID, etc.
	targetName: string;
	requiredAmount: number;
	currentAmount: number;
	isComplete: boolean;
	isHidden: boolean; // Reveals when previous objectives complete
}

/**
 * Quest definition
 */
export interface Quest {
	id: string;
	title: string;
	description: string;
	difficulty: QuestDifficulty;
	objectives: QuestObjective[];
	rewards: QuestReward[];
	status: QuestStatus;
	giver?: string; // NPC who gives the quest
	requiredLevel: number;
	recommendedLevel: number;
	timeLimit?: number; // In game-hours, optional
	biome?: BiomeType; // Quest location
	isMainStory: boolean;
	isRepeatable: boolean;
	prerequisiteQuestIds: string[];
	acceptedAt?: string;
	completedAt?: string;
}

/**
 * Quest journal (player's quest log)
 */
export interface QuestJournal {
	activeQuests: Quest[];
	completedQuests: Quest[];
	failedQuests: Quest[];
	availableQuests: Quest[];
}

/**
 * Quest generator configuration
 */
export interface QuestGeneratorConfig {
	playerLevel: number;
	currentBiome: BiomeType;
	completedQuestCount: number;
	discoveredSpecies: string[];
	seed?: number;
}

// ========================================
// PROCEDURAL QUEST GENERATION
// ========================================

/**
 * Quest templates for procedural generation
 */
const QUEST_TEMPLATES: Array<{
	titleTemplate: string;
	descriptionTemplate: string;
	objectiveType: ObjectiveType;
	difficultyRange: [QuestDifficulty, QuestDifficulty];
}> = [
	{
		titleTemplate: 'Hunt the {TARGET}',
		descriptionTemplate:
			'Wild {TARGET}s have been causing trouble in the area. Defeat {AMOUNT} of them to restore peace.',
		objectiveType: 'DefeatMonsters',
		difficultyRange: ['Easy', 'Hard'],
	},
	{
		titleTemplate: 'Capture a {TARGET}',
		descriptionTemplate:
			'A researcher needs a live {TARGET} for study. Capture one and bring it back.',
		objectiveType: 'CaptureMonster',
		difficultyRange: ['Normal', 'Expert'],
	},
	{
		titleTemplate: 'Gathering: {TARGET}',
		descriptionTemplate: 'Collect {AMOUNT} {TARGET}s for the village supplies.',
		objectiveType: 'CollectItems',
		difficultyRange: ['Easy', 'Normal'],
	},
	{
		titleTemplate: 'Explore the {TARGET}',
		descriptionTemplate:
			'An uncharted area has been discovered. Explore the {TARGET} and report back.',
		objectiveType: 'ExploreArea',
		difficultyRange: ['Normal', 'Hard'],
	},
	{
		titleTemplate: 'Message for {TARGET}',
		descriptionTemplate: 'Deliver an important message to {TARGET} in the next town.',
		objectiveType: 'TalkToNPC',
		difficultyRange: ['Easy', 'Easy'],
	},
	{
		titleTemplate: 'Battle Challenge',
		descriptionTemplate: 'Prove your strength by winning {AMOUNT} battles against wild monsters.',
		objectiveType: 'WinBattles',
		difficultyRange: ['Normal', 'Expert'],
	},
	{
		titleTemplate: 'Species Discovery',
		descriptionTemplate:
			'The Beastlight Society is cataloging monsters. Discover {AMOUNT} new species.',
		objectiveType: 'DiscoverSpecies',
		difficultyRange: ['Hard', 'Legendary'],
	},
];

/**
 * Target names by category
 */
const TARGET_NAMES = {
	monsters: [
		'Emberpup',
		'Aquafin',
		'Voltkit',
		'Shadow Wisp',
		'Stone Golem',
		'Vine Serpent',
		'Frost Drake',
		'Thunder Hawk',
	],
	items: [
		'Healing Herb',
		'Crystal Shard',
		'Monster Essence',
		'Ancient Relic',
		'Elemental Core',
		'Rare Mushroom',
		'Dragon Scale',
		'Mystic Feather',
	],
	areas: [
		'Forgotten Ruins',
		'Crystal Cavern',
		'Ancient Forest',
		'Volcanic Peak',
		'Frozen Tundra',
		'Shadowy Marsh',
		'Floating Islands',
		'Desert Temple',
	],
	npcs: [
		'Elder Marcus',
		'Professor Willow',
		'Captain Stone',
		'Merchant Luna',
		'Scholar Thorne',
		'Healer Rosa',
		'Ranger Fox',
		'Master Chen',
	],
};

/**
 * Difficulty scaling
 */
const DIFFICULTY_SCALING: Record<
	QuestDifficulty,
	{
		amountMultiplier: number;
		levelOffset: number;
		goldReward: number;
		expReward: number;
	}
> = {
	Easy: { amountMultiplier: 1, levelOffset: -2, goldReward: 50, expReward: 100 },
	Normal: {
		amountMultiplier: 1.5,
		levelOffset: 0,
		goldReward: 100,
		expReward: 200,
	},
	Hard: {
		amountMultiplier: 2,
		levelOffset: 2,
		goldReward: 200,
		expReward: 400,
	},
	Expert: {
		amountMultiplier: 2.5,
		levelOffset: 5,
		goldReward: 400,
		expReward: 800,
	},
	Legendary: {
		amountMultiplier: 3,
		levelOffset: 10,
		goldReward: 1000,
		expReward: 2000,
	},
};

/**
 * Simple seeded random number generator
 */
class SeededRandom {
	private seed: number;

	constructor(seed: number) {
		this.seed = seed;
	}

	next(): number {
		this.seed = (this.seed * 1103515245 + 12345) & 0x7fffffff;
		return this.seed / 0x7fffffff;
	}

	nextInt(max: number): number {
		return Math.floor(this.next() * max);
	}

	pick<T>(array: T[]): T {
		return array[this.nextInt(array.length)];
	}
}

/**
 * Generate a procedural quest
 */
export function generateQuest(config: QuestGeneratorConfig): Quest {
	const seed = config.seed ?? Date.now();
	const rng = new SeededRandom(seed);

	// Pick a random template
	const template = rng.pick(QUEST_TEMPLATES);

	// Determine difficulty based on player level and quest count
	const difficulties: QuestDifficulty[] = ['Easy', 'Normal', 'Hard', 'Expert', 'Legendary'];
	const minDiffIdx = difficulties.indexOf(template.difficultyRange[0]);
	const maxDiffIdx = difficulties.indexOf(template.difficultyRange[1]);
	const difficultyIdx = Math.min(
		maxDiffIdx,
		minDiffIdx + Math.floor(config.playerLevel / 10) + rng.nextInt(2),
	);
	const difficulty = difficulties[difficultyIdx];
	const scaling = DIFFICULTY_SCALING[difficulty];

	// Pick target based on objective type
	let target: string;
	let targetName: string;

	switch (template.objectiveType) {
		case 'DefeatMonsters':
		case 'CaptureMonster':
			targetName = rng.pick(TARGET_NAMES.monsters);
			target = targetName.toLowerCase().replace(/\s+/g, '_');
			break;
		case 'CollectItems':
		case 'DeliverItem':
			targetName = rng.pick(TARGET_NAMES.items);
			target = targetName.toLowerCase().replace(/\s+/g, '_');
			break;
		case 'ExploreArea':
			targetName = rng.pick(TARGET_NAMES.areas);
			target = targetName.toLowerCase().replace(/\s+/g, '_');
			break;
		case 'TalkToNPC':
			targetName = rng.pick(TARGET_NAMES.npcs);
			target = targetName.toLowerCase().replace(/\s+/g, '_');
			break;
		default:
			targetName = 'Unknown';
			target = 'unknown';
	}

	// Calculate required amount
	const baseAmount =
		template.objectiveType === 'CaptureMonster' ||
		template.objectiveType === 'ExploreArea' ||
		template.objectiveType === 'TalkToNPC'
			? 1
			: 3 + rng.nextInt(5);
	const requiredAmount = Math.floor(baseAmount * scaling.amountMultiplier);

	// Generate title and description
	const title = template.titleTemplate
		.replace('{TARGET}', targetName)
		.replace('{AMOUNT}', String(requiredAmount));

	const description = template.descriptionTemplate
		.replace('{TARGET}', targetName)
		.replace('{AMOUNT}', String(requiredAmount));

	// Calculate levels
	const requiredLevel = Math.max(1, config.playerLevel + scaling.levelOffset - 3);
	const recommendedLevel = config.playerLevel + scaling.levelOffset;

	// Generate rewards
	const rewards: QuestReward[] = [
		{
			type: 'Gold',
			amount: Math.floor(scaling.goldReward * (1 + config.playerLevel * 0.1)),
		},
		{
			type: 'Experience',
			amount: Math.floor(scaling.expReward * (1 + config.playerLevel * 0.05)),
		},
	];

	// Chance for item reward on harder quests
	if (difficultyIdx >= 2 && rng.next() < 0.3) {
		rewards.push({
			type: 'Item',
			itemId: rng.pick(TARGET_NAMES.items).toLowerCase().replace(/\s+/g, '_'),
			amount: 1,
		});
	}

	// Create quest
	const quest: Quest = {
		id: `quest_${seed}_${rng.nextInt(10000)}`,
		title,
		description,
		difficulty,
		objectives: [
			{
				id: `obj_${seed}_0`,
				type: template.objectiveType,
				description: `${template.objectiveType === 'CaptureMonster' ? 'Capture' : template.objectiveType === 'DefeatMonsters' ? 'Defeat' : 'Complete'} ${requiredAmount} ${targetName}`,
				target,
				targetName,
				requiredAmount,
				currentAmount: 0,
				isComplete: false,
				isHidden: false,
			},
		],
		rewards,
		status: 'Available',
		requiredLevel,
		recommendedLevel,
		biome: config.currentBiome,
		isMainStory: false,
		isRepeatable: template.objectiveType !== 'ExploreArea',
		prerequisiteQuestIds: [],
	};

	return quest;
}

/**
 * Generate multiple quests for a region
 */
export function generateQuestsForRegion(config: QuestGeneratorConfig, count: number): Quest[] {
	const quests: Quest[] = [];
	const baseSeed = config.seed ?? Date.now();

	for (let i = 0; i < count; i++) {
		quests.push(
			generateQuest({
				...config,
				seed: baseSeed + i * 12345,
			}),
		);
	}

	return quests;
}

// ========================================
// QUEST MANAGEMENT
// ========================================

/**
 * Quest manager for handling quest state
 */
export class QuestManager {
	private journal: QuestJournal;

	constructor() {
		this.journal = {
			activeQuests: [],
			completedQuests: [],
			failedQuests: [],
			availableQuests: [],
		};
	}

	getJournal(): Readonly<QuestJournal> {
		return this.journal;
	}

	/**
	 * Accept a quest
	 */
	acceptQuest(questId: string): boolean {
		const idx = this.journal.availableQuests.findIndex((q) => q.id === questId);
		if (idx < 0) return false;

		const quest = this.journal.availableQuests.splice(idx, 1)[0];
		quest.status = 'Active';
		quest.acceptedAt = new Date().toISOString();
		this.journal.activeQuests.push(quest);

		return true;
	}

	/**
	 * Update quest progress
	 */
	updateProgress(
		questId: string,
		objectiveId: string,
		progress: number,
	): { questComplete: boolean; objectiveComplete: boolean } {
		const quest = this.journal.activeQuests.find((q) => q.id === questId);
		if (!quest) return { questComplete: false, objectiveComplete: false };

		const objective = quest.objectives.find((o) => o.id === objectiveId);
		if (!objective || objective.isComplete)
			return { questComplete: false, objectiveComplete: false };

		objective.currentAmount = Math.min(
			objective.currentAmount + progress,
			objective.requiredAmount,
		);

		const objectiveComplete = objective.currentAmount >= objective.requiredAmount;
		if (objectiveComplete) {
			objective.isComplete = true;

			// Reveal hidden objectives
			const hiddenObj = quest.objectives.find((o) => o.isHidden && !o.isComplete);
			if (hiddenObj) {
				hiddenObj.isHidden = false;
			}
		}

		// Check if all objectives are complete
		const questComplete = quest.objectives.every((o) => o.isComplete);
		if (questComplete) {
			this.completeQuest(questId);
		}

		return { questComplete, objectiveComplete };
	}

	/**
	 * Complete a quest and award rewards
	 */
	private completeQuest(questId: string): QuestReward[] {
		const idx = this.journal.activeQuests.findIndex((q) => q.id === questId);
		if (idx < 0) return [];

		const quest = this.journal.activeQuests.splice(idx, 1)[0];
		quest.status = 'Completed';
		quest.completedAt = new Date().toISOString();
		this.journal.completedQuests.push(quest);

		return quest.rewards;
	}

	/**
	 * Abandon a quest
	 */
	abandonQuest(questId: string): boolean {
		const idx = this.journal.activeQuests.findIndex((q) => q.id === questId);
		if (idx < 0) return false;

		const quest = this.journal.activeQuests.splice(idx, 1)[0];
		quest.status = 'Failed';

		// Reset progress
		for (const obj of quest.objectives) {
			obj.currentAmount = 0;
			obj.isComplete = false;
		}

		// If repeatable, make available again
		if (quest.isRepeatable) {
			quest.status = 'Available';
			this.journal.availableQuests.push(quest);
		} else {
			this.journal.failedQuests.push(quest);
		}

		return true;
	}

	/**
	 * Add available quest
	 */
	addAvailableQuest(quest: Quest): void {
		if (!this.journal.availableQuests.find((q) => q.id === quest.id)) {
			this.journal.availableQuests.push(quest);
		}
	}

	/**
	 * Get quests by objective type (for tracking progress)
	 */
	getActiveQuestsByObjective(type: ObjectiveType): Quest[] {
		return this.journal.activeQuests.filter((q) =>
			q.objectives.some((o) => o.type === type && !o.isComplete),
		);
	}

	/**
	 * Notify of event for auto-tracking
	 */
	notifyEvent(
		eventType: ObjectiveType,
		targetId: string,
		amount: number = 1,
	): Array<{ questId: string; objectiveId: string; complete: boolean }> {
		const results: Array<{
			questId: string;
			objectiveId: string;
			complete: boolean;
		}> = [];

		for (const quest of this.journal.activeQuests) {
			for (const objective of quest.objectives) {
				if (
					objective.type === eventType &&
					objective.target === targetId &&
					!objective.isComplete
				) {
					const { objectiveComplete } = this.updateProgress(quest.id, objective.id, amount);
					results.push({
						questId: quest.id,
						objectiveId: objective.id,
						complete: objectiveComplete,
					});
				}
			}
		}

		return results;
	}
}

// ========================================
// MAIN STORY QUESTS
// ========================================

export const MAIN_STORY_QUESTS: Quest[] = [
	{
		id: 'main_001',
		title: 'The Awakening',
		description:
			'You awaken in the village of Beastlight with no memory of your past. Find the Elder to learn what happened.',
		difficulty: 'Easy',
		objectives: [
			{
				id: 'main_001_obj1',
				type: 'TalkToNPC',
				description: 'Speak with Elder Marcus',
				target: 'elder_marcus',
				targetName: 'Elder Marcus',
				requiredAmount: 1,
				currentAmount: 0,
				isComplete: false,
				isHidden: false,
			},
		],
		rewards: [{ type: 'Experience', amount: 100 }],
		status: 'Available',
		requiredLevel: 1,
		recommendedLevel: 1,
		isMainStory: true,
		isRepeatable: false,
		prerequisiteQuestIds: [],
	},
	{
		id: 'main_002',
		title: 'Your First Companion',
		description:
			'The Elder says you must bond with a monster to begin your journey. Choose your starter companion.',
		difficulty: 'Easy',
		objectives: [
			{
				id: 'main_002_obj1',
				type: 'CaptureMonster',
				description: 'Capture a starter monster (Emberpup, Aquafin, or Voltkit)',
				target: 'starter',
				targetName: 'Starter Monster',
				requiredAmount: 1,
				currentAmount: 0,
				isComplete: false,
				isHidden: false,
			},
		],
		rewards: [
			{ type: 'Experience', amount: 200 },
			{ type: 'Gold', amount: 100 },
		],
		status: 'Available',
		requiredLevel: 1,
		recommendedLevel: 1,
		isMainStory: true,
		isRepeatable: false,
		prerequisiteQuestIds: ['main_001'],
	},
	{
		id: 'main_003',
		title: 'The Corruption Spreads',
		description: 'Strange corrupted monsters have appeared in the forest. Investigate the source.',
		difficulty: 'Normal',
		objectives: [
			{
				id: 'main_003_obj1',
				type: 'DefeatMonsters',
				description: 'Defeat corrupted monsters',
				target: 'corrupted',
				targetName: 'Corrupted Monster',
				requiredAmount: 5,
				currentAmount: 0,
				isComplete: false,
				isHidden: false,
			},
			{
				id: 'main_003_obj2',
				type: 'ExploreArea',
				description: 'Find the source of corruption',
				target: 'corrupted_grove',
				targetName: 'Corrupted Grove',
				requiredAmount: 1,
				currentAmount: 0,
				isComplete: false,
				isHidden: true,
			},
		],
		rewards: [
			{ type: 'Experience', amount: 500 },
			{ type: 'Gold', amount: 300 },
			{ type: 'Item', itemId: 'cleansing_crystal', amount: 1 },
		],
		status: 'Available',
		requiredLevel: 5,
		recommendedLevel: 8,
		biome: 'Forest',
		isMainStory: true,
		isRepeatable: false,
		prerequisiteQuestIds: ['main_002'],
	},
];
