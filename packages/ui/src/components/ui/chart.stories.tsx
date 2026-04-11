import type { Meta, StoryObj } from '@storybook/react-vite';
import { Bar, BarChart, CartesianGrid, Line, LineChart, XAxis, YAxis } from 'recharts';
import {
  type ChartConfig,
  ChartContainer,
  ChartLegend,
  ChartLegendContent,
  ChartTooltip,
  ChartTooltipContent,
} from './chart';

const barData = [
  { month: 'Jan', desktop: 186, mobile: 80 },
  { month: 'Feb', desktop: 305, mobile: 200 },
  { month: 'Mar', desktop: 237, mobile: 120 },
  { month: 'Apr', desktop: 73, mobile: 190 },
  { month: 'May', desktop: 209, mobile: 130 },
  { month: 'Jun', desktop: 214, mobile: 140 },
];

const barConfig = {
  desktop: {
    label: 'Desktop',
    color: '#6366f1',
  },
  mobile: {
    label: 'Mobile',
    color: '#ec4899',
  },
} satisfies ChartConfig;

const meta: Meta = {
  title: 'UI/Display/Chart',
  component: ChartContainer,
};

export default meta;
type Story = StoryObj;

/** Default bar chart with tooltip and legend */
export const Default: Story = {
  render: () => (
    <ChartContainer config={barConfig} className="min-h-[300px] w-full">
      <BarChart data={barData}>
        <CartesianGrid vertical={false} />
        <XAxis dataKey="month" tickLine={false} axisLine={false} />
        <YAxis tickLine={false} axisLine={false} />
        <ChartTooltip content={<ChartTooltipContent />} />
        <ChartLegend content={<ChartLegendContent />} />
        <Bar dataKey="desktop" fill="var(--color-desktop)" radius={4} />
        <Bar dataKey="mobile" fill="var(--color-mobile)" radius={4} />
      </BarChart>
    </ChartContainer>
  ),
};

const lineData = [
  { month: 'Jan', revenue: 4000, expenses: 2400 },
  { month: 'Feb', revenue: 3000, expenses: 1398 },
  { month: 'Mar', revenue: 2000, expenses: 3800 },
  { month: 'Apr', revenue: 2780, expenses: 3908 },
  { month: 'May', revenue: 1890, expenses: 4800 },
  { month: 'Jun', revenue: 2390, expenses: 3800 },
];

const lineConfig = {
  revenue: {
    label: 'Revenue',
    color: '#22c55e',
  },
  expenses: {
    label: 'Expenses',
    color: '#ef4444',
  },
} satisfies ChartConfig;

/** Line chart showing trends over time */
export const LineChartStory: Story = {
  name: 'Line Chart',
  render: () => (
    <ChartContainer config={lineConfig} className="min-h-[300px] w-full">
      <LineChart data={lineData}>
        <CartesianGrid vertical={false} />
        <XAxis dataKey="month" tickLine={false} axisLine={false} />
        <YAxis tickLine={false} axisLine={false} />
        <ChartTooltip content={<ChartTooltipContent />} />
        <ChartLegend content={<ChartLegendContent />} />
        <Line type="monotone" dataKey="revenue" stroke="var(--color-revenue)" strokeWidth={2} />
        <Line type="monotone" dataKey="expenses" stroke="var(--color-expenses)" strokeWidth={2} />
      </LineChart>
    </ChartContainer>
  ),
};

const singleConfig = {
  visitors: {
    label: 'Visitors',
    color: '#8b5cf6',
  },
} satisfies ChartConfig;

const singleData = [
  { day: 'Mon', visitors: 120 },
  { day: 'Tue', visitors: 250 },
  { day: 'Wed', visitors: 180 },
  { day: 'Thu', visitors: 310 },
  { day: 'Fri', visitors: 275 },
];

/** Single-series bar chart without legend */
export const SingleSeries: Story = {
  render: () => (
    <ChartContainer config={singleConfig} className="min-h-[300px] w-full">
      <BarChart data={singleData}>
        <CartesianGrid vertical={false} />
        <XAxis dataKey="day" tickLine={false} axisLine={false} />
        <YAxis tickLine={false} axisLine={false} />
        <ChartTooltip content={<ChartTooltipContent />} />
        <Bar dataKey="visitors" fill="var(--color-visitors)" radius={[4, 4, 0, 0]} />
      </BarChart>
    </ChartContainer>
  ),
};
