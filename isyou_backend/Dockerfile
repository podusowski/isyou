FROM ubuntu
ARG target=release
COPY target/${target}/isyou_backend /usr/bin/isyou_backend
COPY Rocket.toml .
ENTRYPOINT [ "isyou_backend" ]