FROM node:21-alpine AS build

ENV CI=true
ENV PORT=3000

WORKDIR /app
COPY package.json .
COPY package-lock.json .
RUN npm ci
COPY . .

RUN npm run build

FROM nginx:alpine
COPY --from=build /app/build /usr/share/nginx/html

RUN rm /etc/nginx/conf.d/default.conf
COPY nginx.conf /etc/nginx/conf.d/

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]