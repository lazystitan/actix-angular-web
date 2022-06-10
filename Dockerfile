FROM rust:1.61.0 as backen
WORKDIR /var/www/http/blog/back
COPY ./back /var/www/http/blog/back
RUN cargo install --path . --root .

FROM node:14.17.0 as front
WORKDIR /var/www/http/blog/app
COPY ./app /var/www/http/blog/app
RUN npm config set registry=http://registry.npm.taobao.org && npm install && npm run build -- --prod

FROM ubuntu:20.10 as production
WORKDIR /var/www/http/blog
RUN apt-get update && apt-get update && apt-get install -y libpq5 && mkdir back && mkdir back/log
COPY ./back/.env /var/www/http/blog/back/
COPY ./back/config/ /var/www/http/blog/back/config/
COPY ./back/ssl/ /var/www/http/blog/back/ssl/
COPY --from=backen /var/www/http/blog/back/bin/ ./back/bin/
COPY --from=front /var/www/http/blog/app/dist/app/ ./app/dist/app/
WORKDIR /var/www/http/blog/back
ENTRYPOINT ["./bin/actix-angular-web", "--prod"]
