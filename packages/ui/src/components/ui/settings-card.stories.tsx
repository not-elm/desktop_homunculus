import type { Meta, StoryObj } from '@storybook/react-vite';
import { Checkbox } from './checkbox';
import { Input } from './input';
import { Label } from './label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from './select';
import { Separator } from './separator';
import { SettingsCard } from './settings-card';

const meta = {
  title: 'UI/Layout/SettingsCard',
  component: SettingsCard,
  args: {
    title: 'Settings',
  },
} satisfies Meta<typeof SettingsCard>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default settings card with title and simple content */
export const Default: Story = {
  args: {
    title: 'General',
    description: 'Basic application settings.',
    children: (
      <div className="flex flex-col gap-2">
        <Label htmlFor="app-name">Application Name</Label>
        <Input id="app-name" placeholder="My Homunculus" />
      </div>
    ),
  },
};

/** Settings card with a description and multiple form fields */
export const WithDescription: Story = {
  args: {
    title: 'Display',
    description: 'Configure how the mascot appears on your desktop.',
    children: (
      <div className="flex flex-col gap-4">
        <div className="flex flex-col gap-2">
          <Label htmlFor="window-opacity">Window Opacity</Label>
          <Input id="window-opacity" type="number" placeholder="100" />
        </div>
        <div className="flex flex-col gap-2">
          <Label htmlFor="scale">Scale Factor</Label>
          <Input id="scale" type="number" placeholder="1.0" />
        </div>
      </div>
    ),
  },
};

/** Settings card showing checkbox toggles for preferences */
export const WithToggles: Story = {
  args: {
    title: 'Notifications',
    description: 'Choose which notifications to receive.',
    children: (
      <div className="flex flex-col gap-3">
        <div className="flex items-center gap-2">
          <Checkbox id="notify-mods" defaultChecked />
          <Label htmlFor="notify-mods">Mod updates</Label>
        </div>
        <div className="flex items-center gap-2">
          <Checkbox id="notify-system" defaultChecked />
          <Label htmlFor="notify-system">System alerts</Label>
        </div>
        <div className="flex items-center gap-2">
          <Checkbox id="notify-chat" />
          <Label htmlFor="notify-chat">Chat messages</Label>
        </div>
      </div>
    ),
  },
};

/** Settings card with a select dropdown and separated sections */
export const WithSelect: Story = {
  args: {
    title: 'Audio',
    description: 'Configure speech and sound settings.',
    children: (
      <div className="flex flex-col gap-4">
        <div className="flex flex-col gap-2">
          <Label>TTS Engine</Label>
          <Select defaultValue="voicevox">
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Select engine" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="voicevox">VoiceVox</SelectItem>
              <SelectItem value="system">System TTS</SelectItem>
              <SelectItem value="none">None</SelectItem>
            </SelectContent>
          </Select>
        </div>
        <Separator />
        <div className="flex flex-col gap-2">
          <Label htmlFor="volume">Volume</Label>
          <Input id="volume" type="number" placeholder="80" />
        </div>
      </div>
    ),
  },
};
