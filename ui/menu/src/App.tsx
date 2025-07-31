import './App.css'
import { useEffect, useState } from "react";
import { Menus } from "./Menu.tsx";
import { mods } from "@homunculus/api";

function App() {
    const [menus, setMenus] = useState<mods.ModMenuMetadata[]>([]);
    useEffect(() => {
        mods.menus().then(menus => {
            setMenus(menus);
        }).catch(err => {
            console.error("Failed to fetch menus:", err);
        });
    }, []);
    return (
        <Menus menus={menus} />
    )
}

export default App
