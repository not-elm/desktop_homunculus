#!/usr/bin/env tsx

/// <reference types="node" />

import { app } from "@hmcs/sdk";
import { output } from "@hmcs/sdk/commands";

await app.exit();
output.succeed();
