import React from 'react';
import { motion } from 'framer-motion';
import { TokenStats } from '../types';
import { FiTrendingUp, FiTrendingDown, FiDollarSign, FiActivity, FiPieChart, FiBarChart2 } from 'react-icons/fi';

interface StatsOverviewProps {
  stats: TokenStats;
}

const StatsOverview: React.FC<StatsOverviewProps> = ({ stats }) => {
  const formatNumber = (num: number) => {
    if (num >= 1e12) return `$${(num / 1e12).toFixed(2)}T`;
    if (num >= 1e9) return `$${(num / 1e9).toFixed(2)}B`;
    if (num >= 1e6) return `$${(num / 1e6).toFixed(2)}M`;
    return `$${num.toFixed(2)}`;
  };

  const stats_data = [
    {
      label: 'Total Market Cap',
      value: formatNumber(stats.total_market_cap),
      subValue: `${stats.total_tokens} Cryptocurrencies`,
      icon: FiPieChart,
      color: 'from-blue-500 to-cyan-500',
      bgColor: 'bg-blue-500/10'
    },
    {
      label: '24h Trading Volume',
      value: formatNumber(stats.total_volume_24h),
      subValue: 'Global Volume',
      icon: FiBarChart2,
      color: 'from-purple-500 to-pink-500',
      bgColor: 'bg-purple-500/10'
    },
    {
      label: 'Top Gainer (24h)',
      value: stats.biggest_gainer ? stats.biggest_gainer.symbol.toUpperCase() : 'N/A',
      subValue: stats.biggest_gainer ? `+${stats.biggest_gainer.price_change_percentage_24h.toFixed(2)}%` : '',
      icon: FiTrendingUp,
      color: 'from-green-500 to-emerald-500',
      bgColor: 'bg-green-500/10',
      isGainer: true,
      tokenImage: stats.biggest_gainer?.image
    },
    {
      label: 'Top Loser (24h)',
      value: stats.biggest_loser ? stats.biggest_loser.symbol.toUpperCase() : 'N/A',
      subValue: stats.biggest_loser ? `${stats.biggest_loser.price_change_percentage_24h.toFixed(2)}%` : '',
      icon: FiTrendingDown,
      color: 'from-red-500 to-orange-500',
      bgColor: 'bg-red-500/10',
      isLoser: true,
      tokenImage: stats.biggest_loser?.image
    },
  ];

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      {stats_data.map((stat, index) => (
        <motion.div
          key={stat.label}
          initial={{ opacity: 0, y: 20, scale: 0.95 }}
          animate={{ opacity: 1, y: 0, scale: 1 }}
          transition={{ delay: index * 0.1, type: 'spring', stiffness: 200 }}
          whileHover={{ y: -4, transition: { duration: 0.2 } }}
          className={`relative overflow-hidden rounded-2xl p-5 border border-dark-700/50 ${stat.bgColor} backdrop-blur-sm group cursor-pointer`}
        >
          {/* Animated gradient border */}
          <div className="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity">
            <div className={`absolute inset-0 bg-gradient-to-r ${stat.color} opacity-10`} />
          </div>
          
          {/* Glow effect */}
          <div className={`absolute -top-10 -right-10 w-32 h-32 rounded-full bg-gradient-to-br ${stat.color} opacity-10 blur-3xl group-hover:opacity-20 transition-opacity`} />
          
          <div className="relative flex items-start justify-between">
            <div className="flex-1">
              <p className="text-dark-400 text-xs font-medium uppercase tracking-wider mb-2">{stat.label}</p>
              <div className="flex items-center gap-2 mb-1">
                {stat.tokenImage && (
                  <img src={stat.tokenImage} alt="" className="w-6 h-6 rounded-full" />
                )}
                <h3 className="text-2xl font-bold text-white">
                  {stat.value}
                </h3>
              </div>
              {stat.subValue && (
                <p className={`text-sm font-medium ${
                  stat.isGainer ? 'text-green-400' : 
                  stat.isLoser ? 'text-red-400' : 
                  'text-dark-400'
                }`}>
                  {stat.subValue}
                </p>
              )}
            </div>
            <div className={`w-10 h-10 rounded-xl bg-gradient-to-br ${stat.color} flex items-center justify-center shadow-lg transform group-hover:scale-110 transition-transform`}>
              <stat.icon className="w-5 h-5 text-white" />
            </div>
          </div>
        </motion.div>
      ))}
    </div>
  );
};

export default StatsOverview;
