# Actix-Angular-Web

A blog for myself build with actix-web and angular.

## Usage

### Production

```shell
docker image build -t aaweb .
docker container run -it -d --name aaweb -p 80:8080 -p 443:8083 xxx bash
docker exec -it xxx bash
cd /var/www/http/actix-angular-web
./back/bin/actix-angular-web --prod
```

### Develop

```shell
#ng
cd ./app
ng serve
#actix
cargo run
```

## Milestones

- [x] Basic front and back show blogs
- [ ] Blog edit (Working on)
- [ ] Beautify (Working on)
- [x] Deploy script
- [x] Simple log system
- [ ] Cache system
- [x] Database connection pool
- [x] Containerization
- [ ] Comment and github OAuth

### 