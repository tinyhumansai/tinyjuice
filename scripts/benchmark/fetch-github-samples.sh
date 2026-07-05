#!/usr/bin/env bash
# Fetch real-world benchmark fixtures from public GitHub repositories.
#
# Sources are pinned to tags/commits where the upstream layout allows; the
# fetched content is checked in, so the benchmark itself never depends on
# network availability or upstream drift. Every source and its license is
# recorded in docs/benchmark/<category>/ATTRIBUTION.md.
#
# Usage: scripts/benchmark/fetch-github-samples.sh
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
bench_root="$repo_root/docs/benchmark"

# manifest: category|case-dir|artifact|max_lines|license|url
manifest() {
  cat <<'EOF'
github-source|01-ts-vscode-async|input.ts|0|MIT|https://raw.githubusercontent.com/microsoft/vscode/1.95.0/src/vs/base/common/async.ts
github-source|02-ts-vscode-strings|input.ts|0|MIT|https://raw.githubusercontent.com/microsoft/vscode/1.95.0/src/vs/base/common/strings.ts
github-source|03-ts-vscode-uri|input.ts|0|MIT|https://raw.githubusercontent.com/microsoft/vscode/1.95.0/src/vs/base/common/uri.ts
github-source|04-js-express-application|input.js|0|MIT|https://raw.githubusercontent.com/expressjs/express/4.19.2/lib/application.js
github-source|05-js-axios-core|input.js|0|MIT|https://raw.githubusercontent.com/axios/axios/v1.7.2/lib/core/Axios.js
github-source|06-py-django-request|input.py|0|BSD-3-Clause|https://raw.githubusercontent.com/django/django/5.1/django/http/request.py
github-source|07-py-django-paginator|input.py|0|BSD-3-Clause|https://raw.githubusercontent.com/django/django/5.1/django/core/paginator.py
github-source|08-py-flask-app|input.py|0|BSD-3-Clause|https://raw.githubusercontent.com/pallets/flask/3.0.3/src/flask/app.py
github-source|09-py-requests-sessions|input.py|0|Apache-2.0|https://raw.githubusercontent.com/psf/requests/v2.32.3/src/requests/sessions.py
github-source|10-go-gin-gin|input.go|0|MIT|https://raw.githubusercontent.com/gin-gonic/gin/v1.10.0/gin.go
github-source|11-go-gin-context|input.go|0|MIT|https://raw.githubusercontent.com/gin-gonic/gin/v1.10.0/context.go
github-source|12-go-cobra-command|input.go|0|Apache-2.0|https://raw.githubusercontent.com/spf13/cobra/v1.8.0/command.go
github-source|13-go-mux-mux|input.go|0|BSD-3-Clause|https://raw.githubusercontent.com/gorilla/mux/v1.8.1/mux.go
github-source|14-cpp-leveldb-dbimpl|input.cpp|0|BSD-3-Clause|https://raw.githubusercontent.com/google/leveldb/1.23/db/db_impl.cc
github-source|15-cpp-leveldb-versionset|input.cpp|0|BSD-3-Clause|https://raw.githubusercontent.com/google/leveldb/1.23/db/version_set.cc
github-source|16-c-redis-sds|input.c|0|BSD-3-Clause|https://raw.githubusercontent.com/redis/redis/7.2.5/src/sds.c
github-source|17-c-curl-url|input.c|0|curl|https://raw.githubusercontent.com/curl/curl/curl-8_8_0/lib/url.c
github-source|18-rs-tokio-builder|input.rs|0|MIT|https://raw.githubusercontent.com/tokio-rs/tokio/tokio-1.38.0/tokio/src/runtime/builder.rs
github-source|19-rs-ripgrep-walk|input.rs|0|Unlicense/MIT|https://raw.githubusercontent.com/BurntSushi/ripgrep/14.1.0/crates/ignore/src/walk.rs
github-source|20-java-gson-gson|input.java|0|Apache-2.0|https://raw.githubusercontent.com/google/gson/gson-parent-2.11.0/gson/src/main/java/com/google/gson/Gson.java
github-source|21-java-guava-ordering|input.java|0|Apache-2.0|https://raw.githubusercontent.com/google/guava/v33.2.0/guava/src/com/google/common/collect/Ordering.java
github-source|22-kt-okhttp-client|input.kt|0|Apache-2.0|https://raw.githubusercontent.com/square/okhttp/parent-4.12.0/okhttp/src/main/kotlin/okhttp3/OkHttpClient.kt
github-source|23-rb-rails-cache|input.rb|0|MIT|https://raw.githubusercontent.com/rails/rails/v7.1.3/activesupport/lib/active_support/cache.rb
github-source|24-php-laravel-container|input.php|0|MIT|https://raw.githubusercontent.com/laravel/framework/v11.0.0/src/Illuminate/Container/Container.php
github-source|25-swift-argparser-argumentset|input.swift|0|Apache-2.0|https://raw.githubusercontent.com/apple/swift-argument-parser/1.3.0/Sources/ArgumentParser/Parsing/ArgumentSet.swift
github-source|26-cs-newtonsoft-serializer|input.cs|0|MIT|https://raw.githubusercontent.com/JamesNK/Newtonsoft.Json/13.0.3/Src/Newtonsoft.Json/JsonSerializer.cs
github-source|27-xml-gson-pom|input.xml|0|Apache-2.0|https://raw.githubusercontent.com/google/gson/gson-parent-2.11.0/pom.xml
github-source|28-xml-maven-pom|input.xml|0|Apache-2.0|https://raw.githubusercontent.com/apache/maven/maven-3.9.7/pom.xml
github-logs|01-hdfs|input.log|0|loghub (research use)|https://raw.githubusercontent.com/logpai/loghub/master/HDFS/HDFS_2k.log
github-logs|02-hadoop|input.log|0|loghub (research use)|https://raw.githubusercontent.com/logpai/loghub/master/Hadoop/Hadoop_2k.log
github-logs|03-spark|input.log|0|loghub (research use)|https://raw.githubusercontent.com/logpai/loghub/master/Spark/Spark_2k.log
github-logs|04-zookeeper|input.log|0|loghub (research use)|https://raw.githubusercontent.com/logpai/loghub/master/Zookeeper/Zookeeper_2k.log
github-logs|05-bgl|input.log|0|loghub (research use)|https://raw.githubusercontent.com/logpai/loghub/master/BGL/BGL_2k.log
github-logs|06-hpc|input.log|0|loghub (research use)|https://raw.githubusercontent.com/logpai/loghub/master/HPC/HPC_2k.log
github-logs|07-thunderbird|input.log|0|loghub (research use)|https://raw.githubusercontent.com/logpai/loghub/master/Thunderbird/Thunderbird_2k.log
github-logs|08-windows|input.log|0|loghub (research use)|https://raw.githubusercontent.com/logpai/loghub/master/Windows/Windows_2k.log
github-logs|09-linux|input.log|0|loghub (research use)|https://raw.githubusercontent.com/logpai/loghub/master/Linux/Linux_2k.log
github-logs|10-android|input.log|0|loghub (research use)|https://raw.githubusercontent.com/logpai/loghub/master/Android/Android_2k.log
github-logs|11-healthapp|input.log|0|loghub (research use)|https://raw.githubusercontent.com/logpai/loghub/master/HealthApp/HealthApp_2k.log
github-logs|12-apache-error|input.log|0|loghub (research use)|https://raw.githubusercontent.com/logpai/loghub/master/Apache/Apache_2k.log
github-logs|13-proxifier|input.log|0|loghub (research use)|https://raw.githubusercontent.com/logpai/loghub/master/Proxifier/Proxifier_2k.log
github-logs|14-openssh|input.log|0|loghub (research use)|https://raw.githubusercontent.com/logpai/loghub/master/OpenSSH/OpenSSH_2k.log
github-logs|15-openstack|input.log|0|loghub (research use)|https://raw.githubusercontent.com/logpai/loghub/master/OpenStack/OpenStack_2k.log
github-logs|16-mac|input.log|0|loghub (research use)|https://raw.githubusercontent.com/logpai/loghub/master/Mac/Mac_2k.log
github-logs|17-apache-access|input.log|2500|Apache-2.0|https://raw.githubusercontent.com/elastic/examples/master/Common%20Data%20Formats/apache_logs/apache_logs
github-logs|18-nginx-access|input.log|2500|Apache-2.0|https://raw.githubusercontent.com/elastic/examples/master/Common%20Data%20Formats/nginx_logs/nginx_logs
github-logs|19-auth-log|input.log|2500|Apache-2.0|https://raw.githubusercontent.com/elastic/examples/master/Machine%20Learning/Security%20Analytics%20Recipes/suspicious_login_activity/data/auth.log
github-logs|20-caddy-coraza-waf|input.log|1500|MIT|https://raw.githubusercontent.com/crowdsecurity/hub/master/.tests/caddy-crs-anomaly-score/coraza.log
github-logs|21-traefik-flood|input.log|1500|MIT|https://raw.githubusercontent.com/crowdsecurity/hub/master/.tests/tcpudp-flood-traefik/tcpudp-flood-traefik.log
github-logs|22-traefik-http|input.log|1500|MIT|https://raw.githubusercontent.com/crowdsecurity/hub/master/.tests/traefik_base-http-scenario/traefik_base-http-scenario.log
github-logs|23-authelia-bf|input.log|1500|MIT|https://raw.githubusercontent.com/crowdsecurity/hub/master/.tests/authelia-bf/authelia-bf.log
github-logs|24-http-dos|input.log|1500|MIT|https://raw.githubusercontent.com/crowdsecurity/hub/master/.tests/http-dos-bypass-cache/http_dos_bypass_cache.log
github-logs|25-sshesame-honeypot|input.log|1500|MIT|https://raw.githubusercontent.com/crowdsecurity/hub/master/.tests/sshesame/sshesame.log
github-logs|26-gitlab-bf|input.log|1500|MIT|https://raw.githubusercontent.com/crowdsecurity/hub/master/.tests/gitlab-bf/gitlab-bf.log
github-logs|27-suricata-eve|input.log|1500|MIT|https://raw.githubusercontent.com/crowdsecurity/hub/master/.tests/suricata-logs-evelog/suricata-logs-evelog.log
dockerfiles|01-postgres|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/docker-library/postgres/master/17/bookworm/Dockerfile
dockerfiles|02-postgres-16|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/docker-library/postgres/master/16/bookworm/Dockerfile
dockerfiles|03-redis|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/docker-library/redis/master/7.4/debian/Dockerfile
dockerfiles|04-python|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/docker-library/python/master/3.13/bookworm/Dockerfile
dockerfiles|05-golang|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/docker-library/golang/master/1.25/bookworm/Dockerfile
dockerfiles|06-mongo|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/docker-library/mongo/master/7.0/Dockerfile
dockerfiles|07-httpd|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/docker-library/httpd/master/2.4/Dockerfile
dockerfiles|08-rabbitmq|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/docker-library/rabbitmq/master/4.0/ubuntu/Dockerfile
dockerfiles|09-memcached|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/docker-library/memcached/master/1/debian/Dockerfile
dockerfiles|10-tomcat|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/docker-library/tomcat/master/11.0/jdk21/temurin-noble/Dockerfile
dockerfiles|11-wordpress|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/docker-library/wordpress/master/latest/php8.3/apache/Dockerfile
dockerfiles|12-haproxy|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/docker-library/haproxy/master/3.0/Dockerfile
dockerfiles|13-mariadb|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/docker-library/mariadb/master/11.4/Dockerfile
dockerfiles|14-php|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/docker-library/php/master/8.3/bookworm/cli/Dockerfile
dockerfiles|15-ruby|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/docker-library/ruby/master/3.3/bookworm/Dockerfile
dockerfiles|16-nginx|input.dockerfile|0|BSD-2-Clause|https://raw.githubusercontent.com/nginxinc/docker-nginx/master/mainline/debian/Dockerfile
dockerfiles|17-cassandra|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/docker-library/cassandra/master/4.1/Dockerfile
dockerfiles|18-temurin-jdk|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/adoptium/containers/main/21/jdk/ubuntu/noble/Dockerfile
dockerfiles|19-grafana|input.dockerfile|0|AGPL-3.0|https://raw.githubusercontent.com/grafana/grafana/main/Dockerfile
dockerfiles|20-prometheus|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/prometheus/prometheus/main/Dockerfile
dockerfiles|21-kube-apiserver|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/kubernetes/kubernetes/master/build/server-image/kube-apiserver/Dockerfile
dockerfiles|22-moby|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/moby/moby/master/Dockerfile
dockerfiles|23-elasticsearch|input.dockerfile|0|Elastic/SSPL|https://raw.githubusercontent.com/docker-library/elasticsearch/master/8/Dockerfile
dockerfiles|24-node|input.dockerfile|0|MIT|https://raw.githubusercontent.com/nodejs/docker-node/main/22/bookworm/Dockerfile
dockerfiles|25-traefik|input.dockerfile|0|MIT|https://raw.githubusercontent.com/traefik/traefik/master/Dockerfile
dockerfiles|26-busybox-builder|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/docker-library/busybox/master/latest/musl/Dockerfile.builder
dockerfiles|27-sentry-self-hosted|input.dockerfile|0|FSL-1.1|https://raw.githubusercontent.com/getsentry/sentry/master/self-hosted/Dockerfile
dockerfiles|28-airflow|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/apache/airflow/main/Dockerfile
dockerfiles|29-nextjs-example|input.dockerfile|0|MIT|https://raw.githubusercontent.com/vercel/next.js/canary/examples/with-docker/Dockerfile
dockerfiles|30-influxdb|input.dockerfile|0|MIT|https://raw.githubusercontent.com/influxdata/influxdata-docker/master/influxdb/2.7/Dockerfile
dockerfiles|31-julia|input.dockerfile|0|MIT|https://raw.githubusercontent.com/docker-library/julia/master/1.11/bookworm/Dockerfile
dockerfiles|32-caddy|input.dockerfile|0|Apache-2.0|https://raw.githubusercontent.com/caddyserver/caddy-docker/master/2.11/alpine/Dockerfile
EOF
}

# Per-category attribution rows collect in temp files (macOS ships bash 3.2,
# which has no associative arrays).
attr_dir="$(mktemp -d)"
trap 'rm -rf "$attr_dir"' EXIT

fetched=0
failed=0
while IFS='|' read -r category case_dir artifact max_lines license url; do
  [[ -z "$category" ]] && continue
  dest_dir="$bench_root/$category/cases/$case_dir"
  dest="$dest_dir/$artifact"
  mkdir -p "$dest_dir"
  tmp="$(mktemp)"
  if curl -fsSL "$url" -o "$tmp"; then
    if [[ "$max_lines" != "0" ]]; then
      head -n "$max_lines" "$tmp" >"$dest"
      rm -f "$tmp"
    else
      mv "$tmp" "$dest"
    fi
    printf '| `%s` | %s | <%s> |\n' "$case_dir" "$license" "$url" >>"$attr_dir/$category"
    fetched=$((fetched + 1))
    echo "ok   $category/$case_dir"
  else
    rm -f "$tmp"
    failed=$((failed + 1))
    echo "FAIL $category/$case_dir <- $url" >&2
  fi
done < <(manifest)

for attr_file in "$attr_dir"/*; do
  [[ -f "$attr_file" ]] || continue
  category="$(basename "$attr_file")"
  {
    printf '# Fixture Sources\n\n'
    printf 'Fixtures in this category are fetched from public GitHub repositories by\n'
    printf '`scripts/benchmark/fetch-github-samples.sh`. Each file remains under its\n'
    printf 'upstream license; sources are listed below. Files fetched with a line cap\n'
    printf 'are truncated excerpts.\n\n'
    printf '| Case | License | Source |\n'
    printf '| --- | --- | --- |\n'
    cat "$attr_file"
  } >"$bench_root/$category/ATTRIBUTION.md"
done

echo "fetched=$fetched failed=$failed"
[[ "$failed" -eq 0 ]]
