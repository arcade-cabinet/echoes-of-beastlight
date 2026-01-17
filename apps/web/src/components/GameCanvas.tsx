import {
	ArcRotateCamera,
	Color3,
	Color4,
	Engine,
	HemisphericLight,
	MeshBuilder,
	Scene,
	StandardMaterial,
	Vector3,
} from '@babylonjs/core';
import { type BiomeType, generateWildEncounter } from '@echoes-of-beastlight/game-core';
import { useEffect, useRef } from 'react';

interface GameCanvasProps {
	mode: 'world' | 'combat';
}

export function GameCanvas({ mode }: GameCanvasProps) {
	const canvasRef = useRef<HTMLCanvasElement>(null);
	const engineRef = useRef<Engine | null>(null);
	const sceneRef = useRef<Scene | null>(null);

	useEffect(() => {
		if (!canvasRef.current) return;

		// Create Babylon engine
		const engine = new Engine(canvasRef.current, true);
		engineRef.current = engine;

		// Create scene
		const scene = new Scene(engine);
		sceneRef.current = scene;
		scene.clearColor = new Color4(0.1, 0.1, 0.18, 1);

		// Setup based on mode
		if (mode === 'world') {
			setupWorldScene(scene);
		} else {
			setupCombatScene(scene);
		}

		// Start render loop
		engine.runRenderLoop(() => {
			scene.render();
		});

		// Handle resize
		const handleResize = () => engine.resize();
		window.addEventListener('resize', handleResize);

		return () => {
			window.removeEventListener('resize', handleResize);
			scene.dispose();
			engine.dispose();
		};
	}, [mode]);

	return <canvas ref={canvasRef} style={{ flex: 1, outline: 'none' }} tabIndex={0} />;
}

function setupWorldScene(scene: Scene) {
	// Isometric camera
	const camera = new ArcRotateCamera(
		'camera',
		-Math.PI / 4,
		Math.PI / 3,
		20,
		Vector3.Zero(),
		scene,
	);
	camera.attachControl(true);
	camera.lowerBetaLimit = Math.PI / 4;
	camera.upperBetaLimit = Math.PI / 3;

	// Lighting
	const light = new HemisphericLight('light', new Vector3(0, 1, 0.5), scene);
	light.intensity = 0.9;

	// Generate a test grid of tiles
	const gridSize = 10;
	const biomes: BiomeType[] = ['Forest', 'Plains', 'Desert', 'Tundra'];
	const biomeColors: Record<string, Color3> = {
		Forest: new Color3(0.2, 0.5, 0.2),
		Plains: new Color3(0.5, 0.7, 0.3),
		Desert: new Color3(0.9, 0.8, 0.5),
		Tundra: new Color3(0.9, 0.95, 1.0),
	};

	for (let x = 0; x < gridSize; x++) {
		for (let z = 0; z < gridSize; z++) {
			const biome = biomes[Math.floor(Math.random() * biomes.length)];
			const tile = MeshBuilder.CreateBox(
				`tile-${x}-${z}`,
				{ width: 0.95, height: 0.1, depth: 0.95 },
				scene,
			);
			tile.position = new Vector3(x - gridSize / 2, 0, z - gridSize / 2);

			const mat = new StandardMaterial(`mat-${x}-${z}`, scene);
			mat.diffuseColor = biomeColors[biome];
			tile.material = mat;
		}
	}

	// Add a player marker
	const player = MeshBuilder.CreateCylinder('player', { height: 1, diameter: 0.5 }, scene);
	player.position = new Vector3(0, 0.5, 0);
	const playerMat = new StandardMaterial('playerMat', scene);
	playerMat.diffuseColor = new Color3(0.3, 0.6, 1.0);
	player.material = playerMat;

	// Generate some monster encounters
	console.log('=== PROCEDURAL MONSTER GENERATION TEST ===');
	for (let i = 0; i < 5; i++) {
		const monster = generateWildEncounter({
			biome: biomes[Math.floor(Math.random() * biomes.length)],
			minLevel: 1,
			maxLevel: 10,
		});
		console.log(`Monster ${i + 1}:`, monster);
	}
}

function setupCombatScene(scene: Scene) {
	// Side-view camera for combat
	const camera = new ArcRotateCamera(
		'camera',
		-Math.PI / 2,
		Math.PI / 3,
		15,
		Vector3.Zero(),
		scene,
	);
	camera.attachControl(true);

	// Lighting
	const light = new HemisphericLight('light', new Vector3(0, 1, 0), scene);
	light.intensity = 1.0;

	// Battle arena floor
	const ground = MeshBuilder.CreateGround('arena', { width: 20, height: 10 }, scene);
	const groundMat = new StandardMaterial('groundMat', scene);
	groundMat.diffuseColor = new Color3(0.15, 0.15, 0.2);
	ground.material = groundMat;

	// Player party (left side)
	const playerColors = [
		new Color3(0.3, 0.6, 1.0),
		new Color3(0.4, 0.7, 1.0),
		new Color3(0.5, 0.8, 1.0),
	];
	for (let i = 0; i < 3; i++) {
		const fighter = MeshBuilder.CreateBox(
			`player-${i}`,
			{ width: 1, height: 2, depth: 0.5 },
			scene,
		);
		fighter.position = new Vector3(-5, 1, (i - 1) * 2);
		const mat = new StandardMaterial(`playerMat-${i}`, scene);
		mat.diffuseColor = playerColors[i];
		fighter.material = mat;
	}

	// Enemy party (right side)
	const enemyColors = [
		new Color3(1.0, 0.3, 0.3),
		new Color3(1.0, 0.4, 0.4),
		new Color3(1.0, 0.5, 0.5),
	];
	for (let i = 0; i < 3; i++) {
		const enemy = MeshBuilder.CreateBox(`enemy-${i}`, { width: 1, height: 2, depth: 0.5 }, scene);
		enemy.position = new Vector3(5, 1, (i - 1) * 2);
		const mat = new StandardMaterial(`enemyMat-${i}`, scene);
		mat.diffuseColor = enemyColors[i];
		enemy.material = mat;
	}

	console.log('=== COMBAT SCENE INITIALIZED ===');
	console.log('Player party: 3 units (blue)');
	console.log('Enemy party: 3 units (red)');
}
