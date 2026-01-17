import { z } from 'zod';

/**
 * Quest objective types
 * Based on quest-generator vision doc
 */
export const ObjectiveTypeSchema = z.enum([
	'DefeatMonster',
	'TameMonster',
	'CollectItem',
	'TalkToNPC',
	'ExploreArea',
	'DeliverItem',
	'EscortNPC',
	'SolvePuzzle',
]);
export type ObjectiveType = z.infer<typeof ObjectiveTypeSchema>;

/**
 * Quest status
 */
export const QuestStatusSchema = z.enum(['NotStarted', 'InProgress', 'Completed', 'Failed']);
export type QuestStatus = z.infer<typeof QuestStatusSchema>;

/**
 * Quest objective definition
 */
export const QuestObjectiveSchema = z.object({
	id: z.string(),
	type: ObjectiveTypeSchema,
	description: z.string(),
	targetId: z.string().optional(),
	targetCount: z.number().int().min(1).default(1),
	currentCount: z.number().int().min(0).default(0),
	isComplete: z.boolean().default(false),
	isOptional: z.boolean().default(false),
});
export type QuestObjective = z.infer<typeof QuestObjectiveSchema>;

/**
 * Quest reward definition
 */
export const QuestRewardSchema = z.object({
	experience: z.number().int().min(0).default(0),
	gold: z.number().int().min(0).default(0),
	items: z.array(
		z.object({
			itemId: z.string(),
			quantity: z.number().int().min(1),
		}),
	),
	unlocks: z.array(z.string()).default([]), // area/feature IDs
});
export type QuestReward = z.infer<typeof QuestRewardSchema>;

/**
 * Quest definition
 * Supports procedural generation via quest-generator
 */
export const QuestSchema = z.object({
	id: z.string(),
	title: z.string(),
	description: z.string(),
	giverNpcId: z.string().optional(),
	status: QuestStatusSchema,
	objectives: z.array(QuestObjectiveSchema),
	rewards: QuestRewardSchema,
	prerequisites: z.array(z.string()).default([]), // quest IDs
	isMainStory: z.boolean().default(false),
	chapter: z.number().int().min(1).optional(),
});
export type Quest = z.infer<typeof QuestSchema>;

/**
 * Player's quest journal
 */
export const QuestJournalSchema = z.object({
	active: z.array(QuestSchema),
	completed: z.array(z.string()), // quest IDs
	failed: z.array(z.string()), // quest IDs
});
export type QuestJournal = z.infer<typeof QuestJournalSchema>;
