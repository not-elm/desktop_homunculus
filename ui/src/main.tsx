import {StrictMode} from 'react'
import {createRoot} from 'react-dom/client'
import {Settings} from "./Settings.tsx";
import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';
import "./index.css";
import {createTheme, CssBaseline, ThemeProvider} from "@mui/material";
import {blue, lime, teal} from "@mui/material/colors";

const theme = createTheme({
    palette: {
        mode: 'dark',
        primary: {
            main: lime[400],
        },
        secondary: {
            main: blue[300],
        },
        background: {
            default: teal[800],
        }
    },
    components: {
        MuiCssBaseline: {
            styleOverrides: {
                '::-webkit-scrollbar': {
                    width: '11px',
                },
                '::-webkit-scrollbar-track': {
                    background: lime[400],
                },
                '::-webkit-scrollbar-thumb': {
                    background: 'black',
                    borderRadius: '2px',
                }
            },
        },
    },
});

createRoot(document.getElementById('root')!).render(
    <StrictMode>
        <ThemeProvider theme={theme} defaultMode={"dark"}>
            <CssBaseline/>
            <Settings/>
        </ThemeProvider>
    </StrictMode>,
)
