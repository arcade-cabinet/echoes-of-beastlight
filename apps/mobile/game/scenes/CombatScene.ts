import {
	Scene,
	Vector3,
	MeshBuilder,
	StandardMaterial,
	Color3,
	Animation,
	type ArcRotateCamera,
} from '@babylonjs/core';
import type { CombatState, CharacterStats } from '@echoes-of-beastlight/game-core';

/**
 * CombatScene - Turn-based battle system
 * Renders player party vs enemy party with anime-style effects
 */
export class CombatScene {
	private scene: Scene;
	private camera: ArcRotateCamera;
	private playerMeshes: Map<number, unknown> = new Map();
	private enemyMeshes: Map<number, unknown> = new Map();

	constructor(scene: Scene, camera: ArcRotateCamera) {
		this.scene = scene;
		this.camera = camera;
	}

	/**
	 * Initialize combat with given state
	 */
	initCombat(combatState: CombatState): void {
		this.clearCombat();

		// Position camera for battle view
		this.camera.alpha = -Math.PI / 2;
		this.camera.beta = Math.PI / 4;
		this.camera.radius = 15;
		this.camera.target = Vector3.Zero();

		// Create player party (left side)
		combatState.playerParty.forEach((char, index) => {
			const mesh = this.createCombatantMesh(char, index, true);
			this.playerMeshes.set(index, mesh);
		});

		// Create enemy party (right side)
		combatState.enemyParty.forEach((char, index) => {
			const mesh = this.createCombatantMesh(char, index, false);
			this.enemyMeshes.set(index, mesh);
		});

		// Create battle arena floor
		this.createBattleArena();
	}

	/**
	 * Create a combatant mesh (placeholder - would load sprite/model)
	 */
	private createCombatantMesh(
		char: CharacterStats,
		index: number,
		isPlayer: boolean
	) {
		const mesh = MeshBuilder.CreateBox(
			`combatant-${isPlayer ? 'p' : 'e'}-${index}`,
			{ width: 1, height: 2, depth: 0.5 },
			this.scene
		);

		// Position based on party
		const xOffset = isPlayer ? -5 : 5;
		const zOffset = (index - 1) * 2;
		mesh.position = new Vector3(xOffset, 1, zOffset);

		// Material based on HP status
		const material = new StandardMaterial(`mat-${mesh.name}`, this.scene);
		const hpPercent = char.hp / char.maxHp;

		if (hpPercent > 0.5) {
			material.diffuseColor = isPlayer
				? new Color3(0.3, 0.6, 1.0)
				: new Color3(1.0, 0.3, 0.3);
		} else if (hpPercent > 0.25) {
			material.diffuseColor = new Color3(1.0, 0.7, 0.2);
		} else {
			material.diffuseColor = new Color3(0.8, 0.2, 0.2);
		}

		mesh.material = material;
		return mesh;
	}

	/**
	 * Create battle arena floor
	 */
	private createBattleArena(): void {
		const ground = MeshBuilder.CreateGround(
			'battleArena',
			{ width: 20, height: 10 },
			this.scene
		);

		const material = new StandardMaterial('arenaMat', this.scene);
		material.diffuseColor = new Color3(0.15, 0.15, 0.2);
		ground.material = material;
	}

	/**
	 * Play attack animation
	 */
	playAttackAnimation(
		attackerIndex: number,
		isPlayer: boolean,
		targetIndex: number
	): Promise<void> {
		return new Promise((resolve) => {
			const meshMap = isPlayer ? this.playerMeshes : this.enemyMeshes;
			const mesh = meshMap.get(attackerIndex) as { position: Vector3 } | undefined;
			if (!mesh) {
				resolve();
				return;
			}

			const originalPos = mesh.position.clone();
			const direction = isPlayer ? 1 : -1;

			// Create lunge animation
			const animation = new Animation(
				'attackAnim',
				'position.x',
				30,
				Animation.ANIMATIONTYPE_FLOAT,
				Animation.ANIMATIONLOOPMODE_CONSTANT
			);

			const keys = [
				{ frame: 0, value: originalPos.x },
				{ frame: 10, value: originalPos.x + direction * 3 },
				{ frame: 20, value: originalPos.x },
			];

			animation.setKeys(keys);
			(mesh as unknown as { animations: Animation[] }).animations = [animation];

			this.scene.beginAnimation(mesh, 0, 20, false, 1.0, () => {
				resolve();
			});
		});
	}

	/**
	 * Update combatant HP display
	 */
	updateCombatant(index: number, isPlayer: boolean, stats: CharacterStats): void {
		const meshMap = isPlayer ? this.playerMeshes : this.enemyMeshes;
		const mesh = meshMap.get(index) as { material: StandardMaterial } | undefined;
		if (!mesh?.material) return;

		const material = mesh.material as StandardMaterial;
		const hpPercent = stats.hp / stats.maxHp;

		if (stats.hp <= 0) {
			material.alpha = 0.3;
		} else if (hpPercent < 0.25) {
			material.diffuseColor = new Color3(0.8, 0.2, 0.2);
		} else if (hpPercent < 0.5) {
			material.diffuseColor = new Color3(1.0, 0.7, 0.2);
		}
	}

	/**
	 * Clear combat scene
	 */
	clearCombat(): void {
		for (const mesh of this.playerMeshes.values()) {
			(mesh as { dispose: () => void }).dispose();
		}
		for (const mesh of this.enemyMeshes.values()) {
			(mesh as { dispose: () => void }).dispose();
		}
		this.playerMeshes.clear();
		this.enemyMeshes.clear();
	}

	/**
	 * Cleanup
	 */
	dispose(): void {
		this.clearCombat();
	}
}
