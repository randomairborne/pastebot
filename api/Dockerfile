FROM alpine
ARG TARGETARCH
COPY /${TARGETARCH}-executables/pastebot-api /usr/bin/

ENTRYPOINT "/usr/bin/pastebot-api"
