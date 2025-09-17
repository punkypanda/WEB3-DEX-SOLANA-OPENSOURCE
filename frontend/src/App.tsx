import React from "react";
import { ConnectionProvider, WalletProvider } from "@solana/wallet-adapter-react";
import { PhantomWalletAdapter, SolflareWalletAdapter } from "@solana/wallet-adapter-wallets";
import CreateToken from "./pages/CreateToken";
import Explorer from "./pages/Explorer";
import Swap from "./pages/Swap";

const network = process.env.REACT_APP_SOLANA_NETWORK || "https://api.mainnet-beta.solana.com";

export default function App() {
  const wallets = [new PhantomWalletAdapter(), new SolflareWalletAdapter()];

  return (
    <ConnectionProvider endpoint={network}>
      <WalletProvider wallets={wallets} autoConnect>
        <div className="min-h-screen bg-gray-50">
          <header className="p-4 bg-blue-600 text-white font-bold text-xl">AI DEX</header>
          <main className="container mx-auto p-4 space-y-6">
            <Swap />
            <CreateToken />
            <Explorer />
          </main>
        </div>
      </WalletProvider>
    </ConnectionProvider>
  );
}