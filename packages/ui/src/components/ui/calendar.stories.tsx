import type { Meta, StoryObj } from "@storybook/react-vite";
import { fn } from "storybook/test";
import * as React from "react";
import type { DateRange as DateRangeType } from "react-day-picker";
import { Calendar } from "./calendar";

const meta = {
  title: "UI/Display/Calendar",
  component: Calendar,
  args: {
    onDayClick: fn(),
  },
} satisfies Meta<typeof Calendar>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default calendar showing the current month */
export const Default: Story = {};

/** Calendar with a selected date managed via React state */
export const WithSelectedDate: Story = {
  render: function SelectedDateStory() {
    const [date, setDate] = React.useState<Date | undefined>(new Date());
    return (
      <Calendar
        mode="single"
        selected={date}
        onSelect={setDate}
      />
    );
  },
};

/** Calendar in range selection mode for picking a date range */
export const DateRange: Story = {
  render: function DateRangeStory() {
    const today = new Date();
    const nextWeek = new Date(today);
    nextWeek.setDate(today.getDate() + 7);

    const [range, setRange] = React.useState<DateRangeType | undefined>({
      from: today,
      to: nextWeek,
    });

    return (
      <Calendar
        mode="range"
        selected={range}
        onSelect={setRange}
      />
    );
  },
};

/** Calendar with navigation dropdowns for month and year */
export const WithDropdowns: Story = {
  render: function DropdownStory() {
    const [date, setDate] = React.useState<Date | undefined>(new Date());
    return (
      <Calendar
        mode="single"
        selected={date}
        onSelect={setDate}
        captionLayout="dropdown"
        startMonth={new Date(2020, 0)}
        endMonth={new Date(2030, 11)}
      />
    );
  },
};
