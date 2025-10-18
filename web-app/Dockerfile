# web-app/Dockerfile
FROM node:20-alpine

WORKDIR /app

# Copy package files first to leverage Docker's build cache
COPY package*.json ./

# Install all dependencies needed for the build
RUN npm install

# Copy the rest of your application code
COPY . .

# Run the build script to compile the React frontend
RUN npm run build

# Expose the port your server will listen on
EXPOSE 3000

# The command to start your production server
CMD ["npm", "run", "host"]