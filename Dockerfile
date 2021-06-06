FROM rust:1.52.1 as backen
WORKDIR /var/www/http/
COPY . /var/www/http/
RUN cargo install --path . --root .

FROM node:14.17.0 as front
WORKDIR /var/www/http/
COPY . /var/www/http/
RUN cd ./app && npm config set registry=http://registry.npm.taobao.org && npm install && npm run build -- --prod

FROM ubuntu:20.10 as production
WORKDIR /var/www/http/
RUN apt-get update && apt-get update && apt-get install -y libpq5 && mkdir log
COPY ./.env /var/www/http/
COPY ./config /var/www/http/config
COPY --from=backen /var/www/http/bin/ ./bin/
COPY --from=front /var/www/http/app/dist/app/ ./app/dist/

