FROM node:12.6.0-alpine

WORKDIR /project
COPY package.json ./
COPY package-lock.json ./
RUN npm install
COPY postcss.config.js ./
COPY tsconfig.json ./
COPY webpack.config.js ./
COPY .browserslistrc ./
COPY src ./src

RUN npm run build
