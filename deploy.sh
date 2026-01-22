#!/bin/bash

# Quick deployment script for Railway

echo "🚀 CryptoTracker Pro - Railway Deployment"
echo "=========================================="

# Check if Railway CLI is installed
if ! command -v railway &> /dev/null; then
    echo "❌ Railway CLI not found. Installing..."
    curl -fsSL https://railway.app/install.sh | sh
fi

# Check if logged in
echo ""
echo "📝 Checking Railway authentication..."
railway whoami &> /dev/null
if [ $? -ne 0 ]; then
    echo "⚠️  Not logged in to Railway. Please login:"
    railway login
fi

echo ""
echo "🔧 Setting up backend deployment..."
cd backend

# Initialize Railway project if not exists
if [ ! -f "railway.json" ]; then
    echo "Creating new Railway project..."
    railway init
fi

# Set environment variables
echo ""
echo "📋 Please provide the following information:"
read -p "MongoDB URI (or press Enter to create new): " MONGODB_URI

if [ -z "$MONGODB_URI" ]; then
    echo "Creating new MongoDB instance on Railway..."
    railway add
fi

# Set other variables
railway variables set SERVER_HOST=0.0.0.0
railway variables set SERVER_PORT=8080
railway variables set COINGECKO_API_URL=https://api.coingecko.com/api/v3
railway variables set RUST_LOG=info

if [ ! -z "$MONGODB_URI" ]; then
    railway variables set MONGODB_URI="$MONGODB_URI"
fi

railway variables set DATABASE_NAME=crypto_tracker

# Deploy backend
echo ""
echo "🚀 Deploying backend..."
railway up

# Get backend URL
BACKEND_URL=$(railway domain)

echo ""
echo "✅ Backend deployed at: $BACKEND_URL"
echo ""

# Deploy frontend to Vercel
echo "🎨 Now let's deploy the frontend to Vercel..."
cd ../frontend

# Update .env.production
echo "VITE_API_URL=$BACKEND_URL" > .env.production

echo ""
echo "Choose frontend deployment option:"
echo "1. Deploy to Vercel (recommended)"
echo "2. Skip frontend deployment"
read -p "Select option (1-2): " FRONTEND_OPTION

if [ "$FRONTEND_OPTION" == "1" ]; then
    if ! command -v vercel &> /dev/null; then
        echo "Installing Vercel CLI..."
        npm i -g vercel
    fi
    
    echo "Deploying to Vercel..."
    vercel --prod
fi

echo ""
echo "🎉 Deployment complete!"
echo ""
echo "📝 Summary:"
echo "  Backend:  $BACKEND_URL"
echo "  Frontend: Check Vercel output above"
echo ""
echo "🔧 Next steps:"
echo "  1. Update CORS in backend/src/main.rs with your frontend URL"
echo "  2. Test your application"
echo "  3. Setup custom domain (optional)"
echo ""
echo "📚 See DEPLOYMENT.md for more details"
