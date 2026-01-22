import React, { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { FiSearch, FiStar, FiGrid, FiX, FiFilter } from 'react-icons/fi';

interface SearchBarProps {
  value: string;
  onChange: (value: string) => void;
  viewMode: 'all' | 'favorites';
  onViewModeChange: (mode: 'all' | 'favorites') => void;
}

const SearchBar: React.FC<SearchBarProps> = ({ value, onChange, viewMode, onViewModeChange }) => {
  const [isFocused, setIsFocused] = useState(false);

  return (
    <div className="flex flex-col lg:flex-row gap-4 items-stretch lg:items-center">
      {/* Search Input */}
      <div className="flex-1 relative">
        <motion.div
          animate={{
            boxShadow: isFocused 
              ? '0 0 0 2px rgba(14, 165, 233, 0.3), 0 4px 20px rgba(14, 165, 233, 0.1)' 
              : '0 0 0 1px rgba(255, 255, 255, 0.05)'
          }}
          className="relative rounded-xl overflow-hidden"
        >
          <FiSearch className={`absolute left-4 top-1/2 transform -translate-y-1/2 w-5 h-5 transition-colors ${
            isFocused ? 'text-primary-400' : 'text-dark-400'
          }`} />
          <input
            type="text"
            value={value}
            onChange={(e) => onChange(e.target.value)}
            onFocus={() => setIsFocused(true)}
            onBlur={() => setIsFocused(false)}
            placeholder="Search by name or symbol..."
            className="w-full pl-12 pr-12 py-3.5 bg-dark-800/50 border border-dark-700/50 rounded-xl text-white placeholder-dark-500 focus:outline-none transition-all"
          />
          <AnimatePresence>
            {value && (
              <motion.button
                initial={{ opacity: 0, scale: 0.8 }}
                animate={{ opacity: 1, scale: 1 }}
                exit={{ opacity: 0, scale: 0.8 }}
                onClick={() => onChange('')}
                className="absolute right-4 top-1/2 transform -translate-y-1/2 p-1 rounded-md hover:bg-dark-700 text-dark-400 hover:text-white transition-colors"
              >
                <FiX className="w-4 h-4" />
              </motion.button>
            )}
          </AnimatePresence>
        </motion.div>
      </div>
      
      {/* View Mode Toggle */}
      <div className="flex gap-2 bg-dark-800/50 p-1.5 rounded-xl border border-dark-700/50">
        <motion.button
          whileHover={{ scale: 1.02 }}
          whileTap={{ scale: 0.98 }}
          onClick={() => onViewModeChange('all')}
          className={`relative px-5 py-2.5 rounded-lg flex items-center gap-2 font-medium text-sm transition-all ${
            viewMode === 'all'
              ? 'text-white'
              : 'text-dark-400 hover:text-white'
          }`}
        >
          {viewMode === 'all' && (
            <motion.div
              layoutId="activeTab"
              className="absolute inset-0 bg-gradient-to-r from-primary-500 to-blue-600 rounded-lg shadow-lg shadow-primary-500/30"
            />
          )}
          <span className="relative flex items-center gap-2">
            <FiGrid className="w-4 h-4" />
            All Tokens
          </span>
        </motion.button>
        
        <motion.button
          whileHover={{ scale: 1.02 }}
          whileTap={{ scale: 0.98 }}
          onClick={() => onViewModeChange('favorites')}
          className={`relative px-5 py-2.5 rounded-lg flex items-center gap-2 font-medium text-sm transition-all ${
            viewMode === 'favorites'
              ? 'text-white'
              : 'text-dark-400 hover:text-white'
          }`}
        >
          {viewMode === 'favorites' && (
            <motion.div
              layoutId="activeTab"
              className="absolute inset-0 bg-gradient-to-r from-yellow-500 to-orange-500 rounded-lg shadow-lg shadow-yellow-500/30"
            />
          )}
          <span className="relative flex items-center gap-2">
            <FiStar className={`w-4 h-4 ${viewMode === 'favorites' ? 'fill-current' : ''}`} />
            Favorites
          </span>
        </motion.button>
      </div>
    </div>
  );
};

export default SearchBar;
