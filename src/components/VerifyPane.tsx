import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { verifyClient } from "../lib/client";
import { listen } from "@tauri-apps/api/event";

export default function VerifyPane() {
  const [file, setFile] = useState<File | null>(null);
  const [result, setResult] = useState<{ verified: boolean } | null>(null);
  const [trinityMode, setTrinityMode] = useState(false);

  const handleVerify = async () => {
    if (!file) return;

    const content = new Uint8Array(await file.arrayBuffer());

    try {
      const res = await verifyClient.verify({ fileContent: content });
      setResult({ verified: res.verified });
    } catch (err) {
      alert(`Verify failed: ${err}`);
    }
  };

  const handleDrop = (e: React.DragEvent<HTMLDivElement>) => {
      e.preventDefault();
      if (e.dataTransfer.files?.[0]) setFile(e.dataTransfer.files[0]);
   };

  useEffect(() => {
    invoke<boolean>("get_trinity_mode").then(setTrinityMode);
  }, []);

  useEffect(() => {
    const un = listen<boolean>("trinity-mode-changed", (e) =>
      setTrinityMode(e.payload)
    );
    return () => { un.then(f => f()); };
  }, []);
  
  return (
    <section className="space-y-6">
      <div
        onDrop={handleDrop}
        onDragOver={(e) => e.preventDefault()}
        className="border-2 border-dashed border-black bg-panel p-6 text-center rounded-lg cursor-pointer"
        title="Drop file to verify"
      >
        {file ? file.name : "Drop a file here or click to select"}
        <input
          type="file"
          onChange={(e) => setFile(e.target.files?.[0] ?? null)}
          className="block mx-auto mt-3"
        />
      </div>

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
        title={trinityMode ? "Disabled in Trinity Mode" : ""}
        className={`px-4 py-2 rounded ${
          trinityMode ? 'bg-gray-400 cursor-not-allowed' : 'bg-gray-600 hover:bg-gray-500 text-white'
        }`}
      >
        Check on Blockchain
      </button>
    </section>
  );
}
