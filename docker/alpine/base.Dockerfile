FROM alpine:3.21.2
RUN apk update
RUN apk upgrade
RUN apk add --no-cache curl
RUN apk add --no-cache gzip
RUN apk add --no-cache git
RUN apk add --no-cache build-base
RUN apk add --no-cache pkgconf
RUN apk add --no-cache openssl
RUN apk add --no-cache openssl-dev
RUN apk add --no-cache ca-certificates
RUN apk add --no-cache bash
RUN apk add --no-cache libc6-compat