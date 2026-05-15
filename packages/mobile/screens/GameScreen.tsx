import type { RouteProp } from '@react-navigation/native';
import type { NativeStackNavigationProp } from '@react-navigation/native-stack';
import { useCallback, useEffect } from 'react';
import { ActivityIndicator, Pressable, StyleSheet, Text, View } from 'react-native';

import type { RootStackParamList } from '../App';
import { GameHUD } from '../components/GameHUD';
import { BabylonView } from '../game/BabylonView';
import { useGameStore } from '../stores/gameStore';

type GameScreenProps = {
	navigation: NativeStackNavigationProp<RootStackParamList, 'Game'>;
	route: RouteProp<RootStackParamList, 'Game'>;
};

export function GameScreen({ navigation, route }: GameScreenProps) {
	const { saveId } = route.params ?? {};
	const { initializeGame, loadGame, player, isLoading } = useGameStore();

	useEffect(() => {
		if (saveId) {
			loadGame(saveId);
		} else {
			initializeGame('Hero');
		}
	}, [saveId, initializeGame, loadGame]);

	const handlePause = useCallback(() => {
		// TODO: Show pause menu
	}, []);

	const handleBackToMenu = useCallback(() => {
		navigation.navigate('MainMenu');
	}, [navigation]);

	useEffect(() => {
		const init = async () => {
			if (saveId) {
				await loadGame(saveId).catch(console.error);
			} else {
				initializeGame('Hero');
			}
		};
		init();
	}, [saveId, initializeGame, loadGame]);

	if (isLoading) {
		return (
			<View style={styles.container}>
				<ActivityIndicator size="large" color="#7c3aed" />
			</View>
		);
	}

	if (!player) {
		return (
			<View style={styles.container}>
				<Text style={styles.errorText}>Save not found.</Text>
				<Pressable style={styles.backButton} onPress={handleBackToMenu}>
					<Text style={styles.backButtonText}>Back to Menu</Text>
				</Pressable>
			</View>
		);
	}

	return (
		<View style={styles.container}>
			<BabylonView style={styles.gameView} />
			<GameHUD player={player} onPause={handlePause} />
		</View>
	);
}

const styles = StyleSheet.create({
	container: {
		flex: 1,
		backgroundColor: '#1a1a2e',
		justifyContent: 'center',
	},
	gameView: {
		flex: 1,
	},
	errorText: {
		color: '#fff',
		fontSize: 18,
		textAlign: 'center',
		marginBottom: 20,
	},
	backButton: {
		alignSelf: 'center',
		paddingHorizontal: 16,
		paddingVertical: 8,
		backgroundColor: '#7c3aed',
		borderRadius: 6,
	},
	backButtonText: {
		color: '#fff',
		fontWeight: '600',
	},
});
