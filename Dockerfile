FROM rust
LABEL maintainer="ritonelion@outlook.com"
WORKDIR /var/www/http/
COPY . /var/www/http/
RUN cargo install --path . --root .
RUN apt-get update && apt-get update && apt-get install -y npm
RUN cd ./app && npm config set registry=http://registry.npm.taobao.org && npm install && npm run build --prod
EXPOSE 8080
