import type { Meta, StoryObj } from '@storybook/react-vite';
import { ChevronsUpDown } from 'lucide-react';
import { fn } from 'storybook/test';
import { Button } from './button';
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from './collapsible';

const meta = {
  title: 'UI/Layout/Collapsible',
  component: Collapsible,
  args: {
    onOpenChange: fn(),
  },
} satisfies Meta<typeof Collapsible>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default collapsible section, initially closed */
export const Default: Story = {
  render: () => (
    <Collapsible className="w-[350px] space-y-2">
      <div className="flex items-center justify-between space-x-4">
        <h4 className="text-sm font-semibold">3 items tagged</h4>
        <CollapsibleTrigger asChild>
          <Button variant="ghost" size="icon">
            <ChevronsUpDown className="size-4" />
            <span className="sr-only">Toggle</span>
          </Button>
        </CollapsibleTrigger>
      </div>
      <div className="rounded-md border border-border px-4 py-2 text-sm">
        item-always-visible.vrm
      </div>
      <CollapsibleContent className="space-y-2">
        <div className="rounded-md border border-border px-4 py-2 text-sm">item-hidden-1.vrma</div>
        <div className="rounded-md border border-border px-4 py-2 text-sm">item-hidden-2.vrma</div>
      </CollapsibleContent>
    </Collapsible>
  ),
};

/** Collapsible section that starts open */
export const DefaultOpen: Story = {
  render: () => (
    <Collapsible defaultOpen className="w-[350px] space-y-2">
      <div className="flex items-center justify-between space-x-4">
        <h4 className="text-sm font-semibold">Mod Details</h4>
        <CollapsibleTrigger asChild>
          <Button variant="ghost" size="icon">
            <ChevronsUpDown className="size-4" />
            <span className="sr-only">Toggle</span>
          </Button>
        </CollapsibleTrigger>
      </div>
      <div className="rounded-md border border-border px-4 py-2 text-sm">Version: 1.0.0</div>
      <CollapsibleContent className="space-y-2">
        <div className="rounded-md border border-border px-4 py-2 text-sm">
          Author: Community Contributor
        </div>
        <div className="rounded-md border border-border px-4 py-2 text-sm">License: MIT</div>
        <div className="rounded-md border border-border px-4 py-2 text-sm">
          Dependencies: @homunculus/sdk
        </div>
      </CollapsibleContent>
    </Collapsible>
  ),
};

/** Collapsible with animated content transition */
export const Animated: Story = {
  render: () => (
    <Collapsible className="w-[350px] space-y-2">
      <div className="flex items-center justify-between space-x-4">
        <h4 className="text-sm font-semibold">Advanced Options</h4>
        <CollapsibleTrigger asChild>
          <Button variant="outline" size="sm">
            <ChevronsUpDown className="size-4" />
            Show More
          </Button>
        </CollapsibleTrigger>
      </div>
      <CollapsibleContent className="space-y-2 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0">
        <div className="rounded-md border border-border p-4 text-sm">
          <p className="font-medium">Debug Mode</p>
          <p className="text-muted-foreground">
            Enable detailed logging and inspector tools for development.
          </p>
        </div>
        <div className="rounded-md border border-border p-4 text-sm">
          <p className="font-medium">Experimental Features</p>
          <p className="text-muted-foreground">Try out features that are still in development.</p>
        </div>
      </CollapsibleContent>
    </Collapsible>
  ),
};
