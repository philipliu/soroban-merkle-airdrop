import { useState } from "react";
import * as Contract from "../packages/contract";
import { signTransaction } from "../packages/stellar-wallets-kit";

interface ClaimButtonProps {
    isConnected: boolean;
    walletPublicKey: string | null;
}

export default function ClaimButton(
    { isConnected, walletPublicKey }: ClaimButtonProps,
) {
    const [isLoading, setIsLoading] = useState<boolean>(false);

    const amount = 100;
    const proof = [
        Buffer.from(
            "5a1f3e8294999cb58928c96974a88af852be077c86a139bf9fb44ccd2ca13514",
            "hex",
        ),
        Buffer.from(
            "ca4b9e5036ee918d0b67986fd9ef3261aa5b4dd83ddc1302a030a1e212a68442",
            "hex",
        ),
    ];

    return (
        <div>
            <button
                disabled={isLoading || !isConnected || !walletPublicKey}
                onClick={async () => {
                    if (!walletPublicKey) return;

                    try {
                        const contract = new Contract.Client({
                            ...Contract.networks.testnet,
                            rpcUrl: "https://soroban-testnet.stellar.org:443",
                            publicKey: walletPublicKey,
                        });

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
                Claim
            </button>

            {!isConnected && (
                <div style={{ color: "red", marginTop: "8px" }}>
                    Please connect your wallet first to claim tokens.
                </div>
            )}
        </div>
    );
}
