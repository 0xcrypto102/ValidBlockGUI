import { useState } from 'react';
import AnchorPane from './components/AnchorPane';
import VerifyPane from './components/VerifyPane';
import SettingsPane from './components/SettingsPane';
import './App.css';
import { invoke } from '@tauri-apps/api/core';

function App() {
  const [view, setView] = useState('anchor');

  const tabClass = (tab) =>
    `px-4 py-2 rounded-md font-medium transition-colors duration-200 ${
      view === tab ? 'bg-accent-primary text-black dark:text-white' : 'bg-panel text-[#888888] hover:bg-accent-primary/30'
    }`;

  return (
    <div className="bg-bg text-white min-h-screen flex flex-col">
      <header className="flex justify-center gap-4 p-4 shadow-md bg-panel">
        <button onClick={() => setView('anchor')} className={tabClass('anchor')}>Anchor</button>
        <button onClick={() => setView('verify')} className={tabClass('verify')}>Verify</button>
        <button onClick={() => setView('settings')} className={tabClass('settings')}>Settings</button>
      </header>

      <main className="flex-1 max-w-3xl w-full mx-auto p-6">
        {view === 'anchor' && <AnchorPane />}
        {view === 'verify' && <VerifyPane />}
        {view === 'settings' && <SettingsPane />}
      </main>
    </div>
  );
}

export default App;