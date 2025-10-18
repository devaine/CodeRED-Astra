FROM node:23-alpine

COPY . /codered-astra

WORKDIR /codered-astra

RUN npm i 

EXPOSE 3000

RUN npm run format

RUN npm run build

CMD ["npm", "run", "host"]
