import React from 'react';
import { motion } from 'framer-motion';
import { FiActivity, FiGlobe, FiZap } from 'react-icons/fi';

const Header: React.FC = () => {
  return (
    <header className="relative border-b border-dark-700/50 backdrop-blur-xl bg-dark-900/80 sticky top-0 z-40">
      {/* Animated background gradient */}
      <div className="absolute inset-0 bg-gradient-to-r from-primary-500/5 via-purple-500/5 to-blue-500/5 animate-gradient" />
      
      <div className="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
        <div className="flex items-center justify-between">
          <motion.div
            initial={{ opacity: 0, x: -20 }}
            animate={{ opacity: 1, x: 0 }}
            className="flex items-center gap-4"
          >
            {/* Logo */}
            <div className="relative">
              <motion.div
                animate={{ 
                  boxShadow: [
                    '0 0 20px rgba(14, 165, 233, 0.3)',
                    '0 0 40px rgba(14, 165, 233, 0.5)',
                    '0 0 20px rgba(14, 165, 233, 0.3)'
                  ]
                }}
                transition={{ duration: 2, repeat: Infinity }}
                className="w-12 h-12 rounded-xl bg-gradient-to-br from-primary-500 via-blue-500 to-purple-600 flex items-center justify-center"
              >
                <FiActivity className="w-6 h-6 text-white" />
              </motion.div>
              <motion.div
                animate={{ scale: [1, 1.2, 1] }}
                transition={{ duration: 2, repeat: Infinity }}
                className="absolute -top-1 -right-1 w-3 h-3 bg-green-500 rounded-full border-2 border-dark-900"
              />
            </div>
            
            {/* Title */}
            <div>
              <h1 className="text-2xl md:text-3xl font-bold bg-gradient-to-r from-white via-primary-200 to-primary-400 bg-clip-text text-transparent">
                CryptoTracker Pro
              </h1>
              <div className="flex items-center gap-2 text-dark-400 text-sm">
                <FiGlobe className="w-3.5 h-3.5" />
                <span>Real-time market data powered by TradingView</span>
              </div>
            </div>
          </motion.div>
          
          {/* Status Indicators */}
          <motion.div
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            className="flex items-center gap-4"
          >
            {/* Live indicator */}
            <div className="flex items-center gap-2 px-4 py-2 rounded-xl bg-dark-800/50 border border-dark-700/50">
              <span className="relative flex h-2.5 w-2.5">
                <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span>
                <span className="relative inline-flex rounded-full h-2.5 w-2.5 bg-green-500"></span>
              </span>
              <span className="text-green-400 text-sm font-medium">Live</span>
            </div>
            
            {/* TradingView badge */}
            <div className="hidden md:flex items-center gap-2 px-4 py-2 rounded-xl bg-gradient-to-r from-blue-500/10 to-purple-500/10 border border-blue-500/20">
              <FiZap className="w-4 h-4 text-blue-400" />
              <span className="text-blue-400 text-sm font-medium">TradingView Charts</span>
            </div>
          </motion.div>
        </div>
      </div>
    </header>
  );
};

export default Header;
