import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import { StatusBar } from 'expo-status-bar';
import React from 'react';
import { StyleSheet, View } from 'react-native';
import { GestureHandlerRootView } from 'react-native-gesture-handler';

import { GameScreen } from './screens/GameScreen';
import { MainMenuScreen } from './screens/MainMenuScreen';
import { LoadGameScreen } from './screens/LoadGameScreen';
import { SettingsScreen } from './screens/SettingsScreen';

export type RootStackParamList = {
	MainMenu: undefined;
	Game: { saveId?: string };
	LoadGame: undefined;
	Settings: undefined;
};

const Stack = createNativeStackNavigator<RootStackParamList>();

export default function App() {
	return (
		<GestureHandlerRootView style={styles.container}>
			<NavigationContainer>
				<StatusBar style="light" hidden />
				<Stack.Navigator
					initialRouteName="MainMenu"
					screenOptions={{
						headerShown: false,
						animation: 'fade',
						contentStyle: styles.screen,
					}}
				>
					<Stack.Screen name="MainMenu" component={MainMenuScreen} />
					<Stack.Screen name="Game" component={GameScreen} />
					<Stack.Screen name="LoadGame" component={LoadGameScreen} />
					<Stack.Screen name="Settings" component={SettingsScreen} />
				</Stack.Navigator>
			</NavigationContainer>
		</GestureHandlerRootView>
	);
}

const styles = StyleSheet.create({
	container: {
		flex: 1,
		backgroundColor: '#1a1a2e',
	},
	screen: {
		backgroundColor: '#1a1a2e',
	},
});
