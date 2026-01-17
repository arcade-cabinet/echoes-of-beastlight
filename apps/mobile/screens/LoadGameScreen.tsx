import type { NativeStackNavigationProp } from '@react-navigation/native-stack';
import { FlatList, Pressable, StyleSheet, Text, View } from 'react-native';

import type { RootStackParamList } from '../App';
import { useGameStore } from '../stores/gameStore';

type LoadGameScreenProps = {
	navigation: NativeStackNavigationProp<RootStackParamList, 'LoadGame'>;
};

export function LoadGameScreen({ navigation }: LoadGameScreenProps) {
	const { savedGames } = useGameStore();

	const handleLoadSave = (saveId: string) => {
		navigation.navigate('Game', { saveId });
	};

	const handleBack = () => {
		navigation.goBack();
	};

	const formatPlaytime = (seconds: number) => {
		const hours = Math.floor(seconds / 3600);
		const minutes = Math.floor((seconds % 3600) / 60);
		return `${hours}h ${minutes}m`;
	};

	return (
		<View style={styles.container}>
			<Text style={styles.title}>Load Game</Text>

			{savedGames.length === 0 ? (
				<View style={styles.emptyContainer}>
					<Text style={styles.emptyText}>No saved games found</Text>
				</View>
			) : (
				<FlatList
					data={savedGames}
					keyExtractor={(item) => item.id}
					style={styles.list}
					renderItem={({ item }) => (
						<Pressable
							style={({ pressed }) => [styles.saveItem, pressed && styles.saveItemPressed]}
							onPress={() => handleLoadSave(item.id)}
						>
							<View style={styles.saveInfo}>
								<Text style={styles.saveName}>{item.name}</Text>
								<Text style={styles.saveDetails}>
									Level {item.level} | {formatPlaytime(item.playtime)}
								</Text>
							</View>
							<Text style={styles.saveDate}>{new Date(item.updatedAt).toLocaleDateString()}</Text>
						</Pressable>
					)}
				/>
			)}

			<Pressable style={styles.backButton} onPress={handleBack}>
				<Text style={styles.backButtonText}>Back</Text>
			</Pressable>
		</View>
	);
}

const styles = StyleSheet.create({
	container: {
		flex: 1,
		backgroundColor: '#1a1a2e',
		padding: 20,
	},
	title: {
		fontSize: 32,
		color: '#ffffff',
		fontWeight: 'bold',
		textAlign: 'center',
		marginBottom: 30,
	},
	list: {
		flex: 1,
	},
	emptyContainer: {
		flex: 1,
		justifyContent: 'center',
		alignItems: 'center',
	},
	emptyText: {
		color: '#666',
		fontSize: 18,
	},
	saveItem: {
		backgroundColor: '#2d2d44',
		padding: 16,
		borderRadius: 8,
		marginBottom: 12,
		flexDirection: 'row',
		justifyContent: 'space-between',
		alignItems: 'center',
		borderWidth: 1,
		borderColor: '#3d3d5c',
	},
	saveItemPressed: {
		backgroundColor: '#3d3d5c',
	},
	saveInfo: {
		flex: 1,
	},
	saveName: {
		color: '#ffffff',
		fontSize: 18,
		fontWeight: '600',
		marginBottom: 4,
	},
	saveDetails: {
		color: '#888',
		fontSize: 14,
	},
	saveDate: {
		color: '#666',
		fontSize: 12,
	},
	backButton: {
		backgroundColor: '#2d2d44',
		padding: 16,
		borderRadius: 8,
		alignItems: 'center',
		marginTop: 20,
	},
	backButtonText: {
		color: '#ffffff',
		fontSize: 16,
		fontWeight: '600',
	},
});
