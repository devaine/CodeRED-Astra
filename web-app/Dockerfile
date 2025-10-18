# web-app/Dockerfile
FROM node:20-alpine

WORKDIR /app

# Copy package files first to leverage Docker's build cache
COPY package*.json ./

# Install dependencies
RUN npm ci --only=production=false

# Copy the rest of your application code
COPY . .

# Build the React application
RUN npm run build

# Expose the port
EXPOSE 3000

# Use preview mode for production-like serving
CMD ["npm", "run", "preview"]