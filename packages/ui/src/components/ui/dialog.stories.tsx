import type { Meta, StoryObj } from "@storybook/react-vite";
import { fn } from "storybook/test";
import { Settings, Copy } from "lucide-react";
import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
  DialogClose,
} from "./dialog";
import { Button } from "./button";

const meta = {
  title: "UI/Overlays/Dialog",
  component: Dialog,
  args: {
    onOpenChange: fn(),
  },
} satisfies Meta<typeof Dialog>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default dialog with a trigger button, header, body text, and footer actions */
export const Default: Story = {
  render: (args) => (
    <Dialog {...args}>
      <DialogTrigger asChild>
        <Button>Open Dialog</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Dialog Title</DialogTitle>
          <DialogDescription>
            This is a description of the dialog content. It provides context
            about the action or information being presented.
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">Cancel</Button>
          </DialogClose>
          <Button>Confirm</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  ),
};

/** Dialog containing a form with input fields */
export const WithForm: Story = {
  render: (args) => (
    <Dialog {...args}>
      <DialogTrigger asChild>
        <Button>
          <Settings /> Edit Profile
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Edit Profile</DialogTitle>
          <DialogDescription>
            Make changes to your profile here. Click save when you are done.
          </DialogDescription>
        </DialogHeader>
        <div className="grid gap-4 py-4">
          <div className="grid grid-cols-4 items-center gap-4">
            <label htmlFor="name" className="text-right text-sm font-medium">
              Name
            </label>
            <input
              id="name"
              defaultValue="Homunculus"
              className="col-span-3 h-9 rounded-md border border-border bg-transparent px-3 text-sm"
            />
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <label
              htmlFor="username"
              className="text-right text-sm font-medium"
            >
              Username
            </label>
            <input
              id="username"
              defaultValue="@homunculus"
              className="col-span-3 h-9 rounded-md border border-border bg-transparent px-3 text-sm"
            />
          </div>
        </div>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">Cancel</Button>
          </DialogClose>
          <Button>Save Changes</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  ),
};

/** Dialog with scrollable content for long text */
export const ScrollableContent: Story = {
  render: (args) => (
    <Dialog {...args}>
      <DialogTrigger asChild>
        <Button variant="outline">
          <Copy /> View Terms
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Terms of Service</DialogTitle>
          <DialogDescription>
            Please review the terms below before continuing.
          </DialogDescription>
        </DialogHeader>
        <div className="max-h-[300px] overflow-y-auto rounded-md border border-border p-4 text-sm text-muted-foreground">
          {Array.from({ length: 10 }, (_, i) => (
            <p key={i} className="mb-4">
              Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do
              eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim
              ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut
              aliquip ex ea commodo consequat.
            </p>
          ))}
        </div>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">Decline</Button>
          </DialogClose>
          <Button>Accept</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  ),
};

/** Dialog without the close button in the top-right corner */
export const NoCloseButton: Story = {
  render: (args) => (
    <Dialog {...args}>
      <DialogTrigger asChild>
        <Button variant="secondary">Open Without Close Button</Button>
      </DialogTrigger>
      <DialogContent showCloseButton={false}>
        <DialogHeader>
          <DialogTitle>No Close Button</DialogTitle>
          <DialogDescription>
            This dialog does not have the X close button. The user must use the
            footer actions to dismiss it.
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <DialogClose asChild>
            <Button>Got It</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  ),
};
