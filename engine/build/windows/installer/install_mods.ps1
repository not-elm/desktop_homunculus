# install_mods.ps1 — Post-install MOD setup for Windows MSI installer.
#
# Dynamically discovers official @hmcs/ MODs from the npm registry and
# installs them using the bundled Node.js + pnpm runtime.
#
# This script is designed to be best-effort: failures are logged but
# do not block the installer (WiX Return="ignore").

param(
    [string]$InstallDir = ""
)

$ErrorActionPreference = "Continue"

# ── Resolve paths ────────────────────────────────────────────────────────
if (-not $InstallDir) {
    $InstallDir = Split-Path -Parent $MyInvocation.MyCommand.Path
}

$RuntimeDir = Join-Path $InstallDir "runtime"
$NodeExe = Join-Path $RuntimeDir "node\node.exe"
$PnpmCjs = Join-Path $RuntimeDir "pnpm\bin\pnpm.cjs"
$ModsDir = Join-Path $env:USERPROFILE ".homunculus\mods"

Write-Host "InstallDir: $InstallDir"
Write-Host "ModsDir: $ModsDir"

# ── Verify bundled runtime ───────────────────────────────────────────────
if (-not (Test-Path $NodeExe)) {
    Write-Warning "Bundled Node.js not found at $NodeExe. Skipping MOD install."
    exit 0
}

# ── Create mods directory ────────────────────────────────────────────────
if (-not (Test-Path $ModsDir)) {
    New-Item -ItemType Directory -Path $ModsDir -Force | Out-Null
}

$PackageJson = Join-Path $ModsDir "package.json"
if (-not (Test-Path $PackageJson)) {
    Set-Content -Path $PackageJson -Value "{}"
}

# ── Dynamic MOD discovery ────────────────────────────────────────────────
Write-Host "Discovering official MODs from npm registry..."

$DiscoverScript = @'
const https = require('https');

function fetch(url) {
    return new Promise((resolve, reject) => {
        https.get(url, { timeout: 30000 }, (res) => {
            let data = '';
            res.on('data', (chunk) => data += chunk);
            res.on('end', () => resolve({ status: res.statusCode, body: data }));
        }).on('error', reject);
    });
}

async function main() {
    // List all packages under the @hmcs scope via the org-listing endpoint
    // (used by `npm access list packages <scope>`); /-/v1/search does not
    // reliably index scoped packages.
    const listRes = await fetch('https://registry.npmjs.org/-/org/hmcs/package');
    if (listRes.status !== 200) {
        process.stderr.write('npm org listing failed: HTTP ' + listRes.status + '\n');
        process.exit(1);
    }

    const packages = Object.keys(JSON.parse(listRes.body));
    const mods = [];

    for (const name of packages) {
        try {
            const pkgRes = await fetch('https://registry.npmjs.org/' + encodeURIComponent(name) + '/latest');
            if (pkgRes.status !== 200) continue;
            const pkg = JSON.parse(pkgRes.body);
            if (pkg.homunculus) {
                mods.push(name);
            }
        } catch (_) {}
    }

    process.stdout.write(mods.join(' '));
}

main().catch((e) => {
    process.stderr.write('MOD discovery failed: ' + e.message + '\n');
    process.exit(1);
});
'@

try {
    $OfficialMods = & $NodeExe -e $DiscoverScript
} catch {
    Write-Warning "MOD discovery failed: $_"
    $OfficialMods = ""
}

# ── Install discovered MODs ──────────────────────────────────────────────
if ($OfficialMods -and $OfficialMods.Trim()) {
    $ModList = $OfficialMods.Trim()
    Write-Host "Installing official MODs: $ModList"

    $PnpmArgs = @($PnpmCjs, "-C", $ModsDir, "add", "--ignore-scripts") + $ModList.Split(" ")
    try {
        & $NodeExe @PnpmArgs
        Write-Host "MOD installation complete."
    } catch {
        Write-Warning "MOD installation failed: $_. Run 'hmcs mod install' manually after launch."
    }
} else {
    Write-Warning "Could not discover official MODs. They can be installed later."
}

Write-Host "install_mods.ps1 complete."
