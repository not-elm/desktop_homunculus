import {FC, ReactNode, useState} from "react";
import {MascotSettings} from "../mascot/MascotSettings.tsx";
import {ActionGroupsControl} from "../actions/ActionGroupsControl.tsx";
import {css} from "@emotion/react";
import {Box, Container, Fade, Tab, Tabs} from "@mui/material";
import {SiGithubactions} from "react-icons/si";
import {RiUserSettingsFill} from "react-icons/ri";

export const MenuTabs = () => {
    const [value, setValue] = useState(0);
    return (
        <div css={css`
            display: flex;
            flex-direction: column;
            height: 100%;
            overflow-y: auto;
        `}>
            <Tabs value={value} onChange={(_, v) => {
                setValue(v);
            }}>
                <Tab
                    value={0}
                    icon={<RiUserSettingsFill/>}
                    label={"Mascot"}/>
                <Tab
                    value={1}
                    icon={<SiGithubactions/>}
                    label={"Actions"}/>
            </Tabs>
            <Box height={16}/>
            <div css={css`
                flex: 1;
                min-height: 0;
                overflow-y: auto;
            `}>
                <TabPanel index={0} value={value}>
                    <MascotSettings/>
                </TabPanel>
                <TabPanel index={1} value={value}>
                    <ActionGroupsControl/>
                </TabPanel>
            </div>
        </div>
    )
}

const TabPanel: FC<{
    index: number;
    value: number;
    children: ReactNode;
}> = ({index, value, children}) => {
    return (
        <Fade in={index === value}>
            <Container
                role={"tabpanel"}
                hidden={index != value}
                css={css`
                    height: 100%;
                `}
            >
                {children}
            </Container>
        </Fade>
    )
}
