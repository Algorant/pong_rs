# Use a lightweight nginx image to serve static files
FROM nginx:alpine

# Copy the web files to nginx's default serving directory
COPY web/ /usr/share/nginx/html/

# Copy custom nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Expose port 8080 (Fly.io default)
EXPOSE 8080

# Start nginx
CMD ["nginx", "-g", "daemon off;"]