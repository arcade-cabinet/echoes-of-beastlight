import type { PlayerSave } from '@echoes-of-beastlight/game-core';
import type React from 'react';

interface DebugPanelProps {
	player: PlayerSave | null;
	onBack: () => void;
}

export function DebugPanel({ player, onBack }: DebugPanelProps) {
	return (
		<div style={styles.panel}>
			<h2 style={styles.title}>Debug Panel</h2>

			<button type="button" style={styles.backButton} onClick={onBack}>
				← Back to Menu
			</button>

			<div style={styles.section}>
				<h3>Player State</h3>
				{player ? (
					<>
						<p>Name: {player.name}</p>
						<p>Level: {player.level}</p>
						<p>
							HP: {player.stats.hp}/{player.stats.maxHp}
						</p>
						<p>Attack: {player.stats.attack}</p>
						<p>Defense: {player.stats.defense}</p>
						<p>Gold: {player.inventory.gold}</p>
					</>
				) : (
					<p>No player loaded</p>
				)}
			</div>

			<div style={styles.section}>
				<h3>Controls</h3>
				<ul style={styles.list}>
					<li>Mouse drag: Rotate camera</li>
					<li>Scroll: Zoom in/out</li>
					<li>Check console for logs</li>
				</ul>
			</div>

			<div style={styles.section}>
				<h3>Chrome DevTools</h3>
				<p style={styles.hint}>Open DevTools (F12) to:</p>
				<ul style={styles.list}>
					<li>View console.log output</li>
					<li>Inspect game state</li>
					<li>Debug with breakpoints</li>
					<li>Profile performance</li>
				</ul>
			</div>
		</div>
	);
}

const styles: Record<string, React.CSSProperties> = {
	panel: {
		width: 280,
		backgroundColor: '#111827',
		padding: 16,
		overflowY: 'auto',
		borderLeft: '1px solid #2d2d44',
	},
	title: {
		margin: '0 0 16px 0',
		color: '#7c3aed',
		fontSize: 18,
	},
	backButton: {
		width: '100%',
		padding: '8px 16px',
		marginBottom: 16,
		backgroundColor: '#2d2d44',
		color: '#fff',
		border: 'none',
		borderRadius: 6,
		cursor: 'pointer',
		fontSize: 14,
	},
	section: {
		marginBottom: 20,
		padding: 12,
		backgroundColor: '#1e293b',
		borderRadius: 8,
	},
	list: {
		margin: '8px 0 0 16px',
		padding: 0,
		fontSize: 13,
		color: '#94a3b8',
	},
	hint: {
		fontSize: 13,
		color: '#94a3b8',
		margin: '8px 0',
	},
};
