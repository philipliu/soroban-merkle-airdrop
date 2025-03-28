import { useEffect, useState } from "react";
import {
    connect,
    disconnect,
    getPublicKey,
} from "../packages/stellar-wallets-kit";

interface ConnectWalletProps {
    onWalletUpdate: (connected: boolean, publicKey: string | null) => void;
}

export default function ConnectWallet({ onWalletUpdate }: ConnectWalletProps) {
    const [isConnected, setIsConnected] = useState(false);
    const [publicKey, setPublicKey] = useState<string | null>(null);

    useEffect(() => {
        const checkConnection = async () => {
            try {
                const pk = await getPublicKey();
                if (pk) {
                    setIsConnected(true);
                    setPublicKey(pk);
                    onWalletUpdate(true, pk);
                }
            } catch (error) {
                // Not connected
                console.log("No wallet connected");
                setIsConnected(false);
                setPublicKey(null);
                onWalletUpdate(false, null);
            }
        };

        checkConnection();
    }, [onWalletUpdate]);

    const handleWalletAction = async () => {
        if (isConnected) {
            // Disconnect
            try {
                await disconnect();
                setIsConnected(false);
                setPublicKey(null);
                onWalletUpdate(false, null);
            } catch (error) {
                console.error("Failed to disconnect wallet:", error);
            }
        } else {
            // Connect
            try {
                await connect();
                const pk = await getPublicKey();
                setIsConnected(true);
                setPublicKey(pk);
                onWalletUpdate(true, pk);
            } catch (error) {
                console.error("Failed to connect wallet:", error);
            }
        }
    };

    return (
        <div style={{ marginBottom: "20px" }}>
            {isConnected && publicKey && (
                <div style={{ marginBottom: "8px" }}>
                    Connected:{" "}
                    <span style={{ fontWeight: "bold" }}>
                        {publicKey.slice(0, 4)}...{publicKey.slice(-4)}
                    </span>
                </div>
            )}
            <button
                onClick={handleWalletAction}
                style={{
                    padding: "8px 16px",
                    backgroundColor: isConnected ? "#f44336" : "#4CAF50",
                    color: "white",
                    border: "none",
                    borderRadius: "4px",
                    cursor: "pointer",
                }}
            >
                {isConnected ? "Disconnect" : "Connect Wallet"}
            </button>
        </div>
    );
}
