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

#FROM development as dev-envs
#RUN <<EOF
#apt-get update
#apt-get install -y --no-install-recommends git
#EOF
#
#RUN <<EOF
#useradd -s /bin/bash -m vscode
#groupadd docker
#usermod -aG docker vscode
#EOF
## install Docker tools (cli, buildx, compose)
#COPY --from=gloursdocker/docker / /
#CMD [ "npm", "start" ]
#
#FROM development AS builder
#
#RUN npm run build
#
#FROM nginx:1.13-alpine
#
#COPY --from=builder /app/build /usr/share/nginx/html
