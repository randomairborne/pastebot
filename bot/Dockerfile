FROM alpine
ARG TARGETARCH
COPY /${TARGETARCH}-executables/pastebot /usr/bin/

ENTRYPOINT "/usr/bin/pastebot"
