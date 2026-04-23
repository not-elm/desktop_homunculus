import type { Preview } from '@storybook/react-vite';
import {
  defaultInitialGlobals,
  sceneGlobalType,
  themeGlobalType,
  withSceneLayer,
} from '../src/storybook/preview';

import '../src/index.css';
import './storybook.css';

const preview: Preview = {
  parameters: {
    layout: 'fullscreen',
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },
  },
  globalTypes: {
    scene: sceneGlobalType,
    theme: themeGlobalType,
  },
  initialGlobals: defaultInitialGlobals,
  decorators: [withSceneLayer],
};

export default preview;
