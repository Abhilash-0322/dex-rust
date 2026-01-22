# 🚀 Quick Deploy - 5 Minutes

## Easiest Way: Railway + Vercel (100% Free)

### Step 1: Deploy Backend (Railway)

```bash
# Install Railway CLI
curl -fsSL https://railway.app/install.sh | sh

# Login
railway login

# Deploy backend
cd backend
railway init
railway up

# Add MongoDB
railway add

# Get your backend URL
railway domain
# Save this URL: https://your-app.railway.app
```

### Step 2: Deploy Frontend (Vercel)

```bash
cd ../frontend

# Update API URL
echo "VITE_API_URL=https://your-app.railway.app" > .env.production

# Deploy
npx vercel --prod

# Follow prompts
```

### Step 3: Update CORS

Update `backend/src/main.rs` line ~35:

```rust
let cors = Cors::default()
    .allowed_origin("https://your-frontend.vercel.app")  // <-- Add this
    .allowed_any_method()
    .allowed_any_header();
```

Redeploy backend:
```bash
cd backend
railway up
```

### Done! 🎉

Your app is live at:
- Frontend: https://your-app.vercel.app
- Backend: https://your-app.railway.app

---

## Alternative: One-Click Heroku Deploy

[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy)

---

## Troubleshooting

**API not connecting?**
- Check VITE_API_URL in Vercel environment variables
- Verify CORS origin in backend

**MongoDB errors?**
- On Railway: `railway add` to add MongoDB
- Get MongoDB Atlas free tier: https://mongodb.com/cloud/atlas

**Build failing?**
- Railway: Check logs with `railway logs`
- Vercel: Check build logs in dashboard

---

## What You Get FREE:

✅ Professional HTTPS URLs  
✅ Automatic deployments on git push  
✅ Database (MongoDB)  
✅ 99.9% uptime  
✅ No credit card required (Vercel)  
✅ $5/month credit (Railway)  

---

## Need Help?

See detailed guide: [DEPLOYMENT.md](./DEPLOYMENT.md)

Or run the automated script:
```bash
./deploy.sh
```
