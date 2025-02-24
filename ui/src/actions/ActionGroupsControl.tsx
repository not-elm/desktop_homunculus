import {Card, CardContent, Grid, List, Typography} from "@mui/material";
import {css} from "@emotion/react";
import {FC, useEffect, useMemo, useState} from "react";
import {ActionControl} from "./action/ActionControl.tsx";
import {WebWindow} from "../../../../bevy_webview_projects/tool/api";
import {isProduction} from "../env.ts";
import {emit, invoke} from "bevy_flurx_api";

export type ActionGroupProperties = {
    [group: string]: ActionsInGroup;
}

export type ActionsInGroup = {
    [name: string]: ActionProperties;
}

export interface ActionProperties {
    is_repeat_animation: boolean;
    transition: TransitionMode;
}

export type TransitionType = "auto" | "manual" | "none";

export interface Action {
    group: string;
    name: string;
}

export interface TransitionMode {
    type: TransitionType;

    /**
     * Represents the range of time to transition to the next action when in auto mode.
     *
     * The value is undefined when the transition mode is not auto.
     */
    min_secs?: number;

    /**
     * Represents the range of time to transition to the next action when in auto mode.
     *
     * The value is undefined when the transition mode is not auto.
     */
    max_secs?: number;

    /**
     * Represents the action to transition to when in manual mode.
     *
     * The value is undefined when the transition mode is not manual.
     */
    next?: Action;
}

export const ActionGroupsControl = () => {
    const {groupEntries, allActions} = useActionGroups();
    return (
        <div
            css={css`
                display: flex;
                flex-direction: column;
                height: 100%;
                min-height: 0;
            `}>
            {groupEntries.map(([group, actionsInGroup]) => (
                <ActionGroupControl
                    key={group}
                    allActions={allActions}
                    group={group}
                    actionsInGroup={actionsInGroup}
                />
            ))}
        </div>
    )
}

const ActionGroupControl: FC<{
    group: string;
    actionsInGroup: ActionsInGroup;
    allActions: Action[];
}> = (p) => {
    return (
        <Grid
            container
            css={css`padding: 8px;`}>
            <Card css={css`
                height: fit-content;
                width: 100%;
            `}>
                <CardContent>
                    <Typography variant="h5" component="div">
                        {p.group}
                    </Typography>
                    <List>
                        {Object.entries(p.actionsInGroup).sort().map(([name, properties]) => (
                            <ActionControl
                                key={name}
                                allActions={p.allActions}
                                action={{group: p.group, name}}
                                properties={properties}
                            />
                        ))}
                    </List>
                </CardContent>
            </Card>
        </Grid>
    )
}

export const emitActionUpdated = (action: Action, properties: ActionProperties) => {
    if (isProduction) {
        emit("update_action", {
            action,
            properties,
        })
    }
};

const useActionGroups = () => {
    const [groups, setGroups] = useState<ActionGroupProperties>({});
    const allActions: Action[] = useMemo(() => {
        return Object
            .entries(groups)
            .flatMap(([group, actionsInGroup]) => Object.keys(actionsInGroup).map(name => ({
                group: group,
                name: name,
            } as Action)));
    }, [groups])
    const groupEntries = useMemo(() => sortedGroups(groups), [groups]);

    useEffect(() => {
        if (isProduction) {
            const unListen = WebWindow.current().listen("actions", (groups: ActionGroupProperties) => {
                setGroups(groups);
            })
            return () => {
                unListen();
            }
        } else {
            setGroups(mockActionGroups);
        }
    }, [groups]);

    useEffect(() => {
        if (isProduction) {
            invoke("request_send_actions").catch(console.error);
        }
    }, []);

    return {
        groups,
        groupEntries,
        allActions,
    };
}

const sortedGroups = (groups: ActionGroupProperties) => {
    const defaults = [
        "idle",
        "drag",
        "sit_down"
    ];
    let arr: [string, ActionsInGroup][] = Object
        .entries(groups)
        .filter(([group]) => !defaults.includes(group))
        .sort();
    const defaultActionsInGroup = defaults
        .map((group) => [group, groups[group]] as [string, ActionsInGroup])
        .filter(([_, actions]) => actions);
    return [...defaultActionsInGroup, ...arr];
}

const mockActionGroups = {
    "idle": {
        "index": {
            is_repeat_animation: true,
            transition: {
                type: "auto",
                timeRange: [10, 60],
            },
        },
        "jump": {
            is_repeat_animation: true,
            transition: {
                type: "auto",
                timeRange: [10, 60],
            },
        }
    },
} as ActionGroupProperties;