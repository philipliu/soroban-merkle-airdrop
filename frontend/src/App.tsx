import { useState } from "react";
import "./App.css";
import ClaimButton from "./components/ClaimButton";
import ConnectWallet from "./components/ConnectWallet";

function App() {
  const [walletPublicKey, setWalletPublicKey] = useState<string | null>(null);
  const [isConnected, setIsConnected] = useState<boolean>(false);

  const handleWalletUpdate = (connected: boolean, publicKey: string | null) => {
    setIsConnected(connected);
    setWalletPublicKey(publicKey);
  };

  return (
    <div>
      <ConnectWallet onWalletUpdate={handleWalletUpdate}></ConnectWallet>
      <ClaimButton
        isConnected={isConnected}
        walletPublicKey={walletPublicKey}
      >
      </ClaimButton>
    </div>
  );
}

export default App;
