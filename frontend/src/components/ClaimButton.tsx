import { useEffect, useState, useCallback, useRef } from "react";
import * as Contract from "../packages/contract";
import { signTransaction } from "../packages/stellar-wallets-kit";
import { useWalletStore } from "../store/walletStore";

interface ProofEntry {
    index: number;
    receiver: {
        address: string;
        amount: number;
    };
    proofs: string[];
}

export default function ClaimButton() {
    const { isConnected, publicKey } = useWalletStore();

    const eligibilityChecked = useRef<boolean>(false);

    const [isLoading, setIsLoading] = useState<boolean>(false);
    const [contractId, setContractId] = useState<string | null>(null);
    const [proofData, setProofData] = useState<ProofEntry[]>([]);
    const [isDataLoaded, setIsDataLoaded] = useState<boolean>(false);
    const [isSimulating, setIsSimulating] = useState<boolean>(false);
    const [canClaim, setCanClaim] = useState<boolean>(false);
    const [statusMessage, setStatusMessage] = useState<string | null>(null);
    const [statusType, setStatusType] = useState<'info' | 'success' | 'error' | 'warning'>('info');

    useEffect(() => {
        try {
            const envData = import.meta.env.VITE_PROOF_DATA;
            if (envData) {
                setProofData(JSON.parse(envData));
                setIsDataLoaded(true);
            }
        } catch (error) {
            console.error("Failed to parse proof data:", error);
        }
    }, []);

    useEffect(() => {
        const contractId = import.meta.env.VITE_CONTRACT_ID;
        if (contractId) {
            setContractId(contractId);
        } else {
            console.error("Contract ID is not set in environment variables.");
        }
    }, []);

    useEffect(() => {
        if (publicKey) {
            eligibilityChecked.current = false;
        }
    }, [publicKey]);

    const walletProof = publicKey
        ? proofData.find((entry) => entry.receiver.address === publicKey)
        : undefined;

    const index = walletProof?.index || 0;
    const amount = walletProof?.receiver.amount || 0;
    const proofHexes = walletProof?.proofs || [];
    const proof = proofHexes.map((hex) => Buffer.from(hex, "hex"));

    const getContract = useCallback(() => {
        if (!contractId || !publicKey) return null;
        
        return new Contract.Client({
            ...Contract.networks.testnet,
            rpcUrl: "https://soroban-testnet.stellar.org:443",
            contractId: contractId,
            publicKey: publicKey,
        });
    }, [contractId, publicKey]);

    // Simulate transaction to check eligibility
    const simulateTransaction = useCallback(async (forceCheck = false) => {
        if (eligibilityChecked.current && !forceCheck) {
            return;
        }

        if (!publicKey || !walletProof || !contractId || !isConnected || !isDataLoaded) {
            setCanClaim(false);
            return;
        }

        try {
            setIsSimulating(true);
            setStatusMessage("Checking eligibility...");
            setStatusType('info');
            
            const contract = getContract();
            if (!contract) {
                throw new Error("Failed to initialize contract");
            }
            
            // Simulate the transaction
            const tx = await contract.claim({
                index,
                receiver: publicKey,
                amount: BigInt(amount),
                proof,
            });
            
            if (tx.result.isErr()) {
                setCanClaim(false);
                setStatusMessage("You are not eligible to claim tokens.");
                setStatusType('error');
            } else {
                setCanClaim(true);
                setStatusMessage("You are eligible to claim tokens!");
                setStatusType('success');
            }
        } catch (error) {
            console.error("Simulation error:", error);
            setCanClaim(false);
            setStatusType('error');
        } finally {
            setIsSimulating(false);
            eligibilityChecked.current = true;
        }
    }, [publicKey, walletProof, contractId, isConnected, isDataLoaded, amount, proof, getContract]);

    useEffect(() => {
        // Only run if we haven't checked eligibility yet AND we have all required data
        if (!eligibilityChecked.current && publicKey && walletProof && contractId && isConnected && isDataLoaded) {
            simulateTransaction();
        }
    }, [publicKey, walletProof, contractId, isConnected, isDataLoaded, simulateTransaction]);

    const getStatusColor = () => {
        switch (statusType) {
            case 'success': return 'text-green-700 bg-green-50';
            case 'error': return 'text-red-700 bg-red-50';
            case 'warning': return 'text-yellow-700 bg-yellow-50';
            default: return 'text-blue-700 bg-blue-50';
        }
    };

    const formatAmount = (amount: number) => {
        return (amount / 10000000).toLocaleString(undefined, {
            minimumFractionDigits: 0,
            maximumFractionDigits: 7
        });
    };

    return (
        <div>
            {/* Airdrop amount */}
            {isConnected && walletProof && (
                <div className="mb-4">
                    <span className="text-sm text-gray-600">Your allocation:</span>
                    <div className="mt-1 text-2xl font-bold text-gray-900">
                        {formatAmount(amount)} XLM
                    </div>
                </div>
            )}
            
            {/* Status message */}
            {statusMessage && (
                <div className={`${getStatusColor()} p-3 rounded-md mb-4 text-sm`}>
                    {statusMessage}
                </div>
            )}

            {/* Claim button */}
            <button
                type="button"
                disabled={isLoading || isSimulating || !canClaim || !isConnected}
                onClick={async () => {
                    if (!publicKey || !walletProof || !canClaim) return;

                    try {
                        setIsLoading(true);
                        setStatusMessage("Processing transaction...");
                        setStatusType('info');
                        
                        const contract = getContract();
                        if (!contract) {
                            throw new Error("Failed to initialize contract");
                        }
                        
                        const tx = await contract.claim({
                            index,
                            receiver: publicKey,
                            amount: BigInt(amount),
                            proof,
                        });
                        
                        const result = await tx.signAndSend({
                            signTransaction: async (xdr: string) => {
                                return signTransaction(xdr);
                            },
                        });
                        
                        console.log("Transaction result:", result);
                        
                        setStatusMessage("Tokens claimed successfully!");
                        setStatusType('success');
                        setCanClaim(false);
                    } catch (error) {
                        console.error("Claim error:", error);
                        setStatusMessage(`Error: ${error}`);
                        setStatusType('error');
                    } finally {
                        setIsLoading(false);
                    }
                }}
                className={`
                    w-full flex justify-center py-3 px-4 border border-transparent rounded-md shadow-sm
                    text-sm font-medium text-white
                    ${!canClaim || !isConnected 
                      ? 'bg-gray-300 cursor-not-allowed' 
                      : isLoading 
                        ? 'bg-orange-400 cursor-wait' 
                        : 'bg-green-600 hover:bg-green-700'}
                    focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500
                    transition-colors duration-150 ease-in-out
                `}
            >
                {isLoading ? (
                    <span className="flex items-center">
                        <svg className="animate-spin -ml-1 mr-2 h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                        </svg>
                        Processing...
                    </span>
                ) : isSimulating ? (
                    "Checking eligibility..."
                ) : !isConnected ? (
                    "Connect wallet first"
                ) : !walletProof ? (
                    "No allocation found"
                ) : canClaim ? (
                    `Claim ${formatAmount(amount)} XLM`
                ) : (
                    "Not eligible"
                )}
            </button>

            {/* Not connected message */}
            {!isConnected && (
                <div className="mt-4 text-sm text-gray-500 text-center">
                    Connect your wallet above to check your eligibility
                </div>
            )}
            
            {/* No allocation message */}
            {isConnected && publicKey && !walletProof && isDataLoaded && (
                <div className="mt-4 text-sm text-red-500 text-center">
                    No tokens are allocated for your wallet address
                </div>
            )}
        </div>
    );
}