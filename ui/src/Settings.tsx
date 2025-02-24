import {MenuHeader} from "./menu/MenuHeader.tsx";
import {Box, Container, Divider} from "@mui/material";
import {MenuTabs} from "./menu/MenuTabs.tsx";
import {css} from "@emotion/react";

export const Settings = () => {
    return (
        <Container

            css={css`
                height: 100%;
                display: flex;
                flex-direction: column;
            `}>
            <MenuHeader/>
            <Divider/>
            <Box height={16}/>
            <MenuTabs/>
        </Container>
    )
}
