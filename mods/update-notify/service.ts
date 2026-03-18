import { app, mods, preferences } from "@hmcs/sdk";

const GITHUB_API_URL =
  "https://api.github.com/repos/not-elm/desktop_homunculus/releases/latest";

interface GitHubRelease {
  tag_name: string;
  html_url: string;
}

interface ReleaseInfo {
  tag: string;
  url: string;
}

await main();
process.exit(0);

async function main(): Promise<void> {
  try {
    const latestRelease = await fetchLatestRelease();
    if (!latestRelease) return;

    const currentVersion = await fetchCurrentVersion();
    const releaseTag = stripVPrefix(latestRelease.tag_name);

    if (releaseTag === currentVersion) return;

    const lastNotified = await preferences.load<string>(
      "update-notify:last-notified",
    );
    if (lastNotified === releaseTag) return;

    const releaseInfo: ReleaseInfo = {
      tag: releaseTag,
      url: latestRelease.html_url,
    };

    await preferences.save("update-notify:release", releaseInfo);
    await preferences.save("update-notify:last-notified", releaseTag);
    await mods.executeCommand({ command: "display-notify" });
  } catch {
    // silently exit on any error
  }
}

async function fetchLatestRelease(): Promise<GitHubRelease | undefined> {
  const res = await fetch(GITHUB_API_URL, {
    headers: { "User-Agent": "desktop-homunculus-update-notify" },
  });
  if (!res.ok) return undefined;
  return (await res.json()) as GitHubRelease;
}

async function fetchCurrentVersion(): Promise<string> {
  const info = await app.info();
  return info.version;
}

function stripVPrefix(tag: string): string {
  return tag.startsWith("v") ? tag.slice(1) : tag;
}
