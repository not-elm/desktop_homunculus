import type { Config } from "@docusaurus/types";
import type * as Preset from "@docusaurus/preset-classic";

const config: Config = {
  title: "Desktop Homunculus",
  tagline: "Your AI-powered desktop companion",
  favicon: "img/favicon.ico",

  url: "https://not-elm.github.io",
  baseUrl: "/desktop-homunculus/",

  organizationName: "not-elm",
  projectName: "desktop-homunculus",

  onBrokenLinks: "throw",

  markdown: {
    hooks: {
      onBrokenMarkdownLinks: "warn",
    },
  },

  i18n: {
    defaultLocale: "en",
    locales: ["en", "ja"],
  },

  plugins: [
    [
      "docusaurus-plugin-openapi-docs",
      {
        id: "api",
        docsPluginId: "classic",
        config: {
          homunculus: {
            specPath: "static/api/open-api.yml",
            outputDir: "docs/reference/api",
            sidebarOptions: {
              groupPathsBy: "tag",
            },
          },
        },
      },
    ],
  ],

  themes: [
    "docusaurus-theme-openapi-docs",
    [
      "@easyops-cn/docusaurus-search-local",
      {
        hashed: true,
        language: ["en", "ja"],
      },
    ],
  ],

  presets: [
    [
      "classic",
      {
        docs: {
          sidebarPath: "./sidebars.ts",
          docItemComponent: "@theme/ApiItem",
          editUrl:
            "https://github.com/not-elm/desktop-homunculus/tree/main/docs/website/",
        },
        blog: false,
        theme: {
          customCss: "./src/css/custom.css",
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    navbar: {
      title: "Desktop Homunculus",
      items: [
        {
          type: "docSidebar",
          sidebarId: "docs",
          position: "left",
          label: "Docs",
        },
        {
          type: "doc",
          docId: "reference/api/homunculus-api",
          position: "left",
          label: "API Reference",
        },
        {
          type: "localeDropdown",
          position: "right",
        },
        {
          href: "https://github.com/not-elm/desktop-homunculus",
          label: "GitHub",
          position: "right",
        },
      ],
    },
    footer: {
      style: "dark",
      links: [
        {
          title: "Docs",
          items: [
            { label: "Getting Started", to: "/docs/getting-started" },
            { label: "MOD Development", to: "/docs/mod-development" },
          ],
        },
        {
          title: "Community",
          items: [
            {
              label: "GitHub Issues",
              href: "https://github.com/not-elm/desktop-homunculus/issues",
            },
            {
              label: "GitHub Discussions",
              href: "https://github.com/not-elm/desktop-homunculus/discussions",
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} not-elm. Licensed per component (MIT/Apache-2.0 for Rust, MIT for TypeScript, CC-BY-4.0 for docs/assets).`,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
