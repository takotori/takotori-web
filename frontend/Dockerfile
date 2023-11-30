# syntax=docker/dockerfile:1.4
FROM node:lts AS development

ENV CI=true
ENV PORT=3000

WORKDIR /release
COPY package.json .
COPY package-lock.json .
RUN npm ci
COPY . .

EXPOSE 3000

CMD [ "npm", "start" ]