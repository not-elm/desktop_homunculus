import type { Meta, StoryObj } from "@storybook/react-vite";
import { fn } from "storybook/test";
import { Settings, User, Bell } from "lucide-react";
import {
  Accordion,
  AccordionItem,
  AccordionTrigger,
  AccordionContent,
} from "./accordion";

const meta = {
  title: "UI/Layout/Accordion",
  component: Accordion,
  args: {
    type: "single" as const,
    collapsible: true,
    onValueChange: fn(),
  },
} satisfies Meta<typeof Accordion>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default accordion with a single expandable item */
export const Default: Story = {
  render: (_args) => (
    <Accordion type="single" collapsible className="w-[400px]">
      <AccordionItem value="item-1">
        <AccordionTrigger>Is this accessible?</AccordionTrigger>
        <AccordionContent>
          Yes. It adheres to the WAI-ARIA design pattern for accordions.
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  ),
};

/** Accordion with multiple items, only one open at a time */
export const MultipleItems: Story = {
  render: (_args) => (
    <Accordion type="single" collapsible className="w-[400px]">
      <AccordionItem value="item-1">
        <AccordionTrigger>What is Desktop Homunculus?</AccordionTrigger>
        <AccordionContent>
          Desktop Homunculus is a cross-platform desktop mascot application built
          with the Bevy game engine. It renders transparent-window VRM 3D
          characters with WebView-based UI overlays.
        </AccordionContent>
      </AccordionItem>
      <AccordionItem value="item-2">
        <AccordionTrigger>How do I install mods?</AccordionTrigger>
        <AccordionContent>
          Mods are NPM packages installed in the mods root directory. Each mod
          declares its assets and scripts in package.json under the
          &quot;homunculus&quot; field.
        </AccordionContent>
      </AccordionItem>
      <AccordionItem value="item-3">
        <AccordionTrigger>Can I customize the character?</AccordionTrigger>
        <AccordionContent>
          Yes. You can load any VRM model and apply animations using VRMA files.
          The SDK provides APIs for controlling poses, expressions, and effects.
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  ),
};

/** Accordion where multiple items can be open simultaneously */
export const MultipleOpen: Story = {
  args: { type: "multiple" as const },
  render: (_args) => (
    <Accordion type="multiple" defaultValue={["item-1"]} className="w-[400px]">
      <AccordionItem value="item-1">
        <AccordionTrigger>Section A (open by default)</AccordionTrigger>
        <AccordionContent>
          This section starts expanded. Other sections can also be opened
          without closing this one.
        </AccordionContent>
      </AccordionItem>
      <AccordionItem value="item-2">
        <AccordionTrigger>Section B</AccordionTrigger>
        <AccordionContent>
          Open this alongside Section A to see multiple panels expanded at once.
        </AccordionContent>
      </AccordionItem>
      <AccordionItem value="item-3">
        <AccordionTrigger>Section C</AccordionTrigger>
        <AccordionContent>
          All three sections can be open at the same time in this mode.
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  ),
};

/** Accordion items with icons alongside the trigger text */
export const WithIcons: Story = {
  render: (_args) => (
    <Accordion type="single" collapsible className="w-[400px]">
      <AccordionItem value="item-1">
        <AccordionTrigger>
          <span className="flex items-center gap-2">
            <User className="size-4" />
            Account Settings
          </span>
        </AccordionTrigger>
        <AccordionContent>
          Manage your profile information, display name, and avatar preferences.
        </AccordionContent>
      </AccordionItem>
      <AccordionItem value="item-2">
        <AccordionTrigger>
          <span className="flex items-center gap-2">
            <Bell className="size-4" />
            Notifications
          </span>
        </AccordionTrigger>
        <AccordionContent>
          Configure which notifications you receive and how they are delivered.
        </AccordionContent>
      </AccordionItem>
      <AccordionItem value="item-3">
        <AccordionTrigger>
          <span className="flex items-center gap-2">
            <Settings className="size-4" />
            Advanced
          </span>
        </AccordionTrigger>
        <AccordionContent>
          Developer options, debug logging, and experimental features.
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  ),
};
