import ClaimButton from "./components/ClaimButton";
import ConnectWallet from "./components/ConnectWallet";

function App() {
  return (
    <div className="min-h-screen bg-gradient-to-b from-gray-50 to-gray-100 py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-md mx-auto">
        <div className="text-center mb-10">
          <h1 className="text-3xl font-extrabold text-gray-900 sm:text-4xl">
            Merkle Tree Airdrop Demo
         </h1>
          <p className="mt-3 text-base text-gray-500 sm:mt-5 sm:text-lg">
            Connect your wallet to claim your tokens.
          </p>
        </div>

        {/* Card for wallet connection */}
        <div className="bg-white overflow-hidden shadow rounded-lg mb-6">
          <div className="px-4 py-5 sm:p-6">
            <h2 className="text-lg font-medium text-gray-900 mb-4">
              Wallet Connection
            </h2>
            <ConnectWallet />
          </div>
        </div>

        {/* Card for token claiming */}
        <div className="bg-white overflow-hidden shadow rounded-lg">
          <div className="px-4 py-5 sm:p-6">
            <h2 className="text-lg font-medium text-gray-900 mb-4">
              Claim Your Tokens
            </h2>
            <ClaimButton />
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;