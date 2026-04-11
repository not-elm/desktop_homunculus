import type { Meta, StoryObj } from '@storybook/react-vite';
import { fn } from 'storybook/test';
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from './carousel';

const meta = {
  title: 'UI/Display/Carousel',
  component: Carousel,
  args: {
    onClick: fn(),
  },
} satisfies Meta<typeof Carousel>;

export default meta;
type Story = StoryObj<typeof meta>;

const SLIDES = [
  { color: 'bg-blue-500', label: 'Slide 1' },
  { color: 'bg-green-500', label: 'Slide 2' },
  { color: 'bg-purple-500', label: 'Slide 3' },
  { color: 'bg-orange-500', label: 'Slide 4' },
  { color: 'bg-pink-500', label: 'Slide 5' },
];

/** Default horizontal carousel with colored slide content */
export const Default: Story = {
  render: () => (
    <div className="mx-auto w-full max-w-xs px-12">
      <Carousel>
        <CarouselContent>
          {SLIDES.map((slide) => (
            <CarouselItem key={slide.label}>
              <div className={`${slide.color} flex h-40 items-center justify-center rounded-lg`}>
                <span className="text-2xl font-bold text-white">{slide.label}</span>
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
          {SLIDES.map((slide) => (
            <CarouselItem key={slide.label} className="pt-4 basis-full">
              <div className={`${slide.color} flex h-40 items-center justify-center rounded-lg`}>
                <span className="text-2xl font-bold text-white">{slide.label}</span>
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
      <Carousel opts={{ align: 'start' }}>
        <CarouselContent>
          {SLIDES.map((slide) => (
            <CarouselItem key={slide.label} className="basis-1/3">
              <div className={`${slide.color} flex h-28 items-center justify-center rounded-lg`}>
                <span className="text-lg font-bold text-white">{slide.label}</span>
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
          {SLIDES.map((slide) => (
            <CarouselItem key={slide.label}>
              <div className={`${slide.color} flex h-40 items-center justify-center rounded-lg`}>
                <span className="text-2xl font-bold text-white">{slide.label}</span>
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
