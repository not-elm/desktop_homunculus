import {FC, useState} from "react";
import {
    Card,
    CardContent,
    Checkbox,
    Collapse,
    Divider,
    FormControl,
    FormControlLabel,
    InputLabel,
    ListItemButton,
    ListItemText,
    MenuItem,
    Select
} from "@mui/material";
import {MdExpandLess, MdExpandMore} from "react-icons/md";
import {css} from "@emotion/react";
import {AutoTransitionControl} from "./transition/AutoTransitionControl.tsx";
import {ManualTransitionControl} from "./transition/ManualTransitionControl.tsx";
import {Action, ActionProperties, emitActionUpdated, TransitionType} from "../ActionGroupsControl.tsx";
import {lime} from "@mui/material/colors";

export interface ActionControlProps {
    allActions: Action[];
    action: Action;
    properties: ActionProperties;
}

export const ActionControl: FC<ActionControlProps> = (p) => {
    const [expanded, setExpanded] = useState(false);
    return (
        <>
            <ListItemButton onClick={() => setExpanded(!expanded)}>
                <ListItemText primary={p.action.name}/>
                {expanded ? <MdExpandLess/> : <MdExpandMore/>}
            </ListItemButton>
            <Collapse in={expanded}>
                <Divider color={lime[400]}/>
                <ActionPropertiesControl {...p} />
            </Collapse>
        </>
    )
}

const ActionPropertiesControl: FC<ActionControlProps> = ({action, properties, allActions}) => {
    return (
        <Card>
            <CardContent css={css`
                display: flex;
                flex-direction: column;
                gap: 16px;
            `}>
                <FormControlLabel
                    control={<Checkbox
                        checked={properties.is_repeat_animation}
                        onChange={e => emitActionUpdated(
                            action,
                            {
                                ...properties,
                                is_repeat_animation: e.target.checked,
                            }
                        )}
                    />}
                    label="Repeat Animation"
                />
                <TransitionModeSelect
                    properties={properties}
                    onChange={(transitionType) => emitActionUpdated(action, ({
                        ...properties,
                        transition: {
                            type: transitionType,
                            min_secs: 10,
                            max_secs: 60,
                            next: allActions[0],
                        },
                    }))}/>
                <AutoTransitionControl
                    expand={properties.transition.type === "auto"}
                    minSecs={properties.transition.min_secs || 10}
                    maxSecs={properties.transition.max_secs || 60}
                    onChange={(timeRange) => {
                        emitActionUpdated(action, {
                            ...properties,
                            transition: {
                                type: "auto",
                                min_secs: timeRange[0],
                                max_secs: timeRange[1],
                            }
                        })
                    }}
                />
                <ManualTransitionControl
                    expand={properties.transition.type === "manual"}
                    allActions={allActions}
                    action={properties.transition.next || allActions[0]}
                    onChange={nextAction => emitActionUpdated(action, {
                        ...properties,
                        transition: {
                            type: "manual",
                            next: nextAction,
                        }
                    })}/>
            </CardContent>
        </Card>
    )
}

const TransitionModeSelect: FC<{
    properties: ActionProperties;
    onChange: (transitionMode: TransitionType) => void;
}> = ({properties, onChange}) => {
    return (
        <FormControl fullWidth>
            <InputLabel id={"transition-mode-label"}>
                Transition Mode
            </InputLabel>
            <Select
                labelId={"transition-mode-label"}
                label={"Transition Mode"}
                id={"transition-mode-select"}
                value={properties.transition.type}
                onChange={(e) => onChange(e.target.value as TransitionType)}
            >
                <MenuItem value={"none"}>None</MenuItem>
                <MenuItem value={"manual"}>Manual</MenuItem>
                <MenuItem value={"auto"}>Auto</MenuItem>
            </Select>
        </FormControl>
    )
}