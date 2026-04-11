import { zodResolver } from '@hookform/resolvers/zod';
import type { Meta, StoryObj } from '@storybook/react-vite';
import { useForm } from 'react-hook-form';
import { fn } from 'storybook/test';
import { z } from 'zod';
import { Button } from './button';
import { Checkbox } from './checkbox';
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from './form';
import { Input } from './input';
import { Label } from './label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from './select';
import { Textarea } from './textarea';

const meta: Meta = {
  title: 'UI/Forms/Form',
};

export default meta;
type Story = StoryObj;

const onSubmitAction = fn();

const basicSchema = z.object({
  username: z.string().min(3, 'Username must be at least 3 characters'),
  email: z.string().email('Please enter a valid email address'),
});

type BasicFormValues = z.infer<typeof basicSchema>;

function BasicFormExample() {
  const form = useForm<BasicFormValues>({
    // @ts-expect-error — @hookform/resolvers@5.2.2 types incompatible with zod v4.3.x
    resolver: zodResolver(basicSchema),
    defaultValues: {
      username: '',
      email: '',
    },
  });

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmitAction)} className="w-96 space-y-4">
        <FormField
          control={form.control}
          name="username"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Username</FormLabel>
              <FormControl>
                <Input placeholder="Enter username" {...field} />
              </FormControl>
              <FormDescription>Your public display name.</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="email"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Email</FormLabel>
              <FormControl>
                <Input type="email" placeholder="you@example.com" {...field} />
              </FormControl>
              <FormDescription>We will never share your email.</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button type="submit">Submit</Button>
      </form>
    </Form>
  );
}

/** Basic form with text inputs and validation */
export const Default: Story = {
  render: () => <BasicFormExample />,
};

const profileSchema = z.object({
  displayName: z.string().min(2, 'Display name is required'),
  bio: z.string().max(160, 'Bio must be 160 characters or less').optional(),
  role: z.string().min(1, 'Please select a role'),
  agreeToTerms: z.literal(true, {
    error: 'You must agree to the terms',
  }),
});

type ProfileFormValues = z.infer<typeof profileSchema>;

function ProfileFormExample() {
  const form = useForm<ProfileFormValues>({
    // @ts-expect-error — @hookform/resolvers@5.2.2 types incompatible with zod v4.3.x
    resolver: zodResolver(profileSchema),
    defaultValues: {
      displayName: '',
      bio: '',
      role: '',
      agreeToTerms: undefined as unknown as true,
    },
  });

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmitAction)} className="w-96 space-y-4">
        <FormField
          control={form.control}
          name="displayName"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Display Name</FormLabel>
              <FormControl>
                <Input placeholder="Your name" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="bio"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Bio</FormLabel>
              <FormControl>
                <Textarea placeholder="Tell us about yourself" {...field} />
              </FormControl>
              <FormDescription>
                Brief description for your profile. Max 160 characters.
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="role"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Role</FormLabel>
              <Select onValueChange={field.onChange} defaultValue={field.value}>
                <FormControl>
                  <SelectTrigger className="w-full">
                    <SelectValue placeholder="Select a role" />
                  </SelectTrigger>
                </FormControl>
                <SelectContent>
                  <SelectItem value="developer">Developer</SelectItem>
                  <SelectItem value="designer">Designer</SelectItem>
                  <SelectItem value="manager">Manager</SelectItem>
                </SelectContent>
              </Select>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="agreeToTerms"
          render={({ field }) => (
            <FormItem>
              <div className="flex items-center gap-2">
                <FormControl>
                  <Checkbox checked={field.value} onCheckedChange={field.onChange} />
                </FormControl>
                <Label>I agree to the terms and conditions</Label>
              </div>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button type="submit">Save Profile</Button>
      </form>
    </Form>
  );
}

/** Full profile form with multiple field types and validation */
export const ProfileForm: Story = {
  render: () => <ProfileFormExample />,
};

const prefilledSchema = z.object({
  name: z.string().min(1, 'Name is required'),
  email: z.string().email('Invalid email'),
});

type PrefilledFormValues = z.infer<typeof prefilledSchema>;

function PrefilledFormExample() {
  const form = useForm<PrefilledFormValues>({
    // @ts-expect-error — @hookform/resolvers@5.2.2 types incompatible with zod v4.3.x
    resolver: zodResolver(prefilledSchema),
    defaultValues: {
      name: 'Jane Doe',
      email: 'jane@example.com',
    },
  });

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmitAction)} className="w-96 space-y-4">
        <FormField
          control={form.control}
          name="name"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Name</FormLabel>
              <FormControl>
                <Input {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="email"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Email</FormLabel>
              <FormControl>
                <Input type="email" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button type="submit">Update</Button>
      </form>
    </Form>
  );
}

/** Form pre-filled with existing data */
export const Prefilled: Story = {
  render: () => <PrefilledFormExample />,
};

function FormWithDescriptionsExample() {
  const form = useForm({
    defaultValues: {
      apiKey: '',
      webhook: '',
    },
  });

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmitAction)} className="w-96 space-y-4">
        <FormField
          control={form.control}
          name="apiKey"
          render={({ field }) => (
            <FormItem>
              <FormLabel>API Key</FormLabel>
              <FormControl>
                <Input type="password" placeholder="sk-..." {...field} />
              </FormControl>
              <FormDescription>Your secret API key. Keep it safe.</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="webhook"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Webhook URL</FormLabel>
              <FormControl>
                <Input type="url" placeholder="https://example.com/webhook" {...field} />
              </FormControl>
              <FormDescription>Events will be sent to this URL via POST.</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button type="submit">Save Settings</Button>
      </form>
    </Form>
  );
}

/** Form fields with helper descriptions */
export const WithDescriptions: Story = {
  render: () => <FormWithDescriptionsExample />,
};
