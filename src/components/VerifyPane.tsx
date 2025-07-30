import { useState } from 'react';
import { invoke } from "@tauri-apps/api/core";

export default function VerifyPane() {
  const [file, setFile] = useState<File | null>(null);
  const [result, setResult] = useState<{ verified: boolean } | null>(null);
  const [trinityMode, setTrinityMode] = useState(false); // optional: get this from backend

  const handleVerify = async () => {
    if (!file) return;

    const buffer = await file.arrayBuffer();
    const fileContent = Array.from(new Uint8Array(buffer));

    try {
      const verified = await invoke<boolean>('verify_file', { fileContent });
      setResult({ verified });
    } catch (err) {
      alert(`Verify failed: ${err}`);
    }
  };

  return (
    <section className="space-y-6">
      <input
        type="file"
        onChange={(e) => setFile(e.target.files?.[0] ?? null)}
        className="block"
      />

      <button onClick={handleVerify} className="bg-blue-600 hover:bg-blue-500 text-white px-4 py-2 rounded">
        Verify
      </button>

      {result && (
        <div
          className={`p-4 rounded text-white font-medium ${
            result.verified ? 'bg-green-600' : 'bg-red-600'
          }`}
        >
          {result.verified ? '✔ Verified' : '✘ No match'}
        </div>
      )}

      <button
        disabled={trinityMode}
        title={trinityMode ? 'Disabled in Trinity Mode' : ''}
        className={`px-4 py-2 rounded ${
          trinityMode ? 'bg-gray-400 cursor-not-allowed' : 'bg-gray-600 hover:bg-gray-500 text-white'
        }`}
      >
        Check on Blockchain
      </button>
    </section>
  );
}
