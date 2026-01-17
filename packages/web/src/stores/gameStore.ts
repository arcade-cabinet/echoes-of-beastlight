import {
	createNewPlayerSave,
	type PlayerSave,
	type PlayerSettings,
} from '@echoes-of-beastlight/game-core';
import { create } from 'zustand';

interface GameState {
	player: PlayerSave | null;
	isLoading: boolean;
	settings: PlayerSettings;

	initializeGame: (playerName: string) => void;
	updatePlayer: (updates: Partial<PlayerSave>) => void;
	updateSettings: (settings: Partial<PlayerSettings>) => void;
}

export const useGameStore = create<GameState>((set) => ({
	player: null,
	isLoading: false,
	settings: {
		musicVolume: 0.7,
		sfxVolume: 0.8,
		textSpeed: 'normal',
		battleAnimations: true,
	},

	initializeGame: (playerName: string) => {
		set({ isLoading: true });
		const newPlayer = createNewPlayerSave(playerName);
		set({ player: newPlayer, isLoading: false });
		if (import.meta.env.DEV) {
			console.log('Game initialized:', { name: newPlayer.name, id: newPlayer.id });
		}
	},

	updatePlayer: (updates: Partial<PlayerSave>) => {
		set((state) => ({
			player: state.player ? { ...state.player, ...updates } : null,
		}));
	},

	updateSettings: (newSettings: Partial<PlayerSettings>) => {
		set((state) => ({
			settings: { ...state.settings, ...newSettings },
		}));
	},
}));
