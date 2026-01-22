import React, { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { CryptoToken } from '../types';
import { 
  FiStar, 
  FiTrendingUp, 
  FiTrendingDown, 
  FiChevronUp, 
  FiChevronDown,
  FiExternalLink,
  FiBarChart2 
} from 'react-icons/fi';

interface TokenListProps {
  tokens: CryptoToken[];
  onSelectToken: (token: CryptoToken) => void;
  onToggleFavorite: (token: CryptoToken) => void;
}

type SortKey = 'rank' | 'name' | 'price' | 'change' | 'marketCap' | 'volume';
type SortDirection = 'asc' | 'desc';

const TokenList: React.FC<TokenListProps> = ({ tokens, onSelectToken, onToggleFavorite }) => {
  const [sortKey, setSortKey] = useState<SortKey>('rank');
  const [sortDirection, setSortDirection] = useState<SortDirection>('asc');
  const [hoveredToken, setHoveredToken] = useState<string | null>(null);

  const formatNumber = (num: number): string => {
    if (num >= 1e12) return `$${(num / 1e12).toFixed(2)}T`;
    if (num >= 1e9) return `$${(num / 1e9).toFixed(2)}B`;
    if (num >= 1e6) return `$${(num / 1e6).toFixed(2)}M`;
    if (num >= 1e3) return `$${(num / 1e3).toFixed(2)}K`;
    return `$${num.toFixed(2)}`;
  };

  const formatPrice = (price: number): string => {
    if (price >= 1000) {
      return `$${price.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
    } else if (price >= 1) {
      return `$${price.toFixed(4)}`;
    } else if (price >= 0.0001) {
      return `$${price.toFixed(6)}`;
    } else {
      return `$${price.toFixed(10)}`;
    }
  };

  const handleSort = (key: SortKey) => {
    if (sortKey === key) {
      setSortDirection(sortDirection === 'asc' ? 'desc' : 'asc');
    } else {
      setSortKey(key);
      setSortDirection('desc');
    }
  };

  const sortedTokens = [...tokens].sort((a, b) => {
    let comparison = 0;
    switch (sortKey) {
      case 'rank':
        comparison = tokens.indexOf(a) - tokens.indexOf(b);
        break;
      case 'name':
        comparison = a.name.localeCompare(b.name);
        break;
      case 'price':
        comparison = a.current_price - b.current_price;
        break;
      case 'change':
        comparison = a.price_change_percentage_24h - b.price_change_percentage_24h;
        break;
      case 'marketCap':
        comparison = a.market_cap - b.market_cap;
        break;
      case 'volume':
        comparison = a.volume_24h - b.volume_24h;
        break;
    }
    return sortDirection === 'asc' ? comparison : -comparison;
  });

  const SortIcon = ({ columnKey }: { columnKey: SortKey }) => {
    if (sortKey !== columnKey) return null;
    return sortDirection === 'asc' ? (
      <FiChevronUp className="w-4 h-4 text-primary-400" />
    ) : (
      <FiChevronDown className="w-4 h-4 text-primary-400" />
    );
  };

  const HeaderCell: React.FC<{ label: string; sortKey?: SortKey; className?: string; align?: 'left' | 'right' }> = ({
    label,
    sortKey: key,
    className = '',
    align = 'left'
  }) => (
    <th
      className={`px-4 py-4 text-xs font-semibold uppercase tracking-wider cursor-pointer select-none transition-colors hover:text-white ${
        align === 'right' ? 'text-right' : 'text-left'
      } ${className}`}
      onClick={() => key && handleSort(key)}
    >
      <div className={`flex items-center gap-1 ${align === 'right' ? 'justify-end' : ''}`}>
        <span>{label}</span>
        {key && <SortIcon columnKey={key} />}
      </div>
    </th>
  );

  if (tokens.length === 0) {
    return (
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="glass rounded-3xl p-16 text-center"
      >
        <div className="w-16 h-16 mx-auto mb-4 rounded-full bg-dark-700 flex items-center justify-center">
          <FiBarChart2 className="w-8 h-8 text-dark-400" />
        </div>
        <p className="text-dark-400 text-lg mb-2">No tokens found</p>
        <p className="text-dark-500 text-sm">Try adjusting your search or filters</p>
      </motion.div>
    );
  }

  return (
    <div className="bg-gradient-to-br from-dark-800/80 via-dark-900/80 to-dark-950/80 backdrop-blur-xl rounded-3xl border border-dark-700/50 shadow-2xl overflow-hidden">
      {/* Table */}
      <div className="overflow-x-auto">
        <table className="w-full">
          <thead className="bg-dark-800/50 border-b border-dark-700/50">
            <tr className="text-dark-400">
              <HeaderCell label="#" sortKey="rank" className="w-16" />
              <HeaderCell label="Token" sortKey="name" />
              <HeaderCell label="Price" sortKey="price" align="right" />
              <HeaderCell label="24h %" sortKey="change" align="right" />
              <HeaderCell label="Market Cap" sortKey="marketCap" align="right" />
              <HeaderCell label="Volume (24h)" sortKey="volume" align="right" />
              <th className="px-4 py-4 w-20"></th>
            </tr>
          </thead>
          <tbody className="divide-y divide-dark-700/30">
            <AnimatePresence>
              {sortedTokens.slice(0, 100).map((token, index) => (
                <motion.tr
                  key={token.token_id}
                  initial={{ opacity: 0, x: -10 }}
                  animate={{ opacity: 1, x: 0 }}
                  exit={{ opacity: 0, x: 10 }}
                  transition={{ delay: Math.min(index * 0.01, 0.5) }}
                  onMouseEnter={() => setHoveredToken(token.token_id)}
                  onMouseLeave={() => setHoveredToken(null)}
                  onClick={() => onSelectToken(token)}
                  className={`cursor-pointer transition-all duration-200 ${
                    hoveredToken === token.token_id
                      ? 'bg-gradient-to-r from-primary-500/10 via-dark-700/50 to-transparent'
                      : 'hover:bg-dark-700/30'
                  }`}
                >
                  {/* Rank */}
                  <td className="px-4 py-4">
                    <span className="text-dark-400 font-medium text-sm">
                      {tokens.indexOf(token) + 1}
                    </span>
                  </td>

                  {/* Token Info */}
                  <td className="px-4 py-4">
                    <div className="flex items-center gap-3">
                      <div className="relative">
                        {token.image ? (
                          <img
                            src={token.image}
                            alt={token.name}
                            className="w-10 h-10 rounded-full ring-2 ring-dark-600/50"
                          />
                        ) : (
                          <div className="w-10 h-10 rounded-full bg-dark-700 flex items-center justify-center">
                            <span className="text-white font-bold text-sm">
                              {token.symbol.slice(0, 2).toUpperCase()}
                            </span>
                          </div>
                        )}
                        {token.is_favorite && (
                          <div className="absolute -top-1 -right-1 w-4 h-4 bg-yellow-500 rounded-full flex items-center justify-center">
                            <FiStar className="w-2.5 h-2.5 text-dark-900 fill-current" />
                          </div>
                        )}
                      </div>
                      <div>
                        <div className="font-semibold text-white group-hover:text-primary-400 transition-colors flex items-center gap-2">
                          {token.name}
                          {hoveredToken === token.token_id && (
                            <motion.span
                              initial={{ opacity: 0, x: -5 }}
                              animate={{ opacity: 1, x: 0 }}
                            >
                              <FiExternalLink className="w-3.5 h-3.5 text-primary-400" />
                            </motion.span>
                          )}
                        </div>
                        <div className="text-xs text-dark-400 uppercase font-medium">
                          {token.symbol}
                        </div>
                      </div>
                    </div>
                  </td>

                  {/* Price */}
                  <td className="px-4 py-4 text-right">
                    <span className="font-semibold text-white tabular-nums">
                      {formatPrice(token.current_price)}
                    </span>
                  </td>

                  {/* 24h Change */}
                  <td className="px-4 py-4 text-right">
                    <div className={`inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg font-medium text-sm ${
                      token.price_change_percentage_24h >= 0
                        ? 'bg-green-500/15 text-green-400'
                        : 'bg-red-500/15 text-red-400'
                    }`}>
                      {token.price_change_percentage_24h >= 0 ? (
                        <FiTrendingUp className="w-3.5 h-3.5" />
                      ) : (
                        <FiTrendingDown className="w-3.5 h-3.5" />
                      )}
                      <span className="tabular-nums">
                        {Math.abs(token.price_change_percentage_24h).toFixed(2)}%
                      </span>
                    </div>
                  </td>

                  {/* Market Cap */}
                  <td className="px-4 py-4 text-right">
                    <span className="text-dark-300 tabular-nums">
                      {formatNumber(token.market_cap)}
                    </span>
                  </td>

                  {/* Volume */}
                  <td className="px-4 py-4 text-right">
                    <span className="text-dark-300 tabular-nums">
                      {formatNumber(token.volume_24h)}
                    </span>
                  </td>

                  {/* Actions */}
                  <td className="px-4 py-4">
                    <div className="flex items-center justify-end gap-2">
                      <motion.button
                        whileHover={{ scale: 1.2, rotate: 15 }}
                        whileTap={{ scale: 0.9 }}
                        onClick={(e) => {
                          e.stopPropagation();
                          onToggleFavorite(token);
                        }}
                        className={`p-2 rounded-lg transition-colors ${
                          token.is_favorite
                            ? 'bg-yellow-500/20 text-yellow-400'
                            : 'hover:bg-dark-600 text-dark-400 hover:text-yellow-400'
                        }`}
                      >
                        <FiStar className={`w-4 h-4 ${token.is_favorite ? 'fill-current' : ''}`} />
                      </motion.button>
                    </div>
                  </td>
                </motion.tr>
              ))}
            </AnimatePresence>
          </tbody>
        </table>
      </div>

      {/* Footer */}
      {tokens.length > 100 && (
        <div className="px-6 py-4 bg-dark-800/30 border-t border-dark-700/50 text-center">
          <p className="text-dark-400 text-sm">
            Showing 100 of {tokens.length} tokens
          </p>
        </div>
      )}
    </div>
  );
};

export default TokenList;
