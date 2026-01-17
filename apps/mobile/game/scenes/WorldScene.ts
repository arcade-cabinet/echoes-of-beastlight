import {
	Scene,
	Vector3,
	MeshBuilder,
	StandardMaterial,
	Color3,
	type ArcRotateCamera,
} from '@babylonjs/core';
import type { Area, Tile } from '@echoes-of-beastlight/game-core';

/**
 * WorldScene - Renders the overworld tilemap
 * Uses BabylonJS with isometric camera perspective
 */
export class WorldScene {
	private scene: Scene;
	private camera: ArcRotateCamera;
	private tileSize = 1;
	private tiles: Map<string, unknown> = new Map();

	constructor(scene: Scene, camera: ArcRotateCamera) {
		this.scene = scene;
		this.camera = camera;
	}

	/**
	 * Load and render an area's tilemap
	 */
	loadArea(area: Area): void {
		// Clear existing tiles
		this.clearTiles();

		// Create tiles for the area
		for (const tile of area.tiles) {
			this.createTile(tile);
		}

		// Center camera on area
		const centerX = (area.width * this.tileSize) / 2;
		const centerZ = (area.height * this.tileSize) / 2;
		this.camera.target = new Vector3(centerX, 0, centerZ);
	}

	/**
	 * Create a 3D tile mesh
	 */
	private createTile(tile: Tile): void {
		const key = `${tile.x}-${tile.y}`;

		// Create tile mesh (hex or square based on design)
		const tileMesh = MeshBuilder.CreateBox(
			`tile-${key}`,
			{
				width: this.tileSize * 0.95,
				height: 0.1,
				depth: this.tileSize * 0.95,
			},
			this.scene
		);

		// Position tile
		tileMesh.position = new Vector3(
			tile.x * this.tileSize,
			0,
			tile.y * this.tileSize
		);

		// Apply material based on tile type
		const material = new StandardMaterial(`mat-${key}`, this.scene);
		material.diffuseColor = this.getTileColor(tile);
		tileMesh.material = material;

		this.tiles.set(key, tileMesh);
	}

	/**
	 * Get tile color based on biome and type
	 */
	private getTileColor(tile: Tile): Color3 {
		// Biome base colors (32x32 pixel art palette style)
		const biomeColors: Record<string, Color3> = {
			Forest: new Color3(0.2, 0.5, 0.2),
			Desert: new Color3(0.9, 0.8, 0.5),
			Tundra: new Color3(0.9, 0.95, 1.0),
			Swamp: new Color3(0.3, 0.4, 0.3),
			Mountains: new Color3(0.5, 0.5, 0.55),
			Plains: new Color3(0.5, 0.7, 0.3),
			Volcanic: new Color3(0.3, 0.15, 0.1),
			Ocean: new Color3(0.1, 0.3, 0.6),
			Cave: new Color3(0.2, 0.2, 0.25),
			Ruins: new Color3(0.4, 0.35, 0.3),
		};

		// Type modifiers
		if (tile.type === 'Water') {
			return new Color3(0.1, 0.4, 0.7);
		}
		if (tile.type === 'Wall') {
			return new Color3(0.3, 0.3, 0.3);
		}

		return biomeColors[tile.biome] ?? new Color3(0.5, 0.5, 0.5);
	}

	/**
	 * Clear all rendered tiles
	 */
	clearTiles(): void {
		for (const mesh of this.tiles.values()) {
			(mesh as { dispose: () => void }).dispose();
		}
		this.tiles.clear();
	}

	/**
	 * Cleanup
	 */
	dispose(): void {
		this.clearTiles();
	}
}
