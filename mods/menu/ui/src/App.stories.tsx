import type { Meta, StoryObj } from "@storybook/react-vite";
import { App } from "./App";
import { __setMockMenuItems, __resetMocks } from "./__mocks__/homunculus-api";

const meta = {
  title: "Menu/App",
  component: App,
  beforeEach: () => {
    __resetMocks();
  },
} satisfies Meta<typeof App>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default menu with grouped items from 2 mods */
export const Default: Story = {};

/** Empty menu — component returns null when no items */
export const Empty: Story = {
  beforeEach: () => {
    __setMockMenuItems([]);
  },
};

/** Single mod — labels are hidden when only one mod is present */
export const SingleMod: Story = {
  beforeEach: () => {
    __setMockMenuItems([
      { id: "1", modName: "Elmer", text: "Greet", command: "greet" },
      { id: "2", modName: "Elmer", text: "Dance", command: "dance" },
      { id: "3", modName: "Elmer", text: "Wave", command: "wave" },
    ]);
  },
};

/** Many items from multiple mods to verify scroll and stagger behavior */
export const ManyItems: Story = {
  beforeEach: () => {
    __setMockMenuItems([
      { id: "1", modName: "Elmer", text: "Greet", command: "greet" },
      { id: "2", modName: "Elmer", text: "Dance", command: "dance" },
      { id: "3", modName: "VoiceVox", text: "Speak", command: "speak" },
      { id: "4", modName: "VoiceVox", text: "Change Voice", command: "voice" },
      { id: "5", modName: "Expressions", text: "Smile", command: "smile" },
      { id: "6", modName: "Expressions", text: "Surprise", command: "surprise" },
      { id: "7", modName: "Settings", text: "Open Settings", command: "settings" },
      { id: "8", modName: "Settings", text: "About", command: "about" },
    ]);
  },
};
