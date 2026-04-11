import type { Meta, StoryObj } from '@storybook/react-vite';
import { Menu, Settings } from 'lucide-react';
import { Button } from './button';
import {
  Drawer,
  DrawerClose,
  DrawerContent,
  DrawerDescription,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
  DrawerTrigger,
} from './drawer';

const meta = {
  title: 'UI/Overlays/Drawer',
  component: Drawer,
} satisfies Meta<typeof Drawer>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default bottom drawer with header, content, and footer */
export const Default: Story = {
  render: (args) => (
    <Drawer {...args}>
      <DrawerTrigger asChild>
        <Button>Open Drawer</Button>
      </DrawerTrigger>
      <DrawerContent>
        <DrawerHeader>
          <DrawerTitle>Drawer Title</DrawerTitle>
          <DrawerDescription>
            This is a bottom drawer with a drag handle. Swipe down to close.
          </DrawerDescription>
        </DrawerHeader>
        <div className="p-4">
          <p className="text-sm text-muted-foreground">
            Drawer body content goes here. This area can contain any elements.
          </p>
        </div>
        <DrawerFooter>
          <Button>Submit</Button>
          <DrawerClose asChild>
            <Button variant="outline">Cancel</Button>
          </DrawerClose>
        </DrawerFooter>
      </DrawerContent>
    </Drawer>
  ),
};

/** Drawer with form inputs for editing settings */
export const WithForm: Story = {
  render: (args) => (
    <Drawer {...args}>
      <DrawerTrigger asChild>
        <Button variant="outline">
          <Settings /> Adjust Settings
        </Button>
      </DrawerTrigger>
      <DrawerContent>
        <DrawerHeader>
          <DrawerTitle>Display Settings</DrawerTitle>
          <DrawerDescription>Adjust the display settings for your mascot.</DrawerDescription>
        </DrawerHeader>
        <div className="grid gap-4 p-4">
          <div className="grid gap-2">
            <label htmlFor="scale" className="text-sm font-medium">
              Scale
            </label>
            <input
              id="scale"
              type="range"
              min="50"
              max="200"
              defaultValue="100"
              className="w-full"
            />
          </div>
          <div className="grid gap-2">
            <label htmlFor="opacity" className="text-sm font-medium">
              Opacity
            </label>
            <input
              id="opacity"
              type="range"
              min="0"
              max="100"
              defaultValue="100"
              className="w-full"
            />
          </div>
        </div>
        <DrawerFooter>
          <Button>Apply</Button>
          <DrawerClose asChild>
            <Button variant="outline">Cancel</Button>
          </DrawerClose>
        </DrawerFooter>
      </DrawerContent>
    </Drawer>
  ),
};

/** Right-side drawer for navigation or side panels */
export const RightSide: Story = {
  render: (args) => (
    <Drawer {...args} direction="right">
      <DrawerTrigger asChild>
        <Button variant="outline">
          <Menu /> Open Side Panel
        </Button>
      </DrawerTrigger>
      <DrawerContent>
        <DrawerHeader>
          <DrawerTitle>Navigation</DrawerTitle>
          <DrawerDescription>Browse through sections.</DrawerDescription>
        </DrawerHeader>
        <div className="flex flex-col gap-1 p-4">
          {['Dashboard', 'Characters', 'Animations', 'Effects', 'Mods'].map((item) => (
            <button
              key={item}
              className="rounded-md px-3 py-2 text-left text-sm hover:bg-accent transition-colors"
            >
              {item}
            </button>
          ))}
        </div>
        <DrawerFooter>
          <DrawerClose asChild>
            <Button variant="outline">Close</Button>
          </DrawerClose>
        </DrawerFooter>
      </DrawerContent>
    </Drawer>
  ),
};

/** Left-side drawer */
export const LeftSide: Story = {
  render: (args) => (
    <Drawer {...args} direction="left">
      <DrawerTrigger asChild>
        <Button variant="secondary">Open Left Drawer</Button>
      </DrawerTrigger>
      <DrawerContent>
        <DrawerHeader>
          <DrawerTitle>Sidebar</DrawerTitle>
          <DrawerDescription>Left-side drawer panel.</DrawerDescription>
        </DrawerHeader>
        <div className="p-4">
          <p className="text-sm text-muted-foreground">
            This drawer slides in from the left side of the screen.
          </p>
        </div>
        <DrawerFooter>
          <DrawerClose asChild>
            <Button variant="outline">Close</Button>
          </DrawerClose>
        </DrawerFooter>
      </DrawerContent>
    </Drawer>
  ),
};
