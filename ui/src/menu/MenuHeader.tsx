import {RiShutDownLine} from "react-icons/ri";
import {Box, IconButton, Tooltip} from "@mui/material";
import {app, dialog, invoke} from "bevy_flurx_api";
import {css} from "@emotion/react";
import {IoPersonAdd} from "react-icons/io5";
import {isProduction} from "../env.ts";

export const MenuHeader = () => {
    return (
        <Box css={css`
            display: flex;
            justify-content: right;
            align-items: center;
        `}>
            <Box flex={1}/>
            <AddMascotButton/>
            <ExitAppButton/>
        </Box>
    )
}

const ExitAppButton = () => {
    return (
        <Tooltip title={"Exit Application"}>

            <IconButton onClick={async () => {
                if (isProduction) {
                    await app.exit();
                }
            }}>
                <RiShutDownLine/>
            </IconButton>
        </Tooltip>
    )
}

const AddMascotButton = () => {
    return (
        <Tooltip title={"Add Mascot"}>
            <IconButton onClick={async () => {
                const modelPath = await dialog.open({
                    title: "Select Model",
                    filters: [{
                        name: "model",
                        extensions: ["vrm"]
                    }],
                });
                if (isProduction && modelPath) {
                    await invoke("load_mascot", {
                        path: modelPath,
                    });
                }
            }}>
                <IoPersonAdd/>
            </IconButton>
        </Tooltip>
    )
}