import {
  type TransformArgs,
  Vrm,
  preferences,
  repeat,
  sleep,
  Character,
} from "@hmcs/sdk";
const elmerId = "elmer";
const elmerAssetId = "vrm:elmer";
const transform = await preferences.load<TransformArgs>(
  `transform::${elmerAssetId}`,
);
const elmer = await Character.create({
  id: elmerId,
});
const vrm = await elmer.attachVrm(elmerAssetId);
const option = {
  repeat: repeat.forever(),
  transitionSecs: 0.5,
} as const;
await vrm.playVrma({
  asset: "vrma:idle-maid",
  ...option,
});
(await vrm.events()).on("state-change", async (e) => {
  console.log("state-change", e);
  if (e.state === "idle") {
    await vrm.playVrma({
      asset: "vrma:idle-maid",
      ...option,
    });
    await sleep(500);
    await vrm.lookAtCursor();
  } else if (e.state === "drag") {
    await vrm.unlook();
    await vrm.playVrma({
      asset: "vrma:grabbed",
      ...option,
      resetSpringBones: true,
    });
  } else if (e.state === "sitting") {
    await vrm.playVrma({
      asset: "vrma:idle-sitting",
      ...option,
    });
    await sleep(500);
    await vrm.lookAtCursor();
  }
});
