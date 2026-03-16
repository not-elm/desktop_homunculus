---
sidebar_position: 8
---

# bgm.status

Gets the current BGM playback status.

## Parameters

None.

## Returns

`Promise<BgmStatus>`

## Example

```typescript
const status = await audio.bgm.status();
if (status.state === "playing") {
  console.log(`Now playing: ${status.asset} at volume ${status.volume}`);
}
```
