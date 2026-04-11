import type { Meta, StoryObj } from '@storybook/react-vite';
import { CalendarDays, Link as LinkIcon, MapPin } from 'lucide-react';
import { fn } from 'storybook/test';
import { Button } from './button';
import { HoverCard, HoverCardContent, HoverCardTrigger } from './hover-card';

const meta = {
  title: 'UI/Overlays/HoverCard',
  component: HoverCard,
  args: {
    onOpenChange: fn(),
  },
} satisfies Meta<typeof HoverCard>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default hover card showing user profile info on hover */
export const Default: Story = {
  render: (args) => (
    <HoverCard {...args}>
      <HoverCardTrigger asChild>
        <Button variant="link">@homunculus</Button>
      </HoverCardTrigger>
      <HoverCardContent>
        <div className="flex flex-col gap-2">
          <div className="flex items-center gap-2">
            <div className="flex size-10 items-center justify-center rounded-full bg-primary/20 text-sm font-bold">
              H
            </div>
            <div>
              <h4 className="text-sm font-semibold">Homunculus</h4>
              <p className="text-xs text-muted-foreground">@homunculus</p>
            </div>
          </div>
          <p className="text-sm text-muted-foreground">
            Desktop mascot application built with the Bevy game engine.
          </p>
          <div className="flex items-center gap-1 text-xs text-muted-foreground">
            <CalendarDays className="size-3" />
            Joined December 2024
          </div>
        </div>
      </HoverCardContent>
    </HoverCard>
  ),
};

/** Hover card with project details attached to a link */
export const ProjectInfo: Story = {
  render: (args) => (
    <div className="flex items-center gap-1 text-sm">
      Powered by{' '}
      <HoverCard {...args}>
        <HoverCardTrigger asChild>
          <Button variant="link" className="h-auto p-0">
            <LinkIcon className="size-3" />
            Bevy Engine
          </Button>
        </HoverCardTrigger>
        <HoverCardContent side="top">
          <div className="flex flex-col gap-2">
            <h4 className="text-sm font-semibold">Bevy Game Engine</h4>
            <p className="text-xs text-muted-foreground">
              A refreshingly simple data-driven game engine built in Rust. Free and open source
              forever.
            </p>
            <div className="flex items-center gap-3 text-xs text-muted-foreground">
              <span className="flex items-center gap-1">
                <span className="size-2 rounded-full bg-orange-500" />
                Rust
              </span>
              <span>v0.18</span>
              <span>MIT/Apache-2.0</span>
            </div>
          </div>
        </HoverCardContent>
      </HoverCard>
    </div>
  ),
};

/** Hover card showing location details */
export const LocationCard: Story = {
  render: (args) => (
    <HoverCard {...args}>
      <HoverCardTrigger asChild>
        <Button variant="ghost" className="gap-1">
          <MapPin className="size-4" />
          Tokyo, Japan
        </Button>
      </HoverCardTrigger>
      <HoverCardContent className="w-72">
        <div className="flex flex-col gap-2">
          <h4 className="text-sm font-semibold">Tokyo, Japan</h4>
          <p className="text-xs text-muted-foreground">
            Capital and most populous city of Japan. Home to many anime studios and game developers.
          </p>
          <div className="grid grid-cols-2 gap-2 text-xs text-muted-foreground">
            <div>
              <span className="font-medium text-foreground">Population</span>
              <br />
              13.96 million
            </div>
            <div>
              <span className="font-medium text-foreground">Timezone</span>
              <br />
              JST (UTC+9)
            </div>
          </div>
        </div>
      </HoverCardContent>
    </HoverCard>
  ),
};
