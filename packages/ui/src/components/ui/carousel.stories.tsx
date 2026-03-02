import type { Meta, StoryObj } from "@storybook/react-vite";
import { fn } from "storybook/test";
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from "./carousel";

const meta = {
  title: "UI/Display/Carousel",
  component: Carousel,
  args: {
    onClick: fn(),
  },
} satisfies Meta<typeof Carousel>;

export default meta;
type Story = StoryObj<typeof meta>;

const SLIDE_COLORS = [
  "bg-blue-500",
  "bg-green-500",
  "bg-purple-500",
  "bg-orange-500",
  "bg-pink-500",
];

/** Default horizontal carousel with colored slide content */
export const Default: Story = {
  render: () => (
    <div className="mx-auto w-full max-w-xs px-12">
      <Carousel>
        <CarouselContent>
          {SLIDE_COLORS.map((color, index) => (
            <CarouselItem key={index}>
              <div
                className={`${color} flex h-40 items-center justify-center rounded-lg`}
              >
                <span className="text-2xl font-bold text-white">
                  Slide {index + 1}
                </span>
              </div>
            </CarouselItem>
          ))}
        </CarouselContent>
        <CarouselPrevious />
        <CarouselNext />
      </Carousel>
    </div>
  ),
};

/** Vertical carousel orientation */
export const Vertical: Story = {
  render: () => (
    <div className="mx-auto w-full max-w-xs py-12">
      <Carousel orientation="vertical">
        <CarouselContent className="-mt-4 h-[200px]">
          {SLIDE_COLORS.map((color, index) => (
            <CarouselItem key={index} className="pt-4 basis-full">
              <div
                className={`${color} flex h-40 items-center justify-center rounded-lg`}
              >
                <span className="text-2xl font-bold text-white">
                  Slide {index + 1}
                </span>
              </div>
            </CarouselItem>
          ))}
        </CarouselContent>
        <CarouselPrevious />
        <CarouselNext />
      </Carousel>
    </div>
  ),
};

/** Carousel showing multiple items per view with partial slides */
export const MultiplePerView: Story = {
  render: () => (
    <div className="mx-auto w-full max-w-lg px-12">
      <Carousel opts={{ align: "start" }}>
        <CarouselContent>
          {SLIDE_COLORS.map((color, index) => (
            <CarouselItem key={index} className="basis-1/3">
              <div
                className={`${color} flex h-28 items-center justify-center rounded-lg`}
              >
                <span className="text-lg font-bold text-white">
                  {index + 1}
                </span>
              </div>
            </CarouselItem>
          ))}
        </CarouselContent>
        <CarouselPrevious />
        <CarouselNext />
      </Carousel>
    </div>
  ),
};

/** Carousel with loop enabled so it wraps around */
export const WithLoop: Story = {
  render: () => (
    <div className="mx-auto w-full max-w-xs px-12">
      <Carousel opts={{ loop: true }}>
        <CarouselContent>
          {SLIDE_COLORS.map((color, index) => (
            <CarouselItem key={index}>
              <div
                className={`${color} flex h-40 items-center justify-center rounded-lg`}
              >
                <span className="text-2xl font-bold text-white">
                  Slide {index + 1}
                </span>
              </div>
            </CarouselItem>
          ))}
        </CarouselContent>
        <CarouselPrevious />
        <CarouselNext />
      </Carousel>
    </div>
  ),
};
