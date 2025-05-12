import type { FC } from "react";
import { Label } from "../ui/label";
import { Slider } from "../ui/slider";
import * as SliderPrimitive from "@radix-ui/react-slider";
import type { SomeRequired } from "@/lib/utils";

export const NumericSlider: FC<
	SomeRequired<
		React.ComponentProps<typeof SliderPrimitive.Root>,
		"value" | "onValueChange"
	> & {
		label: string;
	}
> = (p) => {
	return (
		<div className="grid gap-2">
			<Label htmlFor="max-fps-slider">{p.label}</Label>
			<div className="flex items-center gap-4">
				<Slider id="max-fps-slider" className="flex-1" {...p} />
				<Label htmlFor="max-fps-slider" className="text-lg w-16 text-center">
					{p.value[0]}
				</Label>
			</div>
		</div>
	);
};
