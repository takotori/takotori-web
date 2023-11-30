FROM node:21-alpine AS build

ENV CI=true
ENV PORT=3000

WORKDIR /app
COPY package.json .
RUN npm i
COPY . .

RUN npm run build

FROM nginx:alpine
COPY --from=build /app/build /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]