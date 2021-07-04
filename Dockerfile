FROM rust:1.52.1 as backen
WORKDIR /var/www/http/blog/back
COPY ./back /var/www/http/blog/back
RUN cargo install --path . --root .

FROM node:14.17.0 as front
WORKDIR /var/www/http/blog/app
COPY ./app /var/www/http/blog/app
RUN npm config set registry=http://registry.npm.taobao.org && npm install && npm run build -- --prod

FROM ubuntu:20.10 as production
WORKDIR /var/www/http/blog
RUN apt-get update && apt-get update && apt-get install -y libpq5 && mkdir log
COPY ./back/.env /var/www/http/back/blog
COPY ./back/config /var/www/http/blog/back/config
COPY ./back/log /var/www/http/blog/back/log/
COPY ./back/ssl /var/www/http/blog/back/ssl/
COPY --from=backen /var/www/http/blog/back/bin/ ./back/bin/
COPY --from=front /var/www/http/blog/app/dist/app/ ./app/dist/app/

