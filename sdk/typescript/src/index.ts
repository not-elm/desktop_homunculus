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
 * **AI Integration**
 * - GPT-powered chat interactions with customizable personalities
 * - Voice synthesis via VoiceVox integration
 * - Dynamic emotional responses and character behaviors
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
 * - Real-time command streaming between processes
 * - Event-driven architecture for mod communication
 * - Type-safe message passing with automatic error handling
 *
 * **Persistent Data Storage**
 * - Type-safe preference storage with JSON serialization
 * - Character position and state persistence
 * - User configuration management
 *
 * ## Quick Start
 *
 * ```typescript
 * import { Vrm, gpt, effects, Webview } from '@desktop-homunculus/sdk';
 *
 * // Spawn a VRM character
 * const character = await Vrm.spawn('my-character-asset');
 *
 * // Make them speak with AI
 * const response = await gpt.chat('Hello, how are you today?', {
 *   vrm: character.entity,
 *   speaker: 1
 * });
 *
 * // Play a celebration effect
 * await effects.stamp('celebration::confetti.gif');
 *
 * // Open an interactive UI
 * const ui = await Webview.open('character-status', {
 *   position: {
 *     vrm: character.entity,
 *     bone: 'head',
 *     offset: [0, 100],
 *     tracking: true
 *   },
 *   transparent: true
 * });
 * ```
 *
 * ## Architecture Overview
 *
 * The SDK is organized into focused namespaces, each handling specific aspects
 * of the Desktop Homunculus ecosystem:
 *
 * ### Core Namespaces
 *
 * - **{@link vrm}** - VRM character management and animation
 * - **{@link gpt}** - AI chat integration and voice synthesis
 * - **{@link entities}** - ECS entity management and transforms
 * - **{@link webviews}** - HTML UI embedding and management
 * - **{@link commands}** - Cross-process communication and events
 * - **{@link effects}** - Visual and audio effects system
 *
 * ### Utility Namespaces
 *
 * - **{@link cameras}** - Coordinate system transformations
 * - **{@link displays}** - Multi-monitor support and display management
 * - **{@link preferences}** - Persistent data storage
 * - **{@link settings}** - Application configuration
 * - **{@link mods}** - Mod system integration
 * - **{@link math}** - Mathematical types and utilities
 *
 * ### Internal Namespaces
 *
 * - **{@link host}** - Low-level HTTP communication (internal use)
 *
 * ## Development Patterns
 *
 * ### Event-Driven Programming
 *
 * ```typescript
 * // Listen for VRM interactions
 * const vrm = await Vrm.findByName('MyCharacter');
 * const events = vrm.events();
 *
 * events.on('pointer-click', async (event) => {
 *   await gpt.chat('Ouch! Why did you click me?', {
 *     vrm: vrm.entity
 *   });
 * });
 *
 * // Cross-process communication
 * commands.stream<{action: string}>('user-actions', async (payload) => {
 *   if (payload.action === 'wave') {
 *     const vrma = await vrm.vrma('wave-animation');
 *     await vrma.play();
 *   }
 * });
 * ```
 *
 * ### State Management
 *
 * ```typescript
 * // Save and restore application state
 * interface AppState {
 *   characters: Array<{name: string, position: Transform}>;
 *   activeMode: 'chat' | 'idle' | 'presentation';
 * }
 *
 * // Save state
 * const state: AppState = {
 *   characters: await Promise.all(
 *     (await Vrm.findAll()).map(async vrm => ({
 *       name: await vrm.name(),
 *       position: await entities.transform(vrm.entity)
 *     }))
 *   ),
 *   activeMode: 'chat'
 * };
 *
 * await preferences.save('app-state', state);
 *
 * // Restore state
 * const savedState = await preferences.load<AppState>('app-state');
 * ```
 *
 * ### Multi-Modal Interfaces
 *
 * ```typescript
 * // Create immersive experiences combining multiple systems
 * async function createInteractiveExperience() {
 *   // Position character
 *   const vrm = await Vrm.spawn('guide-character');
 *   await entities.setTransform(vrm.entity, {
 *     translation: [0, 0, 2]
 *   });
 *
 *   // Setup AI personality
 *   await gpt.saveSystemPrompt(
 *     'You are a friendly guide who helps users learn the system.',
 *     { vrm: vrm.entity }
 *   );
 *
 *   // Create floating UI
 *   const interface = await Webview.open('tutorial-interface', {
 *     position: {
 *       vrm: vrm.entity,
 *       bone: 'rightHand',
 *       offset: [100, 0],
 *       tracking: true
 *     },
 *     transparent: true,
 *     showToolbar: false
 *   });
 *
 *   // Setup interaction handlers
 *   const events = vrm.events();
 *   events.on('pointer-click', async () => {
 *     await effects.sound('ui::click.wav');
 *     await gpt.chat('Hello! How can I help you today?', {
 *       vrm: vrm.entity,
 *       speaker: 2
 *     });
 *   });
 * }
 * ```
 *
 * @packageDocumentation
 */

export * from "./displays";
export * from "./effects";
export * from "./functions";
export * from "./host";
export * from "./preferences";
export * from "./math";
export * from "./scripts";
export * from "./settings";
export * from "./shadowPanel";
export * from "./vrm";
export * from "./vrma";
export * from "./webviews";
export * from "./mods";
export * from "./gpt";
export * from "./commands";
export * from "./cameras";
export * from "./entities";
export * from "./app";