/**
 * Desktop Homunculus TypeScript SDK
 *
 * A comprehensive SDK for building mods and extensions for Desktop Homunculus,
 * a cross-platform desktop mascot application built with Bevy game engine.
 *
 * ## Core Features
 *
 * **VRM Character Management**
 * - Load and control 3D VRM avatars
 * - Play VRMA animations and manage character states
 * - Real-time character interaction and event handling
 *
 * **3D World Interaction**
 * - Entity Component System (ECS) for managing 3D objects
 * - Transform manipulation (position, rotation, scale)
 * - Coordinate system conversions between screen and world space
 *
 * **UI and Webview System**
 * - Embed HTML/CSS/JavaScript interfaces in 3D space
 * - Position webviews relative to characters or fixed in space
 * - Transparent overlays and interactive UI elements
 *
 * **Effects and Media**
 * - Visual stamp effects and sound playback
 * - Multi-monitor support for targeted display output
 * - Asset-based media management through the mod system
 *
 * **Cross-Process Communication**
 * - Real-time signal streaming between processes
 * - Event-driven architecture for mod communication
 * - Type-safe message passing with automatic error handling
 *
 * **Persistent Data Storage**
 * - Type-safe preference storage with JSON serialization
 * - User configuration management
 *
 * @packageDocumentation
 */

export * from './app';
export * from './assets';
export * from './audio';
export * from './coordinates';
export * from './dialog';
export * from './displays';
export * from './effects';
export * from './entities';
export * as fileDialog from './file-dialog.js';
export * from './host';
export * from './math';
export * from './mods';
export * from './persona';
export * from './preferences';
export * from './processes';
export * from './settings';
export * from './shadowPanel';
export * from './signals';
export * from './speech';
export * from './stt';
export * from './utils';
export * from './webviews';
