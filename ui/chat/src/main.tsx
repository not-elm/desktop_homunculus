import { createRoot } from 'react-dom/client'
import "@radix-ui/themes/styles.css";
import { motion } from 'motion/react';
import { SettingsSidebar } from "./SettingsSidebar.tsx";
import { Thread } from './components/assistant-ui/thread.tsx';
import { HomunculusProvider } from './api/runtime.tsx';
import "@homunculus/core/dist/index.css";
import "@homunculus/core/src/index.css";
import './index.css'
import './animation.css'

createRoot(document.getElementById('root')!).render(
    <motion.div
        className='h-screen w-screen flex flex-col bg-transparent'
        initial={{ scale: 0.0 }}
        animate={{ scale: 1.0 }}
        transition={{
            duration: 0.5,
            ease: "easeInOut",
        }}
    >
        <HomunculusProvider>
            <Thread />
            <SettingsSidebar />
        </HomunculusProvider>
    </motion.div>
)
