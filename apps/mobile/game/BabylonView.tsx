import { useCallback, useEffect, useRef } from 'react';
import { View, type ViewStyle } from 'react-native';
import { EngineView, useEngine } from '@babylonjs/react-native';
import { Scene, ArcRotateCamera, HemisphericLight, Vector3, Color4 } from '@babylonjs/core';

interface BabylonViewProps {
	style?: ViewStyle;
}

export function BabylonView({ style }: BabylonViewProps) {
	const engine = useEngine();
	const sceneRef = useRef<Scene | null>(null);

	const onInitialized = useCallback(
		(view: unknown) => {
			if (!engine) return;

			// Create the scene
			const scene = new Scene(engine);
			sceneRef.current = scene;

			// Set dark background color
			scene.clearColor = new Color4(0.1, 0.1, 0.18, 1);

			// Create isometric-style camera (matching neo-tokyo BabylonJS docs)
			const camera = new ArcRotateCamera(
				'camera',
				-Math.PI / 4, // Alpha - horizontal rotation
				Math.PI / 3, // Beta - vertical angle for isometric feel
				20, // Radius - distance from target
				Vector3.Zero(),
				scene
			);

			// Limit camera movement for fixed isometric view
			camera.lowerBetaLimit = Math.PI / 4;
			camera.upperBetaLimit = Math.PI / 3;
			camera.lowerRadiusLimit = 15;
			camera.upperRadiusLimit = 30;

			// Basic ambient light
			const light = new HemisphericLight('light', new Vector3(0, 1, 0.5), scene);
			light.intensity = 0.9;

			// Start render loop
			engine.runRenderLoop(() => {
				scene.render();
			});

			// TODO: Initialize game systems
			// - Load tilemap
			// - Spawn player
			// - Initialize monster encounters
		},
		[engine]
	);

	useEffect(() => {
		return () => {
			// Cleanup on unmount
			if (sceneRef.current) {
				sceneRef.current.dispose();
			}
		};
	}, []);

	return (
		<View style={style}>
			<EngineView camera={null} onInitialized={onInitialized} />
		</View>
	);
}
