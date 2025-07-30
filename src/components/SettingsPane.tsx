import { useState, useEffect } from 'react';
import { invoke } from "@tauri-apps/api/core";

export default function SettingsPane() {
  const [anchorPolicy, setAnchorPolicy] = useState('LocalOnly');
  const [trinity, setTrinity] = useState(false);
  const [rpcEndpoint, setRpcEndpoint] = useState('http://127.0.0.1:8080');

  useEffect(() => {
    const getSetting = async() => {
      // load current settings on mount
      await invoke('get_settings').then((res: any) => {
        setRpcEndpoint(res.rpc_endpoint);
        setAnchorPolicy(res.default_policy);
        setTrinity(res.trinity_mode);
      });
    }
    getSetting();
  }, []);

  const updateSettings = () => {
    invoke('put_settings', {
      newSettings: {
        rpc_endpoint: rpcEndpoint,
        default_policy: anchorPolicy,
        wallet_id: '',
        trinity_mode: trinity,
      }
    });
  };

  const toggleTrinity = (checked: boolean) => {
    setTrinity(checked);
    invoke('toggle_trinity_mode', { enable: checked });
    updateSettings();
  };

  return (
    <section className="space-y-6">
      <div>
        <label className="block mb-1 text-sm font-semibold text-black">Default Anchor Policy</label>
        <select
          value={anchorPolicy}
          onChange={(e) => setAnchorPolicy(e.target.value)}
          className="w-full p-2 rounded-md text-black border-1 border-solid border-black"
        >
          <option value="LocalOnly">Local-only</option>
          <option value="OnChain">On-chain</option>
        </select>
      </div>

      <div>
        <label className="flex items-center gap-2 text-black">
          <input
            type="checkbox"
            checked={trinity}
            onChange={(e) => toggleTrinity(e.target.checked)}
          />
          Enable Trinity Mode
        </label>
      </div>

      <div>
        <label className="block mb-1 text-sm font-semibold text-black">RPC Endpoint</label>
        <input
          type="text"
          value={rpcEndpoint}
          onChange={(e) => setRpcEndpoint(e.target.value)}
          onBlur={updateSettings}
          className="w-full p-2 rounded-md text-black border-1 border-solid border-black"
        />
      </div>
    </section>
  );
}
