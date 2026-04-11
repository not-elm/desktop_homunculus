import {
  defaultInitialGlobals,
  sceneGlobalType,
  themeGlobalType,
  withSceneLayer,
} from '@hmcs/ui/storybook';
import type { Preview } from '@storybook/react-vite';

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
  decorators: [withSceneLayer as any],
};

export default preview;
