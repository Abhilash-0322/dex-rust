export interface CryptoToken {
  _id?: string;
  token_id: string;
  symbol: string;
  name: string;
  current_price: number;
  market_cap: number;
  volume_24h: number;
  price_change_24h: number;
  price_change_percentage_24h: number;
  high_24h?: number;
  low_24h?: number;
  circulating_supply?: number;
  total_supply?: number;
  ath?: number;
  ath_change_percentage?: number;
  atl?: number;
  atl_change_percentage?: number;
  image?: string;
  last_updated: string;
  is_favorite: boolean;
}

export interface TokenStats {
  total_tokens: number;
  total_market_cap: number;
  total_volume_24h: number;
  avg_price_change_24h: number;
  biggest_gainer?: CryptoToken;
  biggest_loser?: CryptoToken;
}

export interface HistoricalData {
  prices: number[][];
  market_caps: number[][];
  total_volumes: number[][];
}

export interface FavoriteRequest {
  token_id: string;
  is_favorite: boolean;
}
