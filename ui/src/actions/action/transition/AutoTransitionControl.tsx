import {FC} from "react";
import {Box, Collapse, Slider, Typography} from "@mui/material";

/**
 * Controls additional settings for `TransitionMode::auto`.
 *
 * Currently, we can set the range of time to auto transition (secs).
 */
export const AutoTransitionControl: FC<{
    expand: boolean;
    minSecs: number;
    maxSecs: number;
    onChange: (range: [number, number]) => void;
}> = (p) => {
    return (
        <Collapse in={p.expand}>
            <Box
                display={"flex"}
                flexDirection={"column"}
                gap={1}>
                <Typography>
                    Range of time to animation transition
                </Typography>
                <Slider
                    min={1}
                    max={600}
                    value={[p.minSecs, p.maxSecs]}
                    valueLabelDisplay="auto"
                    onChange={(_, v) => {
                        if (typeof v !== "number") {
                            p.onChange([v[0], v[1]])
                        }
                    }}
                />
            </Box>
        </Collapse>
    )
}
