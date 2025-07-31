import { createRoot } from 'react-dom/client'
import "@homunculus/core/dist/index.css";
import { Settings } from './Settings'

createRoot(document.getElementById('root')!).render(
  <Settings />
)
