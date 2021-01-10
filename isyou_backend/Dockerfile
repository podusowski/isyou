FROM ubuntu
ARG target=release
COPY target/${target}/isyou_backend /usr/bin/isyou_backend
ENTRYPOINT [ "isyou_backend" ]