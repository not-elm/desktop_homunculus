import type { Meta, StoryObj } from "@storybook/react-vite";
import { fn } from "storybook/test";
import { AlertCircle, Info, Terminal } from "lucide-react";
import { Alert, AlertDescription, AlertTitle } from "./alert";
import { Button } from "./button";

const meta = {
  title: "UI/Display/Alert",
  component: Alert,
  args: {
    onClick: fn(),
  },
  argTypes: {
    variant: {
      control: "select",
      options: ["default", "destructive"],
    },
  },
} satisfies Meta<typeof Alert>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default informational alert with icon, title, and description */
export const Default: Story = {
  render: () => (
    <Alert>
      <Terminal />
      <AlertTitle>Heads up!</AlertTitle>
      <AlertDescription>
        You can add components to your app using the CLI.
      </AlertDescription>
    </Alert>
  ),
};

/** Destructive alert for errors or critical warnings */
export const Destructive: Story = {
  render: () => (
    <Alert variant="destructive">
      <AlertCircle />
      <AlertTitle>Error</AlertTitle>
      <AlertDescription>
        Your session has expired. Please log in again.
      </AlertDescription>
    </Alert>
  ),
};

/** Alert with an action button inside the description */
export const WithAction: Story = {
  render: () => (
    <Alert>
      <Info />
      <AlertTitle>Update available</AlertTitle>
      <AlertDescription>
        <p>A new version of the application is available.</p>
        <Button variant="outline" size="sm" className="mt-2">
          Update now
        </Button>
      </AlertDescription>
    </Alert>
  ),
};

/** Both alert variants displayed together for comparison */
export const AllVariants: Story = {
  render: () => (
    <div className="flex flex-col gap-4">
      <Alert>
        <Info />
        <AlertTitle>Default</AlertTitle>
        <AlertDescription>
          This is a default alert for general information.
        </AlertDescription>
      </Alert>
      <Alert variant="destructive">
        <AlertCircle />
        <AlertTitle>Destructive</AlertTitle>
        <AlertDescription>
          This is a destructive alert for error states.
        </AlertDescription>
      </Alert>
    </div>
  ),
};
