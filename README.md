# Gowtham Sundar - Personal Website & Blog

A modern personal website built with Rust, WebAssembly, and Yew framework. Features interactive 3D simulations, technical blog posts, and professional resume display.

## 🚀 Features

- **Modern Rust + WASM Architecture**: Built with Yew framework for high performance
- **Interactive 3D Simulations**: WebGL-powered mathematical and physical simulations
- **Technical Blog**: Markdown-powered blog for sharing insights and notes
- **Professional Resume Display**: Clean resume presentation with PDF download
- **Responsive Design**: Mobile-first responsive design
- **Fast Loading**: WebAssembly for near-native performance

## 🛠 Tech Stack

- **Frontend**: Rust + Yew (React-like framework for Rust)
- **Build Tool**: Trunk (modern build tool for Rust web apps)
- **Styling**: CSS3 with modern features
- **Graphics**: WebGL2 for 3D simulations
- **Deployment**: GitHub Pages

## 📋 Prerequisites

Before you begin, ensure you have the following installed:

1. **Rust** (latest stable version)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **wasm-pack** (for building WebAssembly)
   ```bash
   cargo install wasm-pack
   ```

3. **Trunk** (build tool)
   ```bash
   cargo install trunk
   ```

4. **WebAssembly target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

## 🏗 Installation & Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/SundarGowtham/resume.git
   cd resume
   ```

2. **Install dependencies**
   ```bash
   cargo check
   ```

3. **Run development server**
   ```bash
   trunk serve
   ```

4. **Open your browser**
   Navigate to `http://127.0.0.1:8080`

## 🔧 Development

### Project Structure

```
├── src/
│   ├── lib.rs              # Main application entry point
│   ├── components/         # Reusable UI components
│   │   ├── mod.rs
│   │   └── navbar.rs       # Navigation component
│   └── pages/              # Page components
│       ├── mod.rs
│       ├── home.rs         # Landing page
│       ├── blog.rs         # Blog listing and posts
│       ├── simulations.rs  # 3D simulations showcase
│       └── resume.rs       # Resume display
├── styles.css              # Global styles
├── index.html              # HTML template
├── Trunk.toml              # Build configuration
├── Cargo.toml              # Rust dependencies
└── Gowtham-Sundar-Resume.pdf
```

### Adding New Blog Posts

1. Edit `src/pages/blog.rs`
2. Add new `BlogPost` entries to the vector
3. The app will automatically display them

### Adding New Simulations

1. Edit `src/pages/simulations.rs`
2. Create new WebGL contexts and rendering logic
3. Add simulation cards to the grid

### Customizing Styles

- Edit `styles.css` for global styles
- Component-specific styles can be added inline or via CSS classes

## 🚀 Building for Production

1. **Build the project**
   ```bash
   trunk build --release
   ```

2. **Deploy to GitHub Pages**
   - The built files will be in the `dist/` directory
   - Copy contents to your GitHub Pages repository
   - Ensure `Gowtham-Sundar-Resume.pdf` is in the root directory

## 🎨 Customization

### Personal Information
- Update contact details in `src/pages/resume.rs`
- Modify the hero section in `src/pages/home.rs`
- Change the navbar brand name in `src/components/navbar.rs`

### Colors and Theming
- Primary colors are defined in `styles.css`
- Gradient backgrounds can be customized in the CSS variables

### Adding New Pages
1. Create a new component in `src/pages/`
2. Add the route to `Route` enum in `src/lib.rs`
3. Update the navbar in `src/components/navbar.rs`

## 📝 Blog Content Management

Currently, blog posts are hardcoded in the Rust code. For a more dynamic approach, consider:

- Loading markdown files at runtime
- Using a headless CMS
- Implementing a static site generation approach

## 🔬 3D Simulations

The simulations use WebGL2 for high-performance graphics. Examples include:

- Fluid dynamics simulation
- Wave propagation visualization
- Neural network training visualization
- Fractal explorers

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

## 🙏 Acknowledgments

- Built with [Yew](https://yew.rs/) - A modern Rust framework for creating multi-threaded front-end web apps
- Styled with modern CSS3 features
- Deployed on GitHub Pages

---

**Happy coding!** 🦀✨
