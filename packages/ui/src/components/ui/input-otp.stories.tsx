import type { Meta, StoryObj } from '@storybook/react-vite';
import { fn } from 'storybook/test';
import { InputOTP, InputOTPGroup, InputOTPSeparator, InputOTPSlot } from './input-otp';
import { Label } from './label';

const meta = {
  title: 'UI/Forms/InputOTP',
  component: InputOTP,
  args: {
    onChange: fn(),
    onComplete: fn(),
    maxLength: 6,
    children: null,
  },
  argTypes: {
    maxLength: {
      control: 'number',
    },
    disabled: {
      control: 'boolean',
    },
  },
} satisfies Meta<typeof InputOTP>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default 6-digit OTP input */
export const Default: Story = {
  args: {
    children: (
      <InputOTPGroup>
        <InputOTPSlot index={0} />
        <InputOTPSlot index={1} />
        <InputOTPSlot index={2} />
        <InputOTPSlot index={3} />
        <InputOTPSlot index={4} />
        <InputOTPSlot index={5} />
      </InputOTPGroup>
    ),
  },
};

/** OTP input with separator in the middle (3 + 3 pattern) */
export const WithSeparator: Story = {
  args: {
    children: (
      <>
        <InputOTPGroup>
          <InputOTPSlot index={0} />
          <InputOTPSlot index={1} />
          <InputOTPSlot index={2} />
        </InputOTPGroup>
        <InputOTPSeparator />
        <InputOTPGroup>
          <InputOTPSlot index={3} />
          <InputOTPSlot index={4} />
          <InputOTPSlot index={5} />
        </InputOTPGroup>
      </>
    ),
  },
};

/** 4-digit PIN pattern */
export const FourDigitPin: Story = {
  args: {
    maxLength: 4,
    children: (
      <InputOTPGroup>
        <InputOTPSlot index={0} />
        <InputOTPSlot index={1} />
        <InputOTPSlot index={2} />
        <InputOTPSlot index={3} />
      </InputOTPGroup>
    ),
  },
  decorators: [
    (Story) => (
      <div className="grid gap-2">
        <Label>Enter PIN</Label>
        <Story />
      </div>
    ),
  ],
};

/** Multiple separator groups (2 + 2 + 2 pattern) */
export const MultipleSeparators: Story = {
  args: {
    children: (
      <>
        <InputOTPGroup>
          <InputOTPSlot index={0} />
          <InputOTPSlot index={1} />
        </InputOTPGroup>
        <InputOTPSeparator />
        <InputOTPGroup>
          <InputOTPSlot index={2} />
          <InputOTPSlot index={3} />
        </InputOTPGroup>
        <InputOTPSeparator />
        <InputOTPGroup>
          <InputOTPSlot index={4} />
          <InputOTPSlot index={5} />
        </InputOTPGroup>
      </>
    ),
  },
};

/** Disabled OTP input */
export const Disabled: Story = {
  args: {
    disabled: true,
    children: (
      <InputOTPGroup>
        <InputOTPSlot index={0} />
        <InputOTPSlot index={1} />
        <InputOTPSlot index={2} />
        <InputOTPSlot index={3} />
        <InputOTPSlot index={4} />
        <InputOTPSlot index={5} />
      </InputOTPGroup>
    ),
  },
};
