import { create } from 'zustand';
import {
	type PlayerSave,
	type PlayerSettings,
	createNewPlayerSave,
} from '@echoes-of-beastlight/game-core';

interface GameState {
	// Player state
	player: PlayerSave | null;
	isLoading: boolean;

	// Settings (persisted separately)
	settings: PlayerSettings;

	// Save slots
	savedGames: Array<{
		id: string;
		name: string;
		level: number;
		playtime: number;
		updatedAt: string;
	}>;

	// Actions
	initializeGame: (playerName: string) => void;
	loadGame: (saveId: string) => void;
	saveGame: () => void;
	updateSettings: (settings: Partial<PlayerSettings>) => void;
	updatePlayer: (updates: Partial<PlayerSave>) => void;
}

export const useGameStore = create<GameState>((set, get) => ({
	player: null,
	isLoading: false,
	settings: {
		musicVolume: 0.7,
		sfxVolume: 0.8,
		textSpeed: 'normal',
		battleAnimations: true,
	},
	savedGames: [],

	initializeGame: (playerName: string) => {
		set({ isLoading: true });

		// Create new player save
		const newPlayer = createNewPlayerSave(playerName);

		set({
			player: newPlayer,
			isLoading: false,
		});
	},

	loadGame: (saveId: string) => {
		set({ isLoading: true });

		// TODO: Load from AsyncStorage
		// For now, just create a new game
		const state = get();
		const savedGame = state.savedGames.find((s) => s.id === saveId);

		if (savedGame) {
			// Would load full save data here
			const player = createNewPlayerSave(savedGame.name);
			player.id = saveId;
			player.level = savedGame.level;
			player.playtime = savedGame.playtime;

			set({
				player,
				isLoading: false,
			});
		} else {
			set({ isLoading: false });
		}
	},

	saveGame: () => {
		const { player, savedGames } = get();
		if (!player) return;

		// Update player's updatedAt
		const now = new Date().toISOString();
		const updatedPlayer = {
			...player,
			updatedAt: now,
		};

		// Update saved games list
		const existingIndex = savedGames.findIndex((s) => s.id === player.id);
		const saveSlot = {
			id: player.id,
			name: player.name,
			level: player.level,
			playtime: player.playtime,
			updatedAt: now,
		};

		const newSavedGames =
			existingIndex >= 0
				? [...savedGames.slice(0, existingIndex), saveSlot, ...savedGames.slice(existingIndex + 1)]
				: [saveSlot, ...savedGames];

		set({
			player: updatedPlayer,
			savedGames: newSavedGames,
		});

		// TODO: Persist to AsyncStorage
	},

	updateSettings: (newSettings: Partial<PlayerSettings>) => {
		set((state) => ({
			settings: { ...state.settings, ...newSettings },
		}));

		// TODO: Persist settings to AsyncStorage
	},

	updatePlayer: (updates: Partial<PlayerSave>) => {
		set((state) => ({
			player: state.player ? { ...state.player, ...updates } : null,
		}));
	},
}));
