# 🪙 CryptoTracker Pro

<div align="center">

**Professional Cryptocurrency Price Tracker with Real-Time TradingView Charts**

![Rust](https://img.shields.io/badge/Rust-1.75-orange?logo=rust)
![React](https://img.shields.io/badge/React-18-blue?logo=react)
![TypeScript](https://img.shields.io/badge/TypeScript-5.3-blue?logo=typescript)
![MongoDB](https://img.shields.io/badge/MongoDB-7.0-green?logo=mongodb)
![License](https://img.shields.io/badge/license-MIT-green)

</div>

---

## ✨ Features

- 🔴 **Real-Time Data**: Live cryptocurrency prices from CoinGecko API
- 📊 **TradingView Charts**: Professional-grade interactive charts with technical indicators
- ⭐ **Favorites System**: Save and track your favorite cryptocurrencies
- 🔍 **Smart Search**: Fast search across 100+ cryptocurrencies
- 📈 **Market Statistics**: Total market cap, 24h volume, top gainers/losers
- 🎨 **Beautiful UI**: Modern glass-morphism design with smooth animations
- ⚡ **High Performance**: Rust backend with intelligent caching
- 📱 **Responsive**: Works perfectly on desktop, tablet, and mobile
- 🌙 **Dark Theme**: Easy on the eyes with professional dark mode
- 🔒 **Rate Limit Protection**: Smart API call management to avoid rate limits

---

## 🎯 Tech Stack

### Backend
- **Rust** (Actix-web) - High-performance async web framework
- **MongoDB** - NoSQL database for caching
- **CoinGecko API** - Real-time cryptocurrency data

### Frontend
- **React 18** - Modern UI library
- **TypeScript** - Type-safe JavaScript
- **Vite** - Lightning-fast build tool
- **Tailwind CSS** - Utility-first styling
- **Framer Motion** - Smooth animations
- **TradingView Widget** - Professional trading charts
- **Axios** - HTTP client

---

## 🚀 Quick Deployment

### Option 1: Automated Script (Recommended)
```bash
./deploy.sh
```

### Option 2: Manual Deploy (5 minutes)
See [QUICKSTART_DEPLOY.md](./QUICKSTART_DEPLOY.md)

### Platforms Supported
- ✅ **Railway** (Backend) - $5/month credit
- ✅ **Vercel** (Frontend) - 100% FREE
- ✅ **Fly.io** (Backend) - 3 FREE VMs
- ✅ **MongoDB Atlas** - FREE 512MB
- ✅ **Render** - FREE tier
- ✅ **Docker** - Self-hosted

**Detailed Guide**: [DEPLOYMENT.md](./DEPLOYMENT.md)

---

## 💻 Development Setup

### Prerequisites
- Rust 1.75+
- Node.js 20+
- MongoDB 7+
- Git

### Backend Setup
```bash
cd backend

# Install dependencies (handled by cargo)
cargo build

# Setup environment
cp .env.example .env
# Edit .env with your MongoDB URI

# Run development server
cargo run

# Server runs at http://localhost:8080
```

### Frontend Setup
```bash
cd frontend

# Install dependencies
npm install

# Run development server
npm run dev

# App runs at http://localhost:3000
```

### Database Setup

**Option 1: Local MongoDB**
```bash
# Install MongoDB
# Ubuntu/Debian
sudo apt-get install mongodb

# macOS
brew install mongodb-community

# Start MongoDB
sudo systemctl start mongodb  # Linux
brew services start mongodb-community  # macOS
```

**Option 2: MongoDB Atlas (Cloud)**
1. Go to [mongodb.com/cloud/atlas](https://mongodb.com/cloud/atlas)
2. Create free cluster (M0)
3. Get connection string
4. Update `.env` file

**Option 3: Docker**
```bash
docker run -d -p 27017:27017 --name mongodb mongo:7.0
```

---

## 📁 Project Structure

```
crypto-tracker/
├── backend/
│   ├── src/
│   │   ├── main.rs           # Entry point & server config
│   │   ├── models.rs         # Data structures
│   │   ├── handlers.rs       # API endpoints
│   │   ├── db.rs             # Database operations
│   │   └── crypto_service.rs # CoinGecko API integration
│   ├── Cargo.toml            # Rust dependencies
│   ├── Dockerfile            # Backend container
│   └── .env                  # Environment variables
│
├── frontend/
│   ├── src/
│   │   ├── components/       # React components
│   │   │   ├── Header.tsx
│   │   │   ├── TokenList.tsx
│   │   │   ├── TokenDetail.tsx  # TradingView charts
│   │   │   ├── SearchBar.tsx
│   │   │   ├── StatsOverview.tsx
│   │   │   └── LoadingSpinner.tsx
│   │   ├── App.tsx           # Main component
│   │   ├── api.ts            # API client
│   │   ├── types.ts          # TypeScript types
│   │   └── index.css         # Global styles
│   ├── package.json          # Node dependencies
│   ├── Dockerfile            # Frontend container
│   ├── tailwind.config.js    # Tailwind configuration
│   └── vite.config.ts        # Vite configuration
│
├── DEPLOYMENT.md             # Detailed deployment guide
├── QUICKSTART_DEPLOY.md      # 5-minute deploy guide
├── deploy.sh                 # Automated deployment script
└── README.md                 # This file
```

---

## 🔧 Environment Variables

### Backend `.env`
```env
MONGODB_URI=mongodb://localhost:27017
DATABASE_NAME=crypto_tracker
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
COINGECKO_API_URL=https://api.coingecko.com/api/v3
RUST_LOG=info
```

### Frontend `.env`
```env
VITE_API_URL=http://localhost:8080
```

---

## 📡 API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/tokens` | GET | Get all cryptocurrencies |
| `/api/tokens/{id}` | GET | Get single token details |
| `/api/tokens/favorite` | POST | Toggle favorite status |
| `/api/favorites` | GET | Get favorite tokens |
| `/api/search?q={query}` | GET | Search tokens |
| `/api/history/{id}/{days}` | GET | Get historical data |
| `/api/stats` | GET | Get market statistics |

---

## 🎨 Key Features Explained

### 1. TradingView Integration
- Official TradingView Advanced Chart Widget
- Real-time candlestick charts
- Technical indicators (RSI, Moving Averages)
- Multiple timeframes (1H, 4H, 1D, 1W, 1M)
- Professional-grade analysis tools

### 2. Smart Caching
- MongoDB caching layer
- 2-second minimum interval between API calls
- 60-second backoff on rate limits
- Returns cached data during rate limits
- Background refresh for fresh data

### 3. Professional UI
- Glass-morphism design
- Smooth Framer Motion animations
- Sortable token list
- Real-time price updates
- Responsive grid layout
- Custom scrollbars
- Loading states
- Error handling

### 4. Rate Limit Protection
- In-memory rate limit tracking
- Automatic backoff on 429 errors
- Intelligent request throttling
- Fallback to cached data
- No API key required

---

## 📊 Performance

- **Backend**: ~1-2ms response time (cached)
- **Frontend**: <100ms initial load
- **TradingView**: <500ms chart render
- **API Calls**: Rate-limited to 1 per 2 seconds
- **Cache Hit Rate**: ~95% after warmup

---

## 🔐 Security

- ✅ HTTPS enforced in production
- ✅ CORS configured for specific origins
- ✅ No sensitive data in frontend
- ✅ Environment variables for secrets
- ✅ Input validation & sanitization
- ✅ Rate limiting protection
- ✅ MongoDB connection string security

---

## 🙏 Acknowledgments

- **CoinGecko** - Cryptocurrency data API
- **TradingView** - Professional charting library
- **MongoDB** - Database platform
- **Vercel** - Frontend hosting
- **Railway** - Backend hosting
- **Rust Community** - Amazing ecosystem
- **React Team** - Incredible framework

---

<div align="center">

Made with ❤️ by the CryptoTracker Team

**[⬆ Back to Top](#-cryptotracker-pro)**

</div>
