import React from "react";

const sceneStyles: Record<string, React.CSSProperties> = {
  "dark-space": {
    background: "oklch(0.08 0.025 250)",
  },
  "vrm-scene": {
    background:
      "linear-gradient(135deg, oklch(0.12 0.04 280) 0%, oklch(0.08 0.03 220) 40%, oklch(0.15 0.05 190) 70%, oklch(0.1 0.04 250) 100%)",
  },
  checkerboard: {
    backgroundImage:
      "repeating-conic-gradient(oklch(0.2 0 0) 0% 25%, oklch(0.15 0 0) 0% 50%)",
    backgroundSize: "20px 20px",
  },
  light: {
    background: "oklch(0.95 0.005 250)",
  },
};

/**
 * 2-layer decorator: scene layer (background) + UI layer (component).
 * Simulates the real CEF transparent-window environment where
 * components render with `backdrop-filter: blur()` over a 3D scene.
 */
export function withSceneLayer(Story: React.ComponentType, context: { globals: Record<string, string> }): React.ReactElement {
  const scene = context.globals["scene"] || "dark-space";
  const theme = context.globals["theme"] || "dark";

  // eslint-disable-next-line react-hooks/rules-of-hooks -- Storybook decorator, not a React component
  React.useEffect(() => {
    const root = document.documentElement;
    if (theme === "dark") {
      root.classList.add("dark");
    } else {
      root.classList.remove("dark");
    }
  }, [theme]);

  return React.createElement(
    "div",
    {
      className: theme === "dark" ? "dark" : "",
      style: {
        position: "relative",
        minHeight: "100vh",
        ...sceneStyles[scene],
      },
    },
    React.createElement("div", {
      style: { position: "absolute", inset: 0, zIndex: 0 },
      "aria-hidden": true,
    }),
    React.createElement(
      "div",
      { style: { position: "relative", zIndex: 1, padding: "2rem" } },
      React.createElement(Story, null)
    )
  );
}

export const sceneGlobalType = {
  description: "Background scene for glassmorphism preview",
  toolbar: {
    title: "Scene",
    icon: "photo",
    items: [
      { value: "dark-space", title: "Dark Space", icon: "moon" },
      { value: "vrm-scene", title: "VRM Scene", icon: "paintbrush" },
      { value: "checkerboard", title: "Checkerboard", icon: "grid" },
      { value: "light", title: "Light", icon: "sun" },
    ],
    dynamicTitle: true,
  },
};

export const themeGlobalType = {
  description: "Theme (controls .dark class)",
  toolbar: {
    title: "Theme",
    icon: "contrast",
    items: [
      { value: "dark", title: "Dark", icon: "moon" },
      { value: "light", title: "Light", icon: "sun" },
    ],
    dynamicTitle: true,
  },
};

export const defaultInitialGlobals = {
  scene: "dark-space",
  theme: "dark",
};
