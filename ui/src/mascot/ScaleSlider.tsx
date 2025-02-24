import {Box, CircularProgress, Slider, Stack, Typography} from "@mui/material";
import {css} from "@emotion/react";
import {FC, useEffect, useState} from "react";
import {emit, invoke} from "bevy_flurx_api";
import {isProduction, sleep} from "../env.ts";

export const ScaleSlider = () => {
    const [initialScale, setInitialScale] = useState<number | undefined>();
    useEffect(() => {
        (async () => {
            if (isProduction) {
                setInitialScale(await invoke("get_scale"))
            } else {
                await sleep(1000)
                setInitialScale(1)
            }
        })();
    }, []);
    return (
        <Box css={css`
            display: flex;
            justify-content: center;
        `}>
            {initialScale ?
                <ScaleSliderContent initialScale={initialScale}/> :
                <CircularProgress variant={"indeterminate"}/>}
        </Box>
    )
}

const ScaleSliderContent: FC<{
    initialScale: number
}> = ({initialScale}) => {
    const [scale, setScale] = useState(initialScale * 50)
    useEffect(() => {
        if (isProduction && scale !== initialScale) {
            emit("scale", {
                scale: scale / 50,
            })
        }
    }, [scale]);
    return (
        <Stack css={css`width: 100%;`}>
            <Typography>
                Scale
            </Typography>
            <Slider
                id={"scale-slider"}
                value={scale}
                onChange={(_, v) => {
                    if (typeof v === "number") {
                        setScale(v)
                    }
                }}
                defaultValue={initialScale}
                valueLabelDisplay="auto"
            />
        </Stack>
    )
}

