import { useState } from 'react';
import { invoke } from "@tauri-apps/api/core";
import { AnchorRequest, Policy } from "../gen/proto/validblock_pb";
import { anchorClient } from "../lib/client";
import { sha256 } from "@noble/hashes/sha256";
import { Buffer } from 'buffer';

export default function AnchorPane() {
  const [file, setFile] = useState<File | null>(null);
  const [memo, setMemo] = useState('');
  const [digest, setDigest] = useState<string | null>(null);
  const isMemoTooLong = memo.length > 47;

  const handleDrop = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    if (e.dataTransfer.files?.[0]) setFile(e.dataTransfer.files[0]);
  };

  const handleAnchor = async (policy: Policy) => {
    if (!file) return;
    // const buffer = await file.arrayBuffer();
    // const fileContent = Array.from(new Uint8Array(buffer));
    try {
    //   const res = await invoke<string>('anchor_file', {
    //     fileContent,
    //     memo,
    //     useOnChain: policy === Policy.ON_CHAIN,
    //   });
    //   setDigest(res);
      const content = new Uint8Array(await file.arrayBuffer());
      const digestHex = Buffer.from(sha256(content)).toString('hex');

      // pre‑flight duplicate ask
      // const already = await invoke<boolean>('digest_exists', { digestHex });
      // if (already && !confirm('Digest already anchored. Anchor again?')) return;

      const req = new AnchorRequest({
        fileContent: content,
        memo,
        policy: policy == Policy.ON_CHAIN ? Policy.ON_CHAIN : Policy.LOCAL_ONLY,
      });

      const res = await anchorClient.anchor(req);
      setDigest(res.digest);
    } catch (err) {
      alert(`Anchor failed: ${err}`);
    }
  };

  return (
    <section className="space-y-6">
      <div
        onDrop={handleDrop}
        onDragOver={(e) => e.preventDefault()}
        className="border-2 border-dashed border-black bg-panel text-black text-center p-6 rounded-lg cursor-pointer"
      >
        <p>{file ? file.name : 'Drop a file here or click to select'}</p>
        <input
          type="file"
          onChange={(e) => setFile(e.target.files?.[0] ?? null)}
          className="block border-1 border-solid border-black mx-auto mt-3 px-5 py-3 bg-gray-500 rounded-md"
        />
      </div>

      <div>
        <label className="block mb-1 text-sm font-medium text-gray-500">Memo (optional)</label>
        <textarea
          value={memo}
          onChange={(e) => setMemo(e.target.value)}
          className="w-full p-3 rounded-md text-black border-1 border-solid border-black"
          placeholder="Max 47 bytes"
        />
        <p className={`text-sm mt-1 ${isMemoTooLong ? 'text-red-500' : 'text-gray-400'}`}>
          {memo.length} / 47 bytes
        </p>
      </div>

      <div className="flex gap-4">
        <button
          className="bg-gray-600 text-white px-4 py-2 rounded-md hover:bg-gray-500"
          onClick={() => handleAnchor(Policy.LOCAL_ONLY)}
        >
          Anchor Locally
        </button>
        <button
          onClick={() => handleAnchor(Policy.ON_CHAIN)}
          disabled={isMemoTooLong}
          className={`px-4 py-2 rounded-md font-semibold ${
            isMemoTooLong ? 'bg-gray-400 cursor-not-allowed' : 'bg-accent-primary hover:bg-orange-600 text-white'
          }`}
        >
          Anchor On-Chain
        </button>
      </div>

      {digest && (
        <div className="mt-4 p-4 bg-green-600 text-white rounded-md">
          Anchored with digest: <code>{digest}</code>
        </div>
      )}
    </section>
  );
}
