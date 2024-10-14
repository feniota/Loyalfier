import { createRoot } from 'react-dom/client';
import { FluentProvider, webLightTheme, webDarkTheme } from '@fluentui/react-components';

import App from './App';

const root = createRoot(document.getElementById('root') as HTMLElement);

root.render(
  <FluentProvider theme={webDarkTheme}>
    <App />
  </FluentProvider>,
);