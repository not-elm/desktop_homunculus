import {
    Button,
    Checkbox,
    Label,
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
    TextareaAutosize
} from "@homunculus/core";
import { gpt, Vrm } from "@homunculus/api";
import { type FC, useEffect, useState } from "react";
import { IconButton, Text } from "@radix-ui/themes";
import { Collapsible } from "radix-ui";
import openSound from "../public/open.mp3";
import closeSound from "../public/close.mp3";
import useSound from 'use-sound';
import { ChevronLeftIcon, ChevronRightIcon } from "lucide-react";

/**
 * System message defines the context such as the character of the VRM.
 */
export const SettingsSidebar = () => {
    const [open, setOpen] = useState(false);
    const [systemPrompt, setSystemPrompt] = useState("");
    const [model, setModel] = useState<string | undefined>(undefined);
    const [useWebSearch, setUseWebSearch] = useState(false);
    const [voicevoxSpeaker, setVoicevoxSpeaker] = useState<number | undefined>(undefined);
    const [playOpen] = useSound(openSound);
    const [playClose] = useSound(closeSound);

    useEffect(() => {
        const vrm = Vrm.caller();
        if (!vrm) return;
        const options = { vrm: vrm.entity };
        gpt.systemPrompt(options).then(prompt => {
            setSystemPrompt(prompt);
        });
        gpt.model(options).then(setModel);
        gpt.useWebSearch(options).then(setUseWebSearch);

        fetch(`http://localhost:3100/gpt/speaker/voicevox?vrm=${vrm.entity}`)
            .then(response => response.json())
            .then(speakerId => {
                setVoicevoxSpeaker(speakerId);
            })
            .catch(console.error);
    }, []);

    return (
        <Collapsible.Root
            className="flex fixed top-0 left-0 h-screen items-start"
            open={open}
            onOpenChange={isOpen => {
                setOpen(isOpen);
                if (isOpen) {
                    playOpen();
                } else {
                    playClose();
                }
            }}
        >
            <Collapsible.Content className="drawer-open w-full h-full">
                <div className="bg-cyan-950 flex flex-col gap-5 p-8 h-full w-[90vw] overflow-y-auto">
                    <Models model={model} setModel={setModel} />
                    <UseWebSearch useWebSearch={useWebSearch} setUseWebSearch={setUseWebSearch} />
                    <VoicevoxSpeaker speaker={voicevoxSpeaker} setSpeaker={setVoicevoxSpeaker} />
                    <SystemPrompt message={systemPrompt} onChange={setSystemPrompt} />
                    <Button
                        className="bg-sidebar-primary text-sidebar-foreground hover:bg-sidebar-accent"
                        onClick={async () => {
                            const vrm = Vrm.caller();
                            if (vrm) {
                                const options = { vrm: vrm.entity };
                                await gpt.saveSystemPrompt(systemPrompt, options);
                                await gpt.saveModel(model, options);
                                await gpt.saveUseWebSearch(useWebSearch, options);
                                if (voicevoxSpeaker !== undefined) {
                                    await gpt.saveVoicevoxSpeaker(voicevoxSpeaker, {
                                        vrm: vrm.entity
                                    });
                                }
                            }
                            playClose();
                            setOpen(false);
                        }}
                    >
                        Save
                    </Button>
                </div>
            </Collapsible.Content>
            <Collapsible.Trigger className="p-2 bg-cyan-950">
                <IconButton variant="ghost" size={"3"}>
                    {open ? (
                        <ChevronLeftIcon width={"22"} height={"22"} />
                    ) : (
                        <ChevronRightIcon width={"22"} height={"22"} />
                    )}
                    {/* <GearIcon className="fill-primary" width={"22"} height={"22"} /> */}
                </IconButton>
            </Collapsible.Trigger>
        </Collapsible.Root>
    )
}

const Models: FC<{
    model: string | undefined;
    setModel: (mode: string) => void;
}> = ({ model, setModel }) => {
    const [availableModels, setSvailableModels] = useState([]);
    useEffect(() => {
        gpt.availableModels().then(models => {
            setSvailableModels(models);
        });
    }, []);
    return (
        <div className="flex flex-col gap-3">
            <Text as="label" size="8">
                GPT Model
            </Text>
            <Select
                value={model}
                onValueChange={setModel}
            >
                <SelectTrigger className="w-full">
                    <SelectValue placeholder="GPT Models" />
                </SelectTrigger>
                <SelectContent className="h-50 overflow-y-auto">
                    {availableModels.map(m => (
                        <SelectItem
                            className="bg-sidebar-accent p-2 hover:bg-sidebar-ring"
                            value={m}
                        >
                            {m}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </div>
    )
}

const fetchVoicevoxSpeakers = async (): Promise<{
    name: string;
    styles: [{ name: string; id: number; }]
}[]> => {
    try {
        const speakers = await fetch("http://localhost:50021/speakers");
        return await speakers.json();
    } catch {
        return [];
    }
}

const SystemPrompt: FC<{
    message: string;
    onChange: (message: string) => void
}> = ({ message, onChange }) => {
    return (
        <div className="flex flex-col gap-3 h-80">
            <Text as="label" size="8">
                System Prompt
            </Text>
            <TextareaAutosize
                className="flex-1 resize-none"
                value={message}
                onChange={e => onChange(e.target.value)}
            />
        </div>
    );
}

const VoicevoxSpeaker: FC<{
    speaker: number | undefined;
    setSpeaker: (speaker: number) => void;
}> = ({ speaker, setSpeaker }) => {
    const [speakers, setSpeakers] = useState<{
        name: string;
        styles: { name: string; id: number; }[]
    }[]>([]);

    useEffect(() => {
        fetchVoicevoxSpeakers().then(setSpeakers);
    }, []);

    useEffect(() => {
        gpt.voicevoxSpeaker({
            vrm: Vrm.caller()?.entity
        }).then(setSpeaker);
    }, []);

    const speakerOptions = speakers.flatMap(speaker =>
        speaker.styles.map(style => ({
            id: style.id,
            label: `${speaker.name}-${style.name}`
        }))
    );

    return (
        <div className="flex flex-col gap-3">
            <Text as="label" size="8">
                VOICEVOX Speaker
            </Text>
            <Select
                value={speaker?.toString()}
                onValueChange={(value) => setSpeaker(parseInt(value))}
            >
                <SelectTrigger className="w-full">
                    <SelectValue placeholder={"Select a speaker"} />
                </SelectTrigger>
                <SelectContent className="h-50 overflow-y-auto">
                    {speakerOptions.map(option => (
                        <SelectItem
                            key={option.id}
                            className="bg-sidebar-accent p-2 hover:bg-sidebar-ring"
                            value={option.id.toString()}
                        >
                            {option.label}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
            <Text as="p" size="1" className="text-red-500">
                NOTE: VOICEVOX application must be running (localhost:50021) for speech synthesis to work.
            </Text>
        </div>
    );
};

const UseWebSearch: FC<{
    useWebSearch: boolean;
    setUseWebSearch: (useWebSearch: boolean) => void;
}> = ({ useWebSearch, setUseWebSearch }) => {
    return (
        <div className="flex flex-col gap-3">
            {/* <Text as="label" size="8">
                Use Web Search
            </Text> */}
            <div className="flex items-center space-x-2">
                <Checkbox
                    id="use-web-search"
                    checked={useWebSearch}
                    onCheckedChange={checked => setUseWebSearch(Boolean(checked))}
                />
                <Label htmlFor="use-web-search" className="text-white">
                    Enable web search for ChatGPT.
                </Label>
            </div>
            <Text as="p" size="1" className="text-red-500">
                NOTE: This feature is only available for specific models (e.g., models with 'search' in their name).
            </Text>
        </div>
    );
};
