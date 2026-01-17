import type { RouteProp } from '@react-navigation/native';
import type { NativeStackNavigationProp } from '@react-navigation/native-stack';
import { useCallback, useEffect } from 'react';
import { StyleSheet, View } from 'react-native';

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

	const _handleBackToMenu = useCallback(() => {
		navigation.navigate('MainMenu');
	}, [navigation]);

	if (isLoading || !player) {
		return <View style={styles.container} />;
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
	},
	gameView: {
		flex: 1,
	},
});
