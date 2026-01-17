/**
 * @echoes-of-beastlight/game-core
 * Shared game logic, schemas, and utilities for Echoes of Beastlight
 *
 * This package is framework-agnostic and can be used by:
 * - React Native mobile app (apps/mobile)
 * - Future web client
 * - Server-side validation
 */

// Re-export generation utilities
export * from './generation/index.js';
// Re-export all schemas
export * from './schemas/index.js';

// Version info
export const VERSION = '1.0.0';
