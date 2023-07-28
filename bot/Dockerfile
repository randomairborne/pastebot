FROM alpine
ARG TARGETARCH
COPY /${TARGETARCH}-executables/pastebot-bot /usr/bin/

ENTRYPOINT "/usr/bin/pastebot-bot"
