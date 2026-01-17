import type {
	BiomeType,
	ObjectiveType,
	Quest,
	QuestObjective,
	QuestReward,
} from '../schemas/index.js';
import { getUUID } from '../utils.js';

/**
 * Quest generation parameters
 */
export interface QuestGenParams {
	playerLevel: number;
	biome: BiomeType;
	isMainStory?: boolean;
	difficulty?: 'easy' | 'normal' | 'hard';
}

/**
 * Quest templates for procedural generation
 */
interface QuestTemplate {
	titleTemplate: string;
	descriptionTemplate: string;
	objectiveTypes: ObjectiveType[];
	baseRewards: {
		experienceMult: number;
		goldMult: number;
	};
}

const QUEST_TEMPLATES: QuestTemplate[] = [
	{
		titleTemplate: 'Hunt the {monster}',
		descriptionTemplate:
			'A dangerous {monster} has been terrorizing the {biome}. Defeat it to restore peace.',
		objectiveTypes: ['DefeatMonster'],
		baseRewards: { experienceMult: 1.5, goldMult: 1.2 },
	},
	{
		titleTemplate: 'Tame a Wild {monster}',
		descriptionTemplate: 'A rare {monster} has been spotted in the {biome}. Try to tame it!',
		objectiveTypes: ['TameMonster'],
		baseRewards: { experienceMult: 2.0, goldMult: 0.8 },
	},
	{
		titleTemplate: 'Gather {item}',
		descriptionTemplate: 'Collect {count} {item} from the {biome} for the village elder.',
		objectiveTypes: ['CollectItem'],
		baseRewards: { experienceMult: 1.0, goldMult: 1.5 },
	},
	{
		titleTemplate: 'Explore the {biome}',
		descriptionTemplate:
			'Map out the unexplored regions of the {biome} and report back what you find.',
		objectiveTypes: ['ExploreArea'],
		baseRewards: { experienceMult: 1.2, goldMult: 1.0 },
	},
	{
		titleTemplate: 'Delivery to {npc}',
		descriptionTemplate: 'Deliver an important package to {npc} in the {biome}.',
		objectiveTypes: ['DeliverItem', 'TalkToNPC'],
		baseRewards: { experienceMult: 0.8, goldMult: 1.8 },
	},
];

/**
 * Difficulty multipliers
 */
const DIFFICULTY_MULTS: Record<'easy' | 'normal' | 'hard', number> = {
	easy: 0.7,
	normal: 1.0,
	hard: 1.5,
};

/**
 * Generate quest objectives
 */
function generateObjectives(
	template: QuestTemplate,
	difficulty: 'easy' | 'normal' | 'hard',
): QuestObjective[] {
	const diffMult = DIFFICULTY_MULTS[difficulty];

	return template.objectiveTypes.map((type, index) => {
		let targetCount = 1;
		if (type === 'CollectItem' || type === 'DefeatMonster') {
			targetCount = Math.ceil(3 * diffMult);
		}

		return {
			id: `obj-${index}`,
			type,
			description: `Complete ${type
				.replace(/([A-Z])/g, ' $1')
				.toLowerCase()
				.trim()}`,
			targetCount,
			currentCount: 0,
			isComplete: false,
			isOptional: index > 0 && Math.random() > 0.7,
		};
	});
}

/**
 * Calculate quest rewards
 */
function calculateRewards(
	template: QuestTemplate,
	playerLevel: number,
	difficulty: 'easy' | 'normal' | 'hard',
): QuestReward {
	const diffMult = DIFFICULTY_MULTS[difficulty];
	const baseExp = 50 + playerLevel * 20;
	const baseGold = 25 + playerLevel * 15;

	return {
		experience: Math.floor(baseExp * template.baseRewards.experienceMult * diffMult),
		gold: Math.floor(baseGold * template.baseRewards.goldMult * diffMult),
		items: [],
		unlocks: [],
	};
}

/**
 * Generate a procedural quest
 */
export function generateQuest(params: QuestGenParams): Quest {
	const difficulty = params.difficulty ?? 'normal';
	const template = QUEST_TEMPLATES[Math.floor(Math.random() * QUEST_TEMPLATES.length)];
	const objectives = generateObjectives(template, difficulty);
	const collectCount = objectives.find((o) => o.type === 'CollectItem')?.targetCount ?? 1;

	// Replace template placeholders
	const title = template.titleTemplate
		.replace('{monster}', 'Wild Creature')
		.replace('{item}', 'Rare Herbs')
		.replace('{npc}', 'Village Elder');

	const description = template.descriptionTemplate
		.replace('{monster}', 'wild creature')
		.replace('{biome}', params.biome.toLowerCase())
		.replace('{item}', 'rare herbs')
		.replace('{count}', String(collectCount))
		.replace('{npc}', 'the Village Elder');

	return {
		id: getUUID(),
		title,
		description,
		status: 'NotStarted',
		objectives,
		rewards: calculateRewards(template, params.playerLevel, difficulty),
		prerequisites: [],
		isMainStory: params.isMainStory ?? false,
	};
}

/**
 * Generate multiple side quests for an area
 */
export function generateAreaQuests(biome: BiomeType, playerLevel: number, count = 3): Quest[] {
	return Array.from({ length: count }, () =>
		generateQuest({
			biome,
			playerLevel,
			isMainStory: false,
			difficulty: ['easy', 'normal', 'hard'][Math.floor(Math.random() * 3)] as
				| 'easy'
				| 'normal'
				| 'hard',
		}),
	);
}
