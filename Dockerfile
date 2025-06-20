# Use nginx alpine image
FROM nginx:alpine

# Copy web files
COPY web/ /usr/share/nginx/html/

# Create a simple nginx config with WASM support
RUN cat > /etc/nginx/conf.d/default.conf << 'EOF'
server {
    listen 8080;
    root /usr/share/nginx/html;
    index index.html;

    # Add WASM MIME type
    location ~ \.wasm$ {
        add_header Content-Type application/wasm;
        add_header Cross-Origin-Embedder-Policy require-corp;
        add_header Cross-Origin-Opener-Policy same-origin;
    }

    # JavaScript files
    location ~ \.js$ {
        add_header Cross-Origin-Embedder-Policy require-corp;
        add_header Cross-Origin-Opener-Policy same-origin;
    }

    # Default handler
    location / {
        add_header Cross-Origin-Embedder-Policy require-corp;
        add_header Cross-Origin-Opener-Policy same-origin;
        try_files $uri $uri/ /index.html;
    }
}
EOF

EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]