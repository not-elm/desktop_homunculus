---
title: "Create VRMA Animations"
sidebar_position: 2
---

# Create VRMA Animations

Create VRMA clips with Blender and the VRM Add-on for Blender, then use those clips in Desktop Homunculus MODs.

This page is a workflow guide, not a full DCC tutorial.  
For exact UI operations, always follow the latest official Blender and VRM Add-on documentation.

## What This Page Covers

- A recommended authoring workflow for VRMA clips
- How to move from DCC authoring to HMCS asset usage
- Where to verify playback after export

## Recommended Workflow

Use **Blender + VRM Add-on for Blender** as the default path for VRMA authoring.  
This workflow can include facial expression tracks in the exported animation clip.

## Before You Start

- Prepare a VRM model that you will animate.
- Keep source files versioned so you can iterate safely.
- Verify licenses and redistribution terms before packaging assets.

## Set Up Blender and VRM Add-on

**Goal**  
Install and configure the authoring environment for VRM-based animation work.

**Use official docs**

- [Blender Manual](https://docs.blender.org/manual/en/latest/)
- [VRM Add-on for Blender documentation](https://vrm-addon-for-blender.info/en/)
- [VRM Add-on for Blender releases](https://github.com/saturday06/VRM-Addon-for-Blender/releases)

**Expected outcome**  
Blender and the VRM Add-on are installed and ready for importing a VRM model.

## Create Animation Clips and Expressions

**Goal**  
Author skeletal motion and expression tracks in your DCC scene.

**Use official docs**

- [VRM Add-on for Blender documentation](https://vrm-addon-for-blender.info/en/)
- [Blender Animation & Rigging manual](https://docs.blender.org/manual/en/latest/animation/index.html)

**Expected outcome**  
Your scene contains a reusable animation clip with the motion and expressions you want to ship.

## Export as VRMA

**Goal**  
Export your authored clip into VRMA format for runtime playback.

**Use official docs**

- [VRM Add-on for Blender documentation](https://vrm-addon-for-blender.info/en/)
- [VRM specification portal](https://vrm.dev/en/)

**Expected outcome**  
You have a `.vrma` file ready to register as an HMCS asset.

## Verify in Desktop Homunculus

**Goal**  
Confirm the exported VRMA behaves as expected in the runtime environment.

**Use official docs**

- [SDK: VRM Animations](../sdk/vrm/animations.md)

**Expected outcome**  
The animation plays correctly on your target character, including expected pose and expression behavior.

## Use VRMA in Your MOD

Register the exported `.vrma` file in your MOD's `package.json` and play it from the SDK.

- [Package Configuration](../project-setup/package-json.md)
- [SDK: VRM Animations](../sdk/vrm/animations.md)

## Troubleshooting and Updates

- If export or playback behavior differs from your setup, check the latest Blender and VRM Add-on docs first.
- If results differ after updates, re-check export options and rerun runtime verification in HMCS.
