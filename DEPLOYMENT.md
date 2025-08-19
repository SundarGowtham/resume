# GitHub Pages Deployment Guide

## ✅ **Your Rust + WASM App is GitHub Pages Ready!**

Your application has been configured for automatic deployment to GitHub Pages using GitHub Actions.

## 🚀 **Deployment Setup**

### **1. Repository Configuration**
1. Go to your GitHub repository settings
2. Navigate to **Pages** section
3. Set **Source** to "GitHub Actions" (not "Deploy from a branch")

### **2. Automatic Deployment**
The `.github/workflows/deploy.yml` workflow will:
- ✅ Trigger on every push to `main` branch
- ✅ Install Rust and Trunk
- ✅ Build your WASM application
- ✅ Deploy to GitHub Pages automatically

### **3. Access Your Site**
After deployment, your site will be available at:
```
https://sundargowtham.github.io/resume/
```

## 🔧 **Manual Deployment (Alternative)**

If you prefer manual deployment:

```bash
# Build for production
trunk build --release

# Copy dist/ contents to your GitHub Pages repository
# OR commit and push the dist/ folder contents to the root of your repo
```

## 📁 **What Gets Deployed**

The `dist/` folder contains:
- `index.html` - Entry point
- `*.wasm` - Your Rust code compiled to WebAssembly (optimized)
- `*.js` - JavaScript bindings
- `styles-*.css` - Minified CSS
- `Gowtham-Sundar-Resume.pdf` - Your resume file

## ⚡ **Performance Features**

✅ **Optimized Build**: Release mode with smaller WASM file (~635KB)  
✅ **Asset Caching**: Hashed filenames for browser caching  
✅ **Fast Loading**: WebAssembly loads near-instantly  
✅ **Progressive Enhancement**: Works even if JS is disabled  

## 🛠 **Development vs Production**

### Development (`trunk serve`)
- Larger WASM files (~2.9MB) with debug info
- Hot reload on file changes
- Source maps for debugging
- Served at `http://127.0.0.1:8080`

### Production (`trunk build --release`)
- Optimized WASM files (~635KB)
- Minified assets
- Proper public URLs for GitHub Pages
- Ready for deployment

## 🔄 **Workflow**

1. **Develop**: `trunk serve` for local development
2. **Commit**: Push changes to GitHub
3. **Deploy**: GitHub Actions automatically builds and deploys
4. **Access**: Your site updates at `https://sundargowtham.github.io/resume/`

## 🎯 **Next Steps**

1. **Push to GitHub**: Commit all these files
2. **Enable GitHub Pages**: Set source to "GitHub Actions"
3. **Wait for deployment**: Check the Actions tab for build status
4. **Visit your site**: Your modern Rust + WASM blog is live!

---

**Happy coding!** 🦀✨

Your simple HTML resume has been transformed into a powerful, modern web application perfect for showcasing complex 3D simulations and technical blog content!
