import { createRoot } from 'react-dom/client';
import { FluentProvider, webLightTheme } from '@fluentui/react-components';

import App from './App';

const root = createRoot(document.getElementById('root') as HTMLElement);

root.render(
  <FluentProvider theme={webLightTheme}>
    <App />
  </FluentProvider>,
);