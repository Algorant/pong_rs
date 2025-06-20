#!/bin/bash

echo "🔨 Building WASM..."
cargo build --target wasm32-unknown-unknown --release

echo "📦 Copying to web directory..."
cp target/wasm32-unknown-unknown/release/pong_rs.wasm web/

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