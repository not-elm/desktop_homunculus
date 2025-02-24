import {Button, Stack} from "@mui/material";
import {ScaleSlider} from "./ScaleSlider.tsx";
import {MdPinDrop} from "react-icons/md";
import {isProduction} from "../env.ts";
import {emit} from "bevy_flurx_api";

export const MascotSettings = () => {
    return (
        <Stack spacing={2} maxWidth={"90%"} alignContent={"center"}>
            <ResetPositionButton/>
            <ScaleSlider/>
        </Stack>
    )
}

const ResetPositionButton = () => {
    return (
        <Button
            variant={"contained"}
            startIcon={<MdPinDrop/>}
            onClick={() => {
                if (isProduction) {
                    emit("reset_position", {
                        _dummy: false,
                    });
                }
            }}>
            Reset Position
        </Button>
    )
}

