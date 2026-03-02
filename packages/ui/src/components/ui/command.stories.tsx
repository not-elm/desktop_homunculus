import * as React from "react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import {
  Calculator,
  Calendar,
  CreditCard,
  Settings,
  User,
  Smile,
  FileText,
  Moon,
  Sun,
} from "lucide-react";
import {
  Command,
  CommandDialog,
  CommandInput,
  CommandList,
  CommandEmpty,
  CommandGroup,
  CommandItem,
  CommandSeparator,
  CommandShortcut,
} from "./command";
import { Button } from "./button";

const meta = {
  title: "UI/Overlays/Command",
  component: Command,
} satisfies Meta<typeof Command>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default inline command palette with search and grouped items */
export const Default: Story = {
  render: () => (
    <Command className="rounded-lg border border-border shadow-md md:min-w-[450px]">
      <CommandInput placeholder="Type a command or search..." />
      <CommandList>
        <CommandEmpty>No results found.</CommandEmpty>
        <CommandGroup heading="Suggestions">
          <CommandItem>
            <Calendar />
            Calendar
          </CommandItem>
          <CommandItem>
            <Smile />
            Search Emoji
          </CommandItem>
          <CommandItem>
            <Calculator />
            Calculator
          </CommandItem>
        </CommandGroup>
        <CommandSeparator />
        <CommandGroup heading="Settings">
          <CommandItem>
            <User />
            Profile
            <CommandShortcut>Cmd+P</CommandShortcut>
          </CommandItem>
          <CommandItem>
            <CreditCard />
            Billing
            <CommandShortcut>Cmd+B</CommandShortcut>
          </CommandItem>
          <CommandItem>
            <Settings />
            Settings
            <CommandShortcut>Cmd+S</CommandShortcut>
          </CommandItem>
        </CommandGroup>
      </CommandList>
    </Command>
  ),
};

/** Command dialog opened via a trigger button, simulating Cmd+K behavior */
export const AsDialog: Story = {
  render: () => {
    const [open, setOpen] = React.useState(false);
    return (
      <>
        <Button variant="outline" onClick={() => setOpen(true)}>
          Open Command Palette (Cmd+K)
        </Button>
        <CommandDialog open={open} onOpenChange={setOpen}>
          <CommandInput placeholder="Type a command or search..." />
          <CommandList>
            <CommandEmpty>No results found.</CommandEmpty>
            <CommandGroup heading="Actions">
              <CommandItem>
                <FileText />
                New File
                <CommandShortcut>Cmd+N</CommandShortcut>
              </CommandItem>
              <CommandItem>
                <Settings />
                Open Settings
                <CommandShortcut>Cmd+,</CommandShortcut>
              </CommandItem>
              <CommandItem>
                <User />
                Switch Profile
              </CommandItem>
            </CommandGroup>
            <CommandSeparator />
            <CommandGroup heading="Theme">
              <CommandItem>
                <Sun />
                Light Mode
              </CommandItem>
              <CommandItem>
                <Moon />
                Dark Mode
              </CommandItem>
            </CommandGroup>
          </CommandList>
        </CommandDialog>
      </>
    );
  },
};

/** Command palette with no results state */
export const EmptyState: Story = {
  render: () => (
    <Command className="rounded-lg border border-border shadow-md md:min-w-[450px]">
      <CommandInput placeholder="Search..." defaultValue="xyznonexistent" />
      <CommandList>
        <CommandEmpty>No results found.</CommandEmpty>
        <CommandGroup heading="Recent">
          <CommandItem>
            <FileText />
            document.tsx
          </CommandItem>
          <CommandItem>
            <FileText />
            settings.json
          </CommandItem>
        </CommandGroup>
      </CommandList>
    </Command>
  ),
};

/** Command palette with file search items */
export const FileSearch: Story = {
  render: () => (
    <Command className="rounded-lg border border-border shadow-md md:min-w-[450px]">
      <CommandInput placeholder="Search files..." />
      <CommandList>
        <CommandEmpty>No files found.</CommandEmpty>
        <CommandGroup heading="Components">
          <CommandItem>
            <FileText />
            src/components/ui/button.tsx
          </CommandItem>
          <CommandItem>
            <FileText />
            src/components/ui/dialog.tsx
          </CommandItem>
          <CommandItem>
            <FileText />
            src/components/ui/command.tsx
          </CommandItem>
        </CommandGroup>
        <CommandSeparator />
        <CommandGroup heading="Configuration">
          <CommandItem>
            <FileText />
            tsconfig.json
          </CommandItem>
          <CommandItem>
            <FileText />
            vite.config.ts
          </CommandItem>
          <CommandItem>
            <FileText />
            package.json
          </CommandItem>
        </CommandGroup>
      </CommandList>
    </Command>
  ),
};
