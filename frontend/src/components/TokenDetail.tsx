import React, { useEffect, useRef, useState, memo } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { CryptoToken } from '../types';
import { 
  FiX, 
  FiStar, 
  FiTrendingUp, 
  FiTrendingDown, 
  FiActivity, 
  FiDollarSign,
  FiBarChart2,
  FiPieChart,
  FiGlobe,
  FiClock,
  FiMaximize2
} from 'react-icons/fi';

interface TokenDetailProps {
  token: CryptoToken;
  onClose: () => void;
  onToggleFavorite: (token: CryptoToken) => void;
}

// TradingView symbol mapping for major cryptocurrencies
const getTradingViewSymbol = (symbol: string, tokenId: string): string => {
  const symbolMap: Record<string, string> = {
    'btc': 'BINANCE:BTCUSDT',
    'eth': 'BINANCE:ETHUSDT',
    'bnb': 'BINANCE:BNBUSDT',
    'xrp': 'BINANCE:XRPUSDT',
    'ada': 'BINANCE:ADAUSDT',
    'doge': 'BINANCE:DOGEUSDT',
    'sol': 'BINANCE:SOLUSDT',
    'dot': 'BINANCE:DOTUSDT',
    'matic': 'BINANCE:MATICUSDT',
    'shib': 'BINANCE:SHIBUSDT',
    'ltc': 'BINANCE:LTCUSDT',
    'trx': 'BINANCE:TRXUSDT',
    'avax': 'BINANCE:AVAXUSDT',
    'link': 'BINANCE:LINKUSDT',
    'atom': 'BINANCE:ATOMUSDT',
    'uni': 'BINANCE:UNIUSDT',
    'etc': 'BINANCE:ETCUSDT',
    'xlm': 'BINANCE:XLMUSDT',
    'near': 'BINANCE:NEARUSDT',
    'fil': 'BINANCE:FILUSDT',
    'apt': 'BINANCE:APTUSDT',
    'arb': 'BINANCE:ARBUSDT',
    'op': 'BINANCE:OPUSDT',
    'inj': 'BINANCE:INJUSDT',
    'sui': 'BINANCE:SUIUSDT',
    'pepe': 'BINANCE:PEPEUSDT',
    'wif': 'BINANCE:WIFUSDT',
    'render': 'BINANCE:RENDERUSDT',
    'fet': 'BINANCE:FETUSDT',
    'floki': 'BINANCE:FLOKIUSDT',
  };
  
  const lowerSymbol = symbol.toLowerCase();
  return symbolMap[lowerSymbol] || `BINANCE:${symbol.toUpperCase()}USDT`;
};

// TradingView Advanced Chart Widget Component
const TradingViewChart: React.FC<{ symbol: string; tokenName: string }> = memo(({ symbol, tokenName }) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [chartInterval, setChartInterval] = useState('D');
  
  const intervals = [
    { label: '1H', value: '60' },
    { label: '4H', value: '240' },
    { label: '1D', value: 'D' },
    { label: '1W', value: 'W' },
    { label: '1M', value: 'M' },
  ];

  useEffect(() => {
    if (!containerRef.current) return;
    
    // Clear previous widget
    containerRef.current.innerHTML = '';
    setIsLoading(true);

    // Create TradingView widget container
    const widgetContainer = document.createElement('div');
    widgetContainer.className = 'tradingview-widget-container';
    widgetContainer.style.height = '100%';
    widgetContainer.style.width = '100%';

    const widgetDiv = document.createElement('div');
    widgetDiv.className = 'tradingview-widget-container__widget';
    widgetDiv.style.height = 'calc(100% - 32px)';
    widgetDiv.style.width = '100%';
    
    widgetContainer.appendChild(widgetDiv);
    containerRef.current.appendChild(widgetContainer);

    // Create and load TradingView script
    const script = document.createElement('script');
    script.src = 'https://s3.tradingview.com/external-embedding/embed-widget-advanced-chart.js';
    script.type = 'text/javascript';
    script.async = true;
    script.innerHTML = JSON.stringify({
      autosize: true,
      symbol: symbol,
      interval: chartInterval,
      timezone: "Etc/UTC",
      theme: "dark",
      style: "1",
      locale: "en",
      enable_publishing: false,
      backgroundColor: "rgba(15, 23, 42, 0)",
      gridColor: "rgba(255, 255, 255, 0.05)",
      hide_top_toolbar: false,
      hide_legend: false,
      save_image: true,
      calendar: false,
      hide_volume: false,
      support_host: "https://www.tradingview.com",
      container_id: widgetDiv.id,
      studies: [
        "RSI@tv-basicstudies",
        "MASimple@tv-basicstudies"
      ],
      show_popup_button: true,
      popup_width: "1000",
      popup_height: "650"
    });

    script.onload = () => {
      setTimeout(() => setIsLoading(false), 1000);
    };

    widgetContainer.appendChild(script);

    return () => {
      if (containerRef.current) {
        containerRef.current.innerHTML = '';
      }
    };
  }, [symbol, chartInterval]);

  return (
    <div className="relative h-full">
      {/* Interval Selector */}
      <div className="absolute top-2 right-2 z-10 flex gap-1 bg-dark-800/80 backdrop-blur-sm rounded-lg p-1">
        {intervals.map((int) => (
          <button
            key={int.value}
            onClick={() => setChartInterval(int.value)}
            className={`px-3 py-1.5 text-xs font-medium rounded-md transition-all ${
              chartInterval === int.value
                ? 'bg-primary-500 text-white shadow-lg shadow-primary-500/30'
                : 'text-dark-400 hover:text-white hover:bg-dark-700'
            }`}
          >
            {int.label}
          </button>
        ))}
      </div>
      
      {/* Loading State */}
      <AnimatePresence>
        {isLoading && (
          <motion.div
            initial={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="absolute inset-0 bg-dark-800 flex items-center justify-center z-20 rounded-2xl"
          >
            <div className="text-center">
              <div className="relative w-16 h-16 mx-auto mb-4">
                <div className="absolute inset-0 rounded-full border-4 border-dark-600"></div>
                <div className="absolute inset-0 rounded-full border-4 border-primary-500 border-t-transparent animate-spin"></div>
              </div>
              <p className="text-dark-400 text-sm">Loading {tokenName} Chart...</p>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
      
      {/* Chart Container */}
      <div ref={containerRef} className="h-full w-full rounded-2xl overflow-hidden" />
    </div>
  );
});

TradingViewChart.displayName = 'TradingViewChart';

// Stat Card Component
const StatCard: React.FC<{
  icon: React.ReactNode;
  label: string;
  value: string;
  subValue?: string;
  trend?: 'up' | 'down' | 'neutral';
}> = ({ icon, label, value, subValue, trend }) => (
  <motion.div
    whileHover={{ scale: 1.02, y: -2 }}
    className="glass rounded-2xl p-5 relative overflow-hidden group"
  >
    <div className="absolute inset-0 bg-gradient-to-br from-primary-500/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity" />
    <div className="relative">
      <div className="flex items-center gap-2 mb-3">
        <div className="p-2 rounded-lg bg-dark-700/50 text-primary-400">
          {icon}
        </div>
        <span className="text-dark-400 text-sm font-medium">{label}</span>
      </div>
      <p className="text-2xl font-bold text-white mb-1">{value}</p>
      {subValue && (
        <p className={`text-sm font-medium ${
          trend === 'up' ? 'text-green-400' : 
          trend === 'down' ? 'text-red-400' : 
          'text-dark-400'
        }`}>
          {subValue}
        </p>
      )}
    </div>
  </motion.div>
);

const TokenDetail: React.FC<TokenDetailProps> = ({ token, onClose, onToggleFavorite }) => {
  const [isFullscreen, setIsFullscreen] = useState(false);
  const tradingViewSymbol = getTradingViewSymbol(token.symbol, token.token_id);

  const formatNumber = (num: number | undefined | null): string => {
    if (num === undefined || num === null) return 'N/A';
    if (num >= 1e12) return `$${(num / 1e12).toFixed(2)}T`;
    if (num >= 1e9) return `$${(num / 1e9).toFixed(2)}B`;
    if (num >= 1e6) return `$${(num / 1e6).toFixed(2)}M`;
    if (num >= 1e3) return `$${(num / 1e3).toFixed(2)}K`;
    return `$${num.toFixed(2)}`;
  };

  const formatSupply = (num: number | undefined | null): string => {
    if (num === undefined || num === null) return 'N/A';
    if (num >= 1e12) return `${(num / 1e12).toFixed(2)}T`;
    if (num >= 1e9) return `${(num / 1e9).toFixed(2)}B`;
    if (num >= 1e6) return `${(num / 1e6).toFixed(2)}M`;
    return num.toLocaleString(undefined, { maximumFractionDigits: 0 });
  };

  const formatPrice = (price: number): string => {
    if (price >= 1000) {
      return price.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 });
    } else if (price >= 1) {
      return price.toFixed(4);
    } else if (price >= 0.0001) {
      return price.toFixed(6);
    } else {
      return price.toFixed(10);
    }
  };

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      onClick={onClose}
      className="fixed inset-0 bg-black/90 backdrop-blur-md z-50 flex items-center justify-center p-2 md:p-4"
    >
      <motion.div
        initial={{ scale: 0.9, opacity: 0, y: 30 }}
        animate={{ scale: 1, opacity: 1, y: 0 }}
        exit={{ scale: 0.9, opacity: 0, y: 30 }}
        transition={{ type: 'spring', damping: 25, stiffness: 300 }}
        onClick={(e) => e.stopPropagation()}
        className={`bg-gradient-to-br from-dark-800 via-dark-900 to-dark-950 rounded-3xl shadow-2xl border border-dark-700/50 overflow-hidden ${
          isFullscreen ? 'fixed inset-2 max-w-none max-h-none' : 'max-w-7xl w-full max-h-[95vh]'
        }`}
      >
        <div className="h-full flex flex-col overflow-hidden">
          {/* Header */}
          <div className="flex-shrink-0 p-6 border-b border-dark-700/50">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-4">
                {token.image && (
                  <motion.div
                    initial={{ scale: 0 }}
                    animate={{ scale: 1 }}
                    transition={{ type: 'spring', damping: 15 }}
                    className="relative"
                  >
                    <div className="absolute inset-0 bg-primary-500/20 rounded-full blur-xl" />
                    <img
                      src={token.image}
                      alt={token.name}
                      className="relative w-14 h-14 md:w-16 md:h-16 rounded-full ring-2 ring-dark-600 shadow-xl"
                    />
                  </motion.div>
                )}
                <div>
                  <div className="flex items-center gap-3">
                    <h2 className="text-2xl md:text-3xl font-bold text-white">{token.name}</h2>
                    <span className="px-3 py-1 rounded-full bg-dark-700 text-dark-300 text-sm font-medium uppercase">
                      {token.symbol}
                    </span>
                  </div>
                  <div className="flex items-center gap-4 mt-2">
                    <span className="text-3xl md:text-4xl font-bold text-white">
                      ${formatPrice(token.current_price)}
                    </span>
                    <div className={`flex items-center gap-1.5 px-3 py-1.5 rounded-full ${
                      token.price_change_percentage_24h >= 0 
                        ? 'bg-green-500/20 text-green-400' 
                        : 'bg-red-500/20 text-red-400'
                    }`}>
                      {token.price_change_percentage_24h >= 0 ? (
                        <FiTrendingUp className="w-4 h-4" />
                      ) : (
                        <FiTrendingDown className="w-4 h-4" />
                      )}
                      <span className="font-semibold">
                        {Math.abs(token.price_change_percentage_24h).toFixed(2)}%
                      </span>
                    </div>
                  </div>
                </div>
              </div>

              <div className="flex items-center gap-2">
                <motion.button
                  whileHover={{ scale: 1.1 }}
                  whileTap={{ scale: 0.9 }}
                  onClick={() => onToggleFavorite(token)}
                  className={`p-3 rounded-xl transition-all ${
                    token.is_favorite 
                      ? 'bg-yellow-500/20 text-yellow-400' 
                      : 'bg-dark-700/50 text-dark-400 hover:text-yellow-400'
                  }`}
                >
                  <FiStar className={`w-5 h-5 ${token.is_favorite ? 'fill-current' : ''}`} />
                </motion.button>

                <motion.button
                  whileHover={{ scale: 1.1 }}
                  whileTap={{ scale: 0.9 }}
                  onClick={() => setIsFullscreen(!isFullscreen)}
                  className="p-3 rounded-xl bg-dark-700/50 text-dark-400 hover:text-white transition-all"
                >
                  <FiMaximize2 className="w-5 h-5" />
                </motion.button>

                <motion.button
                  whileHover={{ scale: 1.1 }}
                  whileTap={{ scale: 0.9 }}
                  onClick={onClose}
                  className="p-3 rounded-xl bg-dark-700/50 text-dark-400 hover:text-white transition-all"
                >
                  <FiX className="w-5 h-5" />
                </motion.button>
              </div>
            </div>
          </div>

          {/* Content */}
          <div className="flex-1 overflow-y-auto p-6">
            {/* Stats Grid */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
              <StatCard
                icon={<FiDollarSign className="w-4 h-4" />}
                label="Market Cap"
                value={formatNumber(token.market_cap)}
              />
              <StatCard
                icon={<FiBarChart2 className="w-4 h-4" />}
                label="24h Volume"
                value={formatNumber(token.volume_24h)}
              />
              <StatCard
                icon={<FiTrendingUp className="w-4 h-4" />}
                label="24h High"
                value={token.high_24h ? `$${formatPrice(token.high_24h)}` : 'N/A'}
              />
              <StatCard
                icon={<FiTrendingDown className="w-4 h-4" />}
                label="24h Low"
                value={token.low_24h ? `$${formatPrice(token.low_24h)}` : 'N/A'}
              />
            </div>

            {/* TradingView Chart */}
            <div className="glass rounded-2xl overflow-hidden mb-6" style={{ height: isFullscreen ? '60vh' : '450px' }}>
              <TradingViewChart symbol={tradingViewSymbol} tokenName={token.name} />
            </div>

            {/* Additional Info */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <StatCard
                icon={<FiActivity className="w-4 h-4" />}
                label="All-Time High"
                value={token.ath ? `$${formatPrice(token.ath)}` : 'N/A'}
                subValue={token.ath_change_percentage ? `${token.ath_change_percentage.toFixed(2)}% from ATH` : undefined}
                trend={token.ath_change_percentage && token.ath_change_percentage > 0 ? 'up' : 'down'}
              />
              <StatCard
                icon={<FiGlobe className="w-4 h-4" />}
                label="All-Time Low"
                value={token.atl ? `$${formatPrice(token.atl)}` : 'N/A'}
                subValue={token.atl_change_percentage ? `${token.atl_change_percentage.toFixed(2)}% from ATL` : undefined}
                trend={token.atl_change_percentage && token.atl_change_percentage > 0 ? 'up' : 'down'}
              />
              <StatCard
                icon={<FiPieChart className="w-4 h-4" />}
                label="Circulating Supply"
                value={formatSupply(token.circulating_supply)}
              />
              <StatCard
                icon={<FiClock className="w-4 h-4" />}
                label="Total Supply"
                value={formatSupply(token.total_supply)}
              />
            </div>

            {/* Trading Info Banner */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.3 }}
              className="mt-6 p-4 rounded-2xl bg-gradient-to-r from-primary-500/10 via-blue-500/10 to-purple-500/10 border border-primary-500/20"
            >
              <div className="flex items-center justify-between flex-wrap gap-4">
                <div className="flex items-center gap-3">
                  <div className="p-2 rounded-lg bg-primary-500/20">
                    <FiActivity className="w-5 h-5 text-primary-400" />
                  </div>
                  <div>
                    <p className="text-white font-medium">Live TradingView Chart</p>
                    <p className="text-dark-400 text-sm">Real-time data from {tradingViewSymbol}</p>
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  <span className="flex items-center gap-1.5 px-3 py-1.5 rounded-full bg-green-500/20 text-green-400 text-sm">
                    <span className="w-2 h-2 rounded-full bg-green-400 animate-pulse" />
                    Live
                  </span>
                </div>
              </div>
            </motion.div>
          </div>
        </div>
      </motion.div>
    </motion.div>
  );
};

export default TokenDetail;
