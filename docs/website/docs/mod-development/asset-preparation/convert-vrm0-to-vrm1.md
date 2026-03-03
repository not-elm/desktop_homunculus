---
title: "Convert VRM0 to VRM1"
sidebar_position: 1
---

# Convert VRM0 to VRM1

Use UniVRM to migrate a VRM 0.x model to VRM 1.0, then use the converted file in your MOD assets.

This page is a navigation guide, not a full replacement for UniVRM's own instructions.  
UniVRM workflows can change across releases, so always follow the latest official documentation first.

## What This Page Covers

- Where to find the official UniVRM docs for VRM0 -> VRM1 migration
- A task-by-task flow you can follow without relying on version-pinned UI screenshots
- How to verify the converted model before registering it as an HMCS asset

## Before You Start

- Keep a backup of your original VRM0 file.
- Check the model's license and redistribution conditions before exporting.
- If any instructions differ from this page, use the latest official UniVRM docs as the source of truth.

## Setup

**Goal**  
Prepare a Unity project with the packages needed for VRM0 import and VRM1 export.

**Use official UniVRM docs**

- [UniVRM 1.0 install guide](https://vrm.dev/en/univrm1/install/)
- [Package composition details (UPM/dependencies)](https://vrm.dev/en/api/project/packages/)

**Expected outcome**  
Your Unity project has the required UniVRM packages and is ready to load VRM0 assets.

## Load VRM0

**Goal**  
Import the source VRM0 model into Unity and prepare it for migration.

**Use official UniVRM docs**

- [VRM Import (Editor workflow)](https://vrm.dev/en/univrm/import/univrm_import/)
- [Migrate VRM-0.x to VRM-1.0 in Editor](https://vrm.dev/en/univrm1/migrate_vrm0/migrate_editor/)

**Expected outcome**  
The VRM0 asset is loaded in Unity and the migration workflow is available.

## Export as VRM1

**Goal**  
Apply migration and export the upgraded model as a VRM 1.0 file.

**Use official UniVRM docs**

- [Migrate VRM-0.x to VRM-1.0 in Editor](https://vrm.dev/en/univrm1/migrate_vrm0/migrate_editor/)
- [VRM export dialog reference](https://vrm.dev/en/univrm/export/univrm_export/)

**Expected outcome**  
You have a VRM 1.0 output file that can be tested and packaged.

## Verify the Converted Model

**Goal**  
Confirm the migrated model is usable before adding it to your MOD.

**Use official UniVRM docs**

- [Migration compatibility notes](https://vrm.dev/en/univrm1/migrate_vrm0/feature/)
- [UniVRM release notes](https://vrm.dev/en/release/)

**Expected outcome**  
You have validated that the converted model is acceptable for your use case, or identified issues that require re-export/tuning.

## Use the VRM1 File in Your MOD

When your model is ready, register it in `package.json` as a `vrm` asset.

- [Package Configuration](../project-setup/package-json.md)

The HMCS docs expect VRM assets to be VRM 1.0 format.

## Troubleshooting and Updates

- If migration behavior differs from your environment, check the latest UniVRM docs and release notes first.
- If visual differences appear after migration, review compatibility notes and re-export settings.
