Running development server
==========================
Development server can be deployed using `docker-compose`.

`Dockerfile` expects existing binary therefore it has to be built first:
```
env -C isyou_backend/ cargo build
```

Docker images can be built directly from `docker-compose`'s `up` command:
```
docker-compose up --build
```

If no weirdness is seen, the site should be available under http://localhost:8000.