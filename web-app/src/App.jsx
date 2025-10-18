import { useState, useEffect } from "react";
import { Cpu, Database, Zap, Activity } from "lucide-react";

function App() {
  const [engineStatus, setEngineStatus] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    checkEngineHealth();
  }, []);

  const checkEngineHealth = async () => {
    try {
      const response = await fetch('/api/health');
      const data = await response.json();
      setEngineStatus(data);
    } catch (error) {
      console.error('Engine health check failed:', error);
      setEngineStatus({ success: false, message: 'Engine offline' });
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-900 via-purple-900 to-violet-900">
      <div className="container mx-auto px-4 py-8">
        {/* Header */}
        <header className="text-center mb-12">
          <div className="flex items-center justify-center mb-4">
            <Zap className="h-12 w-12 text-yellow-400 mr-3" />
            <h1 className="text-4xl font-bold text-white">
              CodeRED-Astra
            </h1>
          </div>
          <p className="text-gray-300 text-lg">
            Hackathon Project - React Frontend + Rust Engine
          </p>
        </header>

        {/* Status Cards */}
        <div className="grid md:grid-cols-2 gap-6 mb-8">
          {/* React App Status */}
          <div className="bg-white/10 backdrop-blur-lg rounded-lg p-6 border border-white/20">
            <div className="flex items-center mb-4">
              <Activity className="h-8 w-8 text-blue-400 mr-3" />
              <h2 className="text-xl font-semibold text-white">React Frontend</h2>
            </div>
            <div className="text-green-400 font-medium">✓ Online</div>
            <p className="text-gray-300 text-sm mt-2">
              Vite + React development environment ready
            </p>
          </div>

          {/* Rust Engine Status */}
          <div className="bg-white/10 backdrop-blur-lg rounded-lg p-6 border border-white/20">
            <div className="flex items-center mb-4">
              <Cpu className="h-8 w-8 text-orange-400 mr-3" />
              <h2 className="text-xl font-semibold text-white">Rust Engine</h2>
            </div>
            <div className={`font-medium ${loading ? 'text-yellow-400' : engineStatus?.success ? 'text-green-400' : 'text-red-400'}`}>
              {loading ? '⏳ Checking...' : engineStatus?.success ? '✓ Online' : '✗ Offline'}
            </div>
            <p className="text-gray-300 text-sm mt-2">
              {loading ? 'Connecting to engine...' : 
               engineStatus?.success ? 'Engine responding normally' : 
               'Engine may still be starting up'}
            </p>
          </div>
        </div>

        {/* Engine Details */}
        {engineStatus?.success && (
          <div className="bg-white/10 backdrop-blur-lg rounded-lg p-6 border border-white/20 mb-8">
            <div className="flex items-center mb-4">
              <Database className="h-6 w-6 text-purple-400 mr-3" />
              <h3 className="text-lg font-semibold text-white">Engine Status</h3>
            </div>
            <div className="grid md:grid-cols-2 gap-4 text-sm">
              <div>
                <span className="text-gray-400">Status:</span>
                <span className="text-white ml-2">{engineStatus.data?.status}</span>
              </div>
              <div>
                <span className="text-gray-400">Last Check:</span>
                <span className="text-white ml-2">
                  {engineStatus.data?.timestamp ? new Date(engineStatus.data.timestamp).toLocaleTimeString() : 'N/A'}
                </span>
              </div>
            </div>
            {engineStatus.message && (
              <div className="mt-3 p-3 bg-yellow-500/20 border border-yellow-500/30 rounded">
                <span className="text-yellow-200 text-sm">{engineStatus.message}</span>
              </div>
            )}
          </div>
        )}

        {/* Quick Actions */}
        <div className="bg-white/10 backdrop-blur-lg rounded-lg p-6 border border-white/20">
          <h3 className="text-lg font-semibold text-white mb-4">Quick Actions</h3>
          <div className="flex flex-wrap gap-3">
            <button 
              onClick={checkEngineHealth}
              className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
            >
              Refresh Status
            </button>
            <button 
              onClick={() => window.open('/api/health', '_blank')}
              className="px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded-lg transition-colors"
            >
              Test API Direct
            </button>
            <button 
              onClick={() => alert('Add your hackathon features here!')}
              className="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors"
            >
              Add Feature
            </button>
          </div>
        </div>

        {/* Development Notes */}
        <div className="mt-8 text-center text-gray-400 text-sm">
          <p>🚀 Ready for hackathon development!</p>
          <p className="mt-1">
            Frontend team: Work in <code className="bg-white/10 px-1 rounded">web-app/src/</code> | 
            Backend team: Work in <code className="bg-white/10 px-1 rounded">rust-engine/src/</code>
          </p>
        </div>
      </div>
    </div>
  );
}

export default App;
