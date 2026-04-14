import { useMemo, useSyncExternalStore } from 'react';
import { Webview } from '@hmcs/sdk';
import { Button, Toolbar } from '@hmcs/ui';
import { useNavigationState } from './hooks/useNavigationState';

export function App() {
  const webview = useMemo(() => Webview.current(), []);
  const nav = useNavigationState(webview);
  const page = useHashPage();

  return (
    <div className="nav-test-panel">
      <Toolbar
        title="Nav Test"
        onClose={handleClose}
        navigation={{
          canGoBack: nav.canGoBack,
          canGoForward: nav.canGoForward,
          onBack: nav.navigateBack,
          onForward: nav.navigateForward,
        }}
      />
      <div className="nav-test-content">
        {page === 'page2' ? <Page2 /> : <Page1 />}
      </div>
    </div>
  );
}

function Page1() {
  return (
    <>
      <div className="nav-test-title">Page 1</div>
      <p className="nav-test-description">
        Click the button below to navigate to Page 2.<br />
        The Back button in the toolbar should become active.
      </p>
      <Button
        variant="outline"
        onClick={() => { window.location.hash = '#page2'; }}
      >
        Go to Page 2
      </Button>
    </>
  );
}

function Page2() {
  return (
    <>
      <div className="nav-test-title">Page 2</div>
      <p className="nav-test-description">
        You are on Page 2.<br />
        Use the Back button in the toolbar to go back to Page 1.<br />
        After going back, the Forward button should become active.
      </p>
    </>
  );
}

function handleClose() {
  const webview = Webview.current();
  webview?.close();
}

function useHashPage(): string {
  const page = useSyncExternalStore(subscribeHash, getHashPage, getHashPage);
  return page;
}

function subscribeHash(callback: () => void): () => void {
  window.addEventListener('hashchange', callback);
  return () => window.removeEventListener('hashchange', callback);
}

function getHashPage(): string {
  const hash = window.location.hash.replace('#', '');
  return hash || 'page1';
}
