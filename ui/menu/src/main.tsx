import { createRoot } from 'react-dom/client'
import '@homunculus/core/dist/index.css';
import "./index.css";
import App from './App.tsx'

createRoot(document.getElementById('root')!).render(
  <App />
)
