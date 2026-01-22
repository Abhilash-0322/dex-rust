import axios from 'axios';
import { CryptoToken, TokenStats, HistoricalData, FavoriteRequest } from './types';

const API_BASE_URL = '/api';

export const cryptoApi = {
  async getTokens(): Promise<CryptoToken[]> {
    const response = await axios.get(`${API_BASE_URL}/tokens`);
    return response.data;
  },

  async getToken(id: string): Promise<CryptoToken> {
    const response = await axios.get(`${API_BASE_URL}/tokens/${id}`);
    return response.data;
  },

  async toggleFavorite(request: FavoriteRequest): Promise<CryptoToken> {
    const response = await axios.post(`${API_BASE_URL}/tokens/favorite`, request);
    return response.data;
  },

  async getFavorites(): Promise<CryptoToken[]> {
    const response = await axios.get(`${API_BASE_URL}/favorites`);
    return response.data;
  },

  async searchTokens(query: string): Promise<CryptoToken[]> {
    const response = await axios.get(`${API_BASE_URL}/search`, {
      params: { q: query }
    });
    return response.data;
  },

  async getHistoricalData(tokenId: string, days: number): Promise<HistoricalData> {
    const response = await axios.get(`${API_BASE_URL}/history/${tokenId}/${days}`);
    return response.data;
  },

  async getStats(): Promise<TokenStats> {
    const response = await axios.get(`${API_BASE_URL}/stats`);
    return response.data;
  },
};
