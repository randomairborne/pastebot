FROM alpine
ARG TARGETARCH
COPY /${TARGETARCH}-executables/web /usr/bin/

ENTRYPOINT "/usr/bin/web"
