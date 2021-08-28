# Actix-Angular-Web

A blog for myself build with actix-web and angular. Used by my personal site : https://www.ritonelion.com

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
cd ./back
cargo run
```

## Milestones

- [x] Basic front and back show blogs
- [x] Create new blog
- [ ] Blog edit
    - [x] pure text markdown support
    - [ ] image support (Working on)
- [ ] Post catalogue and navigation
    - [x] click catalogue scroll to correspond content
    - [ ] active style for reference in catalogue of current content (Working on)
- [ ] Beautify (Working on)
- [x] Deploy script
- [x] Simple log system
- [ ] Cache system (Working on)
- [x] Database connection pool
- [x] Containerization
- [ ] Comment and github OAuth
- [ ] tag system

###       
