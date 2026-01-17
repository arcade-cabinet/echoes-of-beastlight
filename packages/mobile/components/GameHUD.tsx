import type { PlayerSave } from '@echoes-of-beastlight/game-core';
import { Pressable, StyleSheet, Text, View } from 'react-native';

interface GameHUDProps {
	player: PlayerSave;
	onPause: () => void;
}

export function GameHUD({ player, onPause }: GameHUDProps) {
	const maxHp = Math.max(1, player.stats.maxHp);
	const hpPercent = Math.min(100, Math.max(0, (player.stats.hp / maxHp) * 100));

	return (
		<View style={styles.container} pointerEvents="box-none">
			{/* Top Left - Player Info */}
			<View style={styles.playerInfo}>
				<Text style={styles.playerName}>{player.name}</Text>
				<Text style={styles.playerLevel}>Lv. {player.level}</Text>

				<View style={styles.healthBarContainer}>
					<View style={[styles.healthBar, { width: `${hpPercent}%` }]} />
					<Text style={styles.healthText}>
						{player.stats.hp}/{player.stats.maxHp}
					</Text>
				</View>
			</View>

			{/* Top Right - Pause Button */}
			<Pressable style={styles.pauseButton} onPress={onPause}>
				<Text style={styles.pauseButtonText}>||</Text>
			</Pressable>

			{/* Bottom Left - Quick Actions */}
			<View style={styles.quickActions}>
				<ActionButton label="Items" onPress={() => {}} />
				<ActionButton label="Party" onPress={() => {}} />
				<ActionButton label="Map" onPress={() => {}} />
			</View>

			{/* Bottom Right - Gold */}
			<View style={styles.goldContainer}>
				<Text style={styles.goldText}>{player.inventory.gold} G</Text>
			</View>
		</View>
	);
}

interface ActionButtonProps {
	label: string;
	onPress: () => void;
}

function ActionButton({ label, onPress }: ActionButtonProps) {
	return (
		<Pressable style={styles.actionButton} onPress={onPress}>
			<Text style={styles.actionButtonText}>{label}</Text>
		</Pressable>
	);
}

const styles = StyleSheet.create({
	container: {
		...StyleSheet.absoluteFillObject,
		padding: 16,
	},
	playerInfo: {
		position: 'absolute',
		top: 16,
		left: 16,
		backgroundColor: 'rgba(0, 0, 0, 0.7)',
		padding: 12,
		borderRadius: 8,
		minWidth: 150,
	},
	playerName: {
		color: '#ffffff',
		fontSize: 16,
		fontWeight: 'bold',
	},
	playerLevel: {
		color: '#7c3aed',
		fontSize: 14,
		marginBottom: 8,
	},
	healthBarContainer: {
		height: 20,
		backgroundColor: '#333',
		borderRadius: 4,
		overflow: 'hidden',
		justifyContent: 'center',
	},
	healthBar: {
		position: 'absolute',
		top: 0,
		left: 0,
		bottom: 0,
		backgroundColor: '#22c55e',
		borderRadius: 4,
	},
	healthText: {
		color: '#ffffff',
		fontSize: 12,
		textAlign: 'center',
		fontWeight: '600',
	},
	pauseButton: {
		position: 'absolute',
		top: 16,
		right: 16,
		backgroundColor: 'rgba(0, 0, 0, 0.7)',
		width: 44,
		height: 44,
		borderRadius: 22,
		justifyContent: 'center',
		alignItems: 'center',
	},
	pauseButtonText: {
		color: '#ffffff',
		fontSize: 18,
		fontWeight: 'bold',
	},
	quickActions: {
		position: 'absolute',
		bottom: 16,
		left: 16,
		flexDirection: 'row',
		gap: 8,
	},
	actionButton: {
		backgroundColor: 'rgba(0, 0, 0, 0.7)',
		paddingVertical: 8,
		paddingHorizontal: 16,
		borderRadius: 6,
		borderWidth: 1,
		borderColor: '#7c3aed',
	},
	actionButtonText: {
		color: '#ffffff',
		fontSize: 14,
		fontWeight: '600',
	},
	goldContainer: {
		position: 'absolute',
		bottom: 16,
		right: 16,
		backgroundColor: 'rgba(0, 0, 0, 0.7)',
		paddingVertical: 8,
		paddingHorizontal: 16,
		borderRadius: 6,
	},
	goldText: {
		color: '#fbbf24',
		fontSize: 16,
		fontWeight: 'bold',
	},
});
