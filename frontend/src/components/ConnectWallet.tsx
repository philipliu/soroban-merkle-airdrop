import { useEffect } from "react";
import { useWalletStore } from "../store/walletStore";

interface ConnectWalletProps {
  onWalletUpdate?: (connected: boolean, publicKey: string | null) => void;
}

export default function ConnectWallet({ onWalletUpdate }: ConnectWalletProps) {
  const { publicKey, isConnected, isConnecting, connectWallet, disconnectWallet, checkConnection } = useWalletStore();
  
  useEffect(() => {
    if (onWalletUpdate) {
      onWalletUpdate(isConnected, publicKey);
    }
  }, [isConnected, publicKey, onWalletUpdate]);
  
  const handleWalletAction = async () => {
    await checkConnection();
    if (isConnected) {
      await disconnectWallet();
    } else {
      await connectWallet();
    }
  };
  
  return (
    <div>
      {isConnected && publicKey ? (
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center">
            <div className="h-3 w-3 bg-green-500 rounded-full mr-2 animate-pulse"></div>
            <span className="text-sm text-gray-600">
              Connected: 
              <span className="ml-1 font-medium text-gray-900">
                {publicKey.slice(0, 4)}...{publicKey.slice(-4)}
              </span>
            </span>
          </div>
          <button
            type="button"
            onClick={handleWalletAction}
            disabled={isConnecting}
            className={`
              px-3 py-1 text-xs font-medium rounded-md
              ${isConnecting ? 'bg-gray-300 cursor-not-allowed' : 'bg-red-100 text-red-700 hover:bg-red-200'}
              transition-colors duration-150 ease-in-out
            `}
          >
            Disconnect
          </button>
        </div>
      ) : (
        <button
          type="button"
          onClick={handleWalletAction}
          disabled={isConnecting}
          className={`
            w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm
            text-sm font-medium text-white
            ${isConnecting ? 'bg-indigo-400 cursor-wait' : 'bg-indigo-600 hover:bg-indigo-700'}
            focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500
            transition-colors duration-150 ease-in-out
          `}
        >
          {isConnecting ? (
            <span className="flex items-center">
              <svg className="animate-spin -ml-1 mr-2 h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              Connecting...
            </span>
          ) : (
            'Connect Wallet'
          )}
        </button>
      )}
    </div>
  );
}