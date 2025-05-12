import { NumericSlider, SettingsCard } from "@homunculus/core";
import { useEffect, useState } from "react";
import { settings, Vrm, shadowPanel, entities } from "@homunculus/api";
import type { FC } from "react";
import { motion } from "framer-motion";

export const Settings = () => {
	const vrm = Vrm.caller();
	return (
		<motion.div
			initial={{ opacity: 0, y: 20 }}
			animate={{ opacity: 1, y: 0 }}
			exit={{ opacity: 0, y: 20 }}
			transition={{ duration: 0.2 }}
			className="w-screen grid gap-4 grid-cols-1"
		>
			<GeneralSettings />
			{vrm && <VrmSettings vrm={vrm} />}
		</motion.div>
	);
};

const GeneralSettings: FC = () => {
	return (
		<SettingsCard
			title="General"
			description="General settings for the application."
		>
			<MaxFpsSlider />
			<ShadowPanelAlphaSlider />
		</SettingsCard>
	);
};

const MaxFpsSlider = () => {
	const [fps, setFps] = useState<number | undefined>(undefined);
	useEffect(() => {
		settings.fpsLimit().then(setFps);
	}, []);
	useEffect(() => {
		if (fps && Number.isInteger(fps) && 10 <= fps) {
			settings.saveFpsLimit(fps);
		}
	}, [fps]);
	return (
		<NumericSlider
			label="Max FPS"
			min={10}
			max={244}
			step={1}
			value={[fps ?? 60]}
			onValueChange={(v) => {
				setFps(v[0]);
			}}
		/>
	);
};

const VrmSettings: FC<{
	vrm: Vrm;
}> = ({ vrm }) => {
	const [name, setName] = useState("");
	useEffect(() => {
		vrm.name().then(setName);
	}, []);
	return (
		<SettingsCard title="VRM" description={`Settings for ${name}`}>
			<ScaleSlider vrm={vrm} />
		</SettingsCard>
	);
};

const ScaleSlider: FC<{
	vrm: Vrm;
}> = ({ vrm }) => {
	const [scale, setScale] = useState<number | undefined>(undefined);
	useEffect(() => {
		entities.transform(vrm.entity).then((tf) => setScale(tf.scale[0]));
	}, []);
	useEffect(() => {
		if (!scale) {
			return;
		}
		entities.setTransform(vrm.entity, {
			scale: [scale, scale, scale],
		});
	}, [scale]);
	return scale && (<NumericSlider
		label="Scale"
		min={0.1}
		max={3}
		step={0.01}
		value={[scale]}
		onValueChange={(v) => {
			setScale(v[0]);
		}}
	/>);
};

const ShadowPanelAlphaSlider = () => {
	const [alpha, setAlpha] = useState<number | undefined>(undefined);
	useEffect(() => {
		shadowPanel.alpha().then(setAlpha);
	}, []);
	useEffect(() => {
		if (alpha !== undefined) {
			shadowPanel.setAlpha(alpha);
		}
	}, [alpha]);
	return (
		<NumericSlider
			label="Shadow Alpha"
			min={0}
			max={1}
			step={0.01}
			value={[alpha ?? 0.5]}
			onValueChange={(v) => {
				setAlpha(v[0]);
			}}
		/>
	);
};
