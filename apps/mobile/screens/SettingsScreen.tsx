import type { NativeStackNavigationProp } from '@react-navigation/native-stack';
import React from 'react';
import { Pressable, StyleSheet, Text, View } from 'react-native';
import Slider from '@react-native-community/slider';

import type { RootStackParamList } from '../App';
import { useGameStore } from '../stores/gameStore';

type SettingsScreenProps = {
	navigation: NativeStackNavigationProp<RootStackParamList, 'Settings'>;
};

export function SettingsScreen({ navigation }: SettingsScreenProps) {
	const { settings, updateSettings } = useGameStore();

	const handleBack = () => {
		navigation.goBack();
	};

	return (
		<View style={styles.container}>
			<Text style={styles.title}>Settings</Text>

			<View style={styles.settingsContainer}>
				<View style={styles.settingItem}>
					<Text style={styles.settingLabel}>Music Volume</Text>
					<View style={styles.sliderContainer}>
						<Slider
							style={styles.slider}
							minimumValue={0}
							maximumValue={1}
							value={settings.musicVolume}
							onValueChange={(value) => updateSettings({ musicVolume: value })}
							minimumTrackTintColor="#7c3aed"
							maximumTrackTintColor="#3d3d5c"
							thumbTintColor="#7c3aed"
						/>
						<Text style={styles.sliderValue}>{Math.round(settings.musicVolume * 100)}%</Text>
					</View>
				</View>

				<View style={styles.settingItem}>
					<Text style={styles.settingLabel}>SFX Volume</Text>
					<View style={styles.sliderContainer}>
						<Slider
							style={styles.slider}
							minimumValue={0}
							maximumValue={1}
							value={settings.sfxVolume}
							onValueChange={(value) => updateSettings({ sfxVolume: value })}
							minimumTrackTintColor="#7c3aed"
							maximumTrackTintColor="#3d3d5c"
							thumbTintColor="#7c3aed"
						/>
						<Text style={styles.sliderValue}>{Math.round(settings.sfxVolume * 100)}%</Text>
					</View>
				</View>

				<View style={styles.settingItem}>
					<Text style={styles.settingLabel}>Text Speed</Text>
					<View style={styles.buttonGroup}>
						{(['slow', 'normal', 'fast'] as const).map((speed) => (
							<Pressable
								key={speed}
								style={[
									styles.speedButton,
									settings.textSpeed === speed && styles.speedButtonActive,
								]}
								onPress={() => updateSettings({ textSpeed: speed })}
							>
								<Text
									style={[
										styles.speedButtonText,
										settings.textSpeed === speed && styles.speedButtonTextActive,
									]}
								>
									{speed.charAt(0).toUpperCase() + speed.slice(1)}
								</Text>
							</Pressable>
						))}
					</View>
				</View>

				<View style={styles.settingItem}>
					<Text style={styles.settingLabel}>Battle Animations</Text>
					<Pressable
						style={[styles.toggleButton, settings.battleAnimations && styles.toggleButtonActive]}
						onPress={() => updateSettings({ battleAnimations: !settings.battleAnimations })}
					>
						<Text style={styles.toggleButtonText}>
							{settings.battleAnimations ? 'ON' : 'OFF'}
						</Text>
					</Pressable>
				</View>
			</View>

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
	settingsContainer: {
		flex: 1,
		gap: 24,
	},
	settingItem: {
		gap: 12,
	},
	settingLabel: {
		color: '#ffffff',
		fontSize: 18,
		fontWeight: '600',
	},
	sliderContainer: {
		flexDirection: 'row',
		alignItems: 'center',
		gap: 12,
	},
	slider: {
		flex: 1,
		height: 40,
	},
	sliderValue: {
		color: '#888',
		fontSize: 14,
		width: 45,
		textAlign: 'right',
	},
	buttonGroup: {
		flexDirection: 'row',
		gap: 12,
	},
	speedButton: {
		backgroundColor: '#2d2d44',
		paddingVertical: 10,
		paddingHorizontal: 20,
		borderRadius: 6,
		borderWidth: 1,
		borderColor: '#3d3d5c',
	},
	speedButtonActive: {
		backgroundColor: '#7c3aed',
		borderColor: '#7c3aed',
	},
	speedButtonText: {
		color: '#888',
		fontSize: 14,
		fontWeight: '600',
	},
	speedButtonTextActive: {
		color: '#ffffff',
	},
	toggleButton: {
		backgroundColor: '#2d2d44',
		paddingVertical: 10,
		paddingHorizontal: 30,
		borderRadius: 6,
		borderWidth: 1,
		borderColor: '#3d3d5c',
		alignSelf: 'flex-start',
	},
	toggleButtonActive: {
		backgroundColor: '#7c3aed',
		borderColor: '#7c3aed',
	},
	toggleButtonText: {
		color: '#ffffff',
		fontSize: 14,
		fontWeight: '600',
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
