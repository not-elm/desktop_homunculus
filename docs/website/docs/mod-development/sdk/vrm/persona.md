---
title: "Persona"
sidebar_position: 7
---

# Persona

Get and set character personality data. A persona includes a profile description, a natural-language personality string, OCEAN personality traits, and arbitrary metadata for MOD extensions.

## Import

```typescript
import { Vrm } from "@hmcs/sdk";
```

## Get Persona

`vrm.persona()` returns the character's current persona data.

```typescript
const character = await Vrm.findByName("MyAvatar");
const persona = await character.persona();

console.log("Profile:", persona.profile);
console.log("Personality:", persona.personality);
console.log("Openness:", persona.ocean.openness);
console.log("Metadata:", persona.metadata);
```

## Set Persona

`vrm.setPersona(persona)` replaces the character's entire persona.

```typescript
await character.setPersona({
  profile: "A cheerful virtual assistant who loves to help with coding.",
  personality: "Friendly, curious, and always enthusiastic about learning new things.",
  ocean: {
    openness: 0.9,
    conscientiousness: 0.7,
    extraversion: 0.8,
    agreeableness: 0.85,
    neuroticism: 0.2,
  },
  metadata: {
    favoriteColor: "blue",
    skills: ["typescript", "rust"],
  },
});
```

### Setting Persona at Spawn

You can set the initial persona when spawning a character:

```typescript
const character = await Vrm.spawn("my-mod:character", {
  persona: {
    profile: "A quiet observer who watches the screen.",
    ocean: { extraversion: 0.2, neuroticism: 0.1 },
    metadata: {},
  },
});
```

## OCEAN Model

The Big Five personality traits, each a number from 0.0 to 1.0. All fields are optional.

| Trait | Low (0.0) | High (1.0) |
|---|---|---|
| `openness` | Conservative, practical | Curious, imaginative |
| `conscientiousness` | Spontaneous, flexible | Organized, disciplined |
| `extraversion` | Introverted, reserved | Extroverted, outgoing |
| `agreeableness` | Independent, competitive | Cooperative, trusting |
| `neuroticism` | Stable, calm | Sensitive, emotional |

```typescript
const ocean: Ocean = {
  openness: 0.8,
  conscientiousness: 0.6,
  extraversion: 0.7,
  agreeableness: 0.9,
  neuroticism: 0.3,
};
```

## Metadata

The `metadata` field is a `Record<string, unknown>` for MOD-specific extensions. Use it to store arbitrary data about the character.

```typescript
await character.setPersona({
  profile: "A helpful assistant",
  ocean: {},
  metadata: {
    voiceId: "en-US-1",
    greeting: "Hello! How can I help you today?",
    customTraits: {
      humor: 0.7,
      formality: 0.3,
    },
  },
});
```

## Listening for Changes

Use the [events system](./events) to react when a character's persona is updated:

```typescript
const eventSource = character.events();

eventSource.on("persona-change", (e) => {
  console.log("Persona updated:");
  console.log("  Profile:", e.persona.profile);
  console.log("  Openness:", e.persona.ocean.openness);
});
```

## Types

```typescript
interface Persona {
  /** Character profile/background description. */
  profile: string;
  /** Personality description in natural language. */
  personality?: string | null;
  /** Big Five personality parameters. */
  ocean: Ocean;
  /** Extension metadata for MODs. */
  metadata: Record<string, unknown>;
}

interface Ocean {
  openness?: number;
  conscientiousness?: number;
  extraversion?: number;
  agreeableness?: number;
  neuroticism?: number;
}
```

## Next Steps

- **[Events](./events)** -- Subscribe to `persona-change` and other character events.
- **[Spawn & Find](./spawn-and-find)** -- Set persona at spawn time.
- **[VRM Overview](./)** -- Full API reference table.
