import React, { useState, useEffect, useCallback } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { cryptoApi } from './api';
import { CryptoToken, TokenStats } from './types';
import Header from './components/Header';
import StatsOverview from './components/StatsOverview';
import TokenList from './components/TokenList';
import TokenDetail from './components/TokenDetail';
import SearchBar from './components/SearchBar';
import LoadingSpinner from './components/LoadingSpinner';
import { FiRefreshCw, FiClock } from 'react-icons/fi';

function App() {
  const [tokens, setTokens] = useState<CryptoToken[]>([]);
  const [allTokens, setAllTokens] = useState<CryptoToken[]>([]);
  const [stats, setStats] = useState<TokenStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [refreshing, setRefreshing] = useState(false);
  const [selectedToken, setSelectedToken] = useState<CryptoToken | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [viewMode, setViewMode] = useState<'all' | 'favorites'>('all');
  const [lastUpdated, setLastUpdated] = useState<Date | null>(null);
  const [error, setError] = useState<string | null>(null);

  const loadData = useCallback(async (showLoading = true) => {
    try {
      if (showLoading) setLoading(true);
      else setRefreshing(true);
      setError(null);
      
      const [tokensData, statsData] = await Promise.all([
        cryptoApi.getTokens(),
        cryptoApi.getStats()
      ]);
      
      setAllTokens(tokensData);
      setTokens(tokensData);
      setStats(statsData);
      setLastUpdated(new Date());
    } catch (err) {
      console.error('Error loading data:', err);
      setError('Failed to load data. Please try again.');
    } finally {
      setLoading(false);
      setRefreshing(false);
    }
  }, []);

  useEffect(() => {
    loadData();
    const interval = setInterval(() => loadData(false), 60000);
    return () => clearInterval(interval);
  }, [loadData]);

  const handleSearch = useCallback((query: string) => {
    setSearchQuery(query);
    if (query.trim()) {
      const filtered = allTokens.filter(t => 
        t.name.toLowerCase().includes(query.toLowerCase()) ||
        t.symbol.toLowerCase().includes(query.toLowerCase())
      );
      setTokens(filtered);
    } else {
      setTokens(viewMode === 'favorites' ? allTokens.filter(t => t.is_favorite) : allTokens);
    }
  }, [allTokens, viewMode]);

  const handleToggleFavorite = async (token: CryptoToken) => {
    try {
      const updated = await cryptoApi.toggleFavorite({
        token_id: token.token_id,
        is_favorite: !token.is_favorite
      });
      
      const updateTokens = (list: CryptoToken[]) => 
        list.map(t => t.token_id === token.token_id ? updated : t);
      
      setAllTokens(updateTokens);
      setTokens(updateTokens);
      
      if (selectedToken?.token_id === token.token_id) {
        setSelectedToken(updated);
      }
    } catch (err) {
      console.error('Error toggling favorite:', err);
    }
  };

  const handleViewModeChange = useCallback(async (mode: 'all' | 'favorites') => {
    setViewMode(mode);
    setSearchQuery('');
    
    if (mode === 'favorites') {
      const favorites = allTokens.filter(t => t.is_favorite);
      setTokens(favorites);
    } else {
      setTokens(allTokens);
    }
  }, [allTokens]);

  const formatLastUpdated = (date: Date) => {
    return date.toLocaleTimeString(undefined, { 
      hour: '2-digit', 
      minute: '2-digit',
      second: '2-digit'
    });
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-dark-950 via-dark-900 to-dark-950">
      {/* Animated background */}
      <div className="fixed inset-0 overflow-hidden pointer-events-none">
        <div className="absolute -top-1/2 -left-1/2 w-full h-full bg-gradient-to-br from-primary-500/5 to-purple-500/5 rounded-full blur-3xl" />
        <div className="absolute -bottom-1/2 -right-1/2 w-full h-full bg-gradient-to-tl from-blue-500/5 to-primary-500/5 rounded-full blur-3xl" />
      </div>

      <div className="relative z-10">
        <Header />
        
        <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          {loading ? (
            <LoadingSpinner />
          ) : error ? (
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              className="text-center py-16"
            >
              <div className="w-16 h-16 mx-auto mb-4 rounded-full bg-red-500/20 flex items-center justify-center">
                <span className="text-3xl">⚠️</span>
              </div>
              <p className="text-red-400 mb-4">{error}</p>
              <button
                onClick={() => loadData()}
                className="px-6 py-3 bg-primary-500 text-white rounded-xl hover:bg-primary-600 transition-colors"
              >
                Try Again
              </button>
            </motion.div>
          ) : (
            <>
              {/* Stats Section */}
              {stats && (
                <motion.div
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                >
                  <StatsOverview stats={stats} />
                </motion.div>
              )}
              
              {/* Search and Filters */}
              <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.1 }}
                className="mt-6"
              >
                <SearchBar
                  value={searchQuery}
                  onChange={handleSearch}
                  viewMode={viewMode}
                  onViewModeChange={handleViewModeChange}
                />
              </motion.div>

              {/* Status Bar */}
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                transition={{ delay: 0.2 }}
                className="mt-6 flex items-center justify-between"
              >
                <div className="flex items-center gap-4">
                  <span className="text-dark-400 text-sm">
                    {tokens.length} {viewMode === 'favorites' ? 'favorite' : ''} token{tokens.length !== 1 ? 's' : ''}
                  </span>
                  {lastUpdated && (
                    <span className="text-dark-500 text-sm flex items-center gap-1.5">
                      <FiClock className="w-3.5 h-3.5" />
                      Updated {formatLastUpdated(lastUpdated)}
                    </span>
                  )}
                </div>
                <motion.button
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                  onClick={() => loadData(false)}
                  disabled={refreshing}
                  className="flex items-center gap-2 px-4 py-2 rounded-lg bg-dark-800/50 border border-dark-700/50 text-dark-400 hover:text-white transition-colors disabled:opacity-50"
                >
                  <FiRefreshCw className={`w-4 h-4 ${refreshing ? 'animate-spin' : ''}`} />
                  <span className="text-sm font-medium">Refresh</span>
                </motion.button>
              </motion.div>

              {/* Token List */}
              <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.3 }}
                className="mt-4"
              >
                <TokenList
                  tokens={tokens}
                  onSelectToken={setSelectedToken}
                  onToggleFavorite={handleToggleFavorite}
                />
              </motion.div>
              
              {/* Footer */}
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                transition={{ delay: 0.4 }}
                className="mt-8 text-center text-dark-500 text-sm py-6 border-t border-dark-800"
              >
                <p>Powered by CoinGecko API & TradingView Charts</p>
                <p className="mt-1">Data refreshes automatically every 60 seconds</p>
              </motion.div>
            </>
          )}
        </main>
      </div>

      {/* Token Detail Modal */}
      <AnimatePresence>
        {selectedToken && (
          <TokenDetail
            token={selectedToken}
            onClose={() => setSelectedToken(null)}
            onToggleFavorite={handleToggleFavorite}
          />
        )}
      </AnimatePresence>
    </div>
  );
}

export default App;
