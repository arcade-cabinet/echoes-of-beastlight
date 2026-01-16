import React, { useState } from 'react';
import { GameCanvas } from './components/GameCanvas';
import { DebugPanel } from './components/DebugPanel';
import { useGameStore } from './stores/gameStore';

type Screen = 'menu' | 'game' | 'combat';

export function App() {
	const [screen, setScreen] = useState<Screen>('menu');
	const { initializeGame, player } = useGameStore();

	const handleNewGame = () => {
		initializeGame('WebTestHero');
		setScreen('game');
	};

	const handleTestCombat = () => {
		initializeGame('WebTestHero');
		setScreen('combat');
	};

	if (screen === 'menu') {
		return (
			<div style={styles.menuContainer}>
				<h1 style={styles.title}>Echoes of Beastlight</h1>
				<p style={styles.subtitle}>Web Test Environment (Chrome MCP)</p>

				<div style={styles.buttonGroup}>
					<button style={styles.button} onClick={handleNewGame}>
						Test World Scene
					</button>
					<button style={styles.button} onClick={handleTestCombat}>
						Test Combat Scene
					</button>
				</div>

				<div style={styles.infoBox}>
					<h3>Development Features:</h3>
					<ul>
						<li>BabylonJS 3D rendering in browser</li>
						<li>Same game-core code as mobile</li>
						<li>Chrome DevTools for debugging</li>
						<li>Hot reload for fast iteration</li>
					</ul>
				</div>
			</div>
		);
	}

	return (
		<div style={styles.gameContainer}>
			<GameCanvas mode={screen === 'combat' ? 'combat' : 'world'} />
			<DebugPanel player={player} onBack={() => setScreen('menu')} />
		</div>
	);
}

const styles: Record<string, React.CSSProperties> = {
	menuContainer: {
		display: 'flex',
		flexDirection: 'column',
		alignItems: 'center',
		justifyContent: 'center',
		height: '100vh',
		gap: 24,
	},
	title: {
		fontSize: 48,
		fontWeight: 700,
		color: '#7c3aed',
		margin: 0,
	},
	subtitle: {
		fontSize: 18,
		color: '#94a3b8',
		margin: 0,
	},
	buttonGroup: {
		display: 'flex',
		gap: 16,
		marginTop: 32,
	},
	button: {
		padding: '16px 32px',
		fontSize: 18,
		fontWeight: 600,
		backgroundColor: '#2d2d44',
		color: '#fff',
		border: '2px solid #7c3aed',
		borderRadius: 8,
		cursor: 'pointer',
	},
	infoBox: {
		marginTop: 48,
		padding: 24,
		backgroundColor: '#111827',
		borderRadius: 12,
		maxWidth: 400,
	},
	gameContainer: {
		display: 'flex',
		width: '100vw',
		height: '100vh',
	},
};
