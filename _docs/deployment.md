# Deployment Guide: Roblox Level Builder MVP

This guide covers deploying the Roblox Level Builder MVP to production.

## Overview

- **Backend**: Rust API server deployed to Railway
- **Frontend**: Static site deployed to Cloudflare Pages
- **Architecture**: Frontend (Cloudflare Pages) → Backend (Railway)

---

## Prerequisites

- GitHub repository with code pushed
- Railway account (free tier available)
- Cloudflare account (free tier available)
- Domain name (optional, for custom domain)

---

## Part 1: Backend Deployment (Railway)

### Step 1: Create Railway Account

1. Go to [railway.app](https://railway.app)
2. Sign up with GitHub
3. Verify email address

### Step 2: Create New Project

1. Click "New Project"
2. Select "Deploy from GitHub repo"
3. Select your repository
4. Railway may not auto-detect Rust in a monorepo - that's OK, we'll configure it manually

### Step 3: Configure Backend Service (Monorepo Setup)

**Important**: Since your repo has both `frontend/` and `backend/` directories, Railway needs explicit configuration.

1. **After creating the project, you'll see a service. Click on it to open settings.**

2. **Set Root Directory**:
   - In the service settings, look for "Root Directory" or "Source" section
   - Set Root Directory to: `backend`
   - This tells Railway to treat `backend/` as the project root
   - Railway will now look for `Cargo.toml` in the `backend/` directory

3. **Configure Build Settings**:
   - Go to the "Settings" tab in your service
   - Look for "Build" or "Deploy" section
   - **Build Command**: `cargo build --release`
   - **Start Command**: `./target/release/backend`
     - Alternative if binary not found: `cargo run --release`
   - Railway should auto-detect these, but verify they're set correctly

4. **Verify Nixpacks Detection** (if visible):
   - Railway uses Nixpacks to detect project type
   - It should detect "Rust" when root directory is set to `backend/`
   - If it shows "No framework detected", manually set the buildpack to Rust

5. **Environment Variables**:
   - Go to "Variables" tab
   - `PORT`: **DO NOT SET THIS** - Railway automatically provides it
   - No other environment variables required for MVP

### Step 4: Deploy

1. **Save Settings**: Make sure to save any changes you made to root directory or build settings

2. **Trigger Deployment**:
   - If you just configured settings, Railway should automatically trigger a new deployment
   - If not, click "Redeploy" or push a new commit to trigger deployment
   - You can also manually trigger from the "Deployments" tab

3. **Monitor Build**:
   - Watch the build logs in the "Deployments" tab
   - You should see: `cargo build --release` running
   - Build should complete successfully (usually 2-5 minutes)

4. **After Build Completes**:
   - Railway will automatically start the service
   - Assign a public URL (e.g., `https://your-app.up.railway.app`)
   - The URL will appear in the service overview or "Settings" → "Networking"

5. **Verify Service is Running**:
   - Check the "Metrics" or "Logs" tab
   - You should see the server startup message: `🚀 Backend server running on...`

### Step 5: Verify Backend Deployment

1. **Test Health Endpoint**:
   ```bash
   curl https://your-app.up.railway.app/health
   ```
   Expected response:
   ```json
   {
     "status": "ok",
     "service": "roblox-level-builder-backend"
   }
   ```

2. **Note Backend URL**:
   - Copy the Railway-provided URL
   - Format: `https://your-app.up.railway.app`
   - You'll need this for frontend configuration

### Step 6: Configure CORS (Optional - Production)

Currently, the backend allows all origins (`Any`). For production, you may want to restrict CORS to your frontend domain:

1. Update `backend/src/main.rs`:
   ```rust
   // Replace:
   .allow_origin(Any)
   
   // With:
   .allow_origin("https://your-frontend.pages.dev".parse().unwrap())
   ```

2. Redeploy backend

---

## Part 2: Frontend Deployment (Cloudflare Pages)

### Step 1: Create Cloudflare Account

1. Go to [cloudflare.com](https://cloudflare.com)
2. Sign up (free tier available)
3. Navigate to "Pages" in dashboard

### Step 2: Create New Project

1. Click "Create a project"
2. Select "Connect to Git"
3. Authorize Cloudflare to access your GitHub repository
4. Select your repository

### Step 3: Configure Build Settings

1. **Project Name**: Choose a name (e.g., `roblox-level-builder`)

2. **Build Settings**:
   - **Framework preset**: None (or Vite if available)
   - **Build command**: `cd frontend && npm install && npm run build`
   - **Build output directory**: `frontend/dist`
   - **Root directory**: Leave empty (or set to `/` if using build command with `cd frontend`)

3. **Environment Variables**:
   - Click "Add environment variable"
   - **Variable name**: `VITE_API_URL`
   - **Value**: `https://your-app.up.railway.app/api/export`
     - Replace `your-app.up.railway.app` with your Railway backend URL
   - **Environment**: Production (and Preview if desired)

### Step 4: Deploy

1. Click "Save and Deploy"
2. Cloudflare will:
   - Install dependencies
   - Build the frontend
   - Deploy to CDN
   - Assign a public URL (e.g., `https://your-project.pages.dev`)

3. Wait for deployment to complete (usually 1-3 minutes)

### Step 5: Verify Frontend Deployment

1. **Open Frontend URL**:
   - Visit `https://your-project.pages.dev`
   - Verify the editor loads

2. **Test Export Flow**:
   - Place some blocks
   - Click Export
   - Verify `.rbxlx` file downloads
   - Open in Roblox Studio and verify correctness

---

## Part 3: Production Testing

### Test Checklist

- [ ] Backend health endpoint responds: `GET /health`
- [ ] Frontend loads correctly
- [ ] Export flow works end-to-end
- [ ] CORS works (no console errors)
- [ ] Test from multiple browsers:
  - [ ] Chrome
  - [ ] Firefox
  - [ ] Safari
  - [ ] Edge
- [ ] Error handling works (test with invalid data)
- [ ] Performance is acceptable (export completes in < 5 seconds)

### Testing Export Flow

1. **Place Blocks**:
   - Place 10-20 blocks of different colors
   - Verify block counter updates

2. **Export**:
   - Click Export button
   - Wait for loading overlay
   - Verify file downloads as `level.rbxlx`

3. **Verify in Roblox Studio**:
   - Open `level.rbxlx` in Roblox Studio
   - Verify blocks are visible in Workspace
   - Verify colors match editor
   - Verify spawn location exists
   - Verify no errors or warnings

### Testing Error Handling

1. **Test Invalid Data** (if Phase 9 error handling is complete):
   - Send invalid Space JSON
   - Verify error message displays in frontend

2. **Test Network Errors**:
   - Temporarily disable backend
   - Attempt export
   - Verify error message displays

---

## Part 4: Custom Domain (Optional)

### Cloudflare Pages Custom Domain

1. In Cloudflare Pages project:
   - Go to "Custom domains"
   - Click "Set up a custom domain"
   - Enter your domain (e.g., `levelbuilder.example.com`)
   - Follow DNS configuration instructions

2. **DNS Configuration**:
   - Add CNAME record pointing to Cloudflare Pages
   - Wait for DNS propagation (usually < 5 minutes)

3. **SSL Certificate**:
   - Cloudflare automatically provisions SSL certificate
   - Wait for certificate activation (usually < 5 minutes)

### Railway Custom Domain

1. In Railway project:
   - Go to "Settings" → "Domains"
   - Click "Add Domain"
   - Enter your domain (e.g., `api.levelbuilder.example.com`)
   - Follow DNS configuration instructions

2. **DNS Configuration**:
   - Add CNAME record pointing to Railway
   - Wait for DNS propagation

3. **Update Frontend**:
   - Update `VITE_API_URL` in Cloudflare Pages environment variables
   - Redeploy frontend

---

## Troubleshooting

### Backend Issues

**Issue**: Build fails on Railway
- **Solution**: Check Railway build logs. Ensure `Cargo.toml` is in correct location. Verify Rust version compatibility.

**Issue**: Health endpoint returns 404
- **Solution**: Verify routes are registered in `main.rs`. Check Railway logs for startup errors.

**Issue**: CORS errors in browser
- **Solution**: Verify CORS configuration allows frontend origin. Check browser console for specific CORS error.

### Frontend Issues

**Issue**: Build fails on Cloudflare Pages
- **Solution**: Check build logs. Verify `package.json` and `vite.config.ts` are correct. Ensure Node.js version is compatible.

**Issue**: Export fails with "Network error"
- **Solution**: Verify `VITE_API_URL` is set correctly in Cloudflare Pages environment variables. Check backend is running and accessible.

**Issue**: Export works but file is invalid
- **Solution**: Check backend logs for errors. Verify Space JSON serialization. Test with smaller block counts.

### General Issues

**Issue**: Changes not reflected after deployment
- **Solution**: Clear browser cache. Verify deployment completed successfully. Check for build errors in deployment logs.

**Issue**: Slow export times
- **Solution**: Check Railway logs for performance issues. Consider optimizing RBXLX generation for large block counts.

---

## Monitoring

### Railway Monitoring

- **Logs**: View real-time logs in Railway dashboard
- **Metrics**: Monitor CPU, memory, and request count
- **Alerts**: Set up alerts for deployment failures

### Cloudflare Pages Monitoring

- **Analytics**: View page views and performance metrics
- **Logs**: View build logs and deployment history
- **Alerts**: Set up alerts for build failures

---

## Rollback Procedures

### Railway Rollback

1. Go to Railway project → "Deployments"
2. Find previous successful deployment
3. Click "Redeploy"

### Cloudflare Pages Rollback

1. Go to Cloudflare Pages project → "Deployments"
2. Find previous successful deployment
3. Click "Retry deployment" or "Redeploy"

---

## Cost Estimation

### Railway (Free Tier)
- **Free tier**: $5 credit/month
- **Usage**: MVP should fit within free tier
- **Scaling**: Pay-as-you-go after free tier

### Cloudflare Pages (Free Tier)
- **Free tier**: Unlimited sites, 500 builds/month
- **Usage**: MVP fits comfortably in free tier
- **Scaling**: Free tier sufficient for MVP

---

## Next Steps

After successful deployment:

1. **Monitor**: Watch for errors in production
2. **Optimize**: Improve performance based on real usage
3. **Scale**: Add more resources if needed
4. **Security**: Consider restricting CORS to frontend domain
5. **Analytics**: Add analytics to track usage (optional)

---

## Support

For issues:
1. Check deployment logs (Railway and Cloudflare Pages)
2. Review browser console for frontend errors
3. Test endpoints with `curl` or Postman
4. Verify environment variables are set correctly

---

**Last Updated**: December 2024

