# Dockerfiles

Real Dockerfiles from official Docker library images and popular projects (postgres, redis, python, golang, mongo, nginx, kubernetes, moby, grafana, prometheus, airflow, and more). Sources and licenses in ATTRIBUTION.md.

Each row links to the full raw input and the exact compacted output used by the benchmark. Percentages are **token reduction: higher is better**; 0% means pass-through. `Algorithm` is the compressor-only reduction. `Pass 1` disables CCR (compressed with omission markers, no recovery footer). `Pass 2` is the final model-facing result with CCR enabled — it reads marginally *lower* than Pass 1 only because the recovery footer adds a few dozen bytes to the output.

## Cases

Every case links to the raw input, the exact model-facing output (with the CCR recovery footer), and a unified diff between the two.

| Case | Input | Output (after CCR) | Diff | Original | Algorithm | Pass 1: no CCR | Pass 2: with CCR | Avg latency | CCR |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| `42-n8n` | [input](cases/42-n8n/input.dockerfile) | [output](cases/42-n8n/output.dockerfile) | [diff](cases/42-n8n/compression.diff) | 3.0 KB | 0.0% | 0.0% | 0.0% | 0.025 ms | n/a |
| `41-vault` | [input](cases/41-vault/input.dockerfile) | [output](cases/41-vault/output.dockerfile) | [diff](cases/41-vault/compression.diff) | 7.1 KB | 0.0% | 0.0% | 0.0% | 0.062 ms | n/a |
| `40-clickhouse` | [input](cases/40-clickhouse/input.dockerfile) | [output](cases/40-clickhouse/output.dockerfile) | [diff](cases/40-clickhouse/compression.diff) | 6.1 KB | 0.0% | 0.0% | 0.0% | 0.049 ms | n/a |
| `39-home-assistant` | [input](cases/39-home-assistant/input.dockerfile) | [output](cases/39-home-assistant/output.dockerfile) | [diff](cases/39-home-assistant/compression.diff) | 1.7 KB | 0.0% | 0.0% | 0.0% | 0.000 ms | n/a |
| `38-superset` | [input](cases/38-superset/input.dockerfile) | [output](cases/38-superset/output.dockerfile) | [diff](cases/38-superset/compression.diff) | 7.1 KB | 0.0% | 0.0% | 0.0% | 0.061 ms | n/a |
| `37-mastodon` | [input](cases/37-mastodon/input.dockerfile) | [output](cases/37-mastodon/output.dockerfile) | [diff](cases/37-mastodon/compression.diff) | 13.8 KB | 0.0% | 0.0% | 0.0% | 0.167 ms | n/a |
| `36-ollama` | [input](cases/36-ollama/input.dockerfile) | [output](cases/36-ollama/output.dockerfile) | [diff](cases/36-ollama/compression.diff) | 5.8 KB | 0.0% | 0.0% | 0.0% | 0.045 ms | n/a |
| `35-keycloak` | [input](cases/35-keycloak/input.dockerfile) | [output](cases/35-keycloak/output.dockerfile) | [diff](cases/35-keycloak/compression.diff) | 2.7 KB | 0.0% | 0.0% | 0.0% | 0.022 ms | n/a |
| `34-gitea` | [input](cases/34-gitea/input.dockerfile) | [output](cases/34-gitea/output.dockerfile) | [diff](cases/34-gitea/compression.diff) | 2.1 KB | 0.0% | 0.0% | 0.0% | 0.034 ms | n/a |
| `32-caddy` | [input](cases/32-caddy/input.dockerfile) | [output](cases/32-caddy/output.dockerfile) | [diff](cases/32-caddy/compression.diff) | 3.1 KB | 0.0% | 0.0% | 0.0% | 0.025 ms | n/a |
| `31-julia` | [input](cases/31-julia/input.dockerfile) | [output](cases/31-julia/output.dockerfile) | [diff](cases/31-julia/compression.diff) | 2.7 KB | 0.0% | 0.0% | 0.0% | 0.031 ms | n/a |
| `30-influxdb` | [input](cases/30-influxdb/input.dockerfile) | [output](cases/30-influxdb/output.dockerfile) | [diff](cases/30-influxdb/compression.diff) | 5.1 KB | 0.0% | 0.0% | 0.0% | 0.040 ms | n/a |
| `29-nextjs-example` | [input](cases/29-nextjs-example/input.dockerfile) | [output](cases/29-nextjs-example/output.dockerfile) | [diff](cases/29-nextjs-example/compression.diff) | 3.9 KB | 0.0% | 0.0% | 0.0% | 0.031 ms | n/a |
| `28-airflow` | [input](cases/28-airflow/input.dockerfile) | [output](cases/28-airflow/output.dockerfile) | [diff](cases/28-airflow/compression.diff) | 93.0 KB | 0.0% | 0.0% | 0.0% | 0.441 ms | n/a |
| `27-sentry-self-hosted` | [input](cases/27-sentry-self-hosted/input.dockerfile) | [output](cases/27-sentry-self-hosted/output.dockerfile) | [diff](cases/27-sentry-self-hosted/compression.diff) | 3.8 KB | 0.0% | 0.0% | 0.0% | 0.034 ms | n/a |
| `26-busybox-builder` | [input](cases/26-busybox-builder/input.dockerfile) | [output](cases/26-busybox-builder/output.dockerfile) | [diff](cases/26-busybox-builder/compression.diff) | 8.1 KB | 0.0% | 0.0% | 0.0% | 0.082 ms | n/a |
| `25-traefik` | [input](cases/25-traefik/input.dockerfile) | [output](cases/25-traefik/output.dockerfile) | [diff](cases/25-traefik/compression.diff) | 219 B | 0.0% | 0.0% | 0.0% | 0.000 ms | n/a |
| `24-node` | [input](cases/24-node/input.dockerfile) | [output](cases/24-node/output.dockerfile) | [diff](cases/24-node/compression.diff) | 3.4 KB | 0.0% | 0.0% | 0.0% | 0.029 ms | n/a |
| `23-elasticsearch` | [input](cases/23-elasticsearch/input.dockerfile) | [output](cases/23-elasticsearch/output.dockerfile) | [diff](cases/23-elasticsearch/compression.diff) | 802 B | 0.0% | 0.0% | 0.0% | 0.000 ms | n/a |
| `22-moby` | [input](cases/22-moby/input.dockerfile) | [output](cases/22-moby/output.dockerfile) | [diff](cases/22-moby/compression.diff) | 25.0 KB | 0.0% | 0.0% | 0.0% | 0.204 ms | n/a |
| `21-kube-apiserver` | [input](cases/21-kube-apiserver/input.dockerfile) | [output](cases/21-kube-apiserver/output.dockerfile) | [diff](cases/21-kube-apiserver/compression.diff) | 1.3 KB | 0.0% | 0.0% | 0.0% | 0.000 ms | n/a |
| `20-prometheus` | [input](cases/20-prometheus/input.dockerfile) | [output](cases/20-prometheus/output.dockerfile) | [diff](cases/20-prometheus/compression.diff) | 1.4 KB | 0.0% | 0.0% | 0.0% | 0.000 ms | n/a |
| `19-grafana` | [input](cases/19-grafana/input.dockerfile) | [output](cases/19-grafana/output.dockerfile) | [diff](cases/19-grafana/compression.diff) | 15.0 KB | 0.0% | 0.0% | 0.0% | 0.126 ms | n/a |
| `18-temurin-jdk` | [input](cases/18-temurin-jdk/input.dockerfile) | [output](cases/18-temurin-jdk/output.dockerfile) | [diff](cases/18-temurin-jdk/compression.diff) | 5.5 KB | 0.0% | 0.0% | 0.0% | 0.052 ms | n/a |
| `17-cassandra` | [input](cases/17-cassandra/input.dockerfile) | [output](cases/17-cassandra/output.dockerfile) | [diff](cases/17-cassandra/compression.diff) | 7.4 KB | 0.0% | 0.0% | 0.0% | 0.066 ms | n/a |
| `16-nginx` | [input](cases/16-nginx/input.dockerfile) | [output](cases/16-nginx/output.dockerfile) | [diff](cases/16-nginx/compression.diff) | 7.3 KB | 0.0% | 0.0% | 0.0% | 0.059 ms | n/a |
| `15-ruby` | [input](cases/15-ruby/input.dockerfile) | [output](cases/15-ruby/output.dockerfile) | [diff](cases/15-ruby/compression.diff) | 3.8 KB | 0.0% | 0.0% | 0.0% | 0.045 ms | n/a |
| `14-php` | [input](cases/14-php/input.dockerfile) | [output](cases/14-php/output.dockerfile) | [diff](cases/14-php/compression.diff) | 7.3 KB | 0.0% | 0.0% | 0.0% | 0.091 ms | n/a |
| `13-mariadb` | [input](cases/13-mariadb/input.dockerfile) | [output](cases/13-mariadb/output.dockerfile) | [diff](cases/13-mariadb/compression.diff) | 6.7 KB | 0.0% | 0.0% | 0.0% | 0.055 ms | n/a |
| `12-haproxy` | [input](cases/12-haproxy/input.dockerfile) | [output](cases/12-haproxy/output.dockerfile) | [diff](cases/12-haproxy/compression.diff) | 3.5 KB | 0.0% | 0.0% | 0.0% | 0.043 ms | n/a |
| `11-wordpress` | [input](cases/11-wordpress/input.dockerfile) | [output](cases/11-wordpress/output.dockerfile) | [diff](cases/11-wordpress/compression.diff) | 6.1 KB | 0.0% | 0.0% | 0.0% | 0.069 ms | n/a |
| `10-tomcat` | [input](cases/10-tomcat/input.dockerfile) | [output](cases/10-tomcat/output.dockerfile) | [diff](cases/10-tomcat/compression.diff) | 5.6 KB | 0.0% | 0.0% | 0.0% | 0.061 ms | n/a |
| `09-memcached` | [input](cases/09-memcached/input.dockerfile) | [output](cases/09-memcached/output.dockerfile) | [diff](cases/09-memcached/compression.diff) | 3.1 KB | 0.0% | 0.0% | 0.0% | 0.043 ms | n/a |
| `08-rabbitmq` | [input](cases/08-rabbitmq/input.dockerfile) | [output](cases/08-rabbitmq/output.dockerfile) | [diff](cases/08-rabbitmq/compression.diff) | 15.0 KB | 0.0% | 0.0% | 0.0% | 0.126 ms | n/a |
| `07-httpd` | [input](cases/07-httpd/input.dockerfile) | [output](cases/07-httpd/output.dockerfile) | [diff](cases/07-httpd/compression.diff) | 8.4 KB | 0.0% | 0.0% | 0.0% | 0.091 ms | n/a |
| `06-mongo` | [input](cases/06-mongo/input.dockerfile) | [output](cases/06-mongo/output.dockerfile) | [diff](cases/06-mongo/compression.diff) | 4.5 KB | 0.0% | 0.0% | 0.0% | 0.042 ms | n/a |
| `05-golang` | [input](cases/05-golang/input.dockerfile) | [output](cases/05-golang/output.dockerfile) | [diff](cases/05-golang/compression.diff) | 5.6 KB | 0.0% | 0.0% | 0.0% | 0.053 ms | n/a |
| `04-python` | [input](cases/04-python/input.dockerfile) | [output](cases/04-python/output.dockerfile) | [diff](cases/04-python/compression.diff) | 3.9 KB | 0.0% | 0.0% | 0.0% | 0.044 ms | n/a |
| `03-redis` | [input](cases/03-redis/input.dockerfile) | [output](cases/03-redis/output.dockerfile) | [diff](cases/03-redis/compression.diff) | 7.3 KB | 0.0% | 0.0% | 0.0% | 0.058 ms | n/a |
| `02-postgres-16` | [input](cases/02-postgres-16/input.dockerfile) | [output](cases/02-postgres-16/output.dockerfile) | [diff](cases/02-postgres-16/compression.diff) | 9.7 KB | 0.0% | 0.0% | 0.0% | 0.074 ms | n/a |
| `01-postgres` | [input](cases/01-postgres/input.dockerfile) | [output](cases/01-postgres/output.dockerfile) | [diff](cases/01-postgres/compression.diff) | 9.7 KB | 0.0% | 0.0% | 0.0% | 0.086 ms | n/a |

## What TinyJuice Is Doing

Dockerfiles are a coverage/honesty category: they carry no collapsible structure the current compressors target, so the router should pass them through untouched rather than mangle them.

## Syntax-Aware Samples

### `42-n8n`

- [Full input](cases/42-n8n/input.dockerfile)
- [Full output](cases/42-n8n/output.dockerfile)
- [Input vs output diff](cases/42-n8n/compression.diff)

Input excerpt:

```text
ARG NODE_VERSION=20
ARG N8N_VERSION=snapshot

# 1. Create an image to build n8n
FROM --platform=linux/amd64 n8nio/base:${NODE_VERSION} AS builder

# Build the application from source
WORKDIR /src
COPY . /src
RUN --mount=type=cache,id=pnpm-store,target=/root/.local/share/pnpm/store --mount=type=cache,id=pnpm-metadata,target=/root/.cache/pnpm/metadata DOCKER_BUILD=true pnpm install --frozen-lockfile
RUN pnpm build

# Delete all dev dependencies
RUN jq 'del(.pnpm.patchedDependencies)' package.json > package.json.tmp; mv package.json.tmp package.json
RUN node .github/scripts/trim-fe-packageJson.js

# Delete any source code or typings
RUN find . -type f -name "*.ts" -o -name "*.vue" -o -name "tsconfig.json" -o -name "*.tsbuildinfo" | xargs rm -rf

# Deploy the `n8n` package into /compiled
RUN mkdir /compiled
RUN NODE_ENV=production DOCKER_BUILD=true pnpm --filter=n8n --prod --no-optional --legacy deploy /compiled

# 2. Start with a new clean image with just the code that is needed to run n8n
FROM n8nio/base:${NODE_VERSION}
ENV NODE_ENV=production

ARG N8N_RELEASE_TYPE=dev
ENV N8N_RELEASE_TYPE=${N8N_RELEASE_TYPE}

LABEL org.opencontainers.image.title="n8n"
LABEL org.opencontainers.image.description="Workflow Automation Tool"
LABEL org.opencontainers.image.source="https://github.com/n8n-io/n8n"
LABEL org.opencontainers.image.url="https://n8n.io"
LABEL org.opencontainers.image.version=${N8N_VERSION}


```

Output excerpt:

```text
ARG NODE_VERSION=20
ARG N8N_VERSION=snapshot

# 1. Create an image to build n8n
FROM --platform=linux/amd64 n8nio/base:${NODE_VERSION} AS builder

# Build the application from source
WORKDIR /src
COPY . /src
RUN --mount=type=cache,id=pnpm-store,target=/root/.local/share/pnpm/store --mount=type=cache,id=pnpm-metadata,target=/root/.cache/pnpm/metadata DOCKER_BUILD=true pnpm install --frozen-lockfile
RUN pnpm build

# Delete all dev dependencies
RUN jq 'del(.pnpm.patchedDependencies)' package.json > package.json.tmp; mv package.json.tmp package.json
RUN node .github/scripts/trim-fe-packageJson.js

# Delete any source code or typings
RUN find . -type f -name "*.ts" -o -name "*.vue" -o -name "tsconfig.json" -o -name "*.tsbuildinfo" | xargs rm -rf

# Deploy the `n8n` package into /compiled
RUN mkdir /compiled
RUN NODE_ENV=production DOCKER_BUILD=true pnpm --filter=n8n --prod --no-optional --legacy deploy /compiled

# 2. Start with a new clean image with just the code that is needed to run n8n
FROM n8nio/base:${NODE_VERSION}
ENV NODE_ENV=production

ARG N8N_RELEASE_TYPE=dev
ENV N8N_RELEASE_TYPE=${N8N_RELEASE_TYPE}

LABEL org.opencontainers.image.title="n8n"
LABEL org.opencontainers.image.description="Workflow Automation Tool"
LABEL org.opencontainers.image.source="https://github.com/n8n-io/n8n"
LABEL org.opencontainers.image.url="https://n8n.io"
LABEL org.opencontainers.image.version=${N8N_VERSION}


```

### `41-vault`

- [Full input](cases/41-vault/input.dockerfile)
- [Full output](cases/41-vault/output.dockerfile)
- [Input vs output diff](cases/41-vault/compression.diff)

Input excerpt:

```text
# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: BUSL-1.1

## DOCKERHUB DOCKERFILE ##
FROM alpine:3 AS default

ARG BIN_NAME
# NAME and PRODUCT_VERSION are the name of the software in releases.hashicorp.com
# and the version to download. Example: NAME=vault PRODUCT_VERSION=1.2.3.
ARG NAME=vault
ARG PRODUCT_VERSION
ARG PRODUCT_REVISION
# TARGETARCH and TARGETOS are set automatically when --platform is provided.
ARG TARGETOS TARGETARCH

# Additional metadata labels used by container registries, platforms
# and certification scanners.
LABEL name="Vault" \
      maintainer="Vault Team <vault@hashicorp.com>" \
      vendor="HashiCorp" \
      version=${PRODUCT_VERSION} \
      release=${PRODUCT_REVISION} \
      revision=${PRODUCT_REVISION} \
      summary="Vault is a tool for securely accessing secrets." \
      description="Vault is a tool for securely accessing secrets. A secret is anything that you want to tightly control access to, such as API keys, passwords, certificates, and more. Vault provides a unified interface ...

# Copy the license file as per Legal requirement
COPY LICENSE /usr/share/doc/$NAME/LICENSE.txt

# Set ARGs as ENV so that they can be used in ENTRYPOINT/CMD
ENV NAME=$NAME
ENV VERSION=$VERSION

# Create a non-root user to run the software.
RUN addgroup ${NAME} && adduser -S -G ${NAME} ${NAME}


```

Output excerpt:

```text
# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: BUSL-1.1

## DOCKERHUB DOCKERFILE ##
FROM alpine:3 AS default

ARG BIN_NAME
# NAME and PRODUCT_VERSION are the name of the software in releases.hashicorp.com
# and the version to download. Example: NAME=vault PRODUCT_VERSION=1.2.3.
ARG NAME=vault
ARG PRODUCT_VERSION
ARG PRODUCT_REVISION
# TARGETARCH and TARGETOS are set automatically when --platform is provided.
ARG TARGETOS TARGETARCH

# Additional metadata labels used by container registries, platforms
# and certification scanners.
LABEL name="Vault" \
      maintainer="Vault Team <vault@hashicorp.com>" \
      vendor="HashiCorp" \
      version=${PRODUCT_VERSION} \
      release=${PRODUCT_REVISION} \
      revision=${PRODUCT_REVISION} \
      summary="Vault is a tool for securely accessing secrets." \
      description="Vault is a tool for securely accessing secrets. A secret is anything that you want to tightly control access to, such as API keys, passwords, certificates, and more. Vault provides a unified interface ...

# Copy the license file as per Legal requirement
COPY LICENSE /usr/share/doc/$NAME/LICENSE.txt

# Set ARGs as ENV so that they can be used in ENTRYPOINT/CMD
ENV NAME=$NAME
ENV VERSION=$VERSION

# Create a non-root user to run the software.
RUN addgroup ${NAME} && adduser -S -G ${NAME} ${NAME}


```

### `40-clickhouse`

- [Full input](cases/40-clickhouse/input.dockerfile)
- [Full output](cases/40-clickhouse/output.dockerfile)
- [Input vs output diff](cases/40-clickhouse/compression.diff)

Input excerpt:

```text
FROM ubuntu:22.04

# see https://github.com/moby/moby/issues/4032#issuecomment-192327844
# It could be removed after we move on a version 23:04+
ARG DEBIAN_FRONTEND=noninteractive

# ARG for quick switch to a given ubuntu mirror
ARG apt_archive="http://archive.ubuntu.com"

# We shouldn't use `apt upgrade` to not change the upstream image. It's updated biweekly

# user/group precreated explicitly with fixed uid/gid on purpose.
# It is especially important for rootless containers: in that case entrypoint
# can't do chown and owners of mounted volumes should be configured externally.
# We do that in advance at the begining of Dockerfile before any packages will be
# installed to prevent picking those uid / gid by some unrelated software.
# The same uid / gid (101) is used both for alpine and ubuntu.
RUN sed -i "s|http://archive.ubuntu.com|${apt_archive}|g" /etc/apt/sources.list \
    && groupadd -r clickhouse --gid=101 \
    && useradd -r -g clickhouse --uid=101 --home-dir=/var/lib/clickhouse --shell=/bin/bash clickhouse \
    && apt-get update \
    && apt-get install --yes --no-install-recommends \
        ca-certificates \
        locales \
        tzdata \
        wget \
    && rm -rf /var/lib/apt/lists/* /var/cache/debconf /tmp/*

ARG REPO_CHANNEL="stable"
ARG REPOSITORY="deb [signed-by=/usr/share/keyrings/clickhouse-keyring.gpg] https://packages.clickhouse.com/deb ${REPO_CHANNEL} main"
ARG VERSION="25.3.2.39"
ARG PACKAGES="clickhouse-client clickhouse-server clickhouse-common-static"

#docker-official-library:off
# The part between `docker-official-library` tags is related to our builds


```

Output excerpt:

```text
FROM ubuntu:22.04

# see https://github.com/moby/moby/issues/4032#issuecomment-192327844
# It could be removed after we move on a version 23:04+
ARG DEBIAN_FRONTEND=noninteractive

# ARG for quick switch to a given ubuntu mirror
ARG apt_archive="http://archive.ubuntu.com"

# We shouldn't use `apt upgrade` to not change the upstream image. It's updated biweekly

# user/group precreated explicitly with fixed uid/gid on purpose.
# It is especially important for rootless containers: in that case entrypoint
# can't do chown and owners of mounted volumes should be configured externally.
# We do that in advance at the begining of Dockerfile before any packages will be
# installed to prevent picking those uid / gid by some unrelated software.
# The same uid / gid (101) is used both for alpine and ubuntu.
RUN sed -i "s|http://archive.ubuntu.com|${apt_archive}|g" /etc/apt/sources.list \
    && groupadd -r clickhouse --gid=101 \
    && useradd -r -g clickhouse --uid=101 --home-dir=/var/lib/clickhouse --shell=/bin/bash clickhouse \
    && apt-get update \
    && apt-get install --yes --no-install-recommends \
        ca-certificates \
        locales \
        tzdata \
        wget \
    && rm -rf /var/lib/apt/lists/* /var/cache/debconf /tmp/*

ARG REPO_CHANNEL="stable"
ARG REPOSITORY="deb [signed-by=/usr/share/keyrings/clickhouse-keyring.gpg] https://packages.clickhouse.com/deb ${REPO_CHANNEL} main"
ARG VERSION="25.3.2.39"
ARG PACKAGES="clickhouse-client clickhouse-server clickhouse-common-static"

#docker-official-library:off
# The part between `docker-official-library` tags is related to our builds


```

### `39-home-assistant`

- [Full input](cases/39-home-assistant/input.dockerfile)
- [Full output](cases/39-home-assistant/output.dockerfile)
- [Input vs output diff](cases/39-home-assistant/compression.diff)

Input excerpt:

```text
# Automatically generated by hassfest.
#
# To update, run python3 -m script.hassfest -p docker
ARG BUILD_FROM
FROM ${BUILD_FROM}

# Synchronize with homeassistant/core.py:async_stop
ENV \
    S6_SERVICES_GRACETIME=240000 \
    UV_SYSTEM_PYTHON=true \
    UV_NO_CACHE=true

ARG QEMU_CPU

# Home Assistant S6-Overlay
COPY rootfs /

# Needs to be redefined inside the FROM statement to be set for RUN commands
ARG BUILD_ARCH
# Get go2rtc binary
RUN \
    case "${BUILD_ARCH}" in \
        "aarch64") go2rtc_suffix='arm64' ;; \
        "armhf") go2rtc_suffix='armv6' ;; \
        "armv7") go2rtc_suffix='arm' ;; \
        *) go2rtc_suffix=${BUILD_ARCH} ;; \
    esac \
    && curl -L https://github.com/AlexxIT/go2rtc/releases/download/v1.9.9/go2rtc_linux_${go2rtc_suffix} --output /bin/go2rtc \
    && chmod +x /bin/go2rtc \
    # Verify go2rtc can be executed
    && go2rtc --version

# Install uv
RUN pip3 install uv==0.7.1

WORKDIR /usr/src

```

Output excerpt:

```text
# Automatically generated by hassfest.
#
# To update, run python3 -m script.hassfest -p docker
ARG BUILD_FROM
FROM ${BUILD_FROM}

# Synchronize with homeassistant/core.py:async_stop
ENV \
    S6_SERVICES_GRACETIME=240000 \
    UV_SYSTEM_PYTHON=true \
    UV_NO_CACHE=true

ARG QEMU_CPU

# Home Assistant S6-Overlay
COPY rootfs /

# Needs to be redefined inside the FROM statement to be set for RUN commands
ARG BUILD_ARCH
# Get go2rtc binary
RUN \
    case "${BUILD_ARCH}" in \
        "aarch64") go2rtc_suffix='arm64' ;; \
        "armhf") go2rtc_suffix='armv6' ;; \
        "armv7") go2rtc_suffix='arm' ;; \
        *) go2rtc_suffix=${BUILD_ARCH} ;; \
    esac \
    && curl -L https://github.com/AlexxIT/go2rtc/releases/download/v1.9.9/go2rtc_linux_${go2rtc_suffix} --output /bin/go2rtc \
    && chmod +x /bin/go2rtc \
    # Verify go2rtc can be executed
    && go2rtc --version

# Install uv
RUN pip3 install uv==0.7.1

WORKDIR /usr/src

```

### `38-superset`

- [Full input](cases/38-superset/input.dockerfile)
- [Full output](cases/38-superset/output.dockerfile)
- [Input vs output diff](cases/38-superset/compression.diff)

Input excerpt:

```text
#
# Licensed to the Apache Software Foundation (ASF) under one or more
# contributor license agreements.  See the NOTICE file distributed with
# this work for additional information regarding copyright ownership.
# The ASF licenses this file to You under the Apache License, Version 2.0
# (the "License"); you may not use this file except in compliance with
# the License.  You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

######################################################################
# Node stage to deal with static asset construction
######################################################################
ARG PY_VER=3.10-slim-bookworm

# if BUILDPLATFORM is null, set it to 'amd64' (or leave as is otherwise).
ARG BUILDPLATFORM=${BUILDPLATFORM:-amd64}
FROM --platform=${BUILDPLATFORM} node:18-bullseye-slim AS superset-node

ARG NPM_BUILD_CMD="build"

# Somehow we need python3 + build-essential on this side of the house to install node-gyp
RUN apt-get update -qq \
    && apt-get install \
        -yqq --no-install-recommends \
        build-essential \
        python3

ENV BUILD_CMD=${NPM_BUILD_CMD} \

```

Output excerpt:

```text
#
# Licensed to the Apache Software Foundation (ASF) under one or more
# contributor license agreements.  See the NOTICE file distributed with
# this work for additional information regarding copyright ownership.
# The ASF licenses this file to You under the Apache License, Version 2.0
# (the "License"); you may not use this file except in compliance with
# the License.  You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

######################################################################
# Node stage to deal with static asset construction
######################################################################
ARG PY_VER=3.10-slim-bookworm

# if BUILDPLATFORM is null, set it to 'amd64' (or leave as is otherwise).
ARG BUILDPLATFORM=${BUILDPLATFORM:-amd64}
FROM --platform=${BUILDPLATFORM} node:18-bullseye-slim AS superset-node

ARG NPM_BUILD_CMD="build"

# Somehow we need python3 + build-essential on this side of the house to install node-gyp
RUN apt-get update -qq \
    && apt-get install \
        -yqq --no-install-recommends \
        build-essential \
        python3

ENV BUILD_CMD=${NPM_BUILD_CMD} \

```

### `37-mastodon`

- [Full input](cases/37-mastodon/input.dockerfile)
- [Full output](cases/37-mastodon/output.dockerfile)
- [Input vs output diff](cases/37-mastodon/compression.diff)

Input excerpt:

```text
# syntax=docker/dockerfile:1.9

# This file is designed for production server deployment, not local development work
# For a containerized local dev environment, see: https://github.com/mastodon/mastodon/blob/main/README.md#docker

# Please see https://docs.docker.com/engine/reference/builder for information about
# the extended buildx capabilities used in this file.
# Make sure multiarch TARGETPLATFORM is available for interpolation
# See: https://docs.docker.com/build/building/multi-platform/
ARG TARGETPLATFORM=${TARGETPLATFORM}
ARG BUILDPLATFORM=${BUILDPLATFORM}

# Ruby image to use for base image, change with [--build-arg RUBY_VERSION="3.3.x"]
# renovate: datasource=docker depName=docker.io/ruby
ARG RUBY_VERSION="3.3.5"
# # Node version to use in base image, change with [--build-arg NODE_MAJOR_VERSION="20"]
# renovate: datasource=node-version depName=node
ARG NODE_MAJOR_VERSION="20"
# Debian image to use for base image, change with [--build-arg DEBIAN_VERSION="bookworm"]
ARG DEBIAN_VERSION="bookworm"
# Node image to use for base image based on combined variables (ex: 20-bookworm-slim)
FROM docker.io/node:${NODE_MAJOR_VERSION}-${DEBIAN_VERSION}-slim AS node
# Ruby image to use for base image based on combined variables (ex: 3.3.x-slim-bookworm)
FROM docker.io/ruby:${RUBY_VERSION}-slim-${DEBIAN_VERSION} AS ruby

# Resulting version string is vX.X.X-MASTODON_VERSION_PRERELEASE+MASTODON_VERSION_METADATA
# Example: v4.3.0-nightly.2023.11.09+pr-123456
# Overwrite existence of 'alpha.X' in version.rb [--build-arg MASTODON_VERSION_PRERELEASE="nightly.2023.11.09"]
ARG MASTODON_VERSION_PRERELEASE=""
# Append build metadata or fork information to version.rb [--build-arg MASTODON_VERSION_METADATA="pr-123456"]
ARG MASTODON_VERSION_METADATA=""

# Allow Ruby on Rails to serve static files
# See: https://docs.joinmastodon.org/admin/config/#rails_serve_static_files
ARG RAILS_SERVE_STATIC_FILES="true"
# Allow to use YJIT compiler

```

Output excerpt:

```text
# syntax=docker/dockerfile:1.9

# This file is designed for production server deployment, not local development work
# For a containerized local dev environment, see: https://github.com/mastodon/mastodon/blob/main/README.md#docker

# Please see https://docs.docker.com/engine/reference/builder for information about
# the extended buildx capabilities used in this file.
# Make sure multiarch TARGETPLATFORM is available for interpolation
# See: https://docs.docker.com/build/building/multi-platform/
ARG TARGETPLATFORM=${TARGETPLATFORM}
ARG BUILDPLATFORM=${BUILDPLATFORM}

# Ruby image to use for base image, change with [--build-arg RUBY_VERSION="3.3.x"]
# renovate: datasource=docker depName=docker.io/ruby
ARG RUBY_VERSION="3.3.5"
# # Node version to use in base image, change with [--build-arg NODE_MAJOR_VERSION="20"]
# renovate: datasource=node-version depName=node
ARG NODE_MAJOR_VERSION="20"
# Debian image to use for base image, change with [--build-arg DEBIAN_VERSION="bookworm"]
ARG DEBIAN_VERSION="bookworm"
# Node image to use for base image based on combined variables (ex: 20-bookworm-slim)
FROM docker.io/node:${NODE_MAJOR_VERSION}-${DEBIAN_VERSION}-slim AS node
# Ruby image to use for base image based on combined variables (ex: 3.3.x-slim-bookworm)
FROM docker.io/ruby:${RUBY_VERSION}-slim-${DEBIAN_VERSION} AS ruby

# Resulting version string is vX.X.X-MASTODON_VERSION_PRERELEASE+MASTODON_VERSION_METADATA
# Example: v4.3.0-nightly.2023.11.09+pr-123456
# Overwrite existence of 'alpha.X' in version.rb [--build-arg MASTODON_VERSION_PRERELEASE="nightly.2023.11.09"]
ARG MASTODON_VERSION_PRERELEASE=""
# Append build metadata or fork information to version.rb [--build-arg MASTODON_VERSION_METADATA="pr-123456"]
ARG MASTODON_VERSION_METADATA=""

# Allow Ruby on Rails to serve static files
# See: https://docs.joinmastodon.org/admin/config/#rails_serve_static_files
ARG RAILS_SERVE_STATIC_FILES="true"
# Allow to use YJIT compiler

```

### `36-ollama`

- [Full input](cases/36-ollama/input.dockerfile)
- [Full output](cases/36-ollama/output.dockerfile)
- [Input vs output diff](cases/36-ollama/compression.diff)

Input excerpt:

```text
# vim: filetype=dockerfile

ARG FLAVOR=${TARGETARCH}

ARG ROCMVERSION=6.3.3
ARG JETPACK5VERSION=r35.4.1
ARG JETPACK6VERSION=r36.4.0
ARG CMAKEVERSION=3.31.2

# CUDA v11 requires gcc v10.  v10.3 has regressions, so the rockylinux 8.5 AppStream has the latest compatible version
FROM --platform=linux/amd64 rocm/dev-almalinux-8:${ROCMVERSION}-complete AS base-amd64
RUN yum install -y yum-utils \
    && yum-config-manager --add-repo https://dl.rockylinux.org/vault/rocky/8.5/AppStream/\$basearch/os/ \
    && rpm --import https://dl.rockylinux.org/pub/rocky/RPM-GPG-KEY-Rocky-8 \
    && dnf install -y yum-utils ccache gcc-toolset-10-gcc-10.2.1-8.2.el8 gcc-toolset-10-gcc-c++-10.2.1-8.2.el8 gcc-toolset-10-binutils-2.35-11.el8 \
    && yum-config-manager --add-repo https://developer.download.nvidia.com/compute/cuda/repos/rhel8/x86_64/cuda-rhel8.repo
ENV PATH=/opt/rh/gcc-toolset-10/root/usr/bin:$PATH

FROM --platform=linux/arm64 almalinux:8 AS base-arm64
# install epel-release for ccache
RUN yum install -y yum-utils epel-release \
    && dnf install -y clang ccache \
    && yum-config-manager --add-repo https://developer.download.nvidia.com/compute/cuda/repos/rhel8/sbsa/cuda-rhel8.repo
ENV CC=clang CXX=clang++

FROM base-${TARGETARCH} AS base
ARG CMAKEVERSION
RUN curl -fsSL https://github.com/Kitware/CMake/releases/download/v${CMAKEVERSION}/cmake-${CMAKEVERSION}-linux-$(uname -m).tar.gz | tar xz -C /usr/local --strip-components 1
COPY CMakeLists.txt CMakePresets.json .
COPY ml/backend/ggml/ggml ml/backend/ggml/ggml
ENV LDFLAGS=-s

FROM base AS cpu
RUN dnf install -y gcc-toolset-11-gcc gcc-toolset-11-gcc-c++
ENV PATH=/opt/rh/gcc-toolset-11/root/usr/bin:$PATH
RUN --mount=type=cache,target=/root/.ccache \

```

Output excerpt:

```text
# vim: filetype=dockerfile

ARG FLAVOR=${TARGETARCH}

ARG ROCMVERSION=6.3.3
ARG JETPACK5VERSION=r35.4.1
ARG JETPACK6VERSION=r36.4.0
ARG CMAKEVERSION=3.31.2

# CUDA v11 requires gcc v10.  v10.3 has regressions, so the rockylinux 8.5 AppStream has the latest compatible version
FROM --platform=linux/amd64 rocm/dev-almalinux-8:${ROCMVERSION}-complete AS base-amd64
RUN yum install -y yum-utils \
    && yum-config-manager --add-repo https://dl.rockylinux.org/vault/rocky/8.5/AppStream/\$basearch/os/ \
    && rpm --import https://dl.rockylinux.org/pub/rocky/RPM-GPG-KEY-Rocky-8 \
    && dnf install -y yum-utils ccache gcc-toolset-10-gcc-10.2.1-8.2.el8 gcc-toolset-10-gcc-c++-10.2.1-8.2.el8 gcc-toolset-10-binutils-2.35-11.el8 \
    && yum-config-manager --add-repo https://developer.download.nvidia.com/compute/cuda/repos/rhel8/x86_64/cuda-rhel8.repo
ENV PATH=/opt/rh/gcc-toolset-10/root/usr/bin:$PATH

FROM --platform=linux/arm64 almalinux:8 AS base-arm64
# install epel-release for ccache
RUN yum install -y yum-utils epel-release \
    && dnf install -y clang ccache \
    && yum-config-manager --add-repo https://developer.download.nvidia.com/compute/cuda/repos/rhel8/sbsa/cuda-rhel8.repo
ENV CC=clang CXX=clang++

FROM base-${TARGETARCH} AS base
ARG CMAKEVERSION
RUN curl -fsSL https://github.com/Kitware/CMake/releases/download/v${CMAKEVERSION}/cmake-${CMAKEVERSION}-linux-$(uname -m).tar.gz | tar xz -C /usr/local --strip-components 1
COPY CMakeLists.txt CMakePresets.json .
COPY ml/backend/ggml/ggml ml/backend/ggml/ggml
ENV LDFLAGS=-s

FROM base AS cpu
RUN dnf install -y gcc-toolset-11-gcc gcc-toolset-11-gcc-c++
ENV PATH=/opt/rh/gcc-toolset-11/root/usr/bin:$PATH
RUN --mount=type=cache,target=/root/.ccache \

```

### `35-keycloak`

- [Full input](cases/35-keycloak/input.dockerfile)
- [Full output](cases/35-keycloak/output.dockerfile)
- [Input vs output diff](cases/35-keycloak/compression.diff)

Input excerpt:

```text
FROM registry.access.redhat.com/ubi9 AS ubi-micro-build

ARG KEYCLOAK_VERSION=999.0.0-SNAPSHOT
ARG KEYCLOAK_DIST=https://github.com/keycloak/keycloak/releases/download/$KEYCLOAK_VERSION/keycloak-$KEYCLOAK_VERSION.tar.gz

RUN dnf install -y tar gzip

ADD $KEYCLOAK_DIST /tmp/keycloak/

# The next step makes it uniform for local development and upstream built.
# If it is a local tar archive then it is unpacked, if from remote is just downloaded.
RUN (cd /tmp/keycloak && \
    tar -xvf /tmp/keycloak/keycloak-*.tar.gz && \
    rm /tmp/keycloak/keycloak-*.tar.gz) || true

RUN mv /tmp/keycloak/keycloak-* /opt/keycloak && mkdir -p /opt/keycloak/data
RUN chmod -R g+rwX /opt/keycloak

ADD ubi-null.sh /tmp/
RUN bash /tmp/ubi-null.sh java-21-openjdk-headless glibc-langpack-en findutils

FROM registry.access.redhat.com/ubi9-micro
ENV LANG en_US.UTF-8

# Flag for determining app is running in container
ENV KC_RUN_IN_CONTAINER true

COPY --from=ubi-micro-build /tmp/null/rootfs/ /
COPY --from=ubi-micro-build --chown=1000:0 /opt/keycloak /opt/keycloak

RUN echo "keycloak:x:0:root" >> /etc/group && \
    echo "keycloak:x:1000:0:keycloak user:/opt/keycloak:/sbin/nologin" >> /etc/passwd

USER 1000

EXPOSE 8080

```

Output excerpt:

```text
FROM registry.access.redhat.com/ubi9 AS ubi-micro-build

ARG KEYCLOAK_VERSION=999.0.0-SNAPSHOT
ARG KEYCLOAK_DIST=https://github.com/keycloak/keycloak/releases/download/$KEYCLOAK_VERSION/keycloak-$KEYCLOAK_VERSION.tar.gz

RUN dnf install -y tar gzip

ADD $KEYCLOAK_DIST /tmp/keycloak/

# The next step makes it uniform for local development and upstream built.
# If it is a local tar archive then it is unpacked, if from remote is just downloaded.
RUN (cd /tmp/keycloak && \
    tar -xvf /tmp/keycloak/keycloak-*.tar.gz && \
    rm /tmp/keycloak/keycloak-*.tar.gz) || true

RUN mv /tmp/keycloak/keycloak-* /opt/keycloak && mkdir -p /opt/keycloak/data
RUN chmod -R g+rwX /opt/keycloak

ADD ubi-null.sh /tmp/
RUN bash /tmp/ubi-null.sh java-21-openjdk-headless glibc-langpack-en findutils

FROM registry.access.redhat.com/ubi9-micro
ENV LANG en_US.UTF-8

# Flag for determining app is running in container
ENV KC_RUN_IN_CONTAINER true

COPY --from=ubi-micro-build /tmp/null/rootfs/ /
COPY --from=ubi-micro-build --chown=1000:0 /opt/keycloak /opt/keycloak

RUN echo "keycloak:x:0:root" >> /etc/group && \
    echo "keycloak:x:1000:0:keycloak user:/opt/keycloak:/sbin/nologin" >> /etc/passwd

USER 1000

EXPOSE 8080

```

### `34-gitea`

- [Full input](cases/34-gitea/input.dockerfile)
- [Full output](cases/34-gitea/output.dockerfile)
- [Input vs output diff](cases/34-gitea/compression.diff)

Input excerpt:

```text
# Build stage
FROM docker.io/library/golang:1.24-alpine3.22 AS build-env

ARG GOPROXY
ENV GOPROXY=${GOPROXY:-direct}

ARG GITEA_VERSION
ARG TAGS="sqlite sqlite_unlock_notify"
ENV TAGS="bindata timetzdata $TAGS"
ARG CGO_EXTRA_CFLAGS

# Build deps
RUN apk --no-cache add \
    build-base \
    git \
    nodejs \
    npm \
    && rm -rf /var/cache/apk/*

# Setup repo
COPY . ${GOPATH}/src/code.gitea.io/gitea
WORKDIR ${GOPATH}/src/code.gitea.io/gitea

# Checkout version if set
RUN if [ -n "${GITEA_VERSION}" ]; then git checkout "${GITEA_VERSION}"; fi \
 && make clean-all build

# Begin env-to-ini build
RUN go build contrib/environment-to-ini/environment-to-ini.go

# Copy local files
COPY docker/root /tmp/local

# Set permissions
RUN chmod 755 /tmp/local/usr/bin/entrypoint \
              /tmp/local/usr/local/bin/gitea \

```

Output excerpt:

```text
# Build stage
FROM docker.io/library/golang:1.24-alpine3.22 AS build-env

ARG GOPROXY
ENV GOPROXY=${GOPROXY:-direct}

ARG GITEA_VERSION
ARG TAGS="sqlite sqlite_unlock_notify"
ENV TAGS="bindata timetzdata $TAGS"
ARG CGO_EXTRA_CFLAGS

# Build deps
RUN apk --no-cache add \
    build-base \
    git \
    nodejs \
    npm \
    && rm -rf /var/cache/apk/*

# Setup repo
COPY . ${GOPATH}/src/code.gitea.io/gitea
WORKDIR ${GOPATH}/src/code.gitea.io/gitea

# Checkout version if set
RUN if [ -n "${GITEA_VERSION}" ]; then git checkout "${GITEA_VERSION}"; fi \
 && make clean-all build

# Begin env-to-ini build
RUN go build contrib/environment-to-ini/environment-to-ini.go

# Copy local files
COPY docker/root /tmp/local

# Set permissions
RUN chmod 755 /tmp/local/usr/bin/entrypoint \
              /tmp/local/usr/local/bin/gitea \

```

### `32-caddy`

- [Full input](cases/32-caddy/input.dockerfile)
- [Full output](cases/32-caddy/output.dockerfile)
- [Input vs output diff](cases/32-caddy/compression.diff)

Input excerpt:

```text
FROM alpine:3.23

RUN apk add --no-cache \
	ca-certificates \
	curl \
	libcap \
	mailcap

RUN set -eux; \
	mkdir -p \
		/config/caddy \
		/data/caddy \
		/etc/caddy \
		/usr/share/caddy \
	; \
	chmod 1777 /config/caddy /data/caddy; \
	wget -O /etc/caddy/Caddyfile "https://github.com/caddyserver/dist/raw/33ae08ff08d168572df2956ed14fbc4949880d94/config/Caddyfile"; \
	wget -O /usr/share/caddy/index.html "https://github.com/caddyserver/dist/raw/33ae08ff08d168572df2956ed14fbc4949880d94/welcome/index.html"

# https://github.com/caddyserver/caddy/releases
ENV CADDY_VERSION=v2.11.4

RUN set -eux; \
	apkArch="$(apk --print-arch)"; \
	case "$apkArch" in \
		x86_64)  binArch='amd64'; checksum='8220d1f013b6f27510247b2360c9e0ca9f018feebd82515f07635318b34ff9777ccc8fd0b6e6f2486ce3a33fe389fbb7db12d05baa474f4587509fb4f5ebf1c9' ;; \
		armhf)   binArch='armv6'; checksum='d4300f3e0d9af290bebd65721edb145025db680e6a7e6ad2ed917bf8c9d6e72abe788f720cdf046fe210ca1b43f9a7ddcda07a634296d3012527299ab78cc3ed' ;; \
		armv7)   binArch='armv7'; checksum='081959488f0d0da2725aea2d01330bc1501b3e52c1147b6c9da1da007524890ceba6845415355ba8f97aa29535e6f37521da1d7f10c5c63c55bb86c5d9f51bfb' ;; \
		aarch64) binArch='arm64'; checksum='d5a7c423853c24a799765e0e8210d5c7c22a8f56ed37a3cae2fb9f58be138853c02b4efd6b59d576e6d8c7c0d30b9c1592deeaa6a536ff69bcca23b8c1ea709c' ;; \
		ppc64el|ppc64le) binArch='ppc64le'; checksum='74b11a1f098517be7e6b11c7f7da6b5a0be9a18634f9a575c5ba6f5518a1bcbe4fe903f6c32a6d7ba47464fcaf12fde95e5f301cd829c4f41633484b0a4f817a' ;; \
		riscv64) binArch='riscv64'; checksum='05187bfc217a67ab32bbb0ebb9bd27ccbac0f80e41ad47dba272d10c34b1bf00f67a57ba05230eefa791898b2e2f40158631d81c3639cb514d59ac6d4976b044' ;; \
		s390x)   binArch='s390x'; checksum='fc2cd4294bf43e6645ee639532361c8e432a2cc7639134cfa0f0c44724146f4d9a278542ed56399e53aaac857b767d26ae748e754a790d106dd192c4c91c09f5' ;; \
		*) echo >&2 "error: unsupported architecture ($apkArch)"; exit 1 ;;\
	esac; \
	wget -O /tmp/caddy.tar.gz "https://github.com/caddyserver/caddy/releases/download/v2.11.4/caddy_2.11.4_linux_${binArch}.tar.gz"; \
	echo "$checksum  /tmp/caddy.tar.gz" | sha512sum -c; \

```

Output excerpt:

```text
FROM alpine:3.23

RUN apk add --no-cache \
	ca-certificates \
	curl \
	libcap \
	mailcap

RUN set -eux; \
	mkdir -p \
		/config/caddy \
		/data/caddy \
		/etc/caddy \
		/usr/share/caddy \
	; \
	chmod 1777 /config/caddy /data/caddy; \
	wget -O /etc/caddy/Caddyfile "https://github.com/caddyserver/dist/raw/33ae08ff08d168572df2956ed14fbc4949880d94/config/Caddyfile"; \
	wget -O /usr/share/caddy/index.html "https://github.com/caddyserver/dist/raw/33ae08ff08d168572df2956ed14fbc4949880d94/welcome/index.html"

# https://github.com/caddyserver/caddy/releases
ENV CADDY_VERSION=v2.11.4

RUN set -eux; \
	apkArch="$(apk --print-arch)"; \
	case "$apkArch" in \
		x86_64)  binArch='amd64'; checksum='8220d1f013b6f27510247b2360c9e0ca9f018feebd82515f07635318b34ff9777ccc8fd0b6e6f2486ce3a33fe389fbb7db12d05baa474f4587509fb4f5ebf1c9' ;; \
		armhf)   binArch='armv6'; checksum='d4300f3e0d9af290bebd65721edb145025db680e6a7e6ad2ed917bf8c9d6e72abe788f720cdf046fe210ca1b43f9a7ddcda07a634296d3012527299ab78cc3ed' ;; \
		armv7)   binArch='armv7'; checksum='081959488f0d0da2725aea2d01330bc1501b3e52c1147b6c9da1da007524890ceba6845415355ba8f97aa29535e6f37521da1d7f10c5c63c55bb86c5d9f51bfb' ;; \
		aarch64) binArch='arm64'; checksum='d5a7c423853c24a799765e0e8210d5c7c22a8f56ed37a3cae2fb9f58be138853c02b4efd6b59d576e6d8c7c0d30b9c1592deeaa6a536ff69bcca23b8c1ea709c' ;; \
		ppc64el|ppc64le) binArch='ppc64le'; checksum='74b11a1f098517be7e6b11c7f7da6b5a0be9a18634f9a575c5ba6f5518a1bcbe4fe903f6c32a6d7ba47464fcaf12fde95e5f301cd829c4f41633484b0a4f817a' ;; \
		riscv64) binArch='riscv64'; checksum='05187bfc217a67ab32bbb0ebb9bd27ccbac0f80e41ad47dba272d10c34b1bf00f67a57ba05230eefa791898b2e2f40158631d81c3639cb514d59ac6d4976b044' ;; \
		s390x)   binArch='s390x'; checksum='fc2cd4294bf43e6645ee639532361c8e432a2cc7639134cfa0f0c44724146f4d9a278542ed56399e53aaac857b767d26ae748e754a790d106dd192c4c91c09f5' ;; \
		*) echo >&2 "error: unsupported architecture ($apkArch)"; exit 1 ;;\
	esac; \
	wget -O /tmp/caddy.tar.gz "https://github.com/caddyserver/caddy/releases/download/v2.11.4/caddy_2.11.4_linux_${binArch}.tar.gz"; \
	echo "$checksum  /tmp/caddy.tar.gz" | sha512sum -c; \

```

### `31-julia`

- [Full input](cases/31-julia/input.dockerfile)
- [Full output](cases/31-julia/output.dockerfile)
- [Input vs output diff](cases/31-julia/compression.diff)

Input excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM debian:bookworm-slim

RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		ca-certificates \
# ERROR: no download agent available; install curl, wget, or fetch
		curl \
	; \
	rm -rf /var/lib/apt/lists/*

ENV JULIA_PATH /usr/local/julia
ENV PATH $JULIA_PATH/bin:$PATH

# https://julialang.org/juliareleases.asc
# Julia (Binary signing key) <buildbot@julialang.org>
ENV JULIA_GPG 3673DF529D9049477F76B37566E3C7DC03D6E495

# https://julialang.org/downloads/
ENV JULIA_VERSION 1.11.9

RUN set -eux; \
	\
	savedAptMark="$(apt-mark showmanual)"; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		gnupg \
	; \
	rm -rf /var/lib/apt/lists/*; \
	\

```

Output excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM debian:bookworm-slim

RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		ca-certificates \
# ERROR: no download agent available; install curl, wget, or fetch
		curl \
	; \
	rm -rf /var/lib/apt/lists/*

ENV JULIA_PATH /usr/local/julia
ENV PATH $JULIA_PATH/bin:$PATH

# https://julialang.org/juliareleases.asc
# Julia (Binary signing key) <buildbot@julialang.org>
ENV JULIA_GPG 3673DF529D9049477F76B37566E3C7DC03D6E495

# https://julialang.org/downloads/
ENV JULIA_VERSION 1.11.9

RUN set -eux; \
	\
	savedAptMark="$(apt-mark showmanual)"; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		gnupg \
	; \
	rm -rf /var/lib/apt/lists/*; \
	\

```

### `30-influxdb`

- [Full input](cases/30-influxdb/input.dockerfile)
- [Full output](cases/30-influxdb/output.dockerfile)
- [Input vs output diff](cases/30-influxdb/compression.diff)

Input excerpt:

```text
FROM debian:bookworm-slim

RUN export DEBIAN_FRONTEND=noninteractive && \
    # install dependencies \
    apt-get update -y && \
    apt-get install -y --no-install-recommends \
      ca-certificates \
      curl \
      gnupg && \
    # cleanup apt \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install dasel for configuration parsing
RUN case "$(dpkg --print-architecture)" in \
      *amd64) arch=amd64 ;; \
      *arm64) arch=arm64 ;; \
      *) echo 'Unsupported architecture' && exit 1 ;; \
    esac && \
    curl -fL "https://github.com/TomWright/dasel/releases/download/v2.8.1/dasel_linux_${arch}.gz" | gzip -d > /usr/local/bin/dasel && \
    case ${arch} in \
      amd64) echo '21fda0a4dc3c779c42737eca4b37e4f187d7ab91ba6301eed97b801af84a9ea2  /usr/local/bin/dasel' ;; \
      arm64) echo '2c75e63f9884c37578f48788819dda5a5a5c32ec6c4a663eefc19839f44d6291  /usr/local/bin/dasel' ;; \
    esac | sha256sum -c - && \
    chmod +x /usr/local/bin/dasel && \
    dasel --version

RUN groupadd -r influxdb --gid=1000 && \
    useradd -r -g influxdb --uid=1000 --create-home --home-dir=/home/influxdb --shell=/bin/bash influxdb

# Install gosu for easy step-down from root
ENV GOSU_VER 1.16
RUN case "$(dpkg --print-architecture)" in \
      *amd64) arch=amd64 ;; \
      *arm64) arch=arm64 ;; \
      *) echo 'Unsupported architecture' && exit 1 ;; \

```

Output excerpt:

```text
FROM debian:bookworm-slim

RUN export DEBIAN_FRONTEND=noninteractive && \
    # install dependencies \
    apt-get update -y && \
    apt-get install -y --no-install-recommends \
      ca-certificates \
      curl \
      gnupg && \
    # cleanup apt \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install dasel for configuration parsing
RUN case "$(dpkg --print-architecture)" in \
      *amd64) arch=amd64 ;; \
      *arm64) arch=arm64 ;; \
      *) echo 'Unsupported architecture' && exit 1 ;; \
    esac && \
    curl -fL "https://github.com/TomWright/dasel/releases/download/v2.8.1/dasel_linux_${arch}.gz" | gzip -d > /usr/local/bin/dasel && \
    case ${arch} in \
      amd64) echo '21fda0a4dc3c779c42737eca4b37e4f187d7ab91ba6301eed97b801af84a9ea2  /usr/local/bin/dasel' ;; \
      arm64) echo '2c75e63f9884c37578f48788819dda5a5a5c32ec6c4a663eefc19839f44d6291  /usr/local/bin/dasel' ;; \
    esac | sha256sum -c - && \
    chmod +x /usr/local/bin/dasel && \
    dasel --version

RUN groupadd -r influxdb --gid=1000 && \
    useradd -r -g influxdb --uid=1000 --create-home --home-dir=/home/influxdb --shell=/bin/bash influxdb

# Install gosu for easy step-down from root
ENV GOSU_VER 1.16
RUN case "$(dpkg --print-architecture)" in \
      *amd64) arch=amd64 ;; \
      *arm64) arch=arm64 ;; \
      *) echo 'Unsupported architecture' && exit 1 ;; \

```

### `29-nextjs-example`

- [Full input](cases/29-nextjs-example/input.dockerfile)
- [Full output](cases/29-nextjs-example/output.dockerfile)
- [Input vs output diff](cases/29-nextjs-example/compression.diff)

Input excerpt:

```text
# ============================================
# Stage 1: Dependencies Installation Stage
# ============================================

# IMPORTANT: Node.js Version Maintenance
# This Dockerfile uses Node.js 24.13.0-slim, which was the latest LTS version at the time of writing.
# To ensure security and compatibility, regularly update the NODE_VERSION ARG to the latest LTS version.
ARG NODE_VERSION=24.13.0-slim

FROM node:${NODE_VERSION} AS dependencies

# Set working directory
WORKDIR /app

# Copy package-related files first to leverage Docker's caching mechanism
COPY package.json yarn.lock* package-lock.json* pnpm-lock.yaml* .npmrc* ./

# Install project dependencies with frozen lockfile for reproducible builds
RUN --mount=type=cache,target=/root/.npm \
    --mount=type=cache,target=/usr/local/share/.cache/yarn \
    --mount=type=cache,target=/root/.local/share/pnpm/store \
  if [ -f package-lock.json ]; then \
    npm ci --no-audit --no-fund; \
  elif [ -f yarn.lock ]; then \
    corepack enable yarn && yarn install --frozen-lockfile --production=false; \
  elif [ -f pnpm-lock.yaml ]; then \
    corepack enable pnpm && pnpm install --frozen-lockfile; \
  else \
    echo "No lockfile found." && exit 1; \
  fi

# ============================================
# Stage 2: Build Next.js application in standalone mode
# ============================================

FROM node:${NODE_VERSION} AS builder

```

Output excerpt:

```text
# ============================================
# Stage 1: Dependencies Installation Stage
# ============================================

# IMPORTANT: Node.js Version Maintenance
# This Dockerfile uses Node.js 24.13.0-slim, which was the latest LTS version at the time of writing.
# To ensure security and compatibility, regularly update the NODE_VERSION ARG to the latest LTS version.
ARG NODE_VERSION=24.13.0-slim

FROM node:${NODE_VERSION} AS dependencies

# Set working directory
WORKDIR /app

# Copy package-related files first to leverage Docker's caching mechanism
COPY package.json yarn.lock* package-lock.json* pnpm-lock.yaml* .npmrc* ./

# Install project dependencies with frozen lockfile for reproducible builds
RUN --mount=type=cache,target=/root/.npm \
    --mount=type=cache,target=/usr/local/share/.cache/yarn \
    --mount=type=cache,target=/root/.local/share/pnpm/store \
  if [ -f package-lock.json ]; then \
    npm ci --no-audit --no-fund; \
  elif [ -f yarn.lock ]; then \
    corepack enable yarn && yarn install --frozen-lockfile --production=false; \
  elif [ -f pnpm-lock.yaml ]; then \
    corepack enable pnpm && pnpm install --frozen-lockfile; \
  else \
    echo "No lockfile found." && exit 1; \
  fi

# ============================================
# Stage 2: Build Next.js application in standalone mode
# ============================================

FROM node:${NODE_VERSION} AS builder

```

### `28-airflow`

- [Full input](cases/28-airflow/input.dockerfile)
- [Full output](cases/28-airflow/output.dockerfile)
- [Input vs output diff](cases/28-airflow/compression.diff)

Input excerpt:

```text
# syntax=docker/dockerfile:1.4
# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.
#
# THIS DOCKERFILE IS INTENDED FOR PRODUCTION USE AND DEPLOYMENT.
# NOTE! IT IS ALPHA-QUALITY FOR NOW - WE ARE IN A PROCESS OF TESTING IT
#
#
# This is a multi-segmented image. It actually contains two images:
#
# airflow-build-image  - there all airflow dependencies can be installed (and
#                        built - for those dependencies that require
#                        build essentials). Airflow is installed there with
#                        ${HOME}/.local virtualenv which is also considered
#                        As --user folder by python when creating venv with
#                        --system-site-packages
#
# main                 - this is the actual production image that is much
#                        smaller because it does not contain all the build
#                        essentials. Instead the ${HOME}/.local folder
#                        is copied from the build-image - this way we have
#                        only result of installation and we do not need

```

Output excerpt:

```text
# syntax=docker/dockerfile:1.4
# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.
#
# THIS DOCKERFILE IS INTENDED FOR PRODUCTION USE AND DEPLOYMENT.
# NOTE! IT IS ALPHA-QUALITY FOR NOW - WE ARE IN A PROCESS OF TESTING IT
#
#
# This is a multi-segmented image. It actually contains two images:
#
# airflow-build-image  - there all airflow dependencies can be installed (and
#                        built - for those dependencies that require
#                        build essentials). Airflow is installed there with
#                        ${HOME}/.local virtualenv which is also considered
#                        As --user folder by python when creating venv with
#                        --system-site-packages
#
# main                 - this is the actual production image that is much
#                        smaller because it does not contain all the build
#                        essentials. Instead the ${HOME}/.local folder
#                        is copied from the build-image - this way we have
#                        only result of installation and we do not need

```

### `27-sentry-self-hosted`

- [Full input](cases/27-sentry-self-hosted/input.dockerfile)
- [Full output](cases/27-sentry-self-hosted/output.dockerfile)
- [Input vs output diff](cases/27-sentry-self-hosted/compression.diff)

Input excerpt:

```text
FROM scratch AS odiff-amd64
ADD --chmod=755 \
    --checksum=sha256:2c17e6bcf92a58e6668f19f17f4a27fa4b1d70840994f31bd837b55bb6b297d7 \
    https://github.com/dmtrKovalenko/odiff/releases/download/v4.3.2/odiff-linux-x64 \
    /odiff

FROM scratch AS odiff-arm64
ADD --chmod=755 \
    --checksum=sha256:d65f748c463a6aa78fa7bcdd31acd797eaed5160867e7769a3b291cfea42c9a0 \
    https://github.com/dmtrKovalenko/odiff/releases/download/v4.3.2/odiff-linux-arm64 \
    /odiff

ARG TARGETARCH
FROM odiff-${TARGETARCH} AS odiff

FROM python:3.13.1-slim-bookworm

LABEL maintainer="oss@sentry.io"
LABEL org.opencontainers.image.title="Sentry"
LABEL org.opencontainers.image.description="Sentry runtime image"
LABEL org.opencontainers.image.url="https://sentry.io/"
LABEL org.opencontainers.image.documentation="https://develop.sentry.dev/self-hosted/"
LABEL org.opencontainers.image.vendor="Functional Software, Inc."
LABEL org.opencontainers.image.authors="oss@sentry.io"

# add our user and group first to make sure their IDs get assigned consistently
RUN groupadd -r sentry --gid 999 && useradd -r -m -g sentry --uid 999 sentry

RUN : \
    && apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        gosu \
        libexpat1 \
        tini \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

```

Output excerpt:

```text
FROM scratch AS odiff-amd64
ADD --chmod=755 \
    --checksum=sha256:2c17e6bcf92a58e6668f19f17f4a27fa4b1d70840994f31bd837b55bb6b297d7 \
    https://github.com/dmtrKovalenko/odiff/releases/download/v4.3.2/odiff-linux-x64 \
    /odiff

FROM scratch AS odiff-arm64
ADD --chmod=755 \
    --checksum=sha256:d65f748c463a6aa78fa7bcdd31acd797eaed5160867e7769a3b291cfea42c9a0 \
    https://github.com/dmtrKovalenko/odiff/releases/download/v4.3.2/odiff-linux-arm64 \
    /odiff

ARG TARGETARCH
FROM odiff-${TARGETARCH} AS odiff

FROM python:3.13.1-slim-bookworm

LABEL maintainer="oss@sentry.io"
LABEL org.opencontainers.image.title="Sentry"
LABEL org.opencontainers.image.description="Sentry runtime image"
LABEL org.opencontainers.image.url="https://sentry.io/"
LABEL org.opencontainers.image.documentation="https://develop.sentry.dev/self-hosted/"
LABEL org.opencontainers.image.vendor="Functional Software, Inc."
LABEL org.opencontainers.image.authors="oss@sentry.io"

# add our user and group first to make sure their IDs get assigned consistently
RUN groupadd -r sentry --gid 999 && useradd -r -m -g sentry --uid 999 sentry

RUN : \
    && apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        gosu \
        libexpat1 \
        tini \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

```

### `26-busybox-builder`

- [Full input](cases/26-busybox-builder/input.dockerfile)
- [Full output](cases/26-busybox-builder/output.dockerfile)
- [Input vs output diff](cases/26-busybox-builder/compression.diff)

Input excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM alpine:3.23

RUN set -eux; \
	apk add --no-cache \
		bzip2 \
		coreutils \
		curl \
		gcc \
		gnupg \
		linux-headers \
		make \
		musl-dev \
		patch \
		tzdata \
# busybox's tar ironically does not maintain mtime of directories correctly (which we need for SOURCE_DATE_EPOCH / reproducibility)
		tar \
	;

# pub   1024D/ACC9965B 2006-12-12
#       Key fingerprint = C9E9 416F 76E6 10DB D09D  040F 47B7 0C55 ACC9 965B
# uid                  Denis Vlasenko <vda.linux@googlemail.com>
# sub   1024g/2C766641 2006-12-12
RUN mkdir -p ~/.gnupg && gpg --batch --keyserver keyserver.ubuntu.com --recv-keys C9E9416F76E610DBD09D040F47B70C55ACC9965B

# https://busybox.net: 13 May 2026
ENV BUSYBOX_VERSION 1.38.0
ENV BUSYBOX_SHA256 34f9ea6ff8636f2c9241153b9114eefa9e65674a45318ae1ef95bb5f31c53bb2

RUN set -eux; \
	tarball="busybox-${BUSYBOX_VERSION}.tar.bz2"; \

```

Output excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM alpine:3.23

RUN set -eux; \
	apk add --no-cache \
		bzip2 \
		coreutils \
		curl \
		gcc \
		gnupg \
		linux-headers \
		make \
		musl-dev \
		patch \
		tzdata \
# busybox's tar ironically does not maintain mtime of directories correctly (which we need for SOURCE_DATE_EPOCH / reproducibility)
		tar \
	;

# pub   1024D/ACC9965B 2006-12-12
#       Key fingerprint = C9E9 416F 76E6 10DB D09D  040F 47B7 0C55 ACC9 965B
# uid                  Denis Vlasenko <vda.linux@googlemail.com>
# sub   1024g/2C766641 2006-12-12
RUN mkdir -p ~/.gnupg && gpg --batch --keyserver keyserver.ubuntu.com --recv-keys C9E9416F76E610DBD09D040F47B70C55ACC9965B

# https://busybox.net: 13 May 2026
ENV BUSYBOX_VERSION 1.38.0
ENV BUSYBOX_SHA256 34f9ea6ff8636f2c9241153b9114eefa9e65674a45318ae1ef95bb5f31c53bb2

RUN set -eux; \
	tarball="busybox-${BUSYBOX_VERSION}.tar.bz2"; \

```

### `25-traefik`

- [Full input](cases/25-traefik/input.dockerfile)
- [Full output](cases/25-traefik/output.dockerfile)
- [Input vs output diff](cases/25-traefik/compression.diff)

Input excerpt:

```text
# syntax=docker/dockerfile:1.2
FROM alpine:3.24

RUN apk add --no-cache --no-progress ca-certificates tzdata

ARG TARGETPLATFORM
COPY ./dist/$TARGETPLATFORM/traefik /

EXPOSE 80
VOLUME ["/tmp"]

ENTRYPOINT ["/traefik"]

```

Output excerpt:

```text
# syntax=docker/dockerfile:1.2
FROM alpine:3.24

RUN apk add --no-cache --no-progress ca-certificates tzdata

ARG TARGETPLATFORM
COPY ./dist/$TARGETPLATFORM/traefik /

EXPOSE 80
VOLUME ["/tmp"]

ENTRYPOINT ["/traefik"]

```

### `24-node`

- [Full input](cases/24-node/input.dockerfile)
- [Full output](cases/24-node/output.dockerfile)
- [Input vs output diff](cases/24-node/compression.diff)

Input excerpt:

```text
FROM buildpack-deps:bookworm

RUN groupadd --gid 1000 node \
  && useradd --uid 1000 --gid node --shell /bin/bash --create-home node

ENV NODE_VERSION=22.23.1

RUN ARCH= && dpkgArch="$(dpkg --print-architecture)" \
  && case "${dpkgArch##*-}" in \
    amd64) ARCH='x64';; \
    ppc64el) ARCH='ppc64le';; \
    s390x) ARCH='s390x';; \
    arm64) ARCH='arm64';; \
    armhf) ARCH='armv7l';; \
    *) echo "unsupported architecture"; exit 1 ;; \
  esac \
  # use pre-existing gpg directory, see https://github.com/nodejs/docker-node/pull/1895#issuecomment-1550389150
  && export GNUPGHOME="$(mktemp -d)" \
  # gpg keys listed at https://github.com/nodejs/node#release-keys
  && set -ex \
  && for key in \
    5BE8A3F6C8A5C01D106C0AD820B1A390B168D356 \
    DD792F5973C6DE52C432CBDAC77ABFA00DDBF2B7 \
    CC68F5A3106FF448322E48ED27F5E38D5B0A215F \
    8FCCA13FEF1D0C2E91008E09770F7A9A5AE15600 \
    890C08DB8579162FEE0DF9DB8BEAB4DFCF555EF4 \
    C82FA3AE1CBEDC6BE46B9360C43CEC45C17AB93C \
    108F52B48DB57BB0CC439B2997B01419BD92F80A \
    A363A499291CBBC940DD62E41F10027AF002F8B0 \
  ; do \
      { gpg --batch --keyserver hkps://keys.openpgp.org --recv-keys "$key" && gpg --batch --fingerprint "$key"; } || \
      { gpg --batch --keyserver keyserver.ubuntu.com --recv-keys "$key" && gpg --batch --fingerprint "$key"; } ; \
  done \
  && curl -fsSLO --compressed "https://nodejs.org/dist/v$NODE_VERSION/node-v$NODE_VERSION-linux-$ARCH.tar.xz" \
  && curl -fsSLO --compressed "https://nodejs.org/dist/v$NODE_VERSION/SHASUMS256.txt.asc" \
  && gpg --batch --decrypt --output SHASUMS256.txt SHASUMS256.txt.asc \

```

Output excerpt:

```text
FROM buildpack-deps:bookworm

RUN groupadd --gid 1000 node \
  && useradd --uid 1000 --gid node --shell /bin/bash --create-home node

ENV NODE_VERSION=22.23.1

RUN ARCH= && dpkgArch="$(dpkg --print-architecture)" \
  && case "${dpkgArch##*-}" in \
    amd64) ARCH='x64';; \
    ppc64el) ARCH='ppc64le';; \
    s390x) ARCH='s390x';; \
    arm64) ARCH='arm64';; \
    armhf) ARCH='armv7l';; \
    *) echo "unsupported architecture"; exit 1 ;; \
  esac \
  # use pre-existing gpg directory, see https://github.com/nodejs/docker-node/pull/1895#issuecomment-1550389150
  && export GNUPGHOME="$(mktemp -d)" \
  # gpg keys listed at https://github.com/nodejs/node#release-keys
  && set -ex \
  && for key in \
    5BE8A3F6C8A5C01D106C0AD820B1A390B168D356 \
    DD792F5973C6DE52C432CBDAC77ABFA00DDBF2B7 \
    CC68F5A3106FF448322E48ED27F5E38D5B0A215F \
    8FCCA13FEF1D0C2E91008E09770F7A9A5AE15600 \
    890C08DB8579162FEE0DF9DB8BEAB4DFCF555EF4 \
    C82FA3AE1CBEDC6BE46B9360C43CEC45C17AB93C \
    108F52B48DB57BB0CC439B2997B01419BD92F80A \
    A363A499291CBBC940DD62E41F10027AF002F8B0 \
  ; do \
      { gpg --batch --keyserver hkps://keys.openpgp.org --recv-keys "$key" && gpg --batch --fingerprint "$key"; } || \
      { gpg --batch --keyserver keyserver.ubuntu.com --recv-keys "$key" && gpg --batch --fingerprint "$key"; } ; \
  done \
  && curl -fsSLO --compressed "https://nodejs.org/dist/v$NODE_VERSION/node-v$NODE_VERSION-linux-$ARCH.tar.xz" \
  && curl -fsSLO --compressed "https://nodejs.org/dist/v$NODE_VERSION/SHASUMS256.txt.asc" \
  && gpg --batch --decrypt --output SHASUMS256.txt SHASUMS256.txt.asc \

```

### `23-elasticsearch`

- [Full input](cases/23-elasticsearch/input.dockerfile)
- [Full output](cases/23-elasticsearch/output.dockerfile)
- [Input vs output diff](cases/23-elasticsearch/compression.diff)

Input excerpt:

```text
# Elasticsearch 8.11.2

# This image re-bundles the Docker image from the upstream provider, Elastic.
FROM docker.elastic.co/elasticsearch/elasticsearch:8.11.2@sha256:e40b9d3d523f2fe4dc851ad2cc5570f28a58ca6c4efb566cc9688dcaf0df8dec
# Supported Bashbrew Architectures: amd64 arm64v8

# The upstream image was built by:
#   https://github.com/elastic/dockerfiles/tree/v8.11.2/elasticsearch

# The build can be reproduced locally via:
#   docker build 'https://github.com/elastic/dockerfiles.git#v8.11.2:elasticsearch'

# For a full list of supported images and tags visit https://www.docker.elastic.co

# For Elasticsearch documentation visit https://www.elastic.co/guide/en/elasticsearch/reference/current/docker.html

# See https://github.com/docker-library/official-images/pull/4916 for more details.

```

Output excerpt:

```text
# Elasticsearch 8.11.2

# This image re-bundles the Docker image from the upstream provider, Elastic.
FROM docker.elastic.co/elasticsearch/elasticsearch:8.11.2@sha256:e40b9d3d523f2fe4dc851ad2cc5570f28a58ca6c4efb566cc9688dcaf0df8dec
# Supported Bashbrew Architectures: amd64 arm64v8

# The upstream image was built by:
#   https://github.com/elastic/dockerfiles/tree/v8.11.2/elasticsearch

# The build can be reproduced locally via:
#   docker build 'https://github.com/elastic/dockerfiles.git#v8.11.2:elasticsearch'

# For a full list of supported images and tags visit https://www.docker.elastic.co

# For Elasticsearch documentation visit https://www.elastic.co/guide/en/elasticsearch/reference/current/docker.html

# See https://github.com/docker-library/official-images/pull/4916 for more details.

```

### `22-moby`

- [Full input](cases/22-moby/input.dockerfile)
- [Full output](cases/22-moby/output.dockerfile)
- [Input vs output diff](cases/22-moby/compression.diff)

Input excerpt:

```text
# syntax=docker/dockerfile:1

ARG GO_VERSION=1.26.4
ARG BASE_DEBIAN_DISTRO="bookworm"
ARG GOLANG_IMAGE="golang:${GO_VERSION}-${BASE_DEBIAN_DISTRO}"

# XX_VERSION specifies the version of the xx utility to use.
# It must be a valid tag in the docker.io/tonistiigi/xx image repository.
ARG XX_VERSION=1.9.0

# DOCKERCLI_VERSION is the version of the CLI to install in the dev-container.
ARG DOCKERCLI_VERSION=v29.6.0
ARG DOCKERCLI_REPOSITORY="https://github.com/docker/cli.git"

# cli version used for integration-cli tests
ARG DOCKERCLI_INTEGRATION_REPOSITORY="https://github.com/docker/cli.git"
ARG DOCKERCLI_INTEGRATION_VERSION=v25.0.5

# BUILDX_VERSION is the version of buildx to install in the dev container.
ARG BUILDX_VERSION=0.35.0

# COMPOSE_VERSION is the version of compose to install in the dev container.
ARG COMPOSE_VERSION=v5.1.4

ARG SYSTEMD="false"
ARG FIREWALLD="false"
ARG DOCKER_STATIC=1

# REGISTRY_VERSION is the version of the registry to use for integration tests.
# It must be a valid tag in the docker.io/library/registry image repository.
ARG REGISTRY_VERSION=3.1.1

# delve is currently only supported on linux/amd64, linux/arm64, and linux/ppc64le;
# https://github.com/go-delve/delve/blob/v1.26.0/pkg/proc/native/support_sentinel.go#L1
# https://github.com/go-delve/delve/blob/v1.26.0/pkg/proc/native/support_sentinel_linux.go#L1
#

```

Output excerpt:

```text
# syntax=docker/dockerfile:1

ARG GO_VERSION=1.26.4
ARG BASE_DEBIAN_DISTRO="bookworm"
ARG GOLANG_IMAGE="golang:${GO_VERSION}-${BASE_DEBIAN_DISTRO}"

# XX_VERSION specifies the version of the xx utility to use.
# It must be a valid tag in the docker.io/tonistiigi/xx image repository.
ARG XX_VERSION=1.9.0

# DOCKERCLI_VERSION is the version of the CLI to install in the dev-container.
ARG DOCKERCLI_VERSION=v29.6.0
ARG DOCKERCLI_REPOSITORY="https://github.com/docker/cli.git"

# cli version used for integration-cli tests
ARG DOCKERCLI_INTEGRATION_REPOSITORY="https://github.com/docker/cli.git"
ARG DOCKERCLI_INTEGRATION_VERSION=v25.0.5

# BUILDX_VERSION is the version of buildx to install in the dev container.
ARG BUILDX_VERSION=0.35.0

# COMPOSE_VERSION is the version of compose to install in the dev container.
ARG COMPOSE_VERSION=v5.1.4

ARG SYSTEMD="false"
ARG FIREWALLD="false"
ARG DOCKER_STATIC=1

# REGISTRY_VERSION is the version of the registry to use for integration tests.
# It must be a valid tag in the docker.io/library/registry image repository.
ARG REGISTRY_VERSION=3.1.1

# delve is currently only supported on linux/amd64, linux/arm64, and linux/ppc64le;
# https://github.com/go-delve/delve/blob/v1.26.0/pkg/proc/native/support_sentinel.go#L1
# https://github.com/go-delve/delve/blob/v1.26.0/pkg/proc/native/support_sentinel_linux.go#L1
#

```

### `21-kube-apiserver`

- [Full input](cases/21-kube-apiserver/input.dockerfile)
- [Full output](cases/21-kube-apiserver/output.dockerfile)
- [Input vs output diff](cases/21-kube-apiserver/compression.diff)

Input excerpt:

```text
# Copyright 2021 The Kubernetes Authors.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# This file create the kube-apiserver image.
ARG BASEIMAGE
ARG SETCAP_IMAGE

# we use the hosts platform to apply the capabilities to avoid the need
# to setup qemu for the builder.
FROM --platform=linux/$BUILDARCH ${SETCAP_IMAGE}
ARG BINARY
COPY --chmod=755 ${BINARY} /binary_to_set
# We apply cap_net_bind_service so that kube-apiserver can be run as
# non-root and still listen on port less than 1024
RUN ["setcap", "cap_net_bind_service=+ep", "/binary_to_set"]

FROM --platform=linux/$TARGETARCH ${BASEIMAGE}
ARG BINARY

COPY --from=0 /binary_to_set /usr/local/bin/${BINARY}
# NOTE: /go-runner is used here for compatibility
COPY --chmod=755 kube-log-runner /go-runner
ENTRYPOINT ["/go-runner"]

```

Output excerpt:

```text
# Copyright 2021 The Kubernetes Authors.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# This file create the kube-apiserver image.
ARG BASEIMAGE
ARG SETCAP_IMAGE

# we use the hosts platform to apply the capabilities to avoid the need
# to setup qemu for the builder.
FROM --platform=linux/$BUILDARCH ${SETCAP_IMAGE}
ARG BINARY
COPY --chmod=755 ${BINARY} /binary_to_set
# We apply cap_net_bind_service so that kube-apiserver can be run as
# non-root and still listen on port less than 1024
RUN ["setcap", "cap_net_bind_service=+ep", "/binary_to_set"]

FROM --platform=linux/$TARGETARCH ${BASEIMAGE}
ARG BINARY

COPY --from=0 /binary_to_set /usr/local/bin/${BINARY}
# NOTE: /go-runner is used here for compatibility
COPY --chmod=755 kube-log-runner /go-runner
ENTRYPOINT ["/go-runner"]

```

### `20-prometheus`

- [Full input](cases/20-prometheus/input.dockerfile)
- [Full output](cases/20-prometheus/output.dockerfile)
- [Input vs output diff](cases/20-prometheus/compression.diff)

Input excerpt:

```text
ARG ARCH="amd64"
ARG OS="linux"
FROM quay.io/prometheus/busybox-${OS}-${ARCH}:latest
LABEL maintainer="The Prometheus Authors <prometheus-developers@googlegroups.com>"
LABEL org.opencontainers.image.authors="The Prometheus Authors" \
      org.opencontainers.image.vendor="Prometheus" \
      org.opencontainers.image.title="Prometheus" \
      org.opencontainers.image.description="The Prometheus monitoring system and time series database" \
      org.opencontainers.image.source="https://github.com/prometheus/prometheus" \
      org.opencontainers.image.url="https://github.com/prometheus/prometheus" \
      org.opencontainers.image.documentation="https://prometheus.io/docs" \
      org.opencontainers.image.licenses="Apache License 2.0" \
      io.prometheus.image.variant="busybox"

ARG ARCH="amd64"
ARG OS="linux"
COPY .build/${OS}-${ARCH}/prometheus        /bin/prometheus
COPY .build/${OS}-${ARCH}/promtool          /bin/promtool
COPY documentation/examples/prometheus.yml  /etc/prometheus/prometheus.yml
COPY LICENSE                                /LICENSE
COPY NOTICE                                 /NOTICE

WORKDIR /prometheus
RUN chown -R nobody:nobody /etc/prometheus /prometheus && chmod g+w /prometheus

USER       nobody
EXPOSE     9090
VOLUME     [ "/prometheus" ]
ENTRYPOINT [ "/bin/prometheus" ]
CMD        [ "--config.file=/etc/prometheus/prometheus.yml", \
             "--storage.tsdb.path=/prometheus" ]

```

Output excerpt:

```text
ARG ARCH="amd64"
ARG OS="linux"
FROM quay.io/prometheus/busybox-${OS}-${ARCH}:latest
LABEL maintainer="The Prometheus Authors <prometheus-developers@googlegroups.com>"
LABEL org.opencontainers.image.authors="The Prometheus Authors" \
      org.opencontainers.image.vendor="Prometheus" \
      org.opencontainers.image.title="Prometheus" \
      org.opencontainers.image.description="The Prometheus monitoring system and time series database" \
      org.opencontainers.image.source="https://github.com/prometheus/prometheus" \
      org.opencontainers.image.url="https://github.com/prometheus/prometheus" \
      org.opencontainers.image.documentation="https://prometheus.io/docs" \
      org.opencontainers.image.licenses="Apache License 2.0" \
      io.prometheus.image.variant="busybox"

ARG ARCH="amd64"
ARG OS="linux"
COPY .build/${OS}-${ARCH}/prometheus        /bin/prometheus
COPY .build/${OS}-${ARCH}/promtool          /bin/promtool
COPY documentation/examples/prometheus.yml  /etc/prometheus/prometheus.yml
COPY LICENSE                                /LICENSE
COPY NOTICE                                 /NOTICE

WORKDIR /prometheus
RUN chown -R nobody:nobody /etc/prometheus /prometheus && chmod g+w /prometheus

USER       nobody
EXPOSE     9090
VOLUME     [ "/prometheus" ]
ENTRYPOINT [ "/bin/prometheus" ]
CMD        [ "--config.file=/etc/prometheus/prometheus.yml", \
             "--storage.tsdb.path=/prometheus" ]

```

### `19-grafana`

- [Full input](cases/19-grafana/input.dockerfile)
- [Full output](cases/19-grafana/output.dockerfile)
- [Input vs output diff](cases/19-grafana/compression.diff)

Input excerpt:

```text
# syntax=docker/dockerfile:1.7-labs

# to maintain formatting of multiline commands in vscode, add the following to settings.json:
# "docker.languageserver.formatter.ignoreMultilineInstructions": true

ARG GO_IMAGE=go-builder-base
ARG JS_IMAGE=js-builder-base
ARG JS_PLATFORM=linux/amd64

# Default to building locally
ARG GO_SRC=go-builder
ARG JS_SRC=js-builder

# Dependabot cannot update dependencies listed in ARGs
# By using FROM instructions we can delegate dependency updates to dependabot
FROM alpine:3.24.1 AS alpine-base
FROM ubuntu:24.04 AS ubuntu-base
FROM golang:1.26.4-alpine AS go-builder-base
FROM --platform=${JS_PLATFORM} node:24-alpine AS js-builder-base
FROM gcr.io/distroless/static-debian13 AS distroless-base
# Javascript build stage
FROM --platform=${JS_PLATFORM} ${JS_IMAGE} AS js-builder
ARG JS_NODE_ENV=production
ARG JS_YARN_INSTALL_FLAG=--immutable
ARG JS_YARN_BUILD_FLAG=build

ENV NODE_OPTIONS=--max_old_space_size=8000

WORKDIR /tmp/grafana

RUN apk add --no-cache make build-base python3

COPY package.json project.json nx.json yarn.lock .yarnrc.yml ./
COPY .yarn .yarn
COPY packages packages
COPY e2e-playwright e2e-playwright

```

Output excerpt:

```text
# syntax=docker/dockerfile:1.7-labs

# to maintain formatting of multiline commands in vscode, add the following to settings.json:
# "docker.languageserver.formatter.ignoreMultilineInstructions": true

ARG GO_IMAGE=go-builder-base
ARG JS_IMAGE=js-builder-base
ARG JS_PLATFORM=linux/amd64

# Default to building locally
ARG GO_SRC=go-builder
ARG JS_SRC=js-builder

# Dependabot cannot update dependencies listed in ARGs
# By using FROM instructions we can delegate dependency updates to dependabot
FROM alpine:3.24.1 AS alpine-base
FROM ubuntu:24.04 AS ubuntu-base
FROM golang:1.26.4-alpine AS go-builder-base
FROM --platform=${JS_PLATFORM} node:24-alpine AS js-builder-base
FROM gcr.io/distroless/static-debian13 AS distroless-base
# Javascript build stage
FROM --platform=${JS_PLATFORM} ${JS_IMAGE} AS js-builder
ARG JS_NODE_ENV=production
ARG JS_YARN_INSTALL_FLAG=--immutable
ARG JS_YARN_BUILD_FLAG=build

ENV NODE_OPTIONS=--max_old_space_size=8000

WORKDIR /tmp/grafana

RUN apk add --no-cache make build-base python3

COPY package.json project.json nx.json yarn.lock .yarnrc.yml ./
COPY .yarn .yarn
COPY packages packages
COPY e2e-playwright e2e-playwright

```

### `18-temurin-jdk`

- [Full input](cases/18-temurin-jdk/input.dockerfile)
- [Full output](cases/18-temurin-jdk/output.dockerfile)
- [Input vs output diff](cases/18-temurin-jdk/compression.diff)

Input excerpt:

```text
# ------------------------------------------------------------------------------
#             NOTE: THIS FILE IS GENERATED VIA "generate_dockerfiles.py"
#
#                       PLEASE DO NOT EDIT IT DIRECTLY.
# ------------------------------------------------------------------------------
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

FROM ubuntu:24.04

ENV JAVA_HOME=/opt/java/openjdk
ENV PATH=$JAVA_HOME/bin:$PATH

# Default to UTF-8 file.encoding
ENV LANG='en_US.UTF-8' LANGUAGE='en_US:en' LC_ALL='en_US.UTF-8'

RUN set -eux; \
    apt-get update; \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        # curl required for historical reasons, see https://github.com/adoptium/containers/issues/255
        curl \
        wget \
        # gnupg required to verify the signature
        gnupg \
        # java.lang.UnsatisfiedLinkError: libfontmanager.so: libfreetype.so.6: cannot open shared object file: No such file or directory

```

Output excerpt:

```text
# ------------------------------------------------------------------------------
#             NOTE: THIS FILE IS GENERATED VIA "generate_dockerfiles.py"
#
#                       PLEASE DO NOT EDIT IT DIRECTLY.
# ------------------------------------------------------------------------------
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

FROM ubuntu:24.04

ENV JAVA_HOME=/opt/java/openjdk
ENV PATH=$JAVA_HOME/bin:$PATH

# Default to UTF-8 file.encoding
ENV LANG='en_US.UTF-8' LANGUAGE='en_US:en' LC_ALL='en_US.UTF-8'

RUN set -eux; \
    apt-get update; \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        # curl required for historical reasons, see https://github.com/adoptium/containers/issues/255
        curl \
        wget \
        # gnupg required to verify the signature
        gnupg \
        # java.lang.UnsatisfiedLinkError: libfontmanager.so: libfreetype.so.6: cannot open shared object file: No such file or directory

```

### `17-cassandra`

- [Full input](cases/17-cassandra/input.dockerfile)
- [Full output](cases/17-cassandra/output.dockerfile)
- [Input vs output diff](cases/17-cassandra/compression.diff)

Input excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM debian:bookworm-slim

# explicitly set user/group IDs
RUN set -eux; \
	groupadd -r cassandra --gid=999; \
	useradd -r -g cassandra --uid=999 cassandra

RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
# solves warning: "jemalloc shared library could not be preloaded to speed up memory allocations"
		libjemalloc2 \
# "free" is used by cassandra-env.sh
		procps \
# "cqlsh" needs a python interpreter
		python3 \
# "ip" is not required by Cassandra itself, but is commonly used in scripting Cassandra's configuration (since it is so fixated on explicit IP addresses)
		iproute2 \
# Cassandra will automatically use numactl if available
#   https://github.com/apache/cassandra/blob/18bcda2d4c2eba7370a0b21f33eed37cb730bbb3/bin/cassandra#L90-L100
#   https://github.com/apache/cassandra/commit/604c0e87dc67fa65f6904ef9a98a029c9f2f865a
		numactl \
	; \
	rm -rf /var/lib/apt/lists/*; \
# https://issues.apache.org/jira/browse/CASSANDRA-15767 ("bin/cassandra" only looks for "libjemalloc.so" or "libjemalloc.so.1" which doesn't match our "libjemalloc.so.2")
	libjemalloc="$(readlink -e /usr/lib/*/libjemalloc.so.2)"; \
	ln -sT "$libjemalloc" /usr/local/lib/libjemalloc.so; \
	ldconfig

# grab gosu for easy step-down from root

```

Output excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM debian:bookworm-slim

# explicitly set user/group IDs
RUN set -eux; \
	groupadd -r cassandra --gid=999; \
	useradd -r -g cassandra --uid=999 cassandra

RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
# solves warning: "jemalloc shared library could not be preloaded to speed up memory allocations"
		libjemalloc2 \
# "free" is used by cassandra-env.sh
		procps \
# "cqlsh" needs a python interpreter
		python3 \
# "ip" is not required by Cassandra itself, but is commonly used in scripting Cassandra's configuration (since it is so fixated on explicit IP addresses)
		iproute2 \
# Cassandra will automatically use numactl if available
#   https://github.com/apache/cassandra/blob/18bcda2d4c2eba7370a0b21f33eed37cb730bbb3/bin/cassandra#L90-L100
#   https://github.com/apache/cassandra/commit/604c0e87dc67fa65f6904ef9a98a029c9f2f865a
		numactl \
	; \
	rm -rf /var/lib/apt/lists/*; \
# https://issues.apache.org/jira/browse/CASSANDRA-15767 ("bin/cassandra" only looks for "libjemalloc.so" or "libjemalloc.so.1" which doesn't match our "libjemalloc.so.2")
	libjemalloc="$(readlink -e /usr/lib/*/libjemalloc.so.2)"; \
	ln -sT "$libjemalloc" /usr/local/lib/libjemalloc.so; \
	ldconfig

# grab gosu for easy step-down from root

```

### `16-nginx`

- [Full input](cases/16-nginx/input.dockerfile)
- [Full output](cases/16-nginx/output.dockerfile)
- [Input vs output diff](cases/16-nginx/compression.diff)

Input excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "update.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#
FROM debian:trixie-slim

LABEL maintainer="NGINX Docker Maintainers <docker-maint@nginx.com>"

ENV NGINX_VERSION   1.31.2
ENV NJS_VERSION     0.9.9
ENV NJS_RELEASE     1~trixie
ENV ACME_VERSION    0.4.1
ENV PKG_RELEASE     1~trixie
ENV DYNPKG_RELEASE  1~trixie

RUN set -x \
# create nginx user/group first, to be consistent throughout docker variants
    && groupadd --system --gid 101 nginx \
    && useradd --system --gid nginx --no-create-home --home /nonexistent --comment "nginx user" --shell /bin/false --uid 101 nginx \
    && apt-get update \
    && apt-get install --no-install-recommends --no-install-suggests -y gnupg1 ca-certificates \
    && \
    NGINX_GPGKEYS="573BFD6B3D8FBC641079A6ABABF5BD827BD9BF62 8540A6F18833A80E9C1653A42FD21310B49F6B46 9E9BE90EACBCDE69FE9B204CBCDCD8A38D88A2B3"; \
    NGINX_GPGKEY_PATH=/etc/apt/keyrings/nginx-archive-keyring.gpg; \
    export GNUPGHOME="$(mktemp -d)"; \
    found=''; \
    for NGINX_GPGKEY in $NGINX_GPGKEYS; do \
    for server in \
        hkp://keyserver.ubuntu.com:80 \
        pgp.mit.edu \
    ; do \
        echo "Fetching GPG key $NGINX_GPGKEY from $server"; \
        gpg1 --batch --keyserver "$server" --keyserver-options timeout=10 --recv-keys "$NGINX_GPGKEY" && found=yes && break; \
    done; \
    test -z "$found" && echo >&2 "error: failed to fetch GPG key $NGINX_GPGKEY" && exit 1; \

```

Output excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "update.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#
FROM debian:trixie-slim

LABEL maintainer="NGINX Docker Maintainers <docker-maint@nginx.com>"

ENV NGINX_VERSION   1.31.2
ENV NJS_VERSION     0.9.9
ENV NJS_RELEASE     1~trixie
ENV ACME_VERSION    0.4.1
ENV PKG_RELEASE     1~trixie
ENV DYNPKG_RELEASE  1~trixie

RUN set -x \
# create nginx user/group first, to be consistent throughout docker variants
    && groupadd --system --gid 101 nginx \
    && useradd --system --gid nginx --no-create-home --home /nonexistent --comment "nginx user" --shell /bin/false --uid 101 nginx \
    && apt-get update \
    && apt-get install --no-install-recommends --no-install-suggests -y gnupg1 ca-certificates \
    && \
    NGINX_GPGKEYS="573BFD6B3D8FBC641079A6ABABF5BD827BD9BF62 8540A6F18833A80E9C1653A42FD21310B49F6B46 9E9BE90EACBCDE69FE9B204CBCDCD8A38D88A2B3"; \
    NGINX_GPGKEY_PATH=/etc/apt/keyrings/nginx-archive-keyring.gpg; \
    export GNUPGHOME="$(mktemp -d)"; \
    found=''; \
    for NGINX_GPGKEY in $NGINX_GPGKEYS; do \
    for server in \
        hkp://keyserver.ubuntu.com:80 \
        pgp.mit.edu \
    ; do \
        echo "Fetching GPG key $NGINX_GPGKEY from $server"; \
        gpg1 --batch --keyserver "$server" --keyserver-options timeout=10 --recv-keys "$NGINX_GPGKEY" && found=yes && break; \
    done; \
    test -z "$found" && echo >&2 "error: failed to fetch GPG key $NGINX_GPGKEY" && exit 1; \

```

### `15-ruby`

- [Full input](cases/15-ruby/input.dockerfile)
- [Full output](cases/15-ruby/output.dockerfile)
- [Input vs output diff](cases/15-ruby/compression.diff)

Input excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM buildpack-deps:bookworm

# skip installing gem documentation with `gem install`/`gem update`
RUN set -eux; \
	mkdir -p /usr/local/etc; \
	echo 'gem: --no-document' >> /usr/local/etc/gemrc

ENV LANG C.UTF-8

# https://www.ruby-lang.org/en/news/2026/03/26/ruby-3-3-11-released/
ENV RUBY_VERSION 3.3.11
ENV RUBY_DOWNLOAD_URL https://cache.ruby-lang.org/pub/ruby/3.3/ruby-3.3.11.tar.xz
ENV RUBY_DOWNLOAD_SHA256 9116bb2e313203bd78cd3e8dc09284d34b2645f725877623a7185bf3807c8ca3

# some of ruby's build scripts are written in ruby
#   we purge system ruby later to make sure our final image uses what we just built
RUN set -eux; \
	\
	savedAptMark="$(apt-mark showmanual)"; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		dpkg-dev \
		libgdbm-dev \
		ruby \
	; \
	\
	rustArch=; \
	dpkgArch="$(dpkg --print-architecture)"; \
	case "$dpkgArch" in \
		'amd64') rustArch='x86_64-unknown-linux-gnu'; rustupUrl='https://static.rust-lang.org/rustup/archive/1.28.2/x86_64-unknown-linux-gnu/rustup-init'; rustupSha256='20a06e644b0d9bd2fbdbfd52d42540bdde820ea7df86e92e533c073da...

```

Output excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM buildpack-deps:bookworm

# skip installing gem documentation with `gem install`/`gem update`
RUN set -eux; \
	mkdir -p /usr/local/etc; \
	echo 'gem: --no-document' >> /usr/local/etc/gemrc

ENV LANG C.UTF-8

# https://www.ruby-lang.org/en/news/2026/03/26/ruby-3-3-11-released/
ENV RUBY_VERSION 3.3.11
ENV RUBY_DOWNLOAD_URL https://cache.ruby-lang.org/pub/ruby/3.3/ruby-3.3.11.tar.xz
ENV RUBY_DOWNLOAD_SHA256 9116bb2e313203bd78cd3e8dc09284d34b2645f725877623a7185bf3807c8ca3

# some of ruby's build scripts are written in ruby
#   we purge system ruby later to make sure our final image uses what we just built
RUN set -eux; \
	\
	savedAptMark="$(apt-mark showmanual)"; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		dpkg-dev \
		libgdbm-dev \
		ruby \
	; \
	\
	rustArch=; \
	dpkgArch="$(dpkg --print-architecture)"; \
	case "$dpkgArch" in \
		'amd64') rustArch='x86_64-unknown-linux-gnu'; rustupUrl='https://static.rust-lang.org/rustup/archive/1.28.2/x86_64-unknown-linux-gnu/rustup-init'; rustupSha256='20a06e644b0d9bd2fbdbfd52d42540bdde820ea7df86e92e533c073da...

```

### `14-php`

- [Full input](cases/14-php/input.dockerfile)
- [Full output](cases/14-php/output.dockerfile)
- [Input vs output diff](cases/14-php/compression.diff)

Input excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM debian:bookworm-slim

# prevent Debian's PHP packages from being installed
# https://github.com/docker-library/php/pull/542
RUN set -eux; \
	{ \
		echo 'Package: php*'; \
		echo 'Pin: release *'; \
		echo 'Pin-Priority: -1'; \
	} > /etc/apt/preferences.d/no-debian-php

# dependencies required for running "phpize"
# (see persistent deps below)
ENV PHPIZE_DEPS \
		autoconf \
		dpkg-dev \
		file \
		g++ \
		gcc \
		libc-dev \
		make \
		pkg-config \
		re2c

# persistent / runtime deps
RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		$PHPIZE_DEPS \
		ca-certificates \

```

Output excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM debian:bookworm-slim

# prevent Debian's PHP packages from being installed
# https://github.com/docker-library/php/pull/542
RUN set -eux; \
	{ \
		echo 'Package: php*'; \
		echo 'Pin: release *'; \
		echo 'Pin-Priority: -1'; \
	} > /etc/apt/preferences.d/no-debian-php

# dependencies required for running "phpize"
# (see persistent deps below)
ENV PHPIZE_DEPS \
		autoconf \
		dpkg-dev \
		file \
		g++ \
		gcc \
		libc-dev \
		make \
		pkg-config \
		re2c

# persistent / runtime deps
RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		$PHPIZE_DEPS \
		ca-certificates \

```

### `13-mariadb`

- [Full input](cases/13-mariadb/input.dockerfile)
- [Full output](cases/13-mariadb/output.dockerfile)
- [Input vs output diff](cases/13-mariadb/compression.diff)

Input excerpt:

```text
# vim:set ft=dockerfile:
FROM ubuntu:noble

# add our user and group first to make sure their IDs get assigned consistently, regardless of whatever dependencies get added
RUN groupadd -r mysql && useradd -r -g mysql mysql --home-dir /var/lib/mysql && userdel --remove ubuntu

# add gosu for easy step-down from root
# https://github.com/tianon/gosu/releases
# gosu key is B42F6819007F00F88E364FD4036A9C25BF357DD4
ENV GOSU_VERSION 1.19

ARG GPG_KEYS=177F4010FE56CA3336300305F1656F24C74CD1D8
# pub   rsa4096 2016-03-30 [SC]
#         177F 4010 FE56 CA33 3630  0305 F165 6F24 C74C D1D8
# uid           [ unknown] MariaDB Signing Key <signing-key@mariadb.org>
# sub   rsa4096 2016-03-30 [E]
# install "libjemalloc2" as it offers better performance in some cases. Use with LD_PRELOAD
# install "libtcmalloc-minimal4t64" as it may improve performance and fix memory fragmentation issues. Use with LD_PRELOAD
# install "pwgen" for randomizing passwords
# install "tzdata" for /usr/share/zoneinfo/
# install "xz-utils" for .sql.xz docker-entrypoint-initdb.d files
# install "zstd" for .sql.zst docker-entrypoint-initdb.d files
# hadolint ignore=SC2086
RUN set -eux; \
	apt-get update; \
	DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
		ca-certificates \
		gpg \
		gpgv \
		libjemalloc2 \
		libtcmalloc-minimal4t64 \
		pwgen \
		tzdata \
		xz-utils \
		zstd ; \
	savedAptMark="$(apt-mark showmanual)"; \

```

Output excerpt:

```text
# vim:set ft=dockerfile:
FROM ubuntu:noble

# add our user and group first to make sure their IDs get assigned consistently, regardless of whatever dependencies get added
RUN groupadd -r mysql && useradd -r -g mysql mysql --home-dir /var/lib/mysql && userdel --remove ubuntu

# add gosu for easy step-down from root
# https://github.com/tianon/gosu/releases
# gosu key is B42F6819007F00F88E364FD4036A9C25BF357DD4
ENV GOSU_VERSION 1.19

ARG GPG_KEYS=177F4010FE56CA3336300305F1656F24C74CD1D8
# pub   rsa4096 2016-03-30 [SC]
#         177F 4010 FE56 CA33 3630  0305 F165 6F24 C74C D1D8
# uid           [ unknown] MariaDB Signing Key <signing-key@mariadb.org>
# sub   rsa4096 2016-03-30 [E]
# install "libjemalloc2" as it offers better performance in some cases. Use with LD_PRELOAD
# install "libtcmalloc-minimal4t64" as it may improve performance and fix memory fragmentation issues. Use with LD_PRELOAD
# install "pwgen" for randomizing passwords
# install "tzdata" for /usr/share/zoneinfo/
# install "xz-utils" for .sql.xz docker-entrypoint-initdb.d files
# install "zstd" for .sql.zst docker-entrypoint-initdb.d files
# hadolint ignore=SC2086
RUN set -eux; \
	apt-get update; \
	DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
		ca-certificates \
		gpg \
		gpgv \
		libjemalloc2 \
		libtcmalloc-minimal4t64 \
		pwgen \
		tzdata \
		xz-utils \
		zstd ; \
	savedAptMark="$(apt-mark showmanual)"; \

```

### `12-haproxy`

- [Full input](cases/12-haproxy/input.dockerfile)
- [Full output](cases/12-haproxy/output.dockerfile)
- [Input vs output diff](cases/12-haproxy/compression.diff)

Input excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM debian:trixie-slim

# runtime dependencies
RUN set -eux; \
	apt-get install --update -y --no-install-recommends \
# @system-ca: https://github.com/docker-library/haproxy/pull/216
		ca-certificates \
# https://github.com/docker-library/haproxy/issues/59#issuecomment-3418639873
		socat \
	; \
	apt-get dist-clean

# roughly, https://salsa.debian.org/haproxy-team/haproxy/-/blob/732b97ae286906dea19ab5744cf9cf97c364ac1d/debian/haproxy.postinst#L5-6
RUN set -eux; \
	groupadd --gid 99 --system haproxy; \
	useradd \
		--gid haproxy \
		--home-dir /var/lib/haproxy \
		--no-create-home \
		--system \
		--uid 99 \
		haproxy \
	; \
	mkdir /var/lib/haproxy; \
	chown haproxy:haproxy /var/lib/haproxy

ENV HAPROXY_VERSION 3.0.25
ENV HAPROXY_URL https://www.haproxy.org/download/3.0/src/haproxy-3.0.25.tar.gz
ENV HAPROXY_SHA256 0e4fbd90826368297ce4d5374596dd1eff58f3ec00c27312a86e455e4fe1884f


```

Output excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM debian:trixie-slim

# runtime dependencies
RUN set -eux; \
	apt-get install --update -y --no-install-recommends \
# @system-ca: https://github.com/docker-library/haproxy/pull/216
		ca-certificates \
# https://github.com/docker-library/haproxy/issues/59#issuecomment-3418639873
		socat \
	; \
	apt-get dist-clean

# roughly, https://salsa.debian.org/haproxy-team/haproxy/-/blob/732b97ae286906dea19ab5744cf9cf97c364ac1d/debian/haproxy.postinst#L5-6
RUN set -eux; \
	groupadd --gid 99 --system haproxy; \
	useradd \
		--gid haproxy \
		--home-dir /var/lib/haproxy \
		--no-create-home \
		--system \
		--uid 99 \
		haproxy \
	; \
	mkdir /var/lib/haproxy; \
	chown haproxy:haproxy /var/lib/haproxy

ENV HAPROXY_VERSION 3.0.25
ENV HAPROXY_URL https://www.haproxy.org/download/3.0/src/haproxy-3.0.25.tar.gz
ENV HAPROXY_SHA256 0e4fbd90826368297ce4d5374596dd1eff58f3ec00c27312a86e455e4fe1884f


```

### `11-wordpress`

- [Full input](cases/11-wordpress/input.dockerfile)
- [Full output](cases/11-wordpress/output.dockerfile)
- [Input vs output diff](cases/11-wordpress/compression.diff)

Input excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM php:8.3-apache

# persistent dependencies
RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
# Ghostscript is required for rendering PDF previews
		ghostscript \
# pull in AV1 & HEVC image encoding (and not just decoding)
# https://github.com/docker-library/wordpress/issues/996
		libheif-plugin-aomenc \
		libheif-plugin-x265 \
	; \
	rm -rf /var/lib/apt/lists/*

# install the PHP extensions we need (https://make.wordpress.org/hosting/handbook/handbook/server-environment/#php-extensions)
RUN set -ex; \
	\
	savedAptMark="$(apt-mark showmanual)"; \
	\
	apt-get update; \
	apt-get install -y --no-install-recommends \
		libavif-dev \
		libfreetype6-dev \
		libicu-dev \
		libjpeg-dev \
		libmagickwand-dev \
		libpng-dev \
		libwebp-dev \
		libzip-dev \

```

Output excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM php:8.3-apache

# persistent dependencies
RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
# Ghostscript is required for rendering PDF previews
		ghostscript \
# pull in AV1 & HEVC image encoding (and not just decoding)
# https://github.com/docker-library/wordpress/issues/996
		libheif-plugin-aomenc \
		libheif-plugin-x265 \
	; \
	rm -rf /var/lib/apt/lists/*

# install the PHP extensions we need (https://make.wordpress.org/hosting/handbook/handbook/server-environment/#php-extensions)
RUN set -ex; \
	\
	savedAptMark="$(apt-mark showmanual)"; \
	\
	apt-get update; \
	apt-get install -y --no-install-recommends \
		libavif-dev \
		libfreetype6-dev \
		libicu-dev \
		libjpeg-dev \
		libmagickwand-dev \
		libpng-dev \
		libwebp-dev \
		libzip-dev \

```

### `10-tomcat`

- [Full input](cases/10-tomcat/input.dockerfile)
- [Full output](cases/10-tomcat/output.dockerfile)
- [Input vs output diff](cases/10-tomcat/compression.diff)

Input excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM eclipse-temurin:21-jdk-noble

ENV CATALINA_HOME /usr/local/tomcat
ENV PATH $CATALINA_HOME/bin:$PATH
RUN mkdir -p "$CATALINA_HOME"
WORKDIR $CATALINA_HOME

# let "Tomcat Native" live somewhere isolated
ENV TOMCAT_NATIVE_LIBDIR $CATALINA_HOME/native-jni-lib
ENV LD_LIBRARY_PATH ${LD_LIBRARY_PATH:+$LD_LIBRARY_PATH:}$TOMCAT_NATIVE_LIBDIR

ENV TOMCAT_MAJOR 11
ENV TOMCAT_VERSION 11.0.23
ENV TOMCAT_SHA512 f5acb7a1e5ad60dea682ffe16cf075596f0e7b62e460828eb176639714d125ab08c763753fb709e3626ce6f42712c281b4ccf54930354aa07325550d6e6cbdac

RUN set -eux; \
	\
	savedAptMark="$(apt-mark showmanual)"; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		ca-certificates \
		curl \
		gnupg \
	; \
	\
	ddist() { \
		local f="$1"; shift; \
		local distFile="$1"; shift; \
		local mvnFile="${1:-}"; \
		local success=; \

```

Output excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM eclipse-temurin:21-jdk-noble

ENV CATALINA_HOME /usr/local/tomcat
ENV PATH $CATALINA_HOME/bin:$PATH
RUN mkdir -p "$CATALINA_HOME"
WORKDIR $CATALINA_HOME

# let "Tomcat Native" live somewhere isolated
ENV TOMCAT_NATIVE_LIBDIR $CATALINA_HOME/native-jni-lib
ENV LD_LIBRARY_PATH ${LD_LIBRARY_PATH:+$LD_LIBRARY_PATH:}$TOMCAT_NATIVE_LIBDIR

ENV TOMCAT_MAJOR 11
ENV TOMCAT_VERSION 11.0.23
ENV TOMCAT_SHA512 f5acb7a1e5ad60dea682ffe16cf075596f0e7b62e460828eb176639714d125ab08c763753fb709e3626ce6f42712c281b4ccf54930354aa07325550d6e6cbdac

RUN set -eux; \
	\
	savedAptMark="$(apt-mark showmanual)"; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		ca-certificates \
		curl \
		gnupg \
	; \
	\
	ddist() { \
		local f="$1"; shift; \
		local distFile="$1"; shift; \
		local mvnFile="${1:-}"; \
		local success=; \

```

### `09-memcached`

- [Full input](cases/09-memcached/input.dockerfile)
- [Full output](cases/09-memcached/output.dockerfile)
- [Input vs output diff](cases/09-memcached/compression.diff)

Input excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM debian:trixie-slim

# add our user and group first to make sure their IDs get assigned consistently, regardless of whatever dependencies get added
RUN set -eux; \
	groupadd --system --gid 11211 memcache; \
	useradd --system --gid memcache --uid 11211 memcache

# ensure SASL's "libplain.so" is installed as per https://github.com/memcached/memcached/wiki/SASLHowto
RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		libsasl2-modules \
	; \
	rm -rf /var/lib/apt/lists/*

ENV MEMCACHED_VERSION 1.6.43
ENV MEMCACHED_URL https://memcached.org/files/memcached-1.6.43.tar.gz
ENV MEMCACHED_SHA1 37307d13528f355fb282bc8841ca033272e44ac9

RUN set -eux; \
	\
	savedAptMark="$(apt-mark showmanual)"; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		ca-certificates \
		dpkg-dev \
		gcc \
		libc6-dev \
		libevent-dev \
		libio-socket-ssl-perl \

```

Output excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM debian:trixie-slim

# add our user and group first to make sure their IDs get assigned consistently, regardless of whatever dependencies get added
RUN set -eux; \
	groupadd --system --gid 11211 memcache; \
	useradd --system --gid memcache --uid 11211 memcache

# ensure SASL's "libplain.so" is installed as per https://github.com/memcached/memcached/wiki/SASLHowto
RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		libsasl2-modules \
	; \
	rm -rf /var/lib/apt/lists/*

ENV MEMCACHED_VERSION 1.6.43
ENV MEMCACHED_URL https://memcached.org/files/memcached-1.6.43.tar.gz
ENV MEMCACHED_SHA1 37307d13528f355fb282bc8841ca033272e44ac9

RUN set -eux; \
	\
	savedAptMark="$(apt-mark showmanual)"; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		ca-certificates \
		dpkg-dev \
		gcc \
		libc6-dev \
		libevent-dev \
		libio-socket-ssl-perl \

```

### `08-rabbitmq`

- [Full input](cases/08-rabbitmq/input.dockerfile)
- [Full output](cases/08-rabbitmq/output.dockerfile)
- [Input vs output diff](cases/08-rabbitmq/compression.diff)

Input excerpt:

```text
# syntax=docker/dockerfile:1
# check=skip=SecretsUsedInArgOrEnv
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

# vim:noet:
# The official Canonical Ubuntu Focal image is ideal from a security perspective,
# especially for the enterprises that we, the RabbitMQ team, have to deal with
FROM ubuntu:24.04 AS build-base

ARG BUILDKIT_SBOM_SCAN_STAGE=true

RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		build-essential \
		ca-certificates \
		gnupg \
		libncurses5-dev \
		wget

FROM build-base AS openssl-builder

ARG BUILDKIT_SBOM_SCAN_STAGE=true

# Default to a PGP keyserver that pgp-happy-eyeballs recognizes, but allow for substitutions locally
ARG PGP_KEYSERVER=keyserver.ubuntu.com
# If you are building this image locally and are getting `gpg: keyserver receive failed: No data` errors,
# run the build with a different PGP_KEYSERVER, e.g. docker build --tag rabbitmq:4.0 --build-arg PGP_KEYSERVER=pgpkeys.eu 4.0/ubuntu
# For context, see https://github.com/docker-library/official-images/issues/4252

ENV OPENSSL_VERSION=3.5.7
ENV OPENSSL_SOURCE_SHA256="a8c0d28a529ca480f9f36cf5792e2cd21984552a3c8e4aa11a24aa31aeac98e8"

```

Output excerpt:

```text
# syntax=docker/dockerfile:1
# check=skip=SecretsUsedInArgOrEnv
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

# vim:noet:
# The official Canonical Ubuntu Focal image is ideal from a security perspective,
# especially for the enterprises that we, the RabbitMQ team, have to deal with
FROM ubuntu:24.04 AS build-base

ARG BUILDKIT_SBOM_SCAN_STAGE=true

RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		build-essential \
		ca-certificates \
		gnupg \
		libncurses5-dev \
		wget

FROM build-base AS openssl-builder

ARG BUILDKIT_SBOM_SCAN_STAGE=true

# Default to a PGP keyserver that pgp-happy-eyeballs recognizes, but allow for substitutions locally
ARG PGP_KEYSERVER=keyserver.ubuntu.com
# If you are building this image locally and are getting `gpg: keyserver receive failed: No data` errors,
# run the build with a different PGP_KEYSERVER, e.g. docker build --tag rabbitmq:4.0 --build-arg PGP_KEYSERVER=pgpkeys.eu 4.0/ubuntu
# For context, see https://github.com/docker-library/official-images/issues/4252

ENV OPENSSL_VERSION=3.5.7
ENV OPENSSL_SOURCE_SHA256="a8c0d28a529ca480f9f36cf5792e2cd21984552a3c8e4aa11a24aa31aeac98e8"

```

### `07-httpd`

- [Full input](cases/07-httpd/input.dockerfile)
- [Full output](cases/07-httpd/output.dockerfile)
- [Input vs output diff](cases/07-httpd/compression.diff)

Input excerpt:

```text
FROM debian:trixie-slim

# add our user and group first to make sure their IDs get assigned consistently, regardless of whatever dependencies get added
#RUN groupadd -r www-data && useradd -r --create-home -g www-data www-data

ENV HTTPD_PREFIX /usr/local/apache2
ENV PATH $HTTPD_PREFIX/bin:$PATH
RUN mkdir -p "$HTTPD_PREFIX" \
	&& chown www-data:www-data "$HTTPD_PREFIX"
WORKDIR $HTTPD_PREFIX

# install httpd runtime dependencies
# https://httpd.apache.org/docs/2.4/install.html#requirements
RUN set -eux; \
	apt-get install --update -y --no-install-recommends \
# https://github.com/docker-library/httpd/issues/214
		ca-certificates \
		libaprutil1-ldap \
# https://github.com/docker-library/httpd/issues/209
		libldap-common \
	; \
	apt-get dist-clean

ENV HTTPD_VERSION 2.4.68
ENV HTTPD_SHA256 68c74d4df38c26bed4dfbdb8f3baf1eb532f3872357becc1bba5d136f6b63c06

# https://httpd.apache.org/security/vulnerabilities_24.html
ENV HTTPD_PATCHES=""

# see https://httpd.apache.org/docs/2.4/install.html#requirements
RUN set -eux; \
	\
	# mod_http2 mod_lua mod_proxy_html mod_xml2enc
	# https://anonscm.debian.org/cgit/pkg-apache/apache2.git/tree/debian/control?id=adb6f181257af28ee67af15fc49d2699a0080d4c
	savedAptMark="$(apt-mark showmanual)"; \
	apt-get install --update -y --no-install-recommends \

```

Output excerpt:

```text
FROM debian:trixie-slim

# add our user and group first to make sure their IDs get assigned consistently, regardless of whatever dependencies get added
#RUN groupadd -r www-data && useradd -r --create-home -g www-data www-data

ENV HTTPD_PREFIX /usr/local/apache2
ENV PATH $HTTPD_PREFIX/bin:$PATH
RUN mkdir -p "$HTTPD_PREFIX" \
	&& chown www-data:www-data "$HTTPD_PREFIX"
WORKDIR $HTTPD_PREFIX

# install httpd runtime dependencies
# https://httpd.apache.org/docs/2.4/install.html#requirements
RUN set -eux; \
	apt-get install --update -y --no-install-recommends \
# https://github.com/docker-library/httpd/issues/214
		ca-certificates \
		libaprutil1-ldap \
# https://github.com/docker-library/httpd/issues/209
		libldap-common \
	; \
	apt-get dist-clean

ENV HTTPD_VERSION 2.4.68
ENV HTTPD_SHA256 68c74d4df38c26bed4dfbdb8f3baf1eb532f3872357becc1bba5d136f6b63c06

# https://httpd.apache.org/security/vulnerabilities_24.html
ENV HTTPD_PATCHES=""

# see https://httpd.apache.org/docs/2.4/install.html#requirements
RUN set -eux; \
	\
	# mod_http2 mod_lua mod_proxy_html mod_xml2enc
	# https://anonscm.debian.org/cgit/pkg-apache/apache2.git/tree/debian/control?id=adb6f181257af28ee67af15fc49d2699a0080d4c
	savedAptMark="$(apt-mark showmanual)"; \
	apt-get install --update -y --no-install-recommends \

```

### `06-mongo`

- [Full input](cases/06-mongo/input.dockerfile)
- [Full output](cases/06-mongo/output.dockerfile)
- [Input vs output diff](cases/06-mongo/compression.diff)

Input excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM ubuntu:jammy

# add our user and group first to make sure their IDs get assigned consistently, regardless of whatever dependencies get added
RUN set -eux; \
	groupadd --gid 999 --system mongodb; \
	useradd --uid 999 --system --gid mongodb --home-dir /data/db mongodb; \
	mkdir -p /data/db /data/configdb; \
	chown -R mongodb:mongodb /data/db /data/configdb

RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		ca-certificates \
		jq \
		numactl \
		procps \
	; \
	rm -rf /var/lib/apt/lists/*

# grab gosu for easy step-down from root (https://github.com/tianon/gosu/releases)
ENV GOSU_VERSION 1.19
# grab "js-yaml" for parsing mongod's YAML config files (https://github.com/nodeca/js-yaml/releases)
ENV JSYAML_VERSION 3.13.1
ENV JSYAML_CHECKSUM 662e32319bdd378e91f67578e56a34954b0a2e33aca11d70ab9f4826af24b941

RUN set -eux; \
	\
	savedAptMark="$(apt-mark showmanual)"; \
	apt-get update; \
	apt-get install -y --no-install-recommends \

```

Output excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM ubuntu:jammy

# add our user and group first to make sure their IDs get assigned consistently, regardless of whatever dependencies get added
RUN set -eux; \
	groupadd --gid 999 --system mongodb; \
	useradd --uid 999 --system --gid mongodb --home-dir /data/db mongodb; \
	mkdir -p /data/db /data/configdb; \
	chown -R mongodb:mongodb /data/db /data/configdb

RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		ca-certificates \
		jq \
		numactl \
		procps \
	; \
	rm -rf /var/lib/apt/lists/*

# grab gosu for easy step-down from root (https://github.com/tianon/gosu/releases)
ENV GOSU_VERSION 1.19
# grab "js-yaml" for parsing mongod's YAML config files (https://github.com/nodeca/js-yaml/releases)
ENV JSYAML_VERSION 3.13.1
ENV JSYAML_CHECKSUM 662e32319bdd378e91f67578e56a34954b0a2e33aca11d70ab9f4826af24b941

RUN set -eux; \
	\
	savedAptMark="$(apt-mark showmanual)"; \
	apt-get update; \
	apt-get install -y --no-install-recommends \

```

### `05-golang`

- [Full input](cases/05-golang/input.dockerfile)
- [Full output](cases/05-golang/output.dockerfile)
- [Input vs output diff](cases/05-golang/compression.diff)

Input excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM buildpack-deps:bookworm-scm AS build

ENV PATH /usr/local/go/bin:$PATH

ENV GOLANG_VERSION 1.25.11

RUN set -eux; \
	now="$(date '+%s')"; \
	arch="$(dpkg --print-architecture)"; arch="${arch##*-}"; \
	url=; \
	case "$arch" in \
		'amd64') \
			url='https://dl.google.com/go/go1.25.11.linux-amd64.tar.gz'; \
			sha256='34f14304e856893f4ba30c2cacfe93906e9de7915c5f6aaaf3a81cdccd7ba30b'; \
			;; \
		'armhf') \
			url='https://dl.google.com/go/go1.25.11.linux-armv6l.tar.gz'; \
			sha256='492d69badee59cae12e9a36282dfce94041bd4aac88fdddea575a7d99a2bd05d'; \
			;; \
		'arm64') \
			url='https://dl.google.com/go/go1.25.11.linux-arm64.tar.gz'; \
			sha256='c30bf9e156a54ea4e31fbbbf31a712b32734b58cc9a22426fa5ee632d0885124'; \
			;; \
		'i386') \
			url='https://dl.google.com/go/go1.25.11.linux-386.tar.gz'; \
			sha256='a2556ed17549cbec782f4c577c1bd7ad0d0decacffd2666a804304c320a1abe9'; \
			;; \
		'mips64el') \
			url='https://dl.google.com/go/go1.25.11.linux-mips64le.tar.gz'; \
			sha256='75d0543b4d36b01401ecad2a030bb11fc74725b6884805bdac7b54cc040c8594'; \

```

Output excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM buildpack-deps:bookworm-scm AS build

ENV PATH /usr/local/go/bin:$PATH

ENV GOLANG_VERSION 1.25.11

RUN set -eux; \
	now="$(date '+%s')"; \
	arch="$(dpkg --print-architecture)"; arch="${arch##*-}"; \
	url=; \
	case "$arch" in \
		'amd64') \
			url='https://dl.google.com/go/go1.25.11.linux-amd64.tar.gz'; \
			sha256='34f14304e856893f4ba30c2cacfe93906e9de7915c5f6aaaf3a81cdccd7ba30b'; \
			;; \
		'armhf') \
			url='https://dl.google.com/go/go1.25.11.linux-armv6l.tar.gz'; \
			sha256='492d69badee59cae12e9a36282dfce94041bd4aac88fdddea575a7d99a2bd05d'; \
			;; \
		'arm64') \
			url='https://dl.google.com/go/go1.25.11.linux-arm64.tar.gz'; \
			sha256='c30bf9e156a54ea4e31fbbbf31a712b32734b58cc9a22426fa5ee632d0885124'; \
			;; \
		'i386') \
			url='https://dl.google.com/go/go1.25.11.linux-386.tar.gz'; \
			sha256='a2556ed17549cbec782f4c577c1bd7ad0d0decacffd2666a804304c320a1abe9'; \
			;; \
		'mips64el') \
			url='https://dl.google.com/go/go1.25.11.linux-mips64le.tar.gz'; \
			sha256='75d0543b4d36b01401ecad2a030bb11fc74725b6884805bdac7b54cc040c8594'; \

```

### `04-python`

- [Full input](cases/04-python/input.dockerfile)
- [Full output](cases/04-python/output.dockerfile)
- [Input vs output diff](cases/04-python/compression.diff)

Input excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM buildpack-deps:bookworm

# ensure local python is preferred over distribution python
ENV PATH /usr/local/bin:$PATH

# runtime dependencies
RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		libbluetooth-dev \
		tk-dev \
		uuid-dev \
	; \
	rm -rf /var/lib/apt/lists/*

ENV GPG_KEY 7169605F62C751356D054A26A821E680E5FA6305
ENV PYTHON_VERSION 3.13.14
ENV PYTHON_SHA256 639e43243c620a308f968213df9e00f2f8f62332f7adbaa7a7eeb9783057c690

RUN set -eux; \
	\
	wget -O python.tar.xz "https://www.python.org/ftp/python/${PYTHON_VERSION%%[a-z]*}/Python-$PYTHON_VERSION.tar.xz"; \
	echo "$PYTHON_SHA256 *python.tar.xz" | sha256sum -c -; \
	wget -O python.tar.xz.asc "https://www.python.org/ftp/python/${PYTHON_VERSION%%[a-z]*}/Python-$PYTHON_VERSION.tar.xz.asc"; \
	GNUPGHOME="$(mktemp -d)"; export GNUPGHOME; \
	gpg --batch --keyserver hkps://keys.openpgp.org --recv-keys "$GPG_KEY"; \
	gpg --batch --verify python.tar.xz.asc python.tar.xz; \
	gpgconf --kill all; \
	rm -rf "$GNUPGHOME" python.tar.xz.asc; \
	mkdir -p /usr/src/python; \

```

Output excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM buildpack-deps:bookworm

# ensure local python is preferred over distribution python
ENV PATH /usr/local/bin:$PATH

# runtime dependencies
RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		libbluetooth-dev \
		tk-dev \
		uuid-dev \
	; \
	rm -rf /var/lib/apt/lists/*

ENV GPG_KEY 7169605F62C751356D054A26A821E680E5FA6305
ENV PYTHON_VERSION 3.13.14
ENV PYTHON_SHA256 639e43243c620a308f968213df9e00f2f8f62332f7adbaa7a7eeb9783057c690

RUN set -eux; \
	\
	wget -O python.tar.xz "https://www.python.org/ftp/python/${PYTHON_VERSION%%[a-z]*}/Python-$PYTHON_VERSION.tar.xz"; \
	echo "$PYTHON_SHA256 *python.tar.xz" | sha256sum -c -; \
	wget -O python.tar.xz.asc "https://www.python.org/ftp/python/${PYTHON_VERSION%%[a-z]*}/Python-$PYTHON_VERSION.tar.xz.asc"; \
	GNUPGHOME="$(mktemp -d)"; export GNUPGHOME; \
	gpg --batch --keyserver hkps://keys.openpgp.org --recv-keys "$GPG_KEY"; \
	gpg --batch --verify python.tar.xz.asc python.tar.xz; \
	gpgconf --kill all; \
	rm -rf "$GNUPGHOME" python.tar.xz.asc; \
	mkdir -p /usr/src/python; \

```

### `03-redis`

- [Full input](cases/03-redis/input.dockerfile)
- [Full output](cases/03-redis/output.dockerfile)
- [Input vs output diff](cases/03-redis/compression.diff)

Input excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM debian:bookworm-slim

# add our user and group first to make sure their IDs get assigned consistently, regardless of whatever dependencies get added
RUN set -eux; \
	groupadd -r -g 999 redis; \
	useradd -r -g redis -u 999 redis

# runtime dependencies
RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
# add tzdata explicitly for https://github.com/docker-library/redis/issues/138 (see also https://bugs.debian.org/837060 and related)
		tzdata \
	; \
	rm -rf /var/lib/apt/lists/*

# grab gosu for easy step-down from root
# https://github.com/tianon/gosu/releases
ENV GOSU_VERSION 1.17
RUN set -eux; \
	savedAptMark="$(apt-mark showmanual)"; \
	apt-get update; \
	apt-get install -y --no-install-recommends ca-certificates gnupg wget; \
	rm -rf /var/lib/apt/lists/*; \
	arch="$(dpkg --print-architecture | awk -F- '{ print $NF }')"; \
	case "$arch" in \
		'amd64') url='https://github.com/tianon/gosu/releases/download/1.17/gosu-amd64'; sha256='bbc4136d03ab138b1ad66fa4fc051bafc6cc7ffae632b069a53657279a450de3' ;; \
		'arm64') url='https://github.com/tianon/gosu/releases/download/1.17/gosu-arm64'; sha256='c3805a85d17f4454c23d7059bcb97e1ec1af272b90126e79ed002342de08389b' ;; \
		'armel') url='https://github.com/tianon/gosu/releases/download/1.17/gosu-armel'; sha256='f9969910fa141140438c998cfa02f603bf213b11afd466dcde8fa940e700945d' ;; \
		'i386') url='https://github.com/tianon/gosu/releases/download/1.17/gosu-i386'; sha256='087dbb8fe479537e64f9c86fa49ff3b41dee1cbd28739a19aaef83dc8186b1ca' ;; \

```

Output excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM debian:bookworm-slim

# add our user and group first to make sure their IDs get assigned consistently, regardless of whatever dependencies get added
RUN set -eux; \
	groupadd -r -g 999 redis; \
	useradd -r -g redis -u 999 redis

# runtime dependencies
RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
# add tzdata explicitly for https://github.com/docker-library/redis/issues/138 (see also https://bugs.debian.org/837060 and related)
		tzdata \
	; \
	rm -rf /var/lib/apt/lists/*

# grab gosu for easy step-down from root
# https://github.com/tianon/gosu/releases
ENV GOSU_VERSION 1.17
RUN set -eux; \
	savedAptMark="$(apt-mark showmanual)"; \
	apt-get update; \
	apt-get install -y --no-install-recommends ca-certificates gnupg wget; \
	rm -rf /var/lib/apt/lists/*; \
	arch="$(dpkg --print-architecture | awk -F- '{ print $NF }')"; \
	case "$arch" in \
		'amd64') url='https://github.com/tianon/gosu/releases/download/1.17/gosu-amd64'; sha256='bbc4136d03ab138b1ad66fa4fc051bafc6cc7ffae632b069a53657279a450de3' ;; \
		'arm64') url='https://github.com/tianon/gosu/releases/download/1.17/gosu-arm64'; sha256='c3805a85d17f4454c23d7059bcb97e1ec1af272b90126e79ed002342de08389b' ;; \
		'armel') url='https://github.com/tianon/gosu/releases/download/1.17/gosu-armel'; sha256='f9969910fa141140438c998cfa02f603bf213b11afd466dcde8fa940e700945d' ;; \
		'i386') url='https://github.com/tianon/gosu/releases/download/1.17/gosu-i386'; sha256='087dbb8fe479537e64f9c86fa49ff3b41dee1cbd28739a19aaef83dc8186b1ca' ;; \

```

### `02-postgres-16`

- [Full input](cases/02-postgres-16/input.dockerfile)
- [Full output](cases/02-postgres-16/output.dockerfile)
- [Input vs output diff](cases/02-postgres-16/compression.diff)

Input excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM debian:bookworm-slim

# explicitly set user/group IDs
RUN set -eux; \
	groupadd -r postgres --gid=999; \
# https://salsa.debian.org/postgresql/postgresql-common/blob/997d842ee744687d99a2b2d95c1083a2615c79e8/debian/postgresql-common.postinst#L32-35
	useradd -r -g postgres --uid=999 --home-dir=/var/lib/postgresql --shell=/bin/bash postgres; \
# also create the postgres user's home directory with appropriate permissions
# see https://github.com/docker-library/postgres/issues/274
	install --verbose --directory --owner postgres --group postgres --mode 1777 /var/lib/postgresql

RUN set -ex; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		gnupg \
# https://www.postgresql.org/docs/16/app-psql.html#APP-PSQL-META-COMMAND-PSET-PAGER
# https://github.com/postgres/postgres/blob/REL_16_1/src/include/fe_utils/print.h#L25
# (if "less" is available, it gets used as the default pager for psql, and it only adds ~1.5MiB to our image size)
		less \
	; \
	rm -rf /var/lib/apt/lists/*

# grab gosu for easy step-down from root
# https://github.com/tianon/gosu/releases
ENV GOSU_VERSION 1.19
RUN set -eux; \
	savedAptMark="$(apt-mark showmanual)"; \
	apt-get update; \
	apt-get install -y --no-install-recommends ca-certificates wget; \
	rm -rf /var/lib/apt/lists/*; \

```

Output excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM debian:bookworm-slim

# explicitly set user/group IDs
RUN set -eux; \
	groupadd -r postgres --gid=999; \
# https://salsa.debian.org/postgresql/postgresql-common/blob/997d842ee744687d99a2b2d95c1083a2615c79e8/debian/postgresql-common.postinst#L32-35
	useradd -r -g postgres --uid=999 --home-dir=/var/lib/postgresql --shell=/bin/bash postgres; \
# also create the postgres user's home directory with appropriate permissions
# see https://github.com/docker-library/postgres/issues/274
	install --verbose --directory --owner postgres --group postgres --mode 1777 /var/lib/postgresql

RUN set -ex; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		gnupg \
# https://www.postgresql.org/docs/16/app-psql.html#APP-PSQL-META-COMMAND-PSET-PAGER
# https://github.com/postgres/postgres/blob/REL_16_1/src/include/fe_utils/print.h#L25
# (if "less" is available, it gets used as the default pager for psql, and it only adds ~1.5MiB to our image size)
		less \
	; \
	rm -rf /var/lib/apt/lists/*

# grab gosu for easy step-down from root
# https://github.com/tianon/gosu/releases
ENV GOSU_VERSION 1.19
RUN set -eux; \
	savedAptMark="$(apt-mark showmanual)"; \
	apt-get update; \
	apt-get install -y --no-install-recommends ca-certificates wget; \
	rm -rf /var/lib/apt/lists/*; \

```

### `01-postgres`

- [Full input](cases/01-postgres/input.dockerfile)
- [Full output](cases/01-postgres/output.dockerfile)
- [Input vs output diff](cases/01-postgres/compression.diff)

Input excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM debian:bookworm-slim

# explicitly set user/group IDs
RUN set -eux; \
	groupadd -r postgres --gid=999; \
# https://salsa.debian.org/postgresql/postgresql-common/blob/997d842ee744687d99a2b2d95c1083a2615c79e8/debian/postgresql-common.postinst#L32-35
	useradd -r -g postgres --uid=999 --home-dir=/var/lib/postgresql --shell=/bin/bash postgres; \
# also create the postgres user's home directory with appropriate permissions
# see https://github.com/docker-library/postgres/issues/274
	install --verbose --directory --owner postgres --group postgres --mode 1777 /var/lib/postgresql

RUN set -ex; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		gnupg \
# https://www.postgresql.org/docs/16/app-psql.html#APP-PSQL-META-COMMAND-PSET-PAGER
# https://github.com/postgres/postgres/blob/REL_16_1/src/include/fe_utils/print.h#L25
# (if "less" is available, it gets used as the default pager for psql, and it only adds ~1.5MiB to our image size)
		less \
	; \
	rm -rf /var/lib/apt/lists/*

# grab gosu for easy step-down from root
# https://github.com/tianon/gosu/releases
ENV GOSU_VERSION 1.19
RUN set -eux; \
	savedAptMark="$(apt-mark showmanual)"; \
	apt-get update; \
	apt-get install -y --no-install-recommends ca-certificates wget; \
	rm -rf /var/lib/apt/lists/*; \

```

Output excerpt:

```text
#
# NOTE: THIS DOCKERFILE IS GENERATED VIA "apply-templates.sh"
#
# PLEASE DO NOT EDIT IT DIRECTLY.
#

FROM debian:bookworm-slim

# explicitly set user/group IDs
RUN set -eux; \
	groupadd -r postgres --gid=999; \
# https://salsa.debian.org/postgresql/postgresql-common/blob/997d842ee744687d99a2b2d95c1083a2615c79e8/debian/postgresql-common.postinst#L32-35
	useradd -r -g postgres --uid=999 --home-dir=/var/lib/postgresql --shell=/bin/bash postgres; \
# also create the postgres user's home directory with appropriate permissions
# see https://github.com/docker-library/postgres/issues/274
	install --verbose --directory --owner postgres --group postgres --mode 1777 /var/lib/postgresql

RUN set -ex; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		gnupg \
# https://www.postgresql.org/docs/16/app-psql.html#APP-PSQL-META-COMMAND-PSET-PAGER
# https://github.com/postgres/postgres/blob/REL_16_1/src/include/fe_utils/print.h#L25
# (if "less" is available, it gets used as the default pager for psql, and it only adds ~1.5MiB to our image size)
		less \
	; \
	rm -rf /var/lib/apt/lists/*

# grab gosu for easy step-down from root
# https://github.com/tianon/gosu/releases
ENV GOSU_VERSION 1.19
RUN set -eux; \
	savedAptMark="$(apt-mark showmanual)"; \
	apt-get update; \
	apt-get install -y --no-install-recommends ca-certificates wget; \
	rm -rf /var/lib/apt/lists/*; \

```

