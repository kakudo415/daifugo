FROM node

WORKDIR /usr/src/web

COPY package.json package-lock.json ./
RUN npm install

COPY . .

RUN npm run build
ENTRYPOINT ["npm", "run", "start"]
