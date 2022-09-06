FROM rust:bullseye
ARG SRV_KEY
ARG SRV_ADDR
ARG SRV_PORT
ARG SRV_PROT
RUN if [ -z "$SRV_KEY" ]; then echo 'SRV_KEY is not set'; exit 1; fi
RUN if [ -z "$SRV_ADDR" ]; then echo 'SRV_ADDR is not set'; exit 1; fi
RUN if [ -z "$SRV_PORT" ]; then echo 'SRV_PORT is not set'; exit 1; fi
RUN if [ -z "$SRV_PROT" ]; then echo 'SRV_PROT is not set'; exit 1; fi
ENV DEBIAN_FRONTEND=noninteractive
ENV DEBCONF_NOWARNINGS="yes"
RUN apt-get update && apt-get -y install clang lld
COPY . 555NAKE
WORKDIR 555NAKE
ENV SRV_ADDR=$SRV_ADDR
ENV SRV_PORT=$SRV_PORT
ENV SRV_PROT=$SRV_PROT
ENV UUID=$SRV_KEY
RUN make build_server