import type { NativeStackNavigationProp } from '@react-navigation/native-stack';
import React from 'react';
import { Pressable, StyleSheet, Text, View } from 'react-native';

import type { RootStackParamList } from '../App';

type MainMenuScreenProps = {
	navigation: NativeStackNavigationProp<RootStackParamList, 'MainMenu'>;
};

export function MainMenuScreen({ navigation }: MainMenuScreenProps) {
	const handleNewGame = () => {
		navigation.navigate('Game', {});
	};

	const handleLoadGame = () => {
		navigation.navigate('LoadGame');
	};

	const handleSettings = () => {
		navigation.navigate('Settings');
	};

	return (
		<View style={styles.container}>
			<View style={styles.titleContainer}>
				<Text style={styles.title}>Echoes of</Text>
				<Text style={styles.titleAccent}>Beastlight</Text>
			</View>

			<View style={styles.menuContainer}>
				<MenuButton title="New Game" onPress={handleNewGame} />
				<MenuButton title="Load Game" onPress={handleLoadGame} />
				<MenuButton title="Settings" onPress={handleSettings} />
			</View>

			<Text style={styles.version}>v1.0.0 - TypeScript/React Native + Babylon.js</Text>
		</View>
	);
}

type MenuButtonProps = {
	title: string;
	onPress: () => void;
};

function MenuButton({ title, onPress }: MenuButtonProps) {
	return (
		<Pressable
			style={({ pressed }) => [styles.button, pressed && styles.buttonPressed]}
			onPress={onPress}
		>
			<Text style={styles.buttonText}>{title}</Text>
		</Pressable>
	);
}

const styles = StyleSheet.create({
	container: {
		flex: 1,
		backgroundColor: '#1a1a2e',
		justifyContent: 'center',
		alignItems: 'center',
		padding: 20,
	},
	titleContainer: {
		marginBottom: 60,
		alignItems: 'center',
	},
	title: {
		fontSize: 32,
		color: '#e0e0e0',
		fontWeight: '300',
		letterSpacing: 4,
	},
	titleAccent: {
		fontSize: 48,
		color: '#7c3aed',
		fontWeight: 'bold',
		letterSpacing: 2,
	},
	menuContainer: {
		gap: 16,
		width: 280,
	},
	button: {
		backgroundColor: '#2d2d44',
		paddingVertical: 16,
		paddingHorizontal: 32,
		borderRadius: 8,
		borderWidth: 2,
		borderColor: '#7c3aed',
	},
	buttonPressed: {
		backgroundColor: '#7c3aed',
		transform: [{ scale: 0.98 }],
	},
	buttonText: {
		color: '#ffffff',
		fontSize: 18,
		fontWeight: '600',
		textAlign: 'center',
		letterSpacing: 1,
	},
	version: {
		position: 'absolute',
		bottom: 20,
		color: '#666',
		fontSize: 12,
	},
});
