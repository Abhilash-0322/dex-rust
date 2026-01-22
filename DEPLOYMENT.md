# 🚀 Deployment Guide - CryptoTracker Pro

## Overview

This application consists of:
- **Backend**: Rust (Actix-web) API server
- **Frontend**: React (Vite) SPA
- **Database**: MongoDB

## Deployment Options

### Option 1: Separate Deployment (Recommended) ⭐

**Why separate?** Easier to scale, better for free tiers, simpler CI/CD.

#### Frontend → Vercel
- **Cost**: FREE
- **Features**: Automatic deployments, CDN, HTTPS
- **Best for**: Static React apps

#### Backend → Railway
- **Cost**: FREE ($5 credit/month)
- **Features**: Auto-deploy, environment variables, MongoDB hosting
- **Best for**: Backend APIs with database

#### Database → MongoDB Atlas
- **Cost**: FREE (512MB)
- **Features**: Cloud MongoDB, automatic backups
- **Best for**: Small to medium apps

---

### Option 2: All-in-One Deployment

**Serve frontend from backend** - Single server deployment

#### Platforms:
- **Railway**: $5/month credit (free tier)
- **Fly.io**: 3 free VMs
- **Render**: Free tier available

---

## 🔧 Setup Instructions

### 1. Prepare Frontend for Production

```bash
cd frontend
npm run build
# Creates 'dist' folder with static files
```

### 2A. Deploy Separately (Vercel + Railway)

#### Deploy Frontend to Vercel:

```bash
cd frontend
npx vercel deploy --prod
# Follow prompts
# Set environment variable: VITE_API_URL=https://your-backend.railway.app
```

**Or use Vercel Dashboard:**
1. Go to https://vercel.com
2. Import Git repository
3. Set root directory: `frontend`
4. Add environment variable:
   - `VITE_API_URL` = `https://your-backend.railway.app`
5. Deploy!

#### Deploy Backend to Railway:

```bash
# Install Railway CLI
curl -fsSL https://railway.app/install.sh | sh

# Login
railway login

# Create new project
cd backend
railway init

# Add MongoDB
railway add

# Set environment variables (see section below)

# Deploy
railway up
```

**Environment Variables for Backend (Railway):**
```env
MONGODB_URI=mongodb+srv://user:password@cluster.mongodb.net/crypto_tracker
DATABASE_NAME=crypto_tracker
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
COINGECKO_API_URL=https://api.coingecko.com/api/v3
RUST_LOG=info
```

#### Deploy Database to MongoDB Atlas:

1. Go to https://www.mongodb.com/cloud/atlas
2. Create free cluster (M0)
3. Create database user
4. Whitelist IP: `0.0.0.0/0` (allow all)
5. Get connection string
6. Use in Railway backend env vars

---

### 2B. Deploy All-in-One (Railway)

**Serve frontend from Rust backend:**

#### Step 1: Build Frontend
```bash
cd frontend
npm run build
# dist/ folder created
```

#### Step 2: Update Backend to Serve Static Files

Add to `backend/Cargo.toml`:
```toml
[dependencies]
actix-files = "0.6"
```

Update `backend/src/main.rs` (add after routes):
```rust
use actix_files as fs;

// In HttpServer::new
.service(fs::Files::new("/", "../frontend/dist").index_file("index.html"))
```

#### Step 3: Deploy to Railway

```bash
cd backend
railway init
railway add  # Add MongoDB
railway up
```

**Railway will:**
- Build Rust backend
- Serve frontend from `/`
- API accessible at `/api/*`

---

## 📝 Detailed Platform Setup

### Vercel Deployment

1. **Via Git (Recommended):**
   ```bash
   # Push to GitHub
   git add .
   git commit -m "Ready for deployment"
   git push origin main
   
   # Go to vercel.com → New Project → Import Repo
   # Select your repo
   # Root Directory: frontend
   # Framework: Vite
   # Build Command: npm run build
   # Output Directory: dist
   ```

2. **Environment Variables:**
   ```
   VITE_API_URL=https://your-backend-url.railway.app
   ```

3. **Custom Domain (optional):**
   - Add domain in Vercel dashboard
   - Update DNS records

### Railway Deployment

1. **Install CLI:**
   ```bash
   brew install railway  # Mac
   # or
   curl -fsSL https://railway.app/install.sh | sh  # Linux
   ```

2. **Create Project:**
   ```bash
   railway login
   railway init
   ```

3. **Add MongoDB:**
   ```bash
   railway add
   # Select MongoDB
   ```

4. **Set Variables:**
   ```bash
   railway variables set MONGODB_URI="mongodb://..."
   railway variables set SERVER_HOST="0.0.0.0"
   railway variables set SERVER_PORT="8080"
   ```

5. **Deploy:**
   ```bash
   railway up
   ```

6. **Get URL:**
   ```bash
   railway domain
   # Generates: https://your-app.railway.app
   ```

### Fly.io Deployment

1. **Install CLI:**
   ```bash
   curl -L https://fly.io/install.sh | sh
   ```

2. **Login:**
   ```bash
   fly auth login
   ```

3. **Launch App:**
   ```bash
   cd backend
   fly launch
   # Follow prompts
   ```

4. **Set Secrets:**
   ```bash
   fly secrets set MONGODB_URI="mongodb://..."
   fly secrets set SERVER_HOST="0.0.0.0"
   ```

5. **Deploy:**
   ```bash
   fly deploy
   ```

---

## 🔐 Environment Variables

### Backend (.env)
```env
# Production
MONGODB_URI=mongodb+srv://user:pass@cluster.mongodb.net/crypto_tracker
DATABASE_NAME=crypto_tracker
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
COINGECKO_API_URL=https://api.coingecko.com/api/v3
RUST_LOG=info
```

### Frontend (.env.production)
```env
VITE_API_URL=https://your-backend.railway.app
```

---

## 🐳 Docker Deployment (Alternative)

### Build Images:

**Backend Dockerfile** (already created):
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/crypto-tracker-backend /usr/local/bin/
CMD ["crypto-tracker-backend"]
```

**Frontend Dockerfile**:
```dockerfile
FROM node:20-alpine as builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

**Docker Compose**:
```bash
docker-compose up -d
```

Deploy to:
- **AWS ECS/Fargate**
- **Google Cloud Run**
- **DigitalOcean App Platform**
- **Azure Container Instances**

---

## 💰 Cost Comparison

| Platform | Backend | Frontend | Database | Total/Month |
|----------|---------|----------|----------|-------------|
| **Vercel + Railway + Atlas** | $5 credit | FREE | FREE | $0 (with credits) |
| **Railway (All-in-One)** | $5 credit | Included | $5 | $0-5 |
| **Fly.io + Vercel + Atlas** | FREE (3 VMs) | FREE | FREE | $0 |
| **Render (Free)** | FREE* | FREE* | Atlas FREE | $0 |
| **AWS/GCP/Azure** | ~$10-20 | ~$5 | ~$5-15 | $20-40 |

*Render free tier sleeps after 15min inactivity

---

## ✅ Recommended Setup for You

### Best Free Option:
```
Frontend: Vercel (FREE, unlimited)
Backend: Railway ($5 credit/month = FREE for small apps)
Database: MongoDB Atlas (FREE 512MB)
```

**Why this combo?**
- ✅ Completely FREE to start
- ✅ Easy setup (no credit card for Vercel)
- ✅ Automatic HTTPS
- ✅ Auto-deploy on git push
- ✅ Professional URLs
- ✅ Scales easily when you grow

---

## 🚦 Quick Start Commands

### Option 1: Separate (5 minutes)

```bash
# 1. Deploy Frontend to Vercel
cd frontend
npx vercel --prod

# 2. Deploy Backend to Railway
cd ../backend
railway login
railway init
railway up

# 3. Setup MongoDB Atlas
# Go to mongodb.com/cloud/atlas → Create free cluster → Get connection string

# 4. Update Vercel env vars
# Add VITE_API_URL with your Railway URL

# Done! 🎉
```

### Option 2: All-in-One Railway (10 minutes)

```bash
# 1. Build frontend
cd frontend
npm run build

# 2. Update backend to serve static files (see instructions above)

# 3. Deploy to Railway
cd ../backend
railway login
railway init
railway add  # Add MongoDB
railway up

# Done! 🎉
```

---

## 🔍 Testing Production Build

### Test Frontend Locally:
```bash
cd frontend
npm run build
npm run preview  # Serves on http://localhost:4173
```

### Test Backend Locally:
```bash
cd backend
cargo build --release
./target/release/crypto-tracker-backend
```

---

## 📊 Post-Deployment Checklist

- [ ] Backend API responding
- [ ] Frontend loads correctly
- [ ] MongoDB connection working
- [ ] CoinGecko API calls working
- [ ] CORS configured properly
- [ ] Environment variables set
- [ ] HTTPS enabled
- [ ] Custom domain configured (optional)
- [ ] Error monitoring setup (optional - Sentry)
- [ ] Analytics setup (optional - Google Analytics)

---

## 🐛 Common Issues

### CORS Errors:
```rust
// backend/src/main.rs
let cors = Cors::default()
    .allowed_origin("https://your-frontend.vercel.app")
    .allowed_methods(vec!["GET", "POST"])
    .allowed_headers(vec![header::CONTENT_TYPE])
    .max_age(3600);
```

### API URL Not Found:
Create `frontend/.env.production`:
```env
VITE_API_URL=https://your-backend.railway.app
```

### MongoDB Connection Failed:
- Check IP whitelist (0.0.0.0/0 for all IPs)
- Verify connection string
- Check database user permissions

---

## 📞 Support

- **Railway**: https://railway.app/help
- **Vercel**: https://vercel.com/support
- **MongoDB Atlas**: https://www.mongodb.com/docs/atlas/

---

## 🎯 Next Steps After Deployment

1. **Setup Monitoring**: Add error tracking (Sentry, LogRocket)
2. **Add Analytics**: Google Analytics, Plausible
3. **Custom Domain**: Add your own domain
4. **CI/CD**: Setup automatic deployments
5. **Performance**: Enable caching, CDN optimization
6. **SEO**: Add meta tags, sitemap
7. **PWA**: Make it installable

---

**Need help?** The setup I recommend for beginners:
1. Push code to GitHub
2. Connect Vercel (frontend)
3. Connect Railway (backend)
4. Setup MongoDB Atlas
5. Configure environment variables
6. Done! ✨
