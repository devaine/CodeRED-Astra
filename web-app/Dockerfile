FROM node:23-alpine

WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run format && npm run build
EXPOSE 3000
CMD ["npm", "run", "preview"]
