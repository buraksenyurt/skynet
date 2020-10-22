FROM node:11
WORKDIR /app
COPY package*.json  ./
RUN npm install
COPY . .
EXPOSE 5555
CMD ["node", "index.js"]