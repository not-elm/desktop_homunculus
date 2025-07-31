import {type FC, useState} from "react";
import {motion} from "motion/react";
import {
    commands,
    mods,
    type OpenAroundVrm,
    type OpenOptions,
    type OpenPosition,
    scripts,
    Vrm,
    Webview
} from "@homunculus/api";
import {Card} from "@homunculus/core";

export const Menus: FC<{
    menus: mods.ModMenuMetadata[];
}> = ({menus}) => {
    const [finished, setFinished] = useState(false)
    return (
        <motion.div
            animate={{
                transform: ["translateY(-100vh)", "translateY(3vh)", "translateY(0)"],
                opacity: [0, 1, 1],
            }}
            transition={{
                duration: 0.7,
                times: [0, 0.5, 0.7],
                ease: ["easeInOut", "easeIn"],
            }}
            onAnimationComplete={() => setFinished(true)}
            className="w-screen"
        >
            <div
                className={`overflow-y-scroll no-scrollbar flex flex-col gap-3 w-full px-2 ${finished ? "h-screen" : "h-auto"}`}
            >
                {menus.map((menu, i) => {
                    return (
                        <Menu
                            key={menu.text + i}
                            {...menu}
                        />
                    )
                })}
            </div>
        </motion.div>
    )
}

export const Menu: FC<mods.ModMenuMetadata> = (p) => {
    return (
        <Card
            className="h-[80px] flex p-4 transition-transform duration-200 ease-in-out max-w-[90vw] hover:translate-y-[-2px] active:translate-y-[2px] hover:[&_.thumbnail]:opacity-100"
            onClick={async () => {
                const vrm = Vrm.caller();
                if (vrm) {
                    if (p.script) {
                        await scripts.callJavascript(p.script);
                    }
                    if (p.webview) {
                        await openModUi(vrm, p.webview);
                    }
                }
            }}
        >
            <img
                src={p.thumbnail ?? "https://dummyimage.com/80/"}
                className={"h-full aspect-square"}
            />
            <div
                className={"text-2xl font-bold text-white text-left flex-1 flex items-center"}
            >
                <h1 className="select-none drop-shadow-lg">
                    {p.text}
                </h1>
            </div>
        </Card>

    );
};


const openModUi = async (
    vrm: Vrm,
    options: OpenOptions,
) => {
    const width = options?.resolution?.[0] ?? 500;
    const margin = 100;
    let position: OpenPosition | undefined;
    if (isAbsolutePosition(options?.position)) {
        position = options?.position;
    } else if (isVrmRelativePosition(options?.position)) {
        position = {
            vrm: vrm.entity,
            //@ts-ignore
            bone: options?.position?.bone,
            //@ts-ignore
            offset: options?.position?.offset ?? [-width - margin, 0],
            //@ts-ignore
            tracking: options?.position?.tracking ?? true,
        };
    }
    const modWebview = await Webview.open({
        ...options,
        position,
        caller: vrm.entity,
    });
    await commands.send("menu::mod::open", modWebview.entity);
}

const isAbsolutePosition = (position: unknown): position is [number, number] => {
    return Array.isArray(position);
}

const isVrmRelativePosition = (position: unknown): position is OpenAroundVrm => {
    return !!position && typeof position === "object" &&
        // @ts-ignore
        (!!position?.bone || !!position?.offset || !!position?.tracking);
}