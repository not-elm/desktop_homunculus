import type { FC, ReactNode } from "react";
import {
    Card,
    CardContent,
    CardHeader,
    CardTitle,
    CardDescription,
} from "@/components/ui/card";

export const SettingsCard: FC<{ title: string; description?: string; children: ReactNode }> = ({
    title,
    description,
    children,
}) => {
    return (
        <Card className="flex flex-col">
            <CardHeader>
                <CardTitle>{title}</CardTitle>
                {description && <CardDescription>{description}</CardDescription>}
            </CardHeader>
            <CardContent className="flex flex-col gap-4">{children}</CardContent>
        </Card>
    );
};
