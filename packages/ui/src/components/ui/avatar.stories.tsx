import type { Meta, StoryObj } from "@storybook/react-vite";
import { fn } from "storybook/test";
import { Avatar, AvatarFallback, AvatarImage } from "./avatar";

const meta = {
  title: "UI/Display/Avatar",
  component: Avatar,
  args: {
    onClick: fn(),
  },
} satisfies Meta<typeof Avatar>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default avatar with initials fallback */
export const Default: Story = {
  render: () => (
    <Avatar>
      <AvatarFallback>AB</AvatarFallback>
    </Avatar>
  ),
};

/** Avatar with an image source and fallback text when image fails to load */
export const WithImage: Story = {
  render: () => (
    <Avatar>
      <AvatarImage src="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='32' height='32'%3E%3Crect width='32' height='32' fill='%236366f1'/%3E%3Ctext x='50%25' y='55%25' text-anchor='middle' dominant-baseline='middle' fill='white' font-size='14'%3EH%3C/text%3E%3C/svg%3E" alt="User" />
      <AvatarFallback>H</AvatarFallback>
    </Avatar>
  ),
};

/** Multiple avatar sizes using className overrides */
export const Sizes: Story = {
  render: () => (
    <div className="flex items-center gap-4">
      <Avatar className="size-6">
        <AvatarFallback className="text-[10px]">S</AvatarFallback>
      </Avatar>
      <Avatar className="size-8">
        <AvatarFallback className="text-xs">M</AvatarFallback>
      </Avatar>
      <Avatar className="size-10">
        <AvatarFallback className="text-sm">L</AvatarFallback>
      </Avatar>
      <Avatar className="size-14">
        <AvatarFallback className="text-lg">XL</AvatarFallback>
      </Avatar>
    </div>
  ),
};

/** Group of overlapping avatars */
export const Group: Story = {
  render: () => (
    <div className="flex -space-x-3">
      <Avatar className="border-2 border-background">
        <AvatarFallback className="bg-blue-500 text-white">A</AvatarFallback>
      </Avatar>
      <Avatar className="border-2 border-background">
        <AvatarFallback className="bg-green-500 text-white">B</AvatarFallback>
      </Avatar>
      <Avatar className="border-2 border-background">
        <AvatarFallback className="bg-purple-500 text-white">C</AvatarFallback>
      </Avatar>
      <Avatar className="border-2 border-background">
        <AvatarFallback className="bg-orange-500 text-white">D</AvatarFallback>
      </Avatar>
    </div>
  ),
};
