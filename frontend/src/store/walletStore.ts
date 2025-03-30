import { create } from 'zustand';
import { connect, disconnect, getPublicKey } from "../packages/stellar-wallets-kit";

interface WalletState {
  publicKey: string | null;
  isConnected: boolean;
  isConnecting: boolean;
  
  checkConnection: () => Promise<void>;
  connectWallet: () => Promise<void>;
  disconnectWallet: () => Promise<void>;
}

export const useWalletStore = create<WalletState>((set, get) => ({
  publicKey: null,
  isConnected: false,
  isConnecting: false,
  
  checkConnection: async () => {
    try {
      const pk = await getPublicKey();
      if (pk) {
        console.log("Store: Found existing connection:", pk);
        set({ isConnected: true, publicKey: pk });
      }
    } catch (error) {
      console.log("Store: No wallet connected");
      set({ isConnected: false, publicKey: null });
    }
  },
  
  connectWallet: async () => {
    const { isConnected } = get();
    if (isConnected) return;
    
    set({ isConnecting: true });
    
    try {
      await connect();
      
      // Add a delay to allow wallet to initialize
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      const pk = await getPublicKey();
      console.log("Store: Connected with public key:", pk);
      set({ isConnected: true, publicKey: pk, isConnecting: false });
    } catch (error) {
      console.error("Store: Failed to connect wallet:", error);
      set({ isConnected: false, publicKey: null, isConnecting: false });
    }
  },
  
  disconnectWallet: async () => {
    try {
      await disconnect();
      set({ isConnected: false, publicKey: null });
    } catch (error) {
      console.error("Store: Failed to disconnect wallet:", error);
    }
  }
}));