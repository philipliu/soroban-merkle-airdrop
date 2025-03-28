import { useEffect, useState } from "react";
import * as Contract from "../packages/contract";
import { signTransaction } from "../packages/stellar-wallets-kit";

interface ClaimButtonProps {
    isConnected: boolean;
    walletPublicKey: string | null;
}

interface ProofEntry {
    receiver: {
        address: string;
        amount: number;
    };
    proofs: string[];
}

export default function ClaimButton(
    { isConnected, walletPublicKey }: ClaimButtonProps,
) {
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const [proofData, setProofData] = useState<ProofEntry[]>([]);
    const [isDataLoaded, setIsDataLoaded] = useState<boolean>(false);

    useEffect(() => {
        const fetchProofData = async () => {
            try {
                const response = await fetch("/proofs.json");
                const data = await response.json();
                setProofData(data);
                setIsDataLoaded(true);
            } catch (error) {
                console.error("Failed to load proof data:", error);
            }
        };

        fetchProofData();
    }, []);

    const walletProof = walletPublicKey
        ? proofData.find((entry) => entry.receiver.address === walletPublicKey)
        : undefined;

    const amount = walletProof?.receiver.amount || 0;
    const proofHexes = walletProof?.proofs || [];
    const proof = proofHexes.map((hex) => Buffer.from(hex, "hex"));

    const contract = new Contract.Client({
        ...Contract.networks.testnet,
        rpcUrl: "https://soroban-testnet.stellar.org:443",
        contractId: "CB2E23M6KFTKHNVP4JVW34XGZCODNGPPSENAFFX22RAXD2K3BU3NQ4IE", // Your contract ID
    });

    return (
        <div>
            <button
                disabled={isLoading ||
                    !isConnected ||
                    !walletPublicKey ||
                    !isDataLoaded ||
                    !walletProof}
                onClick={async () => {
                    if (!walletPublicKey || !walletProof) return;

                    try {
                        setIsLoading(true);
                        const tx = await contract.claim({
                            receiver: walletPublicKey,
                            amount: BigInt(amount),
                            proof,
                        });
                        const result = await tx.signAndSend({
                            signTransaction: async (xdr: string) => {
                                return signTransaction(xdr);
                            },
                        });
                        console.log("Transaction result:", result.result);
                    } catch (error) {
                        console.error("Claim error:", error);
                    } finally {
                        setIsLoading(false);
                    }
                }}
            >
                {isLoading ? "Processing..." : "Claim"}
            </button>

            {!isConnected && (
                <div style={{ color: "red", marginTop: "8px" }}>
                    Please connect your wallet first to claim tokens.
                </div>
            )}

            {isConnected && walletPublicKey && !walletProof && isDataLoaded && (
                <div style={{ color: "red", marginTop: "8px" }}>
                    No airdrop allocation found for this wallet.
                </div>
            )}

            {isConnected && walletPublicKey && walletProof && (
                <div style={{ color: "green", marginTop: "8px" }}>
                    You can claim {amount / 100000000} tokens.
                </div>
            )}
        </div>
    );
}
