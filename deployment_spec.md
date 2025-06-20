# Macroquad Game Deployment to Fly.io

This document provides step-by-step instructions for deploying a Macroquad-based Rust game to Fly.io via WebAssembly.

## Prerequisites

### Required Software
1. **Rust toolchain** with WebAssembly target:
   ```bash
   # Install Rust if not already installed
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Add WebAssembly target
   rustup target add wasm32-unknown-unknown
   ```

2. **Fly.io CLI**:
   ```bash
   # Install flyctl
   curl -L https://fly.io/install.sh | sh
   
   # Login to Fly.io
   flyctl auth login
   ```

3. **Python 3** (for local development server)
4. **Git** (for version control)

### Required Files in Project Root

Your Macroquad project should have:
- `Cargo.toml` with macroquad dependency
- `src/main.rs` with your game code
- The files described below

## Step 1: Project Setup

### Cargo.toml Configuration
Ensure your `Cargo.toml` includes:
```toml
[package]
name = "your_game_name"
version = "0.1.0"
edition = "2021"

[dependencies]
macroquad = "0.4"
```

## Step 2: Create Web Deployment Files

### Create `web/` Directory
```bash
mkdir web
```

### Download Macroquad WebGL Bundle
Download the official Macroquad WebGL bundle:
```bash
cd web
wget https://github.com/not-fl3/macroquad/releases/download/v0.4.4/mq_js_bundle.js
# OR use curl if wget unavailable:
# curl -L -o mq_js_bundle.js https://github.com/not-fl3/macroquad/releases/download/v0.4.4/mq_js_bundle.js
cd ..
```

### Create `web/index.html`
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Your Game Title</title>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="Content-Security-Policy" content="default-src 'self' 'unsafe-inline' 'unsafe-eval' data: blob:; script-src 'self' 'unsafe-inline' 'unsafe-eval'; worker-src 'self' blob:;">
    <meta name="mobile-web-app-capable" content="yes">
    <link rel="icon" href="data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMzIiIGhlaWdodD0iMzIiIHZpZXdCb3g9IjAgMCAzMiAzMiIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHJlY3Qgd2lkdGg9IjMyIiBoZWlnaHQ9IjMyIiBmaWxsPSIjMWExYTFhIi8+Cjx0ZXh0IHg9IjE2IiB5PSIyMCIgZm9udC1mYW1pbHk9Im1vbm9zcGFjZSIgZm9udC1zaXplPSIxOCIgZmlsbD0iIzAwZmYwMCIgdGV4dC1hbmNob3I9Im1pZGRsZSI+8J+PkyA8L3RleHQ+Cjwvc3ZnPgo=" type="image/svg+xml">
    <style>
        body {
            margin: 0;
            padding: 0;
            background: #1a1a1a;
            color: white;
            font-family: 'Courier New', monospace;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            min-height: 100vh;
        }
        
        h1 {
            color: #00ff00;
            text-shadow: 0 0 10px #00ff00;
            margin-bottom: 10px;
            font-size: 2.5em;
        }
        
        canvas {
            border: 2px solid #00ff00;
            box-shadow: 0 0 20px rgba(0, 255, 0, 0.3);
            margin: 20px 0;
        }
        
        .loading {
            text-align: center;
            color: #00ff00;
            font-size: 1.5em;
        }
        
        @media (max-width: 768px) {
            h1 { font-size: 2em; }
            canvas { 
                max-width: 90vw;
                height: auto;
            }
        }
    </style>
</head>
<body>
    <h1>🎮 YOUR GAME</h1>
    
    <div id="loading" class="loading">
        Loading game...
    </div>
    
    <canvas id="glcanvas" tabindex='1' style="display: none;"></canvas>
    
    <!-- Load Macroquad bundle first -->
    <script src="./mq_js_bundle.js"></script>
    
    <!-- Initialize game -->
    <script>
        console.log("Script starting...");
        
        function isMobile() {
            return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);
        }

        const gameCanvas = document.getElementById("glcanvas");
        const loadingDiv = document.getElementById("loading");
        
        console.log("Canvas element:", gameCanvas);
        console.log("Loading element:", loadingDiv);
        
        // Set canvas size (Macroquad will handle this, but set initial values)
        const targetWidth = isMobile() ? Math.min(window.innerWidth - 40, 600) : 800;
        const targetHeight = isMobile() ? Math.min(window.innerHeight * 0.6, 450) : 600;
        
        gameCanvas.width = targetWidth;
        gameCanvas.height = targetHeight;
        gameCanvas.style.width = targetWidth + 'px';
        gameCanvas.style.height = targetHeight + 'px';

        console.log("Canvas size set to:", gameCanvas.width, "x", gameCanvas.height);

        // Try to load immediately since script is after bundle
        console.log("Checking for load function...");
        console.log("typeof load:", typeof load);
        console.log("window.load:", window.load);
        
        if (typeof load === 'function') {
            console.log("Found load function, starting WASM...");
            loadingDiv.innerHTML = "Starting game...";
            
            // Check if this is Chrome mobile and handle differently
            const isChromeMobile = /Chrome/.test(navigator.userAgent) && /Mobile/.test(navigator.userAgent);
            console.log("Is Chrome Mobile:", isChromeMobile);
            
            if (isChromeMobile) {
                // Chrome mobile needs user interaction first
                loadingDiv.innerHTML = "Tap to start game";
                loadingDiv.style.cursor = "pointer";
                
                const startGame = () => {
                    console.log("Starting game after user interaction (Chrome mobile)");
                    loadingDiv.style.display = "none";
                    gameCanvas.style.display = "block";
                    gameCanvas.focus();
                    
                    try {
                        load("./your_game_name.wasm");
                        console.log("Load function called successfully");
                    } catch (error) {
                        loadingDiv.innerHTML = "Failed to load game: " + error;
                        loadingDiv.style.display = "block";
                        gameCanvas.style.display = "none";
                        console.error("Failed to load WASM:", error);
                    }
                };
                
                loadingDiv.addEventListener('click', startGame);
                loadingDiv.addEventListener('touchstart', startGame);
                
            } else {
                // Show canvas immediately for other browsers
                loadingDiv.style.display = "none";
                gameCanvas.style.display = "block";
                gameCanvas.focus();
                
                try {
                    load("./your_game_name.wasm");
                    console.log("Load function called successfully");
                    console.log("Game canvas is visible and game should start rendering immediately");
                    console.log("Game should be ready!");
                    
                } catch (error) {
                    loadingDiv.innerHTML = "Failed to load game: " + error;
                    loadingDiv.style.display = "block";
                    gameCanvas.style.display = "none";
                    console.error("Failed to load WASM:", error);
                }
            }
        } else {
            // Fallback: wait for page load
            console.log("Load function not immediately available, waiting for page load...");
            
            window.addEventListener('load', function() {
                console.log("Page loaded, rechecking...");
                
                if (typeof load === 'function') {
                    console.log("Found load function after page load");
                    
                    const isChromeMobile = /Chrome/.test(navigator.userAgent) && /Mobile/.test(navigator.userAgent);
                    
                    if (isChromeMobile) {
                        loadingDiv.innerHTML = "Tap to start game";
                        loadingDiv.style.cursor = "pointer";
                        
                        const startGame = () => {
                            loadingDiv.style.display = "none";
                            gameCanvas.style.display = "block";
                            gameCanvas.focus();
                            try {
                                load("./your_game_name.wasm");
                            } catch (error) {
                                loadingDiv.innerHTML = "Failed to load game: " + error;
                                loadingDiv.style.display = "block";
                                gameCanvas.style.display = "none";
                                console.error("Failed to load WASM:", error);
                            }
                        };
                        
                        loadingDiv.addEventListener('click', startGame);
                        loadingDiv.addEventListener('touchstart', startGame);
                    } else {
                        loadingDiv.innerHTML = "Starting game...";
                        loadingDiv.style.display = "none";
                        gameCanvas.style.display = "block";
                        gameCanvas.focus();
                        
                        try {
                            load("./your_game_name.wasm");
                            console.log("Game should start rendering immediately (fallback)");
                        } catch (error) {
                            loadingDiv.innerHTML = "Failed to load game: " + error;
                            loadingDiv.style.display = "block";
                            gameCanvas.style.display = "none";
                            console.error("Failed to load WASM:", error);
                        }
                    }
                } else {
                    loadingDiv.innerHTML = "Game failed to initialize - bundle not loaded";
                    console.error("Load function still not found after page load");
                    console.error("Available globals:", Object.keys(window).filter(k => typeof window[k] === 'function' && !k.startsWith('webkit')).slice(0, 30));
                }
            });
        }

        // Prevent context menu on right click for better game experience
        gameCanvas.addEventListener('contextmenu', (e) => e.preventDefault());
        
        // Auto-focus canvas when clicked
        gameCanvas.addEventListener('click', () => gameCanvas.focus());
    </script>
</body>
</html>
```

**Important**: Replace `your_game_name.wasm` in the HTML with your actual Cargo project name.

## Step 3: Create Development Script

### Create `dev.sh`
```bash
#!/bin/bash

echo "🔨 Building WASM..."
cargo build --target wasm32-unknown-unknown --release

echo "📦 Copying to web directory..."
cp target/wasm32-unknown-unknown/release/your_game_name.wasm web/

echo "🌐 Starting local server at http://localhost:8000"
echo "Press Ctrl+C to stop"

# Create a simple Python server with proper WASM MIME type
cat > web/server.py << 'EOF'
#!/usr/bin/env python3
import http.server
import socketserver
import mimetypes

# Add WASM MIME type
mimetypes.add_type('application/wasm', '.wasm')

class MyHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # Add CORS headers for local development
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        super().end_headers()

PORT = 8000
with socketserver.TCPServer(("", PORT), MyHTTPRequestHandler) as httpd:
    print(f"Server running at http://localhost:{PORT}")
    httpd.serve_forever()
EOF

cd web && python3 server.py
```

**Important**: Replace `your_game_name.wasm` with your actual Cargo project name.

Make the script executable:
```bash
chmod +x dev.sh
```

## Step 4: Create Deployment Configuration

### Create `nginx.conf`
```nginx
server {
    listen 8080;
    root /usr/share/nginx/html;
    index index.html;
    
    # Set MIME types explicitly
    location ~ \.wasm$ {
        default_type application/wasm;
        add_header Cross-Origin-Embedder-Policy require-corp always;
        add_header Cross-Origin-Opener-Policy same-origin always;
    }

    location ~ \.js$ {
        default_type application/javascript;
        add_header Cross-Origin-Embedder-Policy require-corp always;
        add_header Cross-Origin-Opener-Policy same-origin always;
    }

    location / {
        add_header Cross-Origin-Embedder-Policy require-corp always;
        add_header Cross-Origin-Opener-Policy same-origin always;
        try_files $uri $uri/ /index.html;
    }
}
```

### Create `Dockerfile`
```dockerfile
FROM nginx:alpine

COPY web/ /usr/share/nginx/html/
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]
```

### Create `fly.toml`
```toml
app = 'your-app-name'
primary_region = 'mia'

[build]

[http_service]
  internal_port = 8080
  force_https = true
  auto_stop_machines = 'stop'
  auto_start_machines = true
  min_machines_running = 0
  processes = ['app']

[[vm]]
  memory = '1gb'
  cpu_kind = 'shared'
  cpus = 1
```

**Important**: Replace `your-app-name` with your desired Fly.io app name.

## Step 5: Local Development

### Test Locally
```bash
# Build and run local development server
./dev.sh
```

Open http://localhost:8000 in your browser. The game should load and work in both Firefox and Chrome.

## Step 6: Deploy to Fly.io

### Initialize Fly.io App
```bash
# Create app (use the same name as in fly.toml)
flyctl apps create your-app-name

# OR if you want Fly.io to generate the name:
flyctl launch --no-deploy
```

### Build WASM for Production
```bash
# Build the WASM file
cargo build --target wasm32-unknown-unknown --release

# Copy to web directory
cp target/wasm32-unknown-unknown/release/your_game_name.wasm web/
```

### Deploy
```bash
flyctl deploy
```

### Access Your Game
```bash
# Get the URL
flyctl info

# OR open directly
flyctl open
```

## Step 7: Version Control (Optional but Recommended)

### Create `.gitignore`
```gitignore
/target/
**/*.rs.bk
Cargo.lock
web/server.py
web/*.wasm
```

### Initialize Git Repository
```bash
git init
git add .
git commit -m "Initial commit: Macroquad game ready for deployment"

# If you have a GitHub repository:
git remote add origin git@github.com:yourusername/your-repo.git
git push -u origin main
```

## Troubleshooting

### Common Issues

1. **MIME Type Errors in Chrome**:
   - Ensure nginx.conf uses `default_type application/wasm` for .wasm files
   - Verify CORS headers are present

2. **Game Not Loading**:
   - Check browser console for errors
   - Verify WASM file name matches in both HTML and file system
   - Ensure WebAssembly target is installed: `rustup target add wasm32-unknown-unknown`

3. **Local Development Issues**:
   - Make sure Python 3 is installed
   - Port 8000 might be in use - change in dev.sh if needed

4. **Fly.io Deployment Issues**:
   - Verify app name in fly.toml matches created app
   - Check Fly.io logs: `flyctl logs`
   - Ensure Docker builds successfully locally: `docker build .`

### Browser Compatibility

- **Firefox**: Works immediately on all platforms
- **Chrome Desktop**: Works immediately  
- **Chrome Mobile**: Requires user tap to start (handled automatically in the HTML)
- **Safari**: Should work but not extensively tested

### Performance Tips

- Use `--release` flag for production builds
- Consider enabling compression in nginx for larger games
- Monitor memory usage on Fly.io - adjust VM size if needed

## File Structure Summary

Your final project structure should look like:
```
your_game/
├── Cargo.toml
├── src/
│   └── main.rs
├── web/
│   ├── index.html
│   ├── mq_js_bundle.js
│   └── your_game_name.wasm (generated)
├── dev.sh
├── Dockerfile
├── nginx.conf
├── fly.toml
└── deployment_spec.md (this file)
```

This setup provides a complete, reproducible deployment pipeline for any Macroquad game to Fly.io.