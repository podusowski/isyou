Mission
-------
The idea came to me after my second participation is a search for a missing person. The story behind
such searches is simple. Often when someone gets lost, family calls the police and hospitals but
also publishes a note on local social media groups. This mobilizes normal people who join the search
by checking out surrounding neighborhood.


So what does it do?
-------------------
Every search participant might enable a beacon which will repeatedly report current position. Beacons are then displayed on a map so everyone can check what areas were already checked or not.


Running a development server
----------------------------
Development server can be deployed using `docker-compose`.

`Dockerfile` expects existing binary therefore it has to be built first:
```
env -C isyou_backend/ cargo build --release
```

Docker images can be built directly from `docker-compose`'s `up` command:
```
docker-compose up --build
```

If no weirdness is seen, the site should be available under http://localhost:8000.

This will run the system in release mode, if you want a debug, do this instead:
```
env -C isyou_backend/ cargo build && BACKEND_TARGET=debug docker-compose up --build
```
