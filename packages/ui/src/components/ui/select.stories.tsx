import type { Meta, StoryObj } from '@storybook/react-vite';
import { Globe, Monitor, Moon, Sun } from 'lucide-react';
import { fn } from 'storybook/test';
import { Label } from './label';
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectSeparator,
  SelectTrigger,
  SelectValue,
} from './select';

const meta = {
  title: 'UI/Forms/Select',
  component: Select,
  args: {
    onValueChange: fn(),
  },
  argTypes: {
    disabled: {
      control: 'boolean',
    },
  },
} satisfies Meta<typeof Select>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default select with a simple list of options */
export const Default: Story = {
  render: (args) => (
    <Select {...args}>
      <SelectTrigger className="w-56">
        <SelectValue placeholder="Select a fruit" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="apple">Apple</SelectItem>
        <SelectItem value="banana">Banana</SelectItem>
        <SelectItem value="cherry">Cherry</SelectItem>
        <SelectItem value="grape">Grape</SelectItem>
        <SelectItem value="orange">Orange</SelectItem>
      </SelectContent>
    </Select>
  ),
};

/** Select trigger sizes — default and small */
export const Sizes: Story = {
  render: (args) => (
    <div className="flex flex-wrap items-center gap-4">
      <Select {...args}>
        <SelectTrigger size="default" className="w-56">
          <SelectValue placeholder="Default size" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="a">Option A</SelectItem>
          <SelectItem value="b">Option B</SelectItem>
        </SelectContent>
      </Select>
      <Select {...args}>
        <SelectTrigger size="sm" className="w-56">
          <SelectValue placeholder="Small size" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="a">Option A</SelectItem>
          <SelectItem value="b">Option B</SelectItem>
        </SelectContent>
      </Select>
    </div>
  ),
};

/** Select with grouped items and labels */
export const WithGroups: Story = {
  render: (args) => (
    <Select {...args}>
      <SelectTrigger className="w-56">
        <SelectValue placeholder="Select a timezone" />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          <SelectLabel>North America</SelectLabel>
          <SelectItem value="est">Eastern (EST)</SelectItem>
          <SelectItem value="cst">Central (CST)</SelectItem>
          <SelectItem value="pst">Pacific (PST)</SelectItem>
        </SelectGroup>
        <SelectSeparator />
        <SelectGroup>
          <SelectLabel>Asia</SelectLabel>
          <SelectItem value="jst">Japan (JST)</SelectItem>
          <SelectItem value="kst">Korea (KST)</SelectItem>
          <SelectItem value="cst-china">China (CST)</SelectItem>
        </SelectGroup>
      </SelectContent>
    </Select>
  ),
};

/** Select with icons in items */
export const WithIcons: Story = {
  render: (args) => (
    <Select {...args}>
      <SelectTrigger className="w-56">
        <SelectValue placeholder="Select a theme" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="light">
          <Sun className="size-4" /> Light
        </SelectItem>
        <SelectItem value="dark">
          <Moon className="size-4" /> Dark
        </SelectItem>
        <SelectItem value="system">
          <Monitor className="size-4" /> System
        </SelectItem>
      </SelectContent>
    </Select>
  ),
};

/** Select with a label and disabled state */
export const WithLabel: Story = {
  render: (args) => (
    <div className="grid w-56 gap-2">
      <Label htmlFor="language-select">Language</Label>
      <Select {...args}>
        <SelectTrigger id="language-select">
          <SelectValue placeholder="Select language" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="en">
            <Globe className="size-4" /> English
          </SelectItem>
          <SelectItem value="ja">
            <Globe className="size-4" /> Japanese
          </SelectItem>
          <SelectItem value="zh">
            <Globe className="size-4" /> Chinese
          </SelectItem>
        </SelectContent>
      </Select>
    </div>
  ),
};

/** Disabled select */
export const Disabled: Story = {
  render: () => (
    <Select disabled>
      <SelectTrigger className="w-56">
        <SelectValue placeholder="Disabled select" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="a">Option A</SelectItem>
      </SelectContent>
    </Select>
  ),
};
