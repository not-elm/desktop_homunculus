import {FC} from "react";
import {Collapse, FormControl, InputLabel, MenuItem, Select} from "@mui/material";
import {Action} from "../../ActionGroupsControl.tsx";

/**
 * Controls additional settings for `TransitionMode::manual`.
 */
export const ManualTransitionControl: FC<{
    expand: boolean;
    allActions: Action[];
    action: Action;
    onChange: (nextAction: Action) => void;
}> = (p) => {
    return (
        <Collapse in={p.expand}>
            <FormControl fullWidth>
                <InputLabel id={"next-action-label"}>
                    Next Action
                </InputLabel>
                <Select
                    labelId={"next-action-label"}
                    label={"Next Action"}
                    id={"next-action-select"}
                    required={true}
                    value={toValue(p.action)}
                    onChange={(e) => {
                        p.onChange(toState(e.target.value as string));
                    }}
                >
                    {p.allActions.map(state => {
                        const value = toValue(state);
                        return (
                            <MenuItem value={value}>
                                {value}
                            </MenuItem>
                        )
                    })}
                </Select>
            </FormControl>
        </Collapse>
    )
}

const toValue = (state: Action) => `${state.group}:${state.name}`;

const toState = (value: string) => {
    const [group, name] = value.split(":");
    return {group, name};
}