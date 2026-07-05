# GitHub Log Files

Real log files from public GitHub repositories: the loghub corpus (HDFS, Hadoop, Spark, Zookeeper, BGL, HPC, Thunderbird, Windows, Linux, Android, HealthApp, Apache, Proxifier, OpenSSH, OpenStack, macOS), Elastic example datasets (Apache/nginx access logs, auth.log), and CrowdSec parser test fixtures (WAF, Traefik, Authelia, GitLab, Suricata). Sources and licenses in ATTRIBUTION.md.

Each row links to the full raw input and both compacted outputs. Percentages are **token reduction: higher is better**; 0% means pass-through. `Bytes` shows the raw input size -> compressor-only output size and its byte reduction. `Pass 1` disables CCR and is **lossless by construction**: faithful reshapes (JSON tables/minify, HTML->text) still ship because nothing is lost, but anything that *drops* detail (log lines, diff context, search matches, code bodies, sampled JSON rows) passes the original through untouched, since without the cache it could not be recovered. `Pass 2` enables CCR, so information-dropping compression is allowed — every dropped block is offloaded behind a retrieval token. Faithful reshapes (HTML->text) are identical in both passes (Pass 2 is marginally lower only for the recovery footer); pure information-dropping categories are 0% in Pass 1 and compress only in Pass 2; JSON is a hybrid — Pass 1 renders the full lossless markdown table (all rows), and Pass 2 additionally samples the middle away behind retrieval tokens. Each pass links its own output and its own diff against the input.

## Cases

Every case links to the raw input; each pass column carries its percentage plus that pass's exact output and a unified diff against the input.

| Case | Input | Bytes | Pass 1: no CCR | Pass 2: with CCR | Avg latency |
| --- | --- | ---: | ---: | ---: | ---: |
| `15-openstack` | [input](cases/15-openstack/input.log) | 595.1 KB -> 906 B (-100%) | 0.0%<br>[output](cases/15-openstack/output-noccr.log) - [diff](cases/15-openstack/compression-noccr.diff) | 99.8%<br>[output](cases/15-openstack/output.log) - [diff](cases/15-openstack/compression.diff) | 3.048 ms |
| `01-hdfs` | [input](cases/01-hdfs/input.log) | 287.8 KB -> 470 B (-100%) | 0.0%<br>[output](cases/01-hdfs/output-noccr.log) - [diff](cases/01-hdfs/compression-noccr.diff) | 99.8%<br>[output](cases/01-hdfs/output.log) - [diff](cases/01-hdfs/compression.diff) | 1.785 ms |
| `17-apache-access` | [input](cases/17-apache-access/input.log) | 578.0 KB -> 1.7 KB (-100%) | 0.0%<br>[output](cases/17-apache-access/output-noccr.log) - [diff](cases/17-apache-access/compression-noccr.diff) | 99.7%<br>[output](cases/17-apache-access/output.log) - [diff](cases/17-apache-access/compression.diff) | 16.269 ms |
| `11-healthapp` | [input](cases/11-healthapp/input.log) | 187.5 KB -> 1.1 KB (-99%) | 0.0%<br>[output](cases/11-healthapp/output-noccr.log) - [diff](cases/11-healthapp/compression-noccr.diff) | 99.4%<br>[output](cases/11-healthapp/output.log) - [diff](cases/11-healthapp/compression.diff) | 5.069 ms |
| `34-jvm-gc` | [input](cases/34-jvm-gc/input.log) | 235.6 KB -> 1.9 KB (-99%) | 0.0%<br>[output](cases/34-jvm-gc/output-noccr.log) - [diff](cases/34-jvm-gc/compression-noccr.diff) | 99.2%<br>[output](cases/34-jvm-gc/output.log) - [diff](cases/34-jvm-gc/compression.diff) | 11.146 ms |
| `04-zookeeper` | [input](cases/04-zookeeper/input.log) | 279.9 KB -> 4.6 KB (-98%) | 0.0%<br>[output](cases/04-zookeeper/output-noccr.log) - [diff](cases/04-zookeeper/compression-noccr.diff) | 98.3%<br>[output](cases/04-zookeeper/output.log) - [diff](cases/04-zookeeper/compression.diff) | 4.239 ms |
| `30-laravel-app` | [input](cases/30-laravel-app/input.log) | 104.5 KB -> 3.8 KB (-96%) | 0.0%<br>[output](cases/30-laravel-app/output-noccr.log) - [diff](cases/30-laravel-app/compression-noccr.diff) | 96.3%<br>[output](cases/30-laravel-app/output.log) - [diff](cases/30-laravel-app/compression.diff) | 3.147 ms |
| `09-linux` | [input](cases/09-linux/input.log) | 216.5 KB -> 11.2 KB (-95%) | 0.0%<br>[output](cases/09-linux/output-noccr.log) - [diff](cases/09-linux/compression-noccr.diff) | 94.8%<br>[output](cases/09-linux/output.log) - [diff](cases/09-linux/compression.diff) | 6.395 ms |
| `19-auth-log` | [input](cases/19-auth-log/input.log) | 282.2 KB -> 16.4 KB (-94%) | 0.0%<br>[output](cases/19-auth-log/output-noccr.log) - [diff](cases/19-auth-log/compression-noccr.diff) | 94.2%<br>[output](cases/19-auth-log/output.log) - [diff](cases/19-auth-log/compression.diff) | 8.046 ms |
| `12-apache-error` | [input](cases/12-apache-error/input.log) | 171.2 KB -> 10.6 KB (-94%) | 0.0%<br>[output](cases/12-apache-error/output-noccr.log) - [diff](cases/12-apache-error/compression-noccr.diff) | 93.8%<br>[output](cases/12-apache-error/output.log) - [diff](cases/12-apache-error/compression.diff) | 2.532 ms |
| `06-hpc` | [input](cases/06-hpc/input.log) | 151.2 KB -> 9.6 KB (-94%) | 0.0%<br>[output](cases/06-hpc/output-noccr.log) - [diff](cases/06-hpc/compression-noccr.diff) | 93.6%<br>[output](cases/06-hpc/output.log) - [diff](cases/06-hpc/compression.diff) | 5.002 ms |
| `05-bgl` | [input](cases/05-bgl/input.log) | 317.1 KB -> 20.2 KB (-94%) | 0.0%<br>[output](cases/05-bgl/output-noccr.log) - [diff](cases/05-bgl/compression-noccr.diff) | 93.6%<br>[output](cases/05-bgl/output.log) - [diff](cases/05-bgl/compression.diff) | 2.960 ms |
| `13-proxifier` | [input](cases/13-proxifier/input.log) | 237.0 KB -> 15.5 KB (-93%) | 0.0%<br>[output](cases/13-proxifier/output-noccr.log) - [diff](cases/13-proxifier/compression-noccr.diff) | 93.4%<br>[output](cases/13-proxifier/output.log) - [diff](cases/13-proxifier/compression.diff) | 7.157 ms |
| `16-mac` | [input](cases/16-mac/input.log) | 319.4 KB -> 24.3 KB (-92%) | 0.0%<br>[output](cases/16-mac/output-noccr.log) - [diff](cases/16-mac/compression-noccr.diff) | 92.4%<br>[output](cases/16-mac/output.log) - [diff](cases/16-mac/compression.diff) | 10.622 ms |
| `14-openssh` | [input](cases/14-openssh/input.log) | 225.2 KB -> 18.1 KB (-92%) | 0.0%<br>[output](cases/14-openssh/output-noccr.log) - [diff](cases/14-openssh/compression-noccr.diff) | 92.0%<br>[output](cases/14-openssh/output.log) - [diff](cases/14-openssh/compression.diff) | 7.199 ms |
| `08-windows` | [input](cases/08-windows/input.log) | 285.4 KB -> 28.2 KB (-90%) | 0.0%<br>[output](cases/08-windows/output-noccr.log) - [diff](cases/08-windows/compression-noccr.diff) | 90.1%<br>[output](cases/08-windows/output.log) - [diff](cases/08-windows/compression.diff) | 6.754 ms |
| `02-hadoop` | [input](cases/02-hadoop/input.log) | 384.9 KB -> 38.1 KB (-90%) | 0.0%<br>[output](cases/02-hadoop/output-noccr.log) - [diff](cases/02-hadoop/compression-noccr.diff) | 90.1%<br>[output](cases/02-hadoop/output.log) - [diff](cases/02-hadoop/compression.diff) | 4.392 ms |
| `07-thunderbird` | [input](cases/07-thunderbird/input.log) | 325.2 KB -> 32.3 KB (-90%) | 0.0%<br>[output](cases/07-thunderbird/output-noccr.log) - [diff](cases/07-thunderbird/compression-noccr.diff) | 90.0%<br>[output](cases/07-thunderbird/output.log) - [diff](cases/07-thunderbird/compression.diff) | 7.964 ms |
| `20-caddy-coraza-waf` | [input](cases/20-caddy-coraza-waf/input.log) | 427.0 KB -> 53.7 KB (-87%) | 0.0%<br>[output](cases/20-caddy-coraza-waf/output-noccr.log) - [diff](cases/20-caddy-coraza-waf/compression-noccr.diff) | 87.4%<br>[output](cases/20-caddy-coraza-waf/output.log) - [diff](cases/20-caddy-coraza-waf/compression.diff) | 10.113 ms |
| `33-postfix-mail` | [input](cases/33-postfix-mail/input.log) | 16.3 KB -> 3.8 KB (-76%) | 0.0%<br>[output](cases/33-postfix-mail/output-noccr.log) - [diff](cases/33-postfix-mail/compression-noccr.diff) | 75.8%<br>[output](cases/33-postfix-mail/output.log) - [diff](cases/33-postfix-mail/compression.diff) | 1.068 ms |
| `23-authelia-bf` | [input](cases/23-authelia-bf/input.log) | 82.7 KB -> 68.4 KB (-17%) | 0.0%<br>[output](cases/23-authelia-bf/output-noccr.log) - [diff](cases/23-authelia-bf/compression-noccr.diff) | 17.2%<br>[output](cases/23-authelia-bf/output.log) - [diff](cases/23-authelia-bf/compression.diff) | 0.879 ms |
| `29-spark-eventlog` | [input](cases/29-spark-eventlog/input.log) | 412.7 KB -> 353.5 KB (-14%) | 0.0%<br>[output](cases/29-spark-eventlog/output-noccr.log) - [diff](cases/29-spark-eventlog/compression-noccr.diff) | 14.3%<br>[output](cases/29-spark-eventlog/output.log) - [diff](cases/29-spark-eventlog/compression.diff) | 5.343 ms |
| `32-w3c-iis` | [input](cases/32-w3c-iis/input.log) | 47.4 KB -> 47.4 KB (-0%) | 0.0%<br>[output](cases/32-w3c-iis/output-noccr.log) - [diff](cases/32-w3c-iis/compression-noccr.diff) | 0.0%<br>[output](cases/32-w3c-iis/output.log) - [diff](cases/32-w3c-iis/compression.diff) | 0.773 ms |
| `31-zeek-http` | [input](cases/31-zeek-http/input.log) | 71.2 KB -> 71.2 KB (-0%) | 0.0%<br>[output](cases/31-zeek-http/output-noccr.log) - [diff](cases/31-zeek-http/compression-noccr.diff) | 0.0%<br>[output](cases/31-zeek-http/output.log) - [diff](cases/31-zeek-http/compression.diff) | 1.515 ms |
| `27-suricata-eve` | [input](cases/27-suricata-eve/input.log) | 19.4 KB -> 19.4 KB (-0%) | 0.0%<br>[output](cases/27-suricata-eve/output-noccr.log) - [diff](cases/27-suricata-eve/compression-noccr.diff) | 0.0%<br>[output](cases/27-suricata-eve/output.log) - [diff](cases/27-suricata-eve/compression.diff) | 0.003 ms |
| `26-gitlab-bf` | [input](cases/26-gitlab-bf/input.log) | 40.7 KB -> 40.7 KB (-0%) | 0.0%<br>[output](cases/26-gitlab-bf/output-noccr.log) - [diff](cases/26-gitlab-bf/compression-noccr.diff) | 0.0%<br>[output](cases/26-gitlab-bf/output.log) - [diff](cases/26-gitlab-bf/compression.diff) | 0.004 ms |
| `25-sshesame-honeypot` | [input](cases/25-sshesame-honeypot/input.log) | 42.1 KB -> 42.1 KB (-0%) | 0.0%<br>[output](cases/25-sshesame-honeypot/output-noccr.log) - [diff](cases/25-sshesame-honeypot/compression-noccr.diff) | 0.0%<br>[output](cases/25-sshesame-honeypot/output.log) - [diff](cases/25-sshesame-honeypot/compression.diff) | 1.071 ms |
| `24-http-dos` | [input](cases/24-http-dos/input.log) | 73.0 KB -> 73.0 KB (-0%) | 0.0%<br>[output](cases/24-http-dos/output-noccr.log) - [diff](cases/24-http-dos/compression-noccr.diff) | 0.0%<br>[output](cases/24-http-dos/output.log) - [diff](cases/24-http-dos/compression.diff) | 1.149 ms |
| `22-traefik-http` | [input](cases/22-traefik-http/input.log) | 116.4 KB -> 116.4 KB (-0%) | 0.0%<br>[output](cases/22-traefik-http/output-noccr.log) - [diff](cases/22-traefik-http/compression-noccr.diff) | 0.0%<br>[output](cases/22-traefik-http/output.log) - [diff](cases/22-traefik-http/compression.diff) | 0.011 ms |
| `21-traefik-flood` | [input](cases/21-traefik-flood/input.log) | 133.6 KB -> 133.6 KB (-0%) | 0.0%<br>[output](cases/21-traefik-flood/output-noccr.log) - [diff](cases/21-traefik-flood/compression-noccr.diff) | 0.0%<br>[output](cases/21-traefik-flood/output.log) - [diff](cases/21-traefik-flood/compression.diff) | 0.208 ms |
| `18-nginx-access` | [input](cases/18-nginx-access/input.log) | 337.5 KB -> 337.5 KB (-0%) | 0.0%<br>[output](cases/18-nginx-access/output-noccr.log) - [diff](cases/18-nginx-access/compression-noccr.diff) | 0.0%<br>[output](cases/18-nginx-access/output.log) - [diff](cases/18-nginx-access/compression.diff) | 6.449 ms |
| `10-android` | [input](cases/10-android/input.log) | 279.1 KB -> 279.1 KB (-0%) | 0.0%<br>[output](cases/10-android/output-noccr.log) - [diff](cases/10-android/compression-noccr.diff) | 0.0%<br>[output](cases/10-android/output.log) - [diff](cases/10-android/compression.diff) | 5.916 ms |
| `03-spark` | [input](cases/03-spark/input.log) | 196.3 KB -> 196.3 KB (-0%) | 0.0%<br>[output](cases/03-spark/output-noccr.log) - [diff](cases/03-spark/compression-noccr.diff) | 0.0%<br>[output](cases/03-spark/output.log) - [diff](cases/03-spark/compression.diff) | 0.325 ms |

## What TinyJuice Is Doing

The signal-based log path keeps errors, warnings, stack frames, and summaries and collapses the rest behind per-gap retrieval tokens. Logs with no failure signal (pure access logs) are deliberately passed through rather than blindly truncated.

## Syntax-Aware Samples

### `15-openstack`

- [Full input](cases/15-openstack/input.log)
- [Output with CCR](cases/15-openstack/output.log) - [diff](cases/15-openstack/compression.diff)
- [Output without CCR](cases/15-openstack/output-noccr.log) - [diff](cases/15-openstack/compression-noccr.diff)

Input excerpt:

```text
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:00:00.008 25746 INFO nova.osapi_compute.wsgi.server [req-38101a0b-2096-447d-96ea-a692162415ae 113d3a99c3da401fbd62cc2caa5b96d2 54fadb412c4e40cdbaed9335e4c35a9e - - -] 10.1...
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:00:00.272 25746 INFO nova.osapi_compute.wsgi.server [req-9bc36dd9-91c5-4314-898a-47625eb93b09 113d3a99c3da401fbd62cc2caa5b96d2 54fadb412c4e40cdbaed9335e4c35a9e - - -] 10.1...
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:00:01.551 25746 INFO nova.osapi_compute.wsgi.server [req-55db2d8d-cdb7-4b4b-993b-429be84c0c3e 113d3a99c3da401fbd62cc2caa5b96d2 54fadb412c4e40cdbaed9335e4c35a9e - - -] 10.1...
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:00:01.813 25746 INFO nova.osapi_compute.wsgi.server [req-2a3dc421-6604-42a7-9390-a18dc824d5d6 113d3a99c3da401fbd62cc2caa5b96d2 54fadb412c4e40cdbaed9335e4c35a9e - - -] 10.1...
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:00:03.091 25746 INFO nova.osapi_compute.wsgi.server [req-939eb332-c1c1-4e67-99b8-8695f8f1980a 113d3a99c3da401fbd62cc2caa5b96d2 54fadb412c4e40cdbaed9335e4c35a9e - - -] 10.1...
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:00:03.358 25746 INFO nova.osapi_compute.wsgi.server [req-b6a4fa91-7414-432a-b725-52b5613d3ca3 113d3a99c3da401fbd62cc2caa5b96d2 54fadb412c4e40cdbaed9335e4c35a9e - - -] 10.1...
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:00:04.500 2931 INFO nova.compute.manager [req-3ea4052c-895d-4b64-9e2d-04d64c4d94ab - - - - -] [instance: b9000564-fe1a-409b-b8cc-1e88b294cd1d] VM Started (Lifecycle Ev...
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:00:04.562 2931 INFO nova.compute.manager [req-3ea4052c-895d-4b64-9e2d-04d64c4d94ab - - - - -] [instance: b9000564-fe1a-409b-b8cc-1e88b294cd1d] VM Paused (Lifecycle Eve...
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:00:04.693 2931 INFO nova.compute.manager [req-3ea4052c-895d-4b64-9e2d-04d64c4d94ab - - - - -] [instance: b9000564-fe1a-409b-b8cc-1e88b294cd1d] During sync_power_state ...
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:00:04.789 25746 INFO nova.osapi_compute.wsgi.server [req-bbfc3fb8-7cb3-4ac8-801e-c893d1082762 113d3a99c3da401fbd62cc2caa5b96d2 54fadb412c4e40cdbaed9335e4c35a9e - - -] 10.1...
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:00:05.060 25746 INFO nova.osapi_compute.wsgi.server [req-31826992-8435-4e03-bc09-ba9cca2d8ef9 113d3a99c3da401fbd62cc2caa5b96d2 54fadb412c4e40cdbaed9335e4c35a9e - - -] 10.1...
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:00:05.185 2931 INFO nova.virt.libvirt.imagecache [req-addc1839-2ed5-4778-b57e-5854eb7b8b09 - - - - -] image 0673dd71-34c5-4fbb-86c4-40623fbe45b4 at (/var/lib/nova/inst...
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:00:05.186 2931 INFO nova.virt.libvirt.imagecache [req-addc1839-2ed5-4778-b57e-5854eb7b8b09 - - - - -] image 0673dd71-34c5-4fbb-86c4-40623fbe45b4 at (/var/lib/nova/inst...
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:00:05.367 2931 INFO nova.virt.libvirt.imagecache [req-addc1839-2ed5-4778-b57e-5854eb7b8b09 - - - - -] Active base files: /var/lib/nova/instances/_base/a489c868f0c37da9...
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:00:06.321 25746 INFO nova.osapi_compute.wsgi.server [req-7160b3e7-676b-498f-b147-7759d8eaea76 113d3a99c3da401fbd62cc2caa5b96d2 54fadb412c4e40cdbaed9335e4c35a9e - - -] 10.1...
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:00:06.584 25746 INFO nova.osapi_compute.wsgi.server [req-e46f1fc1-61ce-4673-b3c7-f8bd94554273 113d3a99c3da401fbd62cc2caa5b96d2 54fadb412c4e40cdbaed9335e4c35a9e - - -] 10.1...
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:00:07.864 25746 INFO nova.osapi_compute.wsgi.server [req-546e2e6a-b85e-434a-91dc-53a0a9124a4f 113d3a99c3da401fbd62cc2caa5b96d2 54fadb412c4e40cdbaed9335e4c35a9e - - -] 10.1...
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:00:08.137 25746 INFO nova.osapi_compute.wsgi.server [req-e2c35e53-06d3-4feb-84b9-705c94d40e5b 113d3a99c3da401fbd62cc2caa5b96d2 54fadb412c4e40cdbaed9335e4c35a9e - - -] 10.1...
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:00:09.411 25746 INFO nova.osapi_compute.wsgi.server [req-ce9c8a59-c9ba-43b1-9735-318ceabc9216 113d3a99c3da401fbd62cc2caa5b96d2 54fadb412c4e40cdbaed9335e4c35a9e - - -] 10.1...
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:00:09.692 25746 INFO nova.osapi_compute.wsgi.server [req-e1da47c6-0f46-4ce8-940c-05397a5fab9e 113d3a99c3da401fbd62cc2caa5b96d2 54fadb412c4e40cdbaed9335e4c35a9e - - -] 10.1...
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:00:10.279 25743 INFO nova.api.openstack.compute.server_external_events [req-ab451068-9756-4ad9-9d18-5ceaa6424627 f7b8d1f1d4d44643b07fa10ca7d021fb e9746973ac574c6b8a9e8857f...
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:00:10.285 25743 INFO nova.osapi_compute.wsgi.server [req-ab451068-9756-4ad9-9d18-5ceaa6424627 f7b8d1f1d4d44643b07fa10ca7d021fb e9746973ac574c6b8a9e8857f56a7608 - - -] 10.1...
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:00:10.296 2931 INFO nova.compute.manager [req-3ea4052c-895d-4b64-9e2d-04d64c4d94ab - - - - -] [instance: b9000564-fe1a-409b-b8cc-1e88b294cd1d] VM Resumed (Lifecycle Ev...
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:00:10.302 2931 INFO nova.virt.libvirt.driver [-] [instance: b9000564-fe1a-409b-b8cc-1e88b294cd1d] Instance spawned successfully.
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:00:10.303 2931 INFO nova.compute.manager [req-8e64797b-fb99-4c8a-87e5-9a8de673412f 113d3a99c3da401fbd62cc2caa5b96d2 54fadb412c4e40cdbaed9335e4c35a9e - - -] [instance: ...
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:00:10.416 2931 INFO nova.compute.manager [req-3ea4052c-895d-4b64-9e2d-04d64c4d94ab - - - - -] [instance: b9000564-fe1a-409b-b8cc-1e88b294cd1d] During sync_power_state ...
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:00:10.417 2931 INFO nova.compute.manager [req-3ea4052c-895d-4b64-9e2d-04d64c4d94ab - - - - -] [instance: b9000564-fe1a-409b-b8cc-1e88b294cd1d] VM Resumed (Lifecycle Ev...
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:00:10.421 2931 INFO nova.virt.libvirt.imagecache [req-addc1839-2ed5-4778-b57e-5854eb7b8b09 - - - - -] image 0673dd71-34c5-4fbb-86c4-40623fbe45b4 at (/var/lib/nova/inst...
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:00:10.424 2931 INFO nova.virt.libvirt.imagecache [req-addc1839-2ed5-4778-b57e-5854eb7b8b09 - - - - -] image 0673dd71-34c5-4fbb-86c4-40623fbe45b4 at (/var/lib/nova/inst...
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:00:10.470 2931 INFO nova.compute.manager [req-8e64797b-fb99-4c8a-87e5-9a8de673412f 113d3a99c3da401fbd62cc2caa5b96d2 54fadb412c4e40cdbaed9335e4c35a9e - - -] [instance: ...
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:00:10.600 2931 INFO nova.virt.libvirt.imagecache [req-addc1839-2ed5-4778-b57e-5854eb7b8b09 - - - - -] Active base files: /var/lib/nova/instances/_base/a489c868f0c37da9...
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:00:10.978 25746 INFO nova.osapi_compute.wsgi.server [req-d81279b2-d9df-48b7-9c36-edab3801c067 113d3a99c3da401fbd62cc2caa5b96d2 54fadb412c4e40cdbaed9335e4c35a9e - - -] 10.1...
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:00:11.243 25746 INFO nova.osapi_compute.wsgi.server [req-22455aab-13cf-4045-92e8-65371ef51485 113d3a99c3da401fbd62cc2caa5b96d2 54fadb412c4e40cdbaed9335e4c35a9e - - -] 10.1...
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:00:13.658 2931 INFO nova.compute.resource_tracker [req-addc1839-2ed5-4778-b57e-5854eb7b8b09 - - - - -] Auditing locally available compute resources for node cp-1.slowv...
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:00:14.265 2931 INFO nova.compute.resource_tracker [req-addc1839-2ed5-4778-b57e-5854eb7b8b09 - - - - -] Total usable vcpus: 16, total allocated vcpus: 1
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:00:14.266 2931 INFO nova.compute.resource_tracker [req-addc1839-2ed5-4778-b57e-5854eb7b8b09 - - - - -] Final resource view: name=cp-1.slowvm1.tcloud-pg0.utah.cloudlab....

```

Output excerpt:

```text
[... 56 line(s) omitted ... ⟦tj:f1cac8acac01b3338f648c30b01a2392⟧]
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:00:20.345 2931 WARNING nova.virt.libvirt.imagecache [req-addc1839-2ed5-4778-b57e-5854eb7b8b09 - - - - -] Unknown base file: /var/lib/nova/instances/_base/a489c868f0c37...
[... 1239 line(s) omitted ... ⟦tj:f317220fad9770f06e73ddaa19dba616⟧]
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:09:41.850 2931 WARNING nova.compute.manager [req-addc1839-2ed5-4778-b57e-5854eb7b8b09 - - - - -] While synchronizing instance power states, found 1 instances in the da...
[... 703 line(s) omitted ... ⟦tj:bf91b2a52c9097ce48e16a681df54b61⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (595119 bytes): call tinyjuice_retrieve with token "025a1bc64ff5b2ef4a4bda6c4ad5c5c5"]

```

### `01-hdfs`

- [Full input](cases/01-hdfs/input.log)
- [Output with CCR](cases/01-hdfs/output.log) - [diff](cases/01-hdfs/compression.diff)
- [Output without CCR](cases/01-hdfs/output-noccr.log) - [diff](cases/01-hdfs/compression-noccr.diff)

Input excerpt:

```text
081109 203615 148 INFO dfs.DataNode$PacketResponder: PacketResponder 1 for block blk_38865049064139660 terminating
081109 203807 222 INFO dfs.DataNode$PacketResponder: PacketResponder 0 for block blk_-6952295868487656571 terminating
081109 204005 35 INFO dfs.FSNamesystem: BLOCK* NameSystem.addStoredBlock: blockMap updated: 10.251.73.220:50010 is added to blk_7128370237687728475 size 67108864
081109 204015 308 INFO dfs.DataNode$PacketResponder: PacketResponder 2 for block blk_8229193803249955061 terminating
081109 204106 329 INFO dfs.DataNode$PacketResponder: PacketResponder 2 for block blk_-6670958622368987959 terminating
081109 204132 26 INFO dfs.FSNamesystem: BLOCK* NameSystem.addStoredBlock: blockMap updated: 10.251.43.115:50010 is added to blk_3050920587428079149 size 67108864
081109 204324 34 INFO dfs.FSNamesystem: BLOCK* NameSystem.addStoredBlock: blockMap updated: 10.251.203.80:50010 is added to blk_7888946331804732825 size 67108864
081109 204453 34 INFO dfs.FSNamesystem: BLOCK* NameSystem.addStoredBlock: blockMap updated: 10.250.11.85:50010 is added to blk_2377150260128098806 size 67108864
081109 204525 512 INFO dfs.DataNode$PacketResponder: PacketResponder 2 for block blk_572492839287299681 terminating
081109 204655 556 INFO dfs.DataNode$PacketResponder: Received block blk_3587508140051953248 of size 67108864 from /10.251.42.84
081109 204722 567 INFO dfs.DataNode$PacketResponder: Received block blk_5402003568334525940 of size 67108864 from /10.251.214.112
081109 204815 653 INFO dfs.DataNode$DataXceiver: Receiving block blk_5792489080791696128 src: /10.251.30.6:33145 dest: /10.251.30.6:50010
081109 204842 663 INFO dfs.DataNode$DataXceiver: Receiving block blk_1724757848743533110 src: /10.251.111.130:49851 dest: /10.251.111.130:50010
081109 204908 31 INFO dfs.FSNamesystem: BLOCK* NameSystem.addStoredBlock: blockMap updated: 10.251.110.8:50010 is added to blk_8015913224713045110 size 67108864
081109 204925 673 INFO dfs.DataNode$DataXceiver: Receiving block blk_-5623176793330377570 src: /10.251.75.228:53725 dest: /10.251.75.228:50010
081109 205035 28 INFO dfs.FSNamesystem: BLOCK* NameSystem.allocateBlock: /user/root/rand/_temporary/_task_200811092030_0001_m_000590_0/part-00590. blk_-1727475099218615100
081109 205056 710 INFO dfs.DataNode$PacketResponder: PacketResponder 1 for block blk_5017373558217225674 terminating
081109 205157 752 INFO dfs.DataNode$PacketResponder: Received block blk_9212264480425680329 of size 67108864 from /10.251.123.1
081109 205315 29 INFO dfs.FSNamesystem: BLOCK* NameSystem.allocateBlock: /user/root/rand/_temporary/_task_200811092030_0001_m_000742_0/part-00742. blk_-7878121102358435702
081109 205409 28 INFO dfs.FSNamesystem: BLOCK* NameSystem.addStoredBlock: blockMap updated: 10.251.111.130:50010 is added to blk_4568434182693165548 size 67108864
081109 205412 832 INFO dfs.DataNode$PacketResponder: Received block blk_-5704899712662113150 of size 67108864 from /10.251.91.229
081109 205632 28 INFO dfs.FSNamesystem: BLOCK* NameSystem.addStoredBlock: blockMap updated: 10.251.74.79:50010 is added to blk_-4794867979917102672 size 67108864
081109 205739 29 INFO dfs.FSNamesystem: BLOCK* NameSystem.addStoredBlock: blockMap updated: 10.251.38.197:50010 is added to blk_8763662564934652249 size 67108864
081109 205742 1001 INFO dfs.DataNode$PacketResponder: Received block blk_-5861636720645142679 of size 67108864 from /10.251.70.211
081109 205746 29 INFO dfs.FSNamesystem: BLOCK* NameSystem.addStoredBlock: blockMap updated: 10.251.74.134:50010 is added to blk_7453815855294711849 size 67108864
081109 205749 997 INFO dfs.DataNode$DataXceiver: Receiving block blk_-28342503914935090 src: /10.251.123.132:57542 dest: /10.251.123.132:50010
081109 205754 952 INFO dfs.DataNode$PacketResponder: Received block blk_8291449241650212794 of size 67108864 from /10.251.89.155
081109 205858 31 INFO dfs.FSNamesystem: BLOCK* NameSystem.allocateBlock: /user/root/rand/_temporary/_task_200811092030_0001_m_000487_0/part-00487. blk_-5319073033164653435
081109 205931 13 INFO dfs.DataBlockScanner: Verification succeeded for blk_-4980916519894289629
081109 210022 1110 INFO dfs.DataNode$PacketResponder: Received block blk_-5974833545991408899 of size 67108864 from /10.251.31.180
081109 210037 1084 INFO dfs.DataNode$DataXceiver: Receiving block blk_-5009020203888190378 src: /10.251.199.19:52622 dest: /10.251.199.19:50010
081109 210248 1138 INFO dfs.DataNode$PacketResponder: Received block blk_6921674711959888070 of size 67108864 from /10.251.65.203
081109 210407 33 INFO dfs.FSNamesystem: BLOCK* NameSystem.addStoredBlock: blockMap updated: 10.250.7.244:50010 is added to blk_5165786360127153975 size 67108864
081109 210458 1278 INFO dfs.DataNode$DataXceiver: Receiving block blk_2937758977269298350 src: /10.251.194.129:37476 dest: /10.251.194.129:50010
081109 210551 32 INFO dfs.FSNamesystem: BLOCK* NameSystem.addStoredBlock: blockMap updated: 10.250.6.191:50010 is added to blk_673825774073966710 size 67108864
081109 210637 1283 INFO dfs.DataNode$PacketResponder: Received block blk_-7526945448667194862 of size 67108864 from /10.251.203.80

```

Output excerpt:

```text
[... 77 line(s) omitted ... ⟦tj:4bec07bb1d48c444a07b61c7e9766683⟧]
081109 214043 2561 WARN dfs.DataNode$DataXceiver: 10.251.30.85:50010:Got exception while serving blk_-2918118818249673980 to /10.251.90.64:  [×80 first 081109 214043, last 081111 014431]
[... 1922 line(s) omitted ... ⟦tj:277cd8998699c04f24456f04a5793caa⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (287848 bytes): call tinyjuice_retrieve with token "7c967000980c086ed55fa6544ba4f05f"]

```

### `17-apache-access`

- [Full input](cases/17-apache-access/input.log)
- [Output with CCR](cases/17-apache-access/output.log) - [diff](cases/17-apache-access/compression.diff)
- [Output without CCR](cases/17-apache-access/output-noccr.log) - [diff](cases/17-apache-access/compression-noccr.diff)

Input excerpt:

```text
83.149.9.216 - - [17/May/2015:10:05:03 +0000] "GET /presentations/logstash-monitorama-2013/images/kibana-search.png HTTP/1.1" 200 203023 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla/5.0 (Mac...
83.149.9.216 - - [17/May/2015:10:05:43 +0000] "GET /presentations/logstash-monitorama-2013/images/kibana-dashboard3.png HTTP/1.1" 200 171717 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla/5.0 ...
83.149.9.216 - - [17/May/2015:10:05:47 +0000] "GET /presentations/logstash-monitorama-2013/plugin/highlight/highlight.js HTTP/1.1" 200 26185 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla/5.0 ...
83.149.9.216 - - [17/May/2015:10:05:12 +0000] "GET /presentations/logstash-monitorama-2013/plugin/zoom-js/zoom.js HTTP/1.1" 200 7697 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla/5.0 (Macinto...
83.149.9.216 - - [17/May/2015:10:05:07 +0000] "GET /presentations/logstash-monitorama-2013/plugin/notes/notes.js HTTP/1.1" 200 2892 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla/5.0 (Macintos...
83.149.9.216 - - [17/May/2015:10:05:34 +0000] "GET /presentations/logstash-monitorama-2013/images/sad-medic.png HTTP/1.1" 200 430406 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla/5.0 (Macinto...
83.149.9.216 - - [17/May/2015:10:05:57 +0000] "GET /presentations/logstash-monitorama-2013/css/fonts/Roboto-Bold.ttf HTTP/1.1" 200 38720 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla/5.0 (Mac...
83.149.9.216 - - [17/May/2015:10:05:50 +0000] "GET /presentations/logstash-monitorama-2013/css/fonts/Roboto-Regular.ttf HTTP/1.1" 200 41820 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla/5.0 (...
83.149.9.216 - - [17/May/2015:10:05:24 +0000] "GET /presentations/logstash-monitorama-2013/images/frontend-response-codes.png HTTP/1.1" 200 52878 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla...
83.149.9.216 - - [17/May/2015:10:05:50 +0000] "GET /presentations/logstash-monitorama-2013/images/kibana-dashboard.png HTTP/1.1" 200 321631 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla/5.0 (...
83.149.9.216 - - [17/May/2015:10:05:46 +0000] "GET /presentations/logstash-monitorama-2013/images/Dreamhost_logo.svg HTTP/1.1" 200 2126 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla/5.0 (Maci...
83.149.9.216 - - [17/May/2015:10:05:11 +0000] "GET /presentations/logstash-monitorama-2013/images/kibana-dashboard2.png HTTP/1.1" 200 394967 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla/5.0 ...
83.149.9.216 - - [17/May/2015:10:05:19 +0000] "GET /presentations/logstash-monitorama-2013/images/apache-icon.gif HTTP/1.1" 200 8095 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla/5.0 (Macinto...
83.149.9.216 - - [17/May/2015:10:05:33 +0000] "GET /presentations/logstash-monitorama-2013/images/nagios-sms5.png HTTP/1.1" 200 78075 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla/5.0 (Macint...
83.149.9.216 - - [17/May/2015:10:05:00 +0000] "GET /presentations/logstash-monitorama-2013/images/redis.png HTTP/1.1" 200 25230 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla/5.0 (Macintosh; I...
83.149.9.216 - - [17/May/2015:10:05:25 +0000] "GET /presentations/logstash-monitorama-2013/images/elasticsearch.png HTTP/1.1" 200 8026 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla/5.0 (Macin...
83.149.9.216 - - [17/May/2015:10:05:59 +0000] "GET /presentations/logstash-monitorama-2013/images/logstashbook.png HTTP/1.1" 200 54662 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla/5.0 (Macin...
83.149.9.216 - - [17/May/2015:10:05:30 +0000] "GET /presentations/logstash-monitorama-2013/images/github-contributions.png HTTP/1.1" 200 34245 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla/5....
83.149.9.216 - - [17/May/2015:10:05:53 +0000] "GET /presentations/logstash-monitorama-2013/css/print/paper.css HTTP/1.1" 200 4254 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozilla/5.0 (Macintosh;...
83.149.9.216 - - [17/May/2015:10:05:24 +0000] "GET /presentations/logstash-monitorama-2013/images/1983_delorean_dmc-12-pic-38289.jpeg HTTP/1.1" 200 220562 "http://semicomplete.com/presentations/logstash-monitorama-2013/"...
83.149.9.216 - - [17/May/2015:10:05:54 +0000] "GET /presentations/logstash-monitorama-2013/images/simple-inputs-filters-outputs.jpg HTTP/1.1" 200 1168622 "http://semicomplete.com/presentations/logstash-monitorama-2013/" ...
83.149.9.216 - - [17/May/2015:10:05:33 +0000] "GET /presentations/logstash-monitorama-2013/images/tiered-outputs-to-inputs.jpg HTTP/1.1" 200 1079983 "http://semicomplete.com/presentations/logstash-monitorama-2013/" "Mozi...
83.149.9.216 - - [17/May/2015:10:05:56 +0000] "GET /favicon.ico HTTP/1.1" 200 3638 "-" "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/32.0.1700.77 Safari/537.36"
24.236.252.67 - - [17/May/2015:10:05:40 +0000] "GET /favicon.ico HTTP/1.1" 200 3638 "-" "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:26.0) Gecko/20100101 Firefox/26.0"
93.114.45.13 - - [17/May/2015:10:05:14 +0000] "GET /articles/dynamic-dns-with-dhcp/ HTTP/1.1" 200 18848 "http://www.google.ro/url?sa=t&rct=j&q=&esrc=s&source=web&cd=2&ved=0CCwQFjAB&url=http%3A%2F%2Fwww.semicomplete.com%2...
93.114.45.13 - - [17/May/2015:10:05:04 +0000] "GET /reset.css HTTP/1.1" 200 1015 "http://www.semicomplete.com/articles/dynamic-dns-with-dhcp/" "Mozilla/5.0 (X11; Linux x86_64; rv:25.0) Gecko/20100101 Firefox/25.0"
93.114.45.13 - - [17/May/2015:10:05:45 +0000] "GET /style2.css HTTP/1.1" 200 4877 "http://www.semicomplete.com/articles/dynamic-dns-with-dhcp/" "Mozilla/5.0 (X11; Linux x86_64; rv:25.0) Gecko/20100101 Firefox/25.0"
93.114.45.13 - - [17/May/2015:10:05:14 +0000] "GET /favicon.ico HTTP/1.1" 200 3638 "-" "Mozilla/5.0 (X11; Linux x86_64; rv:25.0) Gecko/20100101 Firefox/25.0"
93.114.45.13 - - [17/May/2015:10:05:17 +0000] "GET /images/jordan-80.png HTTP/1.1" 200 6146 "http://www.semicomplete.com/articles/dynamic-dns-with-dhcp/" "Mozilla/5.0 (X11; Linux x86_64; rv:25.0) Gecko/20100101 Firefox/2...
93.114.45.13 - - [17/May/2015:10:05:21 +0000] "GET /images/web/2009/banner.png HTTP/1.1" 200 52315 "http://www.semicomplete.com/style2.css" "Mozilla/5.0 (X11; Linux x86_64; rv:25.0) Gecko/20100101 Firefox/25.0"
66.249.73.135 - - [17/May/2015:10:05:40 +0000] "GET /blog/tags/ipv6 HTTP/1.1" 200 12251 "-" "Mozilla/5.0 (iPhone; CPU iPhone OS 6_0 like Mac OS X) AppleWebKit/536.26 (KHTML, like Gecko) Version/6.0 Mobile/10A5376e Safari...
50.16.19.13 - - [17/May/2015:10:05:10 +0000] "GET /blog/tags/puppet?flav=rss20 HTTP/1.1" 200 14872 "http://www.semicomplete.com/blog/tags/puppet?flav=rss20" "Tiny Tiny RSS/1.11 (http://tt-rss.org/)"
66.249.73.185 - - [17/May/2015:10:05:37 +0000] "GET / HTTP/1.1" 200 37932 "-" "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)"
110.136.166.128 - - [17/May/2015:10:05:35 +0000] "GET /projects/xdotool/ HTTP/1.1" 200 12292 "http://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=5&cad=rja&sqi=2&ved=0CFYQFjAE&url=http%3A%2F%2Fwww.semicomplete.c...
46.105.14.53 - - [17/May/2015:10:05:03 +0000] "GET /blog/tags/puppet?flav=rss20 HTTP/1.1" 200 14872 "-" "UniversalFeedParser/4.2-pre-314-svn +http://feedparser.org/"
110.136.166.128 - - [17/May/2015:10:05:06 +0000] "GET /reset.css HTTP/1.1" 200 1015 "http://www.semicomplete.com/projects/xdotool/" "Mozilla/5.0 (Windows NT 6.2; WOW64; rv:28.0) Gecko/20100101 Firefox/28.0"

```

Output excerpt:

```text
[... 2295 line(s) omitted ... ⟦tj:e1c7e2bf97098762c6554826bdb27b7f⟧]
66.249.73.185 - - [18/May/2015:05:05:38 +0000] "GET /files/firefox-tabsearch/ HTTP/1.1" 200 1865 "-" "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)"
66.249.73.185 - - [18/May/2015:05:05:35 +0000] "GET /files/pam_logfailure/ HTTP/1.1" 200 992 "-" "DoCoMo/2.0 N905i(c100;TB;W24H16) (compatible; Googlebot-Mobile/2.1; +http://www.google.com/bot.html)"
66.249.73.135 - - [18/May/2015:05:05:49 +0000] "GET /blog/geekery/ie-javascript-error.html HTTP/1.1" 200 8600 "-" "Mozilla/5.0 (iPhone; CPU iPhone OS 6_0 like Mac OS X) AppleWebKit/536.26 (KHTML, like Gecko) Version/6.0 ...
76.203.196.208 - - [18/May/2015:05:05:41 +0000] "GET /files/rubygems615/java-ssl-debug-last-request.txt HTTP/1.1" 200 66717 "http://www.google.com/url?sa=t&rct=j&q=what%20is%20affirmtrust%20premium%20on%20blackberry&sour...
209.85.238.199 - - [18/May/2015:05:05:22 +0000] "GET / HTTP/1.1" 200 37932 "-" "Feedfetcher-Google; (+http://www.google.com/feedfetcher.html; 1 subscribers; feed-id=8003088278248648013)"
[... 200 line(s) omitted ... ⟦tj:045645bef857546fb0c784dae2505b98⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (577981 bytes): call tinyjuice_retrieve with token "4bdb559eee69e9cd66b1271b4079d3bf"]

```

### `11-healthapp`

- [Full input](cases/11-healthapp/input.log)
- [Output with CCR](cases/11-healthapp/output.log) - [diff](cases/11-healthapp/compression.diff)
- [Output without CCR](cases/11-healthapp/output-noccr.log) - [diff](cases/11-healthapp/compression-noccr.diff)

Input excerpt:

```text
20171223-22:15:29:606|Step_LSC|30002312|onStandStepChanged 3579
20171223-22:15:29:615|Step_LSC|30002312|onExtend:1514038530000 14 0 4
20171223-22:15:29:633|Step_StandReportReceiver|30002312|onReceive action: android.intent.action.SCREEN_ON
20171223-22:15:29:635|Step_LSC|30002312|processHandleBroadcastAction action:android.intent.action.SCREEN_ON
20171223-22:15:29:635|Step_StandStepCounter|30002312|flush sensor data
20171223-22:15:29:635|Step_SPUtils|30002312| getTodayTotalDetailSteps = 1514038440000##6993##548365##8661##12266##27164404
20171223-22:15:29:636|Step_SPUtils|30002312|setTodayTotalDetailSteps=1514038440000##7007##548365##8661##12361##27173954
20171223-22:15:29:636|Step_LSC|30002312|onStandStepChanged 3579
20171223-22:15:29:645|Step_ExtSDM|30002312|calculateCaloriesWithCache totalCalories=126775
20171223-22:15:29:648|Step_ExtSDM|30002312|calculateAltitudeWithCache totalAltitude=240
20171223-22:15:29:649|Step_StandReportReceiver|30002312|REPORT : 7007 5002 150089 240
20171223-22:15:29:737|Step_LSC|30002312|onExtend:1514038530000 0 0 4
20171223-22:15:29:738|Step_LSC|30002312|onStandStepChanged 3579
20171223-22:15:29:792|Step_LSC|30002312|onStandStepChanged 3580
20171223-22:15:29:800|Step_LSC|30002312|onExtend:1514038530000 1 0 4
20171223-22:15:29:950|Step_SPUtils|30002312| getTodayTotalDetailSteps = 1514038440000##7007##548365##8661##12361##27173954
20171223-22:15:29:950|Step_SPUtils|30002312|setTodayTotalDetailSteps=1514038440000##7008##548365##8661##12456##27174269
20171223-22:15:29:959|Step_ExtSDM|30002312|calculateCaloriesWithCache totalCalories=126797
20171223-22:15:29:962|Step_ExtSDM|30002312|calculateAltitudeWithCache totalAltitude=240
20171223-22:15:29:962|Step_StandReportReceiver|30002312|REPORT : 7008 5003 150111 240
20171223-22:15:30:331|Step_LSC|30002312|onStandStepChanged 3581
20171223-22:15:30:335|Step_LSC|30002312|onExtend:1514038531000 1 0 4
20171223-22:15:30:632|Step_SPUtils|30002312| getTodayTotalDetailSteps = 1514038440000##7008##548365##8661##12456##27174269
20171223-22:15:30:632|Step_SPUtils|30002312|setTodayTotalDetailSteps=1514038440000##7009##548365##8661##12551##27174951
20171223-22:15:30:639|Step_ExtSDM|30002312|calculateCaloriesWithCache totalCalories=126818
20171223-22:15:30:641|Step_ExtSDM|30002312|calculateAltitudeWithCache totalAltitude=240
20171223-22:15:30:642|Step_StandReportReceiver|30002312|REPORT : 7009 5004 150132 240
20171223-22:15:30:841|Step_LSC|30002312|onStandStepChanged 3583
20171223-22:15:30:858|Step_LSC|30002312|onExtend:1514038531000 2 0 4
20171223-22:15:31:142|Step_SPUtils|30002312| getTodayTotalDetailSteps = 1514038440000##7009##548365##8661##12551##27174951
20171223-22:15:31:143|Step_SPUtils|30002312|setTodayTotalDetailSteps=1514038440000##7011##548365##8661##12646##27175461
20171223-22:15:31:157|Step_ExtSDM|30002312|calculateCaloriesWithCache totalCalories=126861
20171223-22:15:31:160|Step_ExtSDM|30002312|calculateAltitudeWithCache totalAltitude=240
20171223-22:15:31:160|Step_StandReportReceiver|30002312|REPORT : 7011 5005 150175 240
20171223-22:15:31:841|Step_LSC|30002312|onStandStepChanged 3584
20171223-22:15:31:862|Step_LSC|30002312|onExtend:1514038532000 1 0 4

```

Output excerpt:

```text
[... 721 line(s) omitted ... ⟦tj:42e36b625eca85939ad149bd3b0b4613⟧]
20171223-22:19:58:380|HiH_HiHealthDataInsertStore|30002312|saveHealthDetailData() saveOneDetailData fail hiHealthData = 1513958400000,type = 40003  [×2]
[... 7 line(s) omitted ... ⟦tj:fff37f786c794df288b851900a7b935c⟧]
20171223-22:19:58:411|HiH_HiHealthBinder|30002312|insertHiHealthData() bulkSaveDetailHiHealthData fail errorCode = 4,errorMessage = ERR_DATA_INSERT 
20171223-22:19:58:411|HiH_HiHealthBinder|30002312|insertHiHealthData() end totalTime = 45
20171223-22:19:58:411|Step_LSC|30002312|uploadStaticsToDB() onResult  type = 4 obj=true
20171223-22:19:58:411|Step_LSC|30002312|uploadStaticsToDB failed message=true
20171223-22:19:58:413|HiH_HiSyncControl|30002312|checkInsertStatus stepSum or calorieSum is enough
20171223-22:19:58:413|HiH_HiAppUtil|30002312|getBinderPackageName packageName = com.huawei.health
[... 1265 line(s) omitted ... ⟦tj:77b1e5c2249a75a828cdd7dae21a371f⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (187456 bytes): call tinyjuice_retrieve with token "95ec36322f5db1e6faaab764c568b670"]

```

### `34-jvm-gc`

- [Full input](cases/34-jvm-gc/input.log)
- [Output with CCR](cases/34-jvm-gc/output.log) - [diff](cases/34-jvm-gc/compression.diff)
- [Output without CCR](cases/34-jvm-gc/output-noccr.log) - [diff](cases/34-jvm-gc/compression-noccr.diff)

Input excerpt:

```text
0.356: [GC pause (young) 4096K->3936K(16M), 0.0121737 secs]
0.374: [GC pause (young) 7008K->7008K(16M), 0.0071930 secs]
0.385: [GC pause (young) 9056K->9056K(16M), 0.0024203 secs]
0.388: [GC pause (young) (initial-mark) 10080K->10080K(16M)0.390: [GC concurrent-mark-start]
, 0.0013065 secs]
0.391: [GC pause (young) 10M->10M(16M), 0.0015247 secs]
0.393: [GC pause (young) 11M->11M(16M), 0.0012886 secs]
0.396: [GC pause (young) 12M->12M(16M), 0.0013073 secs]
0.398: [GC pause (young) 13M->13M(16M), 0.0012917 secs]
0.401: [GC pause (young) 14M->14M(16M), 0.0012613 secs]
0.403: [GC concurrent-mark-end, 0.0126670 sec]
0.403: [GC pause (young) 15M->15M(17M), 0.0022962 secs]
0.406: [GC remark, 0.0006116 secs]
0.407: [GC pause (young) 16M->16M(18M), 0.0025739 secs]
0.410: [GC concurrent-count-start]
0.412: [GC concurrent-count-end, 0.0019007]
0.412: [GC pause (young) 17M->17M(25M), 0.0019315 secs]
0.414: [GC cleanup 17M->17M(31M), 0.0003415 secs]
0.423: [GC pause (young) 18M->18M(31M), 0.0020804 secs]
0.428: [GC pause (young) (initial-mark) 20M->20M(36M), 0.0031446 secs]
0.431: [GC concurrent-mark-start]
0.433: [GC pause (young) 22M->22M(40M), 0.0032816 secs]
0.438: [GC concurrent-mark-end, 0.0066198 sec]
0.438: [GC remark, 0.0004276 secs]
0.439: [GC concurrent-count-start]
0.441: [GC concurrent-count-end, 0.0026967]
0.441: [GC pause (young) 24M->24M(43M), 0.0029821 secs]
0.445: [GC cleanup 24M->24M(45M), 0.0004133 secs]
0.455: [GC pause (young) 27M->27M(45M), 0.0037917 secs]
0.461: [GC pause (young) (initial-mark) 29M->29M(47M), 0.0039982 secs]
0.465: [GC concurrent-mark-start]
0.467: [GC pause (young) 31M->31M(48M), 0.0047062 secs]
0.475: [GC pause (young) 33M->33M(49M), 0.0021363 secs]
0.480: [GC pause (young) 35M->35M(50M), 0.0020271 secs]
0.484: [GC pause (young) 37M->37M(51M), 0.0026350 secs]
0.490: [GC pause (young) 39M->39M(52M), 0.0037867 secs]

```

Output excerpt:

```text
[... 760 line(s) omitted ... ⟦tj:1d285b093b9214353cdcebafcff8d785⟧]
15.925: [GC pause (young)-- 254M->255M(256M), 0.0023087 secs]
15.928: [Full GC 255M->32M(109M), 0.0585270 secs]
15.987: [GC concurrent-mark-abort]
15.999: [GC pause (young) 51M->51M(109M), 0.0181285 secs]
16.022: [GC pause (young) 58M->58M(139M), 0.0084855 secs]
[... 310 line(s) omitted ... ⟦tj:8df719a2a2049eebb0b33d7c5eca2c11⟧]
22.908: [GC pause (young)-- 254M->255M(256M), 0.0022138 secs]
22.917: [Full GC 255M->33M(112M), 0.0634108 secs]
22.980: [GC concurrent-mark-abort]
22.994: [GC pause (young) 52M->52M(112M), 0.0157362 secs]
23.014: [GC pause (young) 60M->60M(141M), 0.0085106 secs]
[... 102 line(s) omitted ... ⟦tj:43506b06c1700a0ce5d4bd3ee624361a⟧]
25.685: [GC pause (young)-- 254M->256M(256M), 0.0014250 secs]
25.703: [Full GC 256M->21M(72M), 0.0803183 secs]
25.792: [GC concurrent-mark-abort]
25.802: [GC pause (young) 34M->34M(72M), 0.0115899 secs]
25.818: [GC pause (young) 43M->43M(109M), 0.0086895 secs]
[... 534 line(s) omitted ... ⟦tj:7ba1f3e60069245d413b7bdf4c672bd7⟧]
37.220: [GC pause (young)-- 254M->255M(256M), 0.0367172 secs]
37.257: [Full GC 255M->27M(92M), 0.1115957 secs]
37.369: [GC concurrent-mark-abort]
37.388: [GC pause (young) 43M->43M(92M), 0.0263047 secs]
37.419: [GC pause (young) 49M->49M(125M), 0.0133398 secs]
[... 1241 line(s) omitted ... ⟦tj:ed59e785966a7e4b00b62c03951fd368⟧]
77.250: [GC pause (young)-- 254M->255M(256M), 0.0041810 secs]
77.256: [Full GC 255M->25M(87M), 0.1581177 secs]
77.417: [GC concurrent-mark-abort]
77.435: [GC pause (young) 40M->40M(87M), 0.0249578 secs]
77.472: [GC pause (young) 51M->51M(121M), 0.0231226 secs]
[... 1489 line(s) omitted ... ⟦tj:a45b159ecd7aeb2e53b14da101ab1329⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (235561 bytes): call tinyjuice_retrieve with token "d8744977a631986bd19845f464437fa3"]

```

### `04-zookeeper`

- [Full input](cases/04-zookeeper/input.log)
- [Output with CCR](cases/04-zookeeper/output.log) - [diff](cases/04-zookeeper/compression.diff)
- [Output without CCR](cases/04-zookeeper/output-noccr.log) - [diff](cases/04-zookeeper/compression-noccr.diff)

Input excerpt:

```text
2015-07-29 17:41:44,747 - INFO  [QuorumPeer[myid=1]/0:0:0:0:0:0:0:0:2181:FastLeaderElection@774] - Notification time out: 3200
2015-07-29 19:04:12,394 - INFO  [/10.10.34.11:3888:QuorumCnxManager$Listener@493] - Received connection request /10.10.34.11:45307
2015-07-29 19:04:29,071 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@688] - Send worker leaving thread
2015-07-29 19:04:29,079 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@679] - Interrupted while waiting for message on queue
2015-07-29 19:13:17,524 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@688] - Send worker leaving thread
2015-07-29 19:13:24,282 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@762] - Connection broken for id 188978561024, my id = 1, error = 
2015-07-29 19:13:24,370 - INFO  [/10.10.34.11:3888:QuorumCnxManager$Listener@493] - Received connection request /10.10.34.13:57707
2015-07-29 19:13:27,721 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@762] - Connection broken for id 188978561024, my id = 1, error = 
2015-07-29 19:13:34,382 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@679] - Interrupted while waiting for message on queue
2015-07-29 19:13:37,626 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@688] - Send worker leaving thread
2015-07-29 19:13:44,301 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@688] - Send worker leaving thread
2015-07-29 19:13:47,731 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@762] - Connection broken for id 188978561024, my id = 1, error = 
2015-07-29 19:13:54,220 - INFO  [/10.10.34.11:3888:QuorumCnxManager$Listener@493] - Received connection request /10.10.34.11:45382
2015-07-29 19:13:54,399 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@762] - Connection broken for id 188978561024, my id = 1, error = 
2015-07-29 19:14:04,406 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@679] - Interrupted while waiting for message on queue
2015-07-29 19:14:07,559 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@765] - Interrupting SendWorker
2015-07-29 19:14:07,653 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@688] - Send worker leaving thread
2015-07-29 19:14:24,329 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@765] - Interrupting SendWorker
2015-07-29 19:14:37,585 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@679] - Interrupted while waiting for message on queue
2015-07-29 19:14:44,256 - INFO  [/10.10.34.11:3888:QuorumCnxManager$Listener@493] - Received connection request /10.10.34.11:45440
2015-07-29 19:14:47,593 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@765] - Interrupting SendWorker
2015-07-29 19:14:54,354 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@688] - Send worker leaving thread
2015-07-29 19:15:24,476 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@679] - Interrupted while waiting for message on queue
2015-07-29 19:15:37,647 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@765] - Interrupting SendWorker
2015-07-29 19:15:37,648 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@688] - Send worker leaving thread
2015-07-29 19:15:54,407 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@679] - Interrupted while waiting for message on queue
2015-07-29 19:15:57,854 - INFO  [/10.10.34.11:3888:QuorumCnxManager$Listener@493] - Received connection request /10.10.34.13:57895
2015-07-29 19:16:04,412 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@679] - Interrupted while waiting for message on queue
2015-07-29 19:16:04,414 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@679] - Interrupted while waiting for message on queue
2015-07-29 19:16:07,659 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@765] - Interrupting SendWorker
2015-07-29 19:16:14,520 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@679] - Interrupted while waiting for message on queue
2015-07-29 19:16:24,348 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@762] - Connection broken for id 188978561024, my id = 1, error = 
2015-07-29 19:16:27,865 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@679] - Interrupted while waiting for message on queue
2015-07-29 19:16:27,865 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@688] - Send worker leaving thread
2015-07-29 19:16:34,433 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@762] - Connection broken for id 188978561024, my id = 1, error = 
2015-07-29 19:16:44,440 - INFO  [/10.10.34.11:3888:QuorumCnxManager$Listener@493] - Received connection request /10.10.34.12:47727

```

Output excerpt:

```text
2015-07-29 17:41:44,747 - INFO  [QuorumPeer[myid=1]/0:0:0:0:0:0:0:0:2181:FastLeaderElection@774] - Notification time out: 3200
2015-07-29 19:04:12,394 - INFO  [/10.10.34.11:3888:QuorumCnxManager$Listener@493] - Received connection request /10.10.34.11:45307
2015-07-29 19:04:29,071 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@688] - Send worker leaving thread  [×262 first 2015-07-29, last 2015-07-29]
2015-07-29 19:04:29,079 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@679] - Interrupted while waiting for message on queue  [×314 first 2015-07-29, last 2015-07-29]
2015-07-29 19:13:17,524 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@688] - Send worker leaving thread
2015-07-29 19:13:24,282 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@762] - Connection broken for id 188978561024, my id = 1, error =   [×289 first 2015-07-29, last 2015-07-29]
[... 9 line(s) omitted ... ⟦tj:29fc0d80898573eb60dc6da52c19392c⟧]
2015-07-29 19:14:07,559 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@765] - Interrupting SendWorker  [×265 first 2015-07-29, last 2015-07-29]
[... 479 line(s) omitted ... ⟦tj:4c3e6cc2b5264f5e2b43e6e617b35df4⟧]
2015-07-29 19:52:05,118 - WARN  [NIOServerCxn.Factory:0.0.0.0/0.0.0.0:2181:NIOServerCnxn@349] - caught end of stream exception  [×4 first 2015-07-29, last 2015-07-29]
[... 7 line(s) omitted ... ⟦tj:935197bb725fffd2a7a66daa9ac317cb⟧]
2015-07-29 21:39:28,234 - INFO  [NIOServerCxn.Factory:0.0.0.0/0.0.0.0:2181:NIOServerCnxnFactory@197] - Accepted socket connection from /10.10.34.26:56952
2015-07-29 23:44:21,576 - INFO  [NIOServerCxn.Factory:0.0.0.0/0.0.0.0:2181:NIOServerCnxn@1001] - Closed socket connection for client /10.10.34.11:49557 which had sessionid 0x14ed93111f20048
2015-07-29 23:44:28,903 - ERROR [CommitProcessor:1:NIOServerCnxn@180] - Unexpected Exception: 
2015-07-29 23:52:08,962 - INFO  [CommitProcessor:1:ZooKeeperServer@595] - Established session 0x14ed93111f2005b with negotiated timeout 10000 for client /10.10.34.28:52117
2015-07-29 23:52:09,163 - INFO  [NIOServerCxn.Factory:0.0.0.0/0.0.0.0:2181:NIOServerCnxnFactory@197] - Accepted socket connection from /10.10.34.30:38562
[... 14 line(s) omitted ... ⟦tj:77254eefea70d6c61dba1dfd7b5508ca⟧]
2015-07-30 16:12:01,554 - WARN  [NIOServerCxn.Factory:0.0.0.0/0.0.0.0:2181:ZooKeeperServer@793] - Connection request from old client /10.10.34.19:33442; will be dropped if server is in r-o mode  [×8 first 2015-07-30, las...
[... 12 line(s) omitted ... ⟦tj:de2f7541ced04ff3e049a4e599a7d8f0⟧]
2015-07-30 17:11:54,937 - WARN  [NIOServerCxn.Factory:0.0.0.0/0.0.0.0:2181:NIOServerCnxn@349] - caught end of stream exception  [×6 first 2015-07-30, last 2015-07-30]
[... 4 line(s) omitted ... ⟦tj:183ab4005a7776f3e90fd774855b5a7b⟧]
2015-07-30 17:49:05,943 - WARN  [NIOServerCxn.Factory:0.0.0.0/0.0.0.0:2181:ZooKeeperServer@793] - Connection request from old client /10.10.34.12:45728; will be dropped if server is in r-o mode  [×5 first 2015-07-30, las...
2015-07-30 17:55:26,200 - WARN  [WorkerSender[myid=1]:QuorumCnxManager@368] - Cannot open channel to 2 at election address /10.10.34.12:3888  [×2 first 2015-07-30, last 2015-07-29]
[... 5 line(s) omitted ... ⟦tj:6d3265b7b0a5d49ab5d51955d6348d3a⟧]
2015-07-30 19:59:02,538 - WARN  [NIOServerCxn.Factory:0.0.0.0/0.0.0.0:2181:ZooKeeperServer@793] - Connection request from old client /10.10.34.13:38053; will be dropped if server is in r-o mode  [×4 first 2015-07-30, las...
[... 204 line(s) omitted ... ⟦tj:6c9aeb079d74b5ca62c30b9ad5e68166⟧]
2015-08-25 11:21:22,561 - WARN  [WorkerSender[myid=1]:QuorumCnxManager@368] - Cannot open channel to 3 at election address /10.10.34.13:3888
2015-07-29 17:42:30,405 - INFO  [QuorumPeer[myid=2]/0:0:0:0:0:0:0:0:2181:Environment@100] - Server environment:java.vendor=Oracle Corporation
2015-07-29 19:03:35,413 - ERROR [LearnerHandler-/10.10.34.11:52225:LearnerHandler@562] - Unexpected exception causing shutdown while sock still open  [×12 first 2015-07-29, last 2015-07-29]
2015-07-29 19:03:54,584 - ERROR [LearnerHandler-/10.10.34.11:52241:LearnerHandler@562] - Unexpected exception causing shutdown while sock still open
2015-07-29 19:04:30,989 - WARN  [LearnerHandler-/10.10.34.11:52264:LearnerHandler@575] - ******* GOODBYE /10.10.34.11:52264 ********
[... 1243 line(s) omitted ... ⟦tj:778a3f8773e49f521a217ca2b493ff9e⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (279891 bytes): call tinyjuice_retrieve with token "e40e0af5ef9eb6e4097200f260b9d1f6"]

```

### `30-laravel-app`

- [Full input](cases/30-laravel-app/input.log)
- [Output with CCR](cases/30-laravel-app/output.log) - [diff](cases/30-laravel-app/compression.diff)
- [Output without CCR](cases/30-laravel-app/output-noccr.log) - [diff](cases/30-laravel-app/compression-noccr.diff)

Input excerpt:

```text
[2023-05-15 18:19:32] local.ERROR: Target class [Fruitcake\Cors\HandleCors] does not exist. {"exception":"[object] (Illuminate\\Contracts\\Container\\BindingResolutionException(code: 0): Target class [Fruitcake\\Cors\\Ha...
[stacktrace]
#0 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Container/Container.php(795): Illuminate\\Container\\Container->build('Fruitcake\\\\Cors\\\\...')
#1 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Foundation/Application.php(933): Illuminate\\Container\\Container->resolve('Fruitcake\\\\Cors\\\\...', Array, true)
#2 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Container/Container.php(731): Illuminate\\Foundation\\Application->resolve('Fruitcake\\\\Cors\\\\...', Array)
#3 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Foundation/Application.php(918): Illuminate\\Container\\Container->make('Fruitcake\\\\Cors\\\\...', Array)
#4 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Pipeline/Pipeline.php(169): Illuminate\\Foundation\\Application->make('Fruitcake\\\\Cors\\\\...')
#5 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Http/Middleware/TrustProxies.php(39): Illuminate\\Pipeline\\Pipeline->Illuminate\\Pipeline\\{closure}(Object(Illuminate\\Http\\Request))
#6 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Pipeline/Pipeline.php(180): Illuminate\\Http\\Middleware\\TrustProxies->handle(Object(Illuminate\\Http\\Request), Object(Closure))
#7 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Pipeline/Pipeline.php(116): Illuminate\\Pipeline\\Pipeline->Illuminate\\Pipeline\\{closure}(Object(Illuminate\\Http\\Request))
#8 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Foundation/Http/Kernel.php(175): Illuminate\\Pipeline\\Pipeline->then(Object(Closure))
#9 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Foundation/Http/Kernel.php(144): Illuminate\\Foundation\\Http\\Kernel->sendRequestThroughRouter(Object(Illuminate\\Http\\Request))
#10 /Users/zhebaoting/Sites/laravel/crm/public/index.php(54): Illuminate\\Foundation\\Http\\Kernel->handle(Object(Illuminate\\Http\\Request))
#11 /Users/zhebaoting/.composer/vendor/laravel/valet/server.php(250): require('/Users/zhebaoti...')
#12 {main}

[previous exception] [object] (ReflectionException(code: -1): Class \"Fruitcake\\Cors\\HandleCors\" does not exist at /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Container/Container.php:91...
[stacktrace]
#0 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Container/Container.php(914): ReflectionClass->__construct('Fruitcake\\\\Cors\\\\...')
#1 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Container/Container.php(795): Illuminate\\Container\\Container->build('Fruitcake\\\\Cors\\\\...')
#2 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Foundation/Application.php(933): Illuminate\\Container\\Container->resolve('Fruitcake\\\\Cors\\\\...', Array, true)
#3 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Container/Container.php(731): Illuminate\\Foundation\\Application->resolve('Fruitcake\\\\Cors\\\\...', Array)
#4 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Foundation/Application.php(918): Illuminate\\Container\\Container->make('Fruitcake\\\\Cors\\\\...', Array)
#5 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Pipeline/Pipeline.php(169): Illuminate\\Foundation\\Application->make('Fruitcake\\\\Cors\\\\...')
#6 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Http/Middleware/TrustProxies.php(39): Illuminate\\Pipeline\\Pipeline->Illuminate\\Pipeline\\{closure}(Object(Illuminate\\Http\\Request))
#7 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Pipeline/Pipeline.php(180): Illuminate\\Http\\Middleware\\TrustProxies->handle(Object(Illuminate\\Http\\Request), Object(Closure))
#8 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Pipeline/Pipeline.php(116): Illuminate\\Pipeline\\Pipeline->Illuminate\\Pipeline\\{closure}(Object(Illuminate\\Http\\Request))
#9 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Foundation/Http/Kernel.php(175): Illuminate\\Pipeline\\Pipeline->then(Object(Closure))
#10 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Foundation/Http/Kernel.php(144): Illuminate\\Foundation\\Http\\Kernel->sendRequestThroughRouter(Object(Illuminate\\Http\\Request))
#11 /Users/zhebaoting/Sites/laravel/crm/public/index.php(54): Illuminate\\Foundation\\Http\\Kernel->handle(Object(Illuminate\\Http\\Request))
#12 /Users/zhebaoting/.composer/vendor/laravel/valet/server.php(250): require('/Users/zhebaoti...')
#13 {main}
"} 
[2023-05-15 18:19:33] local.ERROR: Target class [Fruitcake\Cors\HandleCors] does not exist. {"exception":"[object] (Illuminate\\Contracts\\Container\\BindingResolutionException(code: 0): Target class [Fruitcake\\Cors\\Ha...
[stacktrace]
#0 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Container/Container.php(795): Illuminate\\Container\\Container->build('Fruitcake\\\\Cors\\\\...')

```

Output excerpt:

```text
[2023-05-15 18:19:32] local.ERROR: Target class [Fruitcake\Cors\HandleCors] does not exist. {"exception":"[object] (Illuminate\\Contracts\\Container\\BindingResolutionException(code: 0): Target class [Fruitcake\\Cors\\Ha...
[stacktrace]
#0 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Container/Container.php(795): Illuminate\\Container\\Container->build('Fruitcake\\\\Cors\\\\...')
[... 11 line(s) omitted ... ⟦tj:dd941a6c3f85f4fa1de238cf921ed3d8⟧]
#12 {main}

[previous exception] [object] (ReflectionException(code: -1): Class \"Fruitcake\\Cors\\HandleCors\" does not exist at /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Container/Container.php:91...
[stacktrace]
#0 /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Container/Container.php(914): ReflectionClass->__construct('Fruitcake\\\\Cors\\\\...')
[... 37 line(s) omitted ... ⟦tj:c6e4ba07ec58ca4e33c7572be7274fea⟧]
#9 {main}
"} 
[2023-05-15 18:19:49] local.ERROR: Uncaught ReflectionException: Class "App\Providers\App\Policies\CustomerPolicy" does not exist in Command line code:1  [×29 first 18:19:49, last 18:47:49]
Stack trace:
#0 Command line code(1): ReflectionClass->__construct('App\\Providers\\A...')
#1 [internal function]: {closure}('App\\Providers\\A...', 'App\\Providers\\A...')
#2 Command line code(1): array_map(Object(Closure), Array, Array)
#3 {main}
  thrown {"exception":"[object] (Symfony\\Component\\ErrorHandler\\Error\\FatalError(code: 0): Uncaught ReflectionException: Class \"App\\Providers\\App\\Policies\\CustomerPolicy\" does not exist in Command line code:1  ...
Stack trace:
#0 Command line code(1): ReflectionClass->__construct('App\\\\Providers\\\\A...')
[... 293 line(s) omitted ... ⟦tj:117408dc3f2c2f60f973fecdbc77b21a⟧]
#0 {main}
"} 
[2023-05-15 18:38:12] local.ERROR: Unclosed '{' on line 62 does not match ')' {"exception":"[object] (ParseError(code: 0): Unclosed '{' on line 62 does not match ')' at /Users/zhebaoting/Sites/laravel/crm/app/Admin/boots...
[stacktrace]
#0 /Users/zhebaoting/Sites/laravel/crm/vendor/dcat/laravel-admin/src/Http/Middleware/Bootstrap.php(14): Dcat\\Admin\\Http\\Middleware\\Bootstrap->includeBootstrapFile()
[... 66 line(s) omitted ... ⟦tj:62a73557b7b0ddb7cfd96752c5126292⟧]
#0 {main}
"} 
[2023-05-15 18:39:13] local.ERROR: Unmatched '}' {"exception":"[object] (ParseError(code: 0): Unmatched '}' at /Users/zhebaoting/Sites/laravel/crm/app/Admin/bootstrap.php:65)
[stacktrace]
#0 /Users/zhebaoting/Sites/laravel/crm/vendor/dcat/laravel-admin/src/Http/Middleware/Bootstrap.php(14): Dcat\\Admin\\Http\\Middleware\\Bootstrap->includeBootstrapFile()
[... 194 line(s) omitted ... ⟦tj:27e031b64d7b07ab7c58556679765f1d⟧]
#0 {main}
"} 

```

### `09-linux`

- [Full input](cases/09-linux/input.log)
- [Output with CCR](cases/09-linux/output.log) - [diff](cases/09-linux/compression.diff)
- [Output without CCR](cases/09-linux/output-noccr.log) - [diff](cases/09-linux/compression-noccr.diff)

Input excerpt:

```text
Jun 14 15:16:01 combo sshd(pam_unix)[19939]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 14 15:16:02 combo sshd(pam_unix)[19937]: check pass; user unknown
Jun 14 15:16:02 combo sshd(pam_unix)[19937]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 15 02:04:59 combo sshd(pam_unix)[20882]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20884]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20883]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20885]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20886]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20892]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20893]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20896]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20897]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20898]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 04:06:18 combo su(pam_unix)[21416]: session opened for user cyrus by (uid=0)
Jun 15 04:06:19 combo su(pam_unix)[21416]: session closed for user cyrus
Jun 15 04:06:20 combo logrotate: ALERT exited abnormally with [1]
Jun 15 04:12:42 combo su(pam_unix)[22644]: session opened for user news by (uid=0)
Jun 15 04:12:43 combo su(pam_unix)[22644]: session closed for user news
Jun 15 12:12:34 combo sshd(pam_unix)[23397]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23397]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 15 12:12:34 combo sshd(pam_unix)[23395]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23395]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 15 12:12:34 combo sshd(pam_unix)[23404]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23404]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 15 12:12:34 combo sshd(pam_unix)[23399]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23399]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 15 12:12:34 combo sshd(pam_unix)[23406]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23406]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 15 12:12:34 combo sshd(pam_unix)[23396]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23394]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23407]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23394]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 15 12:12:34 combo sshd(pam_unix)[23403]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23396]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 15 12:12:34 combo sshd(pam_unix)[23407]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 15 12:12:34 combo sshd(pam_unix)[23403]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 

```

Output excerpt:

```text
Jun 14 15:16:01 combo sshd(pam_unix)[19939]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 14 15:16:02 combo sshd(pam_unix)[19937]: check pass; user unknown
Jun 14 15:16:02 combo sshd(pam_unix)[19937]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 15 02:04:59 combo sshd(pam_unix)[20882]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20884]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20883]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20885]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20886]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20892]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20893]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20896]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20897]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20898]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 04:06:18 combo su(pam_unix)[21416]: session opened for user cyrus by (uid=0)
Jun 15 04:06:19 combo su(pam_unix)[21416]: session closed for user cyrus
Jun 15 04:06:20 combo logrotate: ALERT exited abnormally with [1]  [×34 first Jun 15 04:06:20, last Jul 27 04:16:09]
Jun 15 04:12:42 combo su(pam_unix)[22644]: session opened for user news by (uid=0)
Jun 15 04:12:43 combo su(pam_unix)[22644]: session closed for user news
Jun 15 12:12:34 combo sshd(pam_unix)[23397]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23397]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 15 12:12:34 combo sshd(pam_unix)[23395]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23395]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 15 12:12:34 combo sshd(pam_unix)[23404]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23404]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 15 12:12:34 combo sshd(pam_unix)[23399]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23399]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 15 12:12:34 combo sshd(pam_unix)[23406]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23406]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 15 12:12:34 combo sshd(pam_unix)[23396]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23394]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23407]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23394]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 15 12:12:34 combo sshd(pam_unix)[23403]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23396]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 15 12:12:34 combo sshd(pam_unix)[23407]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 15 12:12:34 combo sshd(pam_unix)[23403]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 

```

### `19-auth-log`

- [Full input](cases/19-auth-log/input.log)
- [Output with CCR](cases/19-auth-log/output.log) - [diff](cases/19-auth-log/compression.diff)
- [Output without CCR](cases/19-auth-log/output-noccr.log) - [diff](cases/19-auth-log/compression-noccr.diff)

Input excerpt:

```text
Mar 27 13:06:56 ip-10-77-20-248 sshd[1291]: Server listening on 0.0.0.0 port 22.
Mar 27 13:06:56 ip-10-77-20-248 sshd[1291]: Server listening on :: port 22.
Mar 27 13:06:56 ip-10-77-20-248 systemd-logind[1118]: Watching system buttons on /dev/input/event0 (Power Button)
Mar 27 13:06:56 ip-10-77-20-248 systemd-logind[1118]: Watching system buttons on /dev/input/event1 (Sleep Button)
Mar 27 13:06:56 ip-10-77-20-248 systemd-logind[1118]: New seat seat0.
Mar 27 13:08:09 ip-10-77-20-248 sshd[1361]: Accepted publickey for ubuntu from 85.245.107.41 port 54259 ssh2: RSA SHA256:Kl8kPGZrTiz7g4FO1hyqHdsSBBb5Fge6NWOobN03XJg
Mar 27 13:08:09 ip-10-77-20-248 sshd[1361]: pam_unix(sshd:session): session opened for user ubuntu by (uid=0)
Mar 27 13:08:09 ip-10-77-20-248 systemd: pam_unix(systemd-user:session): session opened for user ubuntu by (uid=0)
Mar 27 13:08:09 ip-10-77-20-248 systemd-logind[1118]: New session 1 of user ubuntu.
Mar 27 13:09:37 ip-10-77-20-248 sudo:   ubuntu : TTY=pts/0 ; PWD=/home/ubuntu ; USER=root ; COMMAND=/usr/bin/curl -L -O https://artifacts.elastic.co/downloads/beats/filebeat/filebeat-5.2.2-amd64.deb
Mar 27 13:09:37 ip-10-77-20-248 sudo: pam_unix(sudo:session): session opened for user root by ubuntu(uid=0)
Mar 27 13:09:38 ip-10-77-20-248 sudo: pam_unix(sudo:session): session closed for user root
Mar 27 13:10:08 ip-10-77-20-248 sudo:   ubuntu : TTY=pts/0 ; PWD=/home/ubuntu ; USER=root ; COMMAND=/usr/bin/apt-key add -
Mar 27 13:10:08 ip-10-77-20-248 sudo: pam_unix(sudo:session): session opened for user root by ubuntu(uid=0)
Mar 27 13:10:09 ip-10-77-20-248 sudo: pam_unix(sudo:session): session closed for user root
Mar 27 13:10:14 ip-10-77-20-248 sudo:   ubuntu : TTY=pts/0 ; PWD=/home/ubuntu ; USER=root ; COMMAND=/usr/bin/apt-get install apt-transport-https
Mar 27 13:10:14 ip-10-77-20-248 sudo: pam_unix(sudo:session): session opened for user root by ubuntu(uid=0)
Mar 27 13:10:14 ip-10-77-20-248 sudo: pam_unix(sudo:session): session closed for user root
Mar 27 13:10:18 ip-10-77-20-248 sudo:   ubuntu : TTY=pts/0 ; PWD=/home/ubuntu ; USER=root ; COMMAND=/usr/bin/tee -a /etc/apt/sources.list.d/elastic-5.x.list
Mar 27 13:10:18 ip-10-77-20-248 sudo: pam_unix(sudo:session): session opened for user root by ubuntu(uid=0)
Mar 27 13:10:18 ip-10-77-20-248 sudo: pam_unix(sudo:session): session closed for user root
Mar 27 13:10:24 ip-10-77-20-248 sudo:   ubuntu : TTY=pts/0 ; PWD=/home/ubuntu ; USER=root ; COMMAND=/usr/bin/apt-get update
Mar 27 13:10:24 ip-10-77-20-248 sudo: pam_unix(sudo:session): session opened for user root by ubuntu(uid=0)
Mar 27 13:10:28 ip-10-77-20-248 sudo: pam_unix(sudo:session): session closed for user root
Mar 27 13:10:28 ip-10-77-20-248 sudo:   ubuntu : TTY=pts/0 ; PWD=/home/ubuntu ; USER=root ; COMMAND=/usr/bin/apt-get install filebeat
Mar 27 13:10:28 ip-10-77-20-248 sudo: pam_unix(sudo:session): session opened for user root by ubuntu(uid=0)
Mar 27 13:10:33 ip-10-77-20-248 sudo: pam_unix(sudo:session): session closed for user root
Mar 27 13:10:53 ip-10-77-20-248 sudo:   ubuntu : TTY=pts/0 ; PWD=/home/ubuntu ; USER=root ; COMMAND=/usr/sbin/update-rc.d filebeat defaults 95 10
Mar 27 13:10:53 ip-10-77-20-248 sudo: pam_unix(sudo:session): session opened for user root by ubuntu(uid=0)
Mar 27 13:10:53 ip-10-77-20-248 sudo: pam_unix(sudo:session): session closed for user root
Mar 27 13:11:31 ip-10-77-20-248 sudo:   ubuntu : TTY=pts/0 ; PWD=/home/ubuntu ; USER=root ; COMMAND=/usr/bin/apt-get update
Mar 27 13:11:31 ip-10-77-20-248 sudo: pam_unix(sudo:session): session opened for user root by ubuntu(uid=0)
Mar 27 13:11:33 ip-10-77-20-248 sudo: pam_unix(sudo:session): session closed for user root
Mar 27 13:11:34 ip-10-77-20-248 sudo:   ubuntu : TTY=pts/0 ; PWD=/home/ubuntu ; USER=root ; COMMAND=/usr/bin/apt-get update
Mar 27 13:11:34 ip-10-77-20-248 sudo: pam_unix(sudo:session): session opened for user root by ubuntu(uid=0)
Mar 27 13:11:35 ip-10-77-20-248 sudo: pam_unix(sudo:session): session closed for user root

```

Output excerpt:

```text
[... 83 line(s) omitted ... ⟦tj:f8561e17c2dd04e4ed7f7a3734ebbb49⟧]
Mar 27 13:44:29 ip-10-77-20-248 sudo: pam_unix(sudo:session): session opened for user root by ubuntu(uid=0)
Mar 27 13:44:29 ip-10-77-20-248 sudo: pam_unix(sudo:session): session closed for user root
Mar 27 14:01:39 ip-10-77-20-248 sshd[2938]: error: maximum authentication attempts exceeded for root from 122.176.37.221 port 37107 ssh2 [preauth]
Mar 27 14:01:39 ip-10-77-20-248 sshd[2938]: Disconnecting: Too many authentication failures [preauth]
Mar 27 14:02:16 ip-10-77-20-248 sshd[2856]: Received disconnect from 85.245.107.41 port 54866:11: disconnected by user
[... 7 line(s) omitted ... ⟦tj:9fc4a93e4c2e7b7b6430c4432ad78e02⟧]
Mar 27 14:54:57 ip-10-77-20-248 sshd[2967]: Invalid user support from 95.152.57.58
Mar 27 14:54:57 ip-10-77-20-248 sshd[2967]: input_userauth_request: invalid user support [preauth]
Mar 27 14:54:58 ip-10-77-20-248 sshd[2967]: error: maximum authentication attempts exceeded for invalid user support from 95.152.57.58 port 53679 ssh2 [preauth]
Mar 27 14:54:58 ip-10-77-20-248 sshd[2967]: Disconnecting: Too many authentication failures [preauth]
Mar 27 15:17:01 ip-10-77-20-248 CRON[2980]: pam_unix(cron:session): session opened for user root by (uid=0)
Mar 27 15:17:01 ip-10-77-20-248 CRON[2980]: pam_unix(cron:session): session closed for user root
Mar 27 15:30:59 ip-10-77-20-248 sshd[2995]: Did not receive identification string from 209.160.24.191
Mar 27 15:46:53 ip-10-77-20-248 sshd[2996]: error: maximum authentication attempts exceeded for root from 90.144.183.19 port 57648 ssh2 [preauth]
Mar 27 15:46:53 ip-10-77-20-248 sshd[2996]: Disconnecting: Too many authentication failures [preauth]
Mar 27 15:48:29 ip-10-77-20-248 sshd[2998]: Accepted publickey for ubuntu from 85.245.107.41 port 57684 ssh2: RSA SHA256:Kl8kPGZrTiz7g4FO1hyqHdsSBBb5Fge6NWOobN03XJg
[... 4 line(s) omitted ... ⟦tj:2a72b07bda3207ac3f3fa293784799b2⟧]
Mar 27 15:49:20 ip-10-77-20-248 sudo: pam_unix(sudo:session): session opened for user root by ubuntu(uid=0)
Mar 27 15:49:21 ip-10-77-20-248 sudo: pam_unix(sudo:session): session closed for user root
Mar 27 15:59:42 ip-10-77-20-248 sshd[3242]: error: maximum authentication attempts exceeded for root from 186.128.152.44 port 34605 ssh2 [preauth]
Mar 27 15:59:42 ip-10-77-20-248 sshd[3242]: Disconnecting: Too many authentication failures [preauth]
Mar 27 16:02:57 ip-10-77-20-248 sudo:   ubuntu : TTY=pts/0 ; PWD=/usr/share/filebeat/bin ; USER=root ; COMMAND=/usr/bin/vim /etc/filebeat/filebeat.yml
[... 203 line(s) omitted ... ⟦tj:d0db68e4424356ef5a18d56e7ac63f85⟧]
Mar 27 18:22:24 ip-10-77-20-248 sshd[14922]: Invalid user admin from 201.177.23.130
Mar 27 18:22:24 ip-10-77-20-248 sshd[14922]: input_userauth_request: invalid user admin [preauth]
Mar 27 18:22:26 ip-10-77-20-248 sshd[14922]: error: maximum authentication attempts exceeded for invalid user admin from 201.177.23.130 port 46784 ssh2 [preauth]
Mar 27 18:22:26 ip-10-77-20-248 sshd[14922]: Disconnecting: Too many authentication failures [preauth]
Mar 27 18:27:18 ip-10-77-20-248 sshd[14924]: error: maximum authentication attempts exceeded for root from 190.178.62.6 port 56562 ssh2 [preauth]
Mar 27 18:27:18 ip-10-77-20-248 sshd[14924]: Disconnecting: Too many authentication failures [preauth]
Mar 27 18:27:19 ip-10-77-20-248 sshd[14926]: error: maximum authentication attempts exceeded for root from 190.178.62.6 port 56567 ssh2 [preauth]
Mar 27 18:27:19 ip-10-77-20-248 sshd[14926]: Disconnecting: Too many authentication failures [preauth]
Mar 27 18:27:20 ip-10-77-20-248 sshd[14928]: Invalid user support from 190.178.62.6
Mar 27 18:27:20 ip-10-77-20-248 sshd[14928]: input_userauth_request: invalid user support [preauth]
Mar 27 18:27:21 ip-10-77-20-248 sshd[14928]: error: maximum authentication attempts exceeded for invalid user support from 190.178.62.6 port 56573 ssh2 [preauth]
Mar 27 18:27:21 ip-10-77-20-248 sshd[14928]: Disconnecting: Too many authentication failures [preauth]

```

### `12-apache-error`

- [Full input](cases/12-apache-error/input.log)
- [Output with CCR](cases/12-apache-error/output.log) - [diff](cases/12-apache-error/compression.diff)
- [Output without CCR](cases/12-apache-error/output-noccr.log) - [diff](cases/12-apache-error/compression-noccr.diff)

Input excerpt:

```text
[Sun Dec 04 04:47:44 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:47:44 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:51:08 2005] [notice] jk2_init() Found child 6725 in scoreboard slot 10
[Sun Dec 04 04:51:09 2005] [notice] jk2_init() Found child 6726 in scoreboard slot 8
[Sun Dec 04 04:51:09 2005] [notice] jk2_init() Found child 6728 in scoreboard slot 6
[Sun Dec 04 04:51:14 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:51:14 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:51:14 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:51:18 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:51:18 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:51:18 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:51:37 2005] [notice] jk2_init() Found child 6736 in scoreboard slot 10
[Sun Dec 04 04:51:38 2005] [notice] jk2_init() Found child 6733 in scoreboard slot 7
[Sun Dec 04 04:51:38 2005] [notice] jk2_init() Found child 6734 in scoreboard slot 9
[Sun Dec 04 04:51:52 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:51:52 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:51:55 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:52:04 2005] [notice] jk2_init() Found child 6738 in scoreboard slot 6
[Sun Dec 04 04:52:04 2005] [notice] jk2_init() Found child 6741 in scoreboard slot 9
[Sun Dec 04 04:52:05 2005] [notice] jk2_init() Found child 6740 in scoreboard slot 7
[Sun Dec 04 04:52:05 2005] [notice] jk2_init() Found child 6737 in scoreboard slot 8
[Sun Dec 04 04:52:12 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:52:12 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:52:12 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:52:15 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:52:15 2005] [error] mod_jk child workerEnv in error state 7
[Sun Dec 04 04:52:15 2005] [error] mod_jk child workerEnv in error state 7
[Sun Dec 04 04:52:36 2005] [notice] jk2_init() Found child 6748 in scoreboard slot 6
[Sun Dec 04 04:52:36 2005] [notice] jk2_init() Found child 6744 in scoreboard slot 10
[Sun Dec 04 04:52:36 2005] [notice] jk2_init() Found child 6745 in scoreboard slot 8
[Sun Dec 04 04:52:49 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:52:49 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:52:52 2005] [error] mod_jk child workerEnv in error state 7
[Sun Dec 04 04:52:52 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:53:05 2005] [notice] jk2_init() Found child 6750 in scoreboard slot 7
[Sun Dec 04 04:53:05 2005] [notice] jk2_init() Found child 6751 in scoreboard slot 9

```

Output excerpt:

```text
[Sun Dec 04 04:47:44 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:47:44 2005] [error] mod_jk child workerEnv in error state 6  [×26 first 04:47:44, last 04:59:38]
[Sun Dec 04 04:51:08 2005] [notice] jk2_init() Found child 6725 in scoreboard slot 10
[Sun Dec 04 04:51:09 2005] [notice] jk2_init() Found child 6726 in scoreboard slot 8
[... 81 line(s) omitted ... ⟦tj:b06490e7af17f4315cc2b62999ebc3f8⟧]
[Sun Dec 04 05:00:03 2005] [notice] jk2_init() Found child 8560 in scoreboard slot 7
[Sun Dec 04 05:00:09 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 05:00:09 2005] [error] mod_jk child workerEnv in error state 6  [×15 first 05:00:09, last 05:15:16]
[Sun Dec 04 05:00:13 2005] [notice] jk2_init() Found child 8565 in scoreboard slot 9
[Sun Dec 04 05:00:13 2005] [notice] jk2_init() Found child 8573 in scoreboard slot 10
[... 39 line(s) omitted ... ⟦tj:72b34dbfb31c906d4cfa6743f6e903d7⟧]
[Sun Dec 04 05:12:30 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 05:12:30 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 05:15:09 2005] [error] [client 222.166.160.184] Directory index forbidden by rule: /var/www/html/
[Sun Dec 04 05:15:13 2005] [notice] jk2_init() Found child 1000 in scoreboard slot 10
[Sun Dec 04 05:15:16 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[... 3 line(s) omitted ... ⟦tj:0f73b4fe379f5290dd1876a82bcf8966⟧]
[Sun Dec 04 06:01:21 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 06:01:21 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 06:01:30 2005] [error] mod_jk child workerEnv in error state 6  [×90 first 06:01:30, last 06:59:47]
[Sun Dec 04 06:01:42 2005] [notice] jk2_init() Found child 32352 in scoreboard slot 9
[Sun Dec 04 06:01:42 2005] [notice] jk2_init() Found child 32353 in scoreboard slot 10
[... 347 line(s) omitted ... ⟦tj:641ad21650e7dbc449bdbb9a7b298027⟧]
[Sun Dec 04 07:02:01 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 07:02:01 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 07:02:03 2005] [error] mod_jk child workerEnv in error state 9  [×27 first 07:02:03, last 07:18:00]
[Sun Dec 04 07:02:03 2005] [error] mod_jk child workerEnv in error state 8
[Sun Dec 04 07:02:03 2005] [error] mod_jk child workerEnv in error state 8
[... 83 line(s) omitted ... ⟦tj:572ddd2a04c74a497cb316fa99bc5610⟧]
[Sun Dec 04 07:18:00 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 07:18:00 2005] [error] mod_jk child workerEnv in error state 7
[Sun Dec 04 07:45:45 2005] [error] [client 63.13.186.196] Directory index forbidden by rule: /var/www/html/
[Sun Dec 04 08:54:17 2005] [error] [client 147.31.138.75] Directory index forbidden by rule: /var/www/html/
[Sun Dec 04 09:35:12 2005] [error] [client 207.203.80.15] Directory index forbidden by rule: /var/www/html/
[Sun Dec 04 10:53:30 2005] [error] [client 218.76.139.20] Directory index forbidden by rule: /var/www/html/
[Sun Dec 04 11:11:07 2005] [error] [client 24.147.151.74] Directory index forbidden by rule: /var/www/html/  [×3 first 11:11:07, last 11:42:43]

```

### `06-hpc`

- [Full input](cases/06-hpc/input.log)
- [Output with CCR](cases/06-hpc/output.log) - [diff](cases/06-hpc/compression.diff)
- [Output without CCR](cases/06-hpc/output-noccr.log) - [diff](cases/06-hpc/compression-noccr.diff)

Input excerpt:

```text
134681 node-246 unix.hw state_change.unavailable 1077804742 1 Component State Change: Component \042SCSI-WWID:01000010:6005-08b4-0001-00c6-0006-3000-003d-0000\042 is in the unavailable state (HWID=1973)
350766 node-109 unix.hw state_change.unavailable 1084680778 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=3180)
344518 node-246 unix.hw state_change.unavailable 1084270955 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=5089)
344448 node-153 unix.hw state_change.unavailable 1084270952 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=4088)
366633 node-200 unix.hw state_change.unavailable 1085100843 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=2538)
366463 node-122 unix.hw state_change.unavailable 1085084674 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=2480)
438190 node-228 unix.hw state_change.unavailable 1097194780 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=3713)
225111 node-10 unix.hw state_change.unavailable 1117296789 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=3891)
360778 node-130 unix.hw state_change.unavailable 1141108031 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=2478)
401569 node-169 unix.hw state_change.unavailable 1142550406 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=2969)
401855 node-187 unix.hw state_change.unavailable 1142553646 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=4159)
460773 node-199 unix.hw state_change.unavailable 1145552100 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=2608)
2568643 node-70 action start 1074119817 1 clusterAddMember  (command 1902)
2570772 node-124 action start 1074123150 1 clusterAddMember  (command 1900)
2571927 node-28 action start 1074125371 1 risBoot  (command 1903)
2572286 node-17 action start 1074126278 1 bootGenvmunix  (command 1903)
2575909 node-162 action start 1074178193 1 boot  (command 1911)
2576195 node-181 action start 1074178628 1 boot  (command 1910)
2599298 node-198 action start 1074297419 1 boot  (command 1978)
2600743 node-57 action start 1074298084 1 boot  (command 1967)
2601401 node-184 action start 1074298390 1 wait  (command 1975)
2612635 node-88 action start 1074535847 1 boot  (command 1999)
2608062 node-238 action start 1074461014 1 halt  (command 1982)
2607813 node-243 action start 1074459063 1 boot  (command 1981)
2600616 node-152 action start 1074298056 1 boot  (command 1973)
2601430 node-159 action start 1074298398 1 wait  (command 1973)
3515 node-216 action start 1075629790 1 wait  (command 2057)
41108 node-93 action start 1076538873 1 boot  (command 2152)
39962 node-134 action start 1076538533 1 wait  (command 2154)
38426 node-17 action start 1076537141 1 boot  (command 2141)
33697 node-251 action start 1076435713 1 boot  (command 2110)
46302 node-114 action start 1076546290 1 boot  (command 2160)
76306 node-57 action start 1077204892 1 halt  (command 2221)
75035 node-116 action start 1077172847 1 wait  (command 2217)
75026 node-119 action start 1077172842 1 wait  (command 2217)
66410 node-226 action start 1076874863 1 wait  (command 2201)

```

Output excerpt:

```text
[Template: <*> <*> unix.hw state_change.unavailable <*> 1 Component State Change: Component <*> is in the unavailable state <*>] (12 lines)
[Template: <*> <*> action start <*> 1 <*> (command <*>] (141 lines)
[Template: <*> <*> boot_cmd success <*> 1 Command has completed successfully] (10 lines)
[... 170 line(s) omitted ... ⟦tj:36898aea9d72de5d27de6e8c8f57ec7a⟧]
128520 3491 boot_cmd new 1111855423 1 Targeting domains:node-D2 and nodes:node-[64-95] child of command 3488
276415 3709 boot_cmd new 1119845316 1 Targeting domains:node-D2 and nodes:node-84
51338 node-3 node psu 1106496000 1 psu failure\ ambient=28
191898 node-238 node psu 1131240275 1 psu failure\ ambient=28
236618 node-104 node psu 1132434391 1 psu failure\ ambient=28
341834 node-118 node psu 1140312091 1 psu failure\ ambient=28
347972 node-118 node psu 1140430530 1 psu failure\ ambient=31
147394 Interconnect-0N00 switch_module temphigh 1129812510 1 Temperature (41C) exceeds warning threshold  [×2]
147494 Interconnect-0N00 switch_module temphigh 1129813980 1 Temperature (41C) exceeds warning threshold
[Template: <*> <*> clusterfilesystem clusterfilesystem.no_server <*> 1 ClusterFileSystem: There is no server for ServerFileSystem domain <*>] (30 lines)
[... 30 line(s) omitted ... ⟦tj:41919d148dfceda7bea088ad8c2c5efc⟧]
286151 node-D0 clusterfilesystem clusterfilesystem.no_server 1134669644 1 ClusterFileSystem: There is no server for ServerFileSystem domain storage531
327002 node-D0 clusterfilesystem clusterfilesystem.no_server 1140098601 1 ClusterFileSystem: There is no server for ServerFileSystem domain storage321
365140 node-69 unix.hw net.niff.down 1085075228 1 NIFF: node node-69 detected a failed network connection on network 5.5.224.0 via interface alt0
401608 node-162 unix.hw net.niff.down 1142550442 1 NIFF: node node-162 detected a failed network connection on network 5.5.224.0 via interface alt0
180537 node-D0 clusterfilesystem fdmn.panic 1131228351 1 ServerFileSystem: An ServerFileSystem domain panic has occurred on storage442
268543 Interconnect-0N00 switch_module fan 1119372267 1 Fan speeds ( 3552 3534 3375 **** 3515 3479 )
268265 Interconnect-0N00 switch_module fan 1119290224 1 Fan speeds ( 3552 3534 3375 **** 3515 3479 )
[Template: <*> Interconnect-0N00 switch_module fan <*> 1 Fan speeds ( <*> <*> <*> <*> <*> <*> )] (86 lines)
[... 86 line(s) omitted ... ⟦tj:84ce7e5e6d4893b6cf9950445115a449⟧]
441254 Interconnect-0N00 switch_module fan 1143979871 1 Fan speeds ( 3534 3534 3375 **** 3497 3479 )
441208 Interconnect-0N00 switch_module fan 1143979502 1 Fan speeds ( 3534 3534 3375 **** 3497 3479 )
130987 3504 boot_cmd abort 1111858191 1 Command has been aborted
27385 Interconnect-0N02 switch_module control 1101771720 1 power/control problem
30700 Interconnect-1T00 switch_module bcast-error 1076189965 1 Link error  [×9]
115576 Interconnect-1T00 switch_module bcast-error 1077793578 1 Link error
115153 Interconnect-0T00 switch_module bcast-error 1077757190 1 Link error
259323 Interconnect-1T00 switch_module bcast-error 1079076297 1 Link error
285855 Interconnect-1T00 switch_module bcast-error 1080298218 1 Link in reset  [×2]
285823 Interconnect-0T00 switch_module bcast-error 1080292236 1 Link error
289491 Interconnect-0T00 switch_module bcast-error 1080825593 1 Link error
288385 Interconnect-0T00 switch_module bcast-error 1080668603 1 Link error

```

### `05-bgl`

- [Full input](cases/05-bgl/input.log)
- [Output with CCR](cases/05-bgl/output.log) - [diff](cases/05-bgl/compression.diff)
- [Output without CCR](cases/05-bgl/output-noccr.log) - [diff](cases/05-bgl/compression-noccr.diff)

Input excerpt:

```text
- 1117838570 2005.06.03 R02-M1-N0-C:J12-U11 2005-06-03-15.42.50.675872 R02-M1-N0-C:J12-U11 RAS KERNEL INFO instruction cache parity error corrected
- 1117838573 2005.06.03 R02-M1-N0-C:J12-U11 2005-06-03-15.42.53.276129 R02-M1-N0-C:J12-U11 RAS KERNEL INFO instruction cache parity error corrected
- 1117838976 2005.06.03 R02-M1-N0-C:J12-U11 2005-06-03-15.49.36.156884 R02-M1-N0-C:J12-U11 RAS KERNEL INFO instruction cache parity error corrected
- 1117838978 2005.06.03 R02-M1-N0-C:J12-U11 2005-06-03-15.49.38.026704 R02-M1-N0-C:J12-U11 RAS KERNEL INFO instruction cache parity error corrected
- 1117842440 2005.06.03 R23-M0-NE-C:J05-U01 2005-06-03-16.47.20.730545 R23-M0-NE-C:J05-U01 RAS KERNEL INFO 63543 double-hummer alignment exceptions
- 1117842974 2005.06.03 R24-M0-N1-C:J13-U11 2005-06-03-16.56.14.254137 R24-M0-N1-C:J13-U11 RAS KERNEL INFO 162 double-hummer alignment exceptions
- 1117843015 2005.06.03 R21-M1-N6-C:J08-U11 2005-06-03-16.56.55.309974 R21-M1-N6-C:J08-U11 RAS KERNEL INFO 141 double-hummer alignment exceptions
- 1117848119 2005.06.03 R16-M1-N2-C:J17-U01 2005-06-03-18.21.59.871925 R16-M1-N2-C:J17-U01 RAS KERNEL INFO CE sym 2, at 0x0b85eee0, mask 0x05
APPREAD 1117869872 2005.06.04 R04-M1-N4-I:J18-U11 2005-06-04-00.24.32.432192 R04-M1-N4-I:J18-U11 RAS APP FATAL ciod: failed to read message prefix on control stream (CioStream socket to 172.16.96.116:33569
APPREAD 1117869876 2005.06.04 R27-M1-N4-I:J18-U01 2005-06-04-00.24.36.222560 R27-M1-N4-I:J18-U01 RAS APP FATAL ciod: failed to read message prefix on control stream (CioStream socket to 172.16.96.116:33370
- 1117942120 2005.06.04 R30-M0-N7-C:J08-U01 2005-06-04-20.28.40.767551 R30-M0-N7-C:J08-U01 RAS KERNEL INFO CE sym 20, at 0x1438f9e0, mask 0x40
- 1117955341 2005.06.05 R25-M0-N7-C:J02-U01 2005-06-05-00.09.01.903373 R25-M0-N7-C:J02-U01 RAS KERNEL INFO generating core.2275
- 1117955392 2005.06.05 R24-M1-N8-C:J09-U11 2005-06-05-00.09.52.516674 R24-M1-N8-C:J09-U11 RAS KERNEL INFO generating core.862
- 1117956980 2005.06.05 R24-M1-NB-C:J15-U11 2005-06-05-00.36.20.945796 R24-M1-NB-C:J15-U11 RAS KERNEL INFO generating core.728
- 1117957045 2005.06.05 R20-M1-N8-C:J04-U01 2005-06-05-00.37.25.012681 R20-M1-N8-C:J04-U01 RAS KERNEL INFO generating core.775
- 1117959501 2005.06.05 R24-M0-NE-C:J14-U11 2005-06-05-01.18.21.778604 R24-M0-NE-C:J14-U11 RAS KERNEL INFO generating core.3276
- 1117959513 2005.06.05 R21-M1-N2-C:J11-U01 2005-06-05-01.18.33.830595 R21-M1-N2-C:J11-U01 RAS KERNEL INFO generating core.1717
- 1117959563 2005.06.05 R24-M0-N8-C:J04-U11 2005-06-05-01.19.23.822135 R24-M0-N8-C:J04-U11 RAS KERNEL INFO generating core.3919
- 1117973759 2005.06.05 R31-M0-NE-C:J05-U11 2005-06-05-05.15.59.416717 R31-M0-NE-C:J05-U11 RAS KERNEL INFO generating core.2079
- 1117973786 2005.06.05 R36-M0-NA-C:J06-U01 2005-06-05-05.16.26.686603 R36-M0-NA-C:J06-U01 RAS KERNEL INFO generating core.1414
- 1117973919 2005.06.05 R33-M0-N4-C:J02-U11 2005-06-05-05.18.39.396608 R33-M0-N4-C:J02-U11 RAS KERNEL INFO generating core.3055
- 1117974206 2005.06.05 R22-M0-ND-C:J10-U11 2005-06-05-05.23.26.239153 R22-M0-ND-C:J10-U11 RAS KERNEL INFO generating core.201
- 1117974463 2005.06.05 R27-M0-N6-C:J10-U01 2005-06-05-05.27.43.336565 R27-M0-N6-C:J10-U01 RAS KERNEL INFO generating core.1125
- 1117975251 2005.06.05 R26-M1-N8-C:J17-U11 2005-06-05-05.40.51.726735 R26-M1-N8-C:J17-U11 RAS KERNEL INFO generating core.412
- 1117976658 2005.06.05 R36-M1-N8-C:J17-U01 2005-06-05-06.04.18.406158 R36-M1-N8-C:J17-U01 RAS KERNEL INFO generating core.7828
- 1117977497 2005.06.05 R33-M1-NB-C:J06-U01 2005-06-05-06.18.17.802159 R33-M1-NB-C:J06-U01 RAS KERNEL INFO generating core.5570
- 1117979227 2005.06.05 R01-M1-N7-C:J04-U11 2005-06-05-06.47.07.157021 R01-M1-N7-C:J04-U11 RAS KERNEL INFO generating core.8275
- 1117982609 2005.06.05 R35-M1-NE-C:J05-U01 2005-06-05-07.43.29.979844 R35-M1-NE-C:J05-U01 RAS KERNEL INFO generating core.4183
- 1117984124 2005.06.05 R36-M1-NF-C:J11-U01 2005-06-05-08.08.44.281729 R36-M1-NF-C:J11-U01 RAS KERNEL INFO generating core.6545
- 1117984130 2005.06.05 R37-M1-NE-C:J13-U01 2005-06-05-08.08.50.547117 R37-M1-NE-C:J13-U01 RAS KERNEL INFO generating core.4245
- 1117984216 2005.06.05 R32-M1-N4-C:J16-U01 2005-06-05-08.10.16.270131 R32-M1-N4-C:J16-U01 RAS KERNEL INFO generating core.6884
- 1117984246 2005.06.05 R30-M1-N3-C:J02-U01 2005-06-05-08.10.46.344235 R30-M1-N3-C:J02-U01 RAS KERNEL FATAL force load/store alignment...............0
- 1117985401 2005.06.05 R34-M1-NE-C:J02-U01 2005-06-05-08.30.01.873693 R34-M1-NE-C:J02-U01 RAS KERNEL INFO generating core.6471
- 1117985413 2005.06.05 R31-M1-N7-C:J05-U11 2005-06-05-08.30.13.824307 R31-M1-N7-C:J05-U11 RAS KERNEL INFO generating core.4155
- 1117985464 2005.06.05 R32-M0-NF-C:J10-U01 2005-06-05-08.31.04.464776 R32-M0-NF-C:J10-U01 RAS KERNEL INFO generating core.449
- 1117985533 2005.06.05 R34-M1-NC-C:J06-U11 2005-06-05-08.32.13.659715 R34-M1-NC-C:J06-U11 RAS KERNEL INFO generating core.6990

```

Output excerpt:

```text
[... 6 line(s) omitted ... ⟦tj:4c3857f0caaaa17d4f1bda683e2f1093⟧]
- 1117843015 2005.06.03 R21-M1-N6-C:J08-U11 2005-06-03-16.56.55.309974 R21-M1-N6-C:J08-U11 RAS KERNEL INFO 141 double-hummer alignment exceptions
- 1117848119 2005.06.03 R16-M1-N2-C:J17-U01 2005-06-03-18.21.59.871925 R16-M1-N2-C:J17-U01 RAS KERNEL INFO CE sym 2, at 0x0b85eee0, mask 0x05
APPREAD 1117869872 2005.06.04 R04-M1-N4-I:J18-U11 2005-06-04-00.24.32.432192 R04-M1-N4-I:J18-U11 RAS APP FATAL ciod: failed to read message prefix on control stream (CioStream socket to 172.16.96.116:33569
APPREAD 1117869876 2005.06.04 R27-M1-N4-I:J18-U01 2005-06-04-00.24.36.222560 R27-M1-N4-I:J18-U01 RAS APP FATAL ciod: failed to read message prefix on control stream (CioStream socket to 172.16.96.116:33370
- 1117942120 2005.06.04 R30-M0-N7-C:J08-U01 2005-06-04-20.28.40.767551 R30-M0-N7-C:J08-U01 RAS KERNEL INFO CE sym 20, at 0x1438f9e0, mask 0x40
- 1117955341 2005.06.05 R25-M0-N7-C:J02-U01 2005-06-05-00.09.01.903373 R25-M0-N7-C:J02-U01 RAS KERNEL INFO generating core.2275
[Template: - <*> 2005.06.05 <*> <*> <*> RAS KERNEL INFO generating <*>] (17 lines, first 2005-06-05-00.09.52.516674, last 2005-06-05-08.08.44.281729)
[... 17 line(s) omitted ... ⟦tj:7a0f467303b92fae9b99cc5b4ef6be13⟧]
- 1117984130 2005.06.05 R37-M1-NE-C:J13-U01 2005-06-05-08.08.50.547117 R37-M1-NE-C:J13-U01 RAS KERNEL INFO generating core.4245
- 1117984216 2005.06.05 R32-M1-N4-C:J16-U01 2005-06-05-08.10.16.270131 R32-M1-N4-C:J16-U01 RAS KERNEL INFO generating core.6884
- 1117984246 2005.06.05 R30-M1-N3-C:J02-U01 2005-06-05-08.10.46.344235 R30-M1-N3-C:J02-U01 RAS KERNEL FATAL force load/store alignment...............0
- 1117985401 2005.06.05 R34-M1-NE-C:J02-U01 2005-06-05-08.30.01.873693 R34-M1-NE-C:J02-U01 RAS KERNEL INFO generating core.6471
- 1117985413 2005.06.05 R31-M1-N7-C:J05-U11 2005-06-05-08.30.13.824307 R31-M1-N7-C:J05-U11 RAS KERNEL INFO generating core.4155
[Template: - <*> 2005.06.05 <*> <*> <*> RAS KERNEL INFO generating <*>] (22 lines, first 2005-06-05-08.31.04.464776, last 2005-06-05-10.29.13.196439)
[Template: - <*> 2005.06.06 R02-M1-N0-C:J12-U11 <*> R02-M1-N0-C:J12-U11 RAS KERNEL INFO instruction cache parity error corrected] (8 lines, first 2005-06-06-10.33.41.093251, last 2005-06-06-11.11.07.787886)
[Template: - <*> <*> <*> <*> <*> RAS KERNEL INFO generating <*>] (13 lines, first 2005-06-06-20.24.16.345770, last 2005-06-07-12.31.30.918396)
[... 48 line(s) omitted ... ⟦tj:2d6a7bd6095344498307e3812537332d⟧]
- 1118183497 2005.06.07 R34-M0-N7-C:J09-U11 2005-06-07-15.31.37.435791 R34-M0-N7-C:J09-U11 RAS KERNEL INFO generating core.122
- 1118183566 2005.06.07 R36-M0-N0-C:J11-U01 2005-06-07-15.32.46.334191 R36-M0-N0-C:J11-U01 RAS KERNEL INFO generating core.1973
- 1118193358 2005.06.07 R11-M0-NC-I:J18-U01 2005-06-07-18.15.58.583443 R11-M0-NC-I:J18-U01 RAS APP FATAL ciod: LOGIN chdir(/p/gb2/glosli/8M_5000K/t800) failed: No such file or directory
- 1118207879 2005.06.07 R17-M1-N7-C:J15-U11 2005-06-07-22.17.59.587113 R17-M1-N7-C:J15-U11 RAS KERNEL INFO generating core.4984
- 1118207897 2005.06.07 R16-M1-NC-C:J06-U11 2005-06-07-22.18.17.203831 R16-M1-NC-C:J06-U11 RAS KERNEL INFO generating core.1822
[... 14 line(s) omitted ... ⟦tj:729a85e5274f9b9ec287c73e7cbaab27⟧]
- 1118371043 2005.06.09 R11-M0-N7-C:J16-U11 2005-06-09-19.37.23.866536 R11-M0-N7-C:J16-U11 RAS KERNEL INFO generating core.8280
- 1118371064 2005.06.09 R00-M1-N6-C:J12-U01 2005-06-09-19.37.44.455502 R00-M1-N6-C:J12-U01 RAS KERNEL INFO generating core.12357
KERNDTLB 1118536327 2005.06.11 R30-M0-N9-C:J16-U01 2005-06-11-17.32.07.581048 R30-M0-N9-C:J16-U01 RAS KERNEL FATAL data TLB error interrupt
KERNDTLB 1118536959 2005.06.11 R30-M0-N9-C:J16-U01 2005-06-11-17.42.39.794840 R30-M0-N9-C:J16-U01 RAS KERNEL FATAL data TLB error interrupt
KERNDTLB 1118537212 2005.06.11 R30-M0-N9-C:J16-U01 2005-06-11-17.46.52.020435 R30-M0-N9-C:J16-U01 RAS KERNEL FATAL data TLB error interrupt
KERNDTLB 1118537261 2005.06.11 R30-M0-N9-C:J16-U01 2005-06-11-17.47.41.103595 R30-M0-N9-C:J16-U01 RAS KERNEL FATAL data TLB error interrupt
KERNDTLB 1118537622 2005.06.11 R30-M0-N9-C:J16-U01 2005-06-11-17.53.42.323869 R30-M0-N9-C:J16-U01 RAS KERNEL FATAL data TLB error interrupt
KERNDTLB 1118537694 2005.06.11 R30-M0-N9-C:J16-U01 2005-06-11-17.54.54.024829 R30-M0-N9-C:J16-U01 RAS KERNEL FATAL data TLB error interrupt
KERNDTLB 1118538129 2005.06.11 R30-M0-N9-C:J16-U01 2005-06-11-18.02.09.666558 R30-M0-N9-C:J16-U01 RAS KERNEL FATAL data TLB error interrupt
KERNDTLB 1118538182 2005.06.11 R30-M0-N9-C:J16-U01 2005-06-11-18.03.02.884252 R30-M0-N9-C:J16-U01 RAS KERNEL FATAL data TLB error interrupt
KERNDTLB 1118538836 2005.06.11 R30-M0-N9-C:J16-U01 2005-06-11-18.13.56.287869 R30-M0-N9-C:J16-U01 RAS KERNEL FATAL data TLB error interrupt
KERNDTLB 1118539141 2005.06.11 R30-M0-N9-C:J16-U01 2005-06-11-18.19.01.277745 R30-M0-N9-C:J16-U01 RAS KERNEL FATAL data TLB error interrupt

```

### `13-proxifier`

- [Full input](cases/13-proxifier/input.log)
- [Output with CCR](cases/13-proxifier/output.log) - [diff](cases/13-proxifier/compression.diff)
- [Output without CCR](cases/13-proxifier/output-noccr.log) - [diff](cases/13-proxifier/compression-noccr.diff)

Input excerpt:

```text
[10.30 16:49:06] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 16:49:06] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 16:49:06] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 16:49:07] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 0 bytes sent, 0 bytes received, lifetime 00:01
[10.30 16:49:07] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 16:49:07] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 16:49:07] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 16:49:07] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 403 bytes sent, 426 bytes received, lifetime <1 sec
[10.30 16:49:07] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 16:49:07] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 16:49:07] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 451 bytes sent, 18846 bytes (18.4 KB) received, lifetime <1 sec
[10.30 16:49:08] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 445 bytes sent, 5174 bytes (5.05 KB) received, lifetime <1 sec
[10.30 16:49:08] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 16:49:08] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 1190 bytes (1.16 KB) sent, 1671 bytes (1.63 KB) received, lifetime 00:02
[10.30 16:49:08] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 16:49:08] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 0 bytes sent, 0 bytes received, lifetime <1 sec
[10.30 16:49:09] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 1165 bytes (1.13 KB) sent, 3098 bytes (3.02 KB) received, lifetime 00:01
[10.30 16:49:09] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 16:49:09] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 1165 bytes (1.13 KB) sent, 815 bytes received, lifetime <1 sec
[10.30 16:49:09] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 16:49:09] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 1165 bytes (1.13 KB) sent, 783 bytes received, lifetime <1 sec
[10.30 16:49:09] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 850 bytes sent, 10547 bytes (10.2 KB) received, lifetime 00:02
[10.30 16:49:09] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 408 bytes sent, 421 bytes received, lifetime 00:03
[10.30 16:49:09] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 1165 bytes (1.13 KB) sent, 0 bytes received, lifetime <1 sec
[10.30 16:49:09] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 16:49:09] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 16:49:09] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 0 bytes sent, 0 bytes received, lifetime <1 sec
[10.30 16:49:09] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 19904 bytes (19.4 KB) sent, 27629 bytes (26.9 KB) received, lifetime 02:19
[10.30 16:49:09] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 16:49:10] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 1118 bytes (1.09 KB) sent, 340 bytes received, lifetime <1 sec
[10.30 16:49:10] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 16:49:10] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 16:49:10] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 1143 bytes (1.11 KB) sent, 365 bytes received, lifetime 00:01
[10.30 16:49:10] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 1093 bytes (1.06 KB) sent, 1006 bytes received, lifetime 00:01
[10.30 16:49:10] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 428 bytes sent, 5365 bytes (5.23 KB) received, lifetime <1 sec
[10.30 16:49:10] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS

```

Output excerpt:

```text
[... 250 line(s) omitted ... ⟦tj:b42d1e05b1e35d07e652dda9a0ace2a0⟧]
[10.30 17:14:20] Dropbox.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 17:15:10] QQ.exe - tcpconn.tencent.com:80 close, 133 bytes sent, 0 bytes received, lifetime <1 sec
[10.30 17:15:42] QQ.exe - tcpconn6.tencent.com:443 error : A connection request was canceled before the completion.
[10.30 17:16:10] QQ.exe - tcpconn4.tencent.com:80 error : Could not connect through proxy proxy.cse.cuhk.edu.hk:5070 - Proxy closed the connection unexpectedly. 
[10.30 17:17:09] SogouCloud.exe - get.sogou.com:80 close, 651 bytes sent, 346 bytes received, lifetime <1 sec
[10.30 17:17:16] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[... 169 line(s) omitted ... ⟦tj:069740731f610afca67abe9705832c9a⟧]
[10.30 17:56:23] WeChat.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 17:56:23] WeChat.exe - proxy.cse.cuhk.edu.hk:5070 close, 451 bytes sent, 353 bytes received, lifetime <1 sec
[10.30 17:57:12] YodaoDict.exe - oimagec7.ydstatic.com:80 error : A connection request was canceled before the completion. 
[10.30 17:59:03] svchost.exe *64 - proxy.cse.cuhk.edu.hk:5070 close, 293 bytes sent, 514 bytes received, lifetime <1 sec
[10.30 17:59:09] firefox.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[Template: [10.30 <*> <*> - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS] (8 lines, first 17:59:10, last 17:59:21)
[... 9 line(s) omitted ... ⟦tj:59e46c945e439af449f484967c3efe46⟧]
[10.30 18:00:10] firefox.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 18:00:10] firefox.exe - proxy.cse.cuhk.edu.hk:5070 close, 950 bytes sent, 3559 bytes (3.47 KB) received, lifetime 01:01
[10.30 18:00:12] FlashPlayerPlugin_18_0_0_209.exe - formi.baidu.com:843 error : Could not connect through proxy proxy.cse.cuhk.edu.hk:5070 - Proxy server cannot establish a connection with the target, status code 403
[10.30 18:00:13] firefox.exe - proxy.cse.cuhk.edu.hk:5070 close, 1212 bytes (1.18 KB) sent, 11440 bytes (11.1 KB) received, lifetime 00:59
[10.30 18:00:15] firefox.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[... 35 line(s) omitted ... ⟦tj:67f05a2d77d2b8076940178e60dbac2c⟧]
[10.30 18:03:47] firefox.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 18:03:47] firefox.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 18:04:14] Skype.exe - 86.99.222.235:443 error : Could not connect through proxy proxy.cse.cuhk.edu.hk:5070 - Proxy closed the connection unexpectedly.
[10.30 18:05:44] firefox.exe - proxy.cse.cuhk.edu.hk:5070 close, 983 bytes sent, 308385 bytes (301 KB) received, lifetime 01:57
[10.30 18:05:44] firefox.exe - proxy.cse.cuhk.edu.hk:5070 close, 983 bytes sent, 268665 bytes (262 KB) received, lifetime 01:57
[... 80 line(s) omitted ... ⟦tj:e4ff493529cc2cd9385df3924dcbec40⟧]
[10.30 20:42:17] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 2464 bytes (2.40 KB) sent, 1010 bytes received, lifetime 02:01
[10.30 20:42:31] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 20:42:33] YodaoDict.exe - oimagea5.ydstatic.com:80 error : A connection request was canceled before the completion.   [×6 first 20:42:33, last 20:59:47]
[10.30 20:42:33] chrome.exe - proxy.cse.cuhk.edu.hk:5070 open through proxy proxy.cse.cuhk.edu.hk:5070 HTTPS
[10.30 20:42:34] chrome.exe - proxy.cse.cuhk.edu.hk:5070 close, 1299 bytes (1.26 KB) sent, 1029 bytes (1.00 KB) received, lifetime <1 sec
[... 293 line(s) omitted ... ⟦tj:9d89469f9fd0b5c76d5eba838e99dd2b⟧]
[10.30 20:59:47] YodaoDict.exe - oimagea5.ydstatic.com:80 error : A connection request was canceled before the completion. 
[10.30 21:03:16] WeChat.exe - proxy.cse.cuhk.edu.hk:5070 close, 259 bytes sent, 5197 bytes (5.07 KB) received, lifetime <1 sec
[10.30 21:03:34] YodaoDict.exe - oimagea5.ydstatic.com:80 error : A connection request was canceled before the completion.   [×6 first 21:03:34, last 21:17:51]

```

### `16-mac`

- [Full input](cases/16-mac/input.log)
- [Output with CCR](cases/16-mac/output.log) - [diff](cases/16-mac/compression.diff)
- [Output without CCR](cases/16-mac/output-noccr.log) - [diff](cases/16-mac/compression-noccr.diff)

Input excerpt:

```text
Jul  1 09:00:55 calvisitor-10-105-160-95 kernel[0]: IOThunderboltSwitch<0>(0x0)::listenerCallback - Thunderbolt HPD packet for route = 0x0 port = 11 unplug = 0
Jul  1 09:01:05 calvisitor-10-105-160-95 com.apple.CDScheduler[43]: Thermal pressure state: 1 Memory pressure state: 0
Jul  1 09:01:06 calvisitor-10-105-160-95 QQ[10018]: FA||Url||taskID[2019352994] dealloc
Jul  1 09:02:26 calvisitor-10-105-160-95 kernel[0]: ARPT: 620701.011328: AirPort_Brcm43xx::syncPowerState: WWEN[enabled]
Jul  1 09:02:26 authorMacBook-Pro kernel[0]: ARPT: 620702.879952: AirPort_Brcm43xx::platformWoWEnable: WWEN[disable]
Jul  1 09:03:11 calvisitor-10-105-160-95 mDNSResponder[91]: mDNS_DeregisterInterface: Frequent transitions for interface awdl0 (FE80:0000:0000:0000:D8A5:90FF:FEF5:7FFF)
Jul  1 09:03:13 calvisitor-10-105-160-95 kernel[0]: ARPT: 620749.901374: IOPMPowerSource Information: onSleep,  SleepType: Normal Sleep,  'ExternalConnected': Yes, 'TimeRemaining': 0,
Jul  1 09:04:33 calvisitor-10-105-160-95 kernel[0]: ARPT: 620750.434035: wl0: wl_update_tcpkeep_seq: Original Seq: 3226706533, Ack: 3871687177, Win size: 4096
Jul  1 09:04:33 authorMacBook-Pro kernel[0]: ARPT: 620752.337198: ARPT: Wake Reason: Wake on Scan offload
Jul  1 09:04:37 authorMacBook-Pro symptomsd[215]: __73-[NetworkAnalyticsEngine observeValueForKeyPath:ofObject:change:context:]_block_invoke unexpected switch value 2
Jul  1 09:12:20 authorMacBook-Pro kernel[0]: IO80211AWDLPeerManager::setAwdlAutoMode Resuming AWDL
Jul  1 09:12:21 calvisitor-10-105-160-95 symptomsd[215]: __73-[NetworkAnalyticsEngine observeValueForKeyPath:ofObject:change:context:]_block_invoke unexpected switch value 2
Jul  1 09:18:16 calvisitor-10-105-160-95 kernel[0]: ARPT: 620896.311264: wl0: MDNS: 0 SRV Recs, 0 TXT Recs
Jul  1 09:19:03 calvisitor-10-105-160-95 kernel[0]: AppleCamIn::systemWakeCall - messageType = 0xE0000340
Jul  1 09:19:03 authorMacBook-Pro configd[53]: setting hostname to "authorMacBook-Pro.local"
Jul  1 09:19:13 calvisitor-10-105-160-95 com.apple.cts[258]: com.apple.icloud.fmfd.heartbeat: scheduler_evaluate_activity told me to run this job; however, but the start time isn't for 439034 seconds.  Ignoring.
Jul  1 09:21:57 authorMacBook-Pro corecaptured[31174]: CCIOReporterFormatter::addRegistryChildToChannelDictionary streams 7
Jul  1 09:21:58 calvisitor-10-105-160-95 com.apple.WebKit.WebContent[25654]: [09:21:58.929] <<<< CRABS >>>> crabsFlumeHostAvailable: [0x7f961cf08cf0] Byte flume reports host available again.
Jul  1 09:22:02 calvisitor-10-105-160-95 com.apple.cts[258]: com.apple.Safari.SafeBrowsing.Update: scheduler_evaluate_activity told me to run this job; however, but the start time isn't for 2450 seconds.  Ignoring.
Jul  1 09:22:25 calvisitor-10-105-160-95 kernel[0]: IO80211AWDLPeerManager::setAwdlAutoMode Resuming AWDL
Jul  1 09:23:26 calvisitor-10-105-160-95 kernel[0]: AirPort: Link Down on awdl0. Reason 1 (Unspecified).
Jul  1 09:23:26 calvisitor-10-105-160-95 kernel[0]: IOThunderboltSwitch<0>(0x0)::listenerCallback - Thunderbolt HPD packet for route = 0x0 port = 11 unplug = 0
Jul  1 09:24:13 calvisitor-10-105-160-95 kernel[0]: PM response took 2010 ms (54, powerd)
Jul  1 09:25:21 calvisitor-10-105-160-95 com.apple.cts[258]: com.apple.icloud.fmfd.heartbeat: scheduler_evaluate_activity told me to run this job; however, but the start time isn't for 438666 seconds.  Ignoring.
Jul  1 09:25:45 calvisitor-10-105-160-95 kernel[0]: ARPT: 621131.293163: wl0: Roamed or switched channel, reason #8, bssid 5c:50:15:4c:18:13, last RSSI -64
Jul  1 09:25:59 calvisitor-10-105-160-95 kernel[0]: ARPT: 621145.554555: IOPMPowerSource Information: onSleep,  SleepType: Normal Sleep,  'ExternalConnected': Yes, 'TimeRemaining': 0,
Jul  1 09:26:41 calvisitor-10-105-160-95 kernel[0]: ARPT: 621146.080894: wl0: wl_update_tcpkeep_seq: Original Seq: 3014995849, Ack: 2590995288, Win size: 4096
Jul  1 09:26:43 calvisitor-10-105-160-95 networkd[195]: nw_nat64_post_new_ifstate successfully changed NAT64 ifstate from 0x4 to 0x8000000000000000
Jul  1 09:26:47 calvisitor-10-105-160-95 com.apple.cts[258]: com.apple.Safari.SafeBrowsing.Update: scheduler_evaluate_activity told me to run this job; however, but the start time isn't for 2165 seconds.  Ignoring.
Jul  1 09:27:01 calvisitor-10-105-160-95 com.apple.cts[258]: com.apple.EscrowSecurityAlert.daily: scheduler_evaluate_activity told me to run this job; however, but the start time isn't for 13090 seconds.  Ignoring.
Jul  1 09:27:06 calvisitor-10-105-160-95 kernel[0]: IO80211AWDLPeerManager::setAwdlSuspendedMode() Suspending AWDL, enterQuietMode(true)
Jul  1 09:28:41 authorMacBook-Pro netbiosd[31198]: network_reachability_changed : network is not reachable, netbiosd is shutting down
Jul  1 09:28:41 authorMacBook-Pro corecaptured[31206]: CCFile::captureLogRun() Exiting CCFile::captureLogRun
Jul  1 09:28:50 calvisitor-10-105-160-95 com.apple.CDScheduler[258]: Thermal pressure state: 1 Memory pressure state: 0
Jul  1 09:28:53 calvisitor-10-105-160-95 com.apple.cts[258]: com.apple.Safari.SafeBrowsing.Update: scheduler_evaluate_activity told me to run this job; however, but the start time isn't for 2039 seconds.  Ignoring.
Jul  1 09:29:02 calvisitor-10-105-160-95 sandboxd[129] ([31211]): com.apple.Addres(31211) deny network-outbound /private/var/run/mDNSResponder

```

Output excerpt:

```text
[... 81 line(s) omitted ... ⟦tj:1eb31fe7bfeda68857ac48737e2203a2⟧]
Jul  1 10:10:24 calvisitor-10-105-160-95 kernel[0]: Sandbox: com.apple.Addres(31328) deny(1) network-outbound /private/var/run/mDNSResponder
Jul  1 10:10:27 calvisitor-10-105-160-95 kernel[0]: Sandbox: com.apple.Addres(31328) deny(1) network-outbound /private/var/run/mDNSResponder
Jul  1 10:13:39 calvisitor-10-105-160-95 secd[276]:  SOSAccountThisDeviceCanSyncWithCircle sync with device failure: Error Domain=com.apple.security.sos.error Code=1035 "Account identity not set" UserInfo={NSDescription=...
Jul  1 10:13:43 calvisitor-10-105-160-95 SpotlightNetHelper[352]: CFPasteboardRef CFPasteboardCreate(CFAllocatorRef, CFStringRef) : failed to create global data
Jul  1 10:13:57 calvisitor-10-105-160-95 kernel[0]: Sandbox: com.apple.Addres(31346) deny(1) network-outbound /private/var/run/mDNSResponder
Jul  1 10:38:53 calvisitor-10-105-160-95 sandboxd[129] ([31376]): com.apple.Addres(31376) deny network-outbound /private/var/run/mDNSResponder
[... 3 line(s) omitted ... ⟦tj:dac81bf55aa63ad65a31370cff72b60f⟧]
Jul  1 10:47:08 calvisitor-10-105-160-95 sandboxd[129] ([31382]): com.apple.Addres(31382) deny network-outbound /private/var/run/mDNSResponder
Jul  1 11:20:51 calvisitor-10-105-160-95 sharingd[30299]: 11:20:51.293 : BTLE discovered device with hash <01faa200 00000000 0000>
Jul  1 11:24:45 calvisitor-10-105-160-95 secd[276]:  securityd_xpc_dictionary_handler cloudd[326] copy_matching Error Domain=NSOSStatusErrorDomain Code=-50 "query missing class name" (paramErr: error in user parameter li...
Jul  1 11:29:32 calvisitor-10-105-160-95 locationd[82]: Location icon should now be in state 'Inactive'
Jul  1 11:38:18 calvisitor-10-105-160-95 kernel[0]: ARPT: 626126.086205: wl0: setup_keepalive: interval 900, retry_interval 30, retry_count 10
[... 12 line(s) omitted ... ⟦tj:c72b03af9d48cb619f2a9f3091d87400⟧]
Jul  1 11:44:25 authorMacBook-Pro kernel[0]: en0::IO80211Interface::postMessage bssid changed
Jul  1 11:44:26 authorMacBook-Pro kernel[0]: IO80211AWDLPeerManager::setAwdlOperatingMode Setting the AWDL operation mode from SUSPENDED to AUTO
Jul  1 11:46:16 calvisitor-10-105-160-95 symptomsd[215]: -[NetworkAnalyticsEngine _writeJournalRecord:fromCellFingerprint:key:atLOI:ofKind:lqm:isFaulty:] Hashing of the primary key failed. Dropping the journal record.
Jul  1 11:46:16 authorMacBook-Pro kernel[0]: AppleCamIn::systemWakeCall - messageType = 0xE0000340
Jul  1 11:46:19 authorMacBook-Pro sharingd[30299]: 11:46:19.229 : Finished generating hashes
[... 4 line(s) omitted ... ⟦tj:aeea1019fd4f6b59810d77573f7786a7⟧]
Jul  1 11:48:28 calvisitor-10-105-160-95 AddressBookSourceSync[31471]: Unrecognized attribute value: t:AbchPersonItemType
Jul  1 11:48:43 calvisitor-10-105-160-95 kernel[0]: PM response took 1938 ms (54, powerd)
Jul  1 11:49:29 calvisitor-10-105-160-95 QQ[10018]: tcp_connection_destination_perform_socket_connect 19110 connectx to 183.57.48.75:80@0 failed: [50] Network is down
Jul  1 11:49:29 calvisitor-10-105-160-95 kernel[0]: AirPort: Link Down on en0. Reason 8 (Disassociated because station leaving).
Jul  1 11:49:29 authorMacBook-Pro sharingd[30299]: 11:49:29.473 : BTLE scanner Powered On
Jul  1 11:49:29 authorMacBook-Pro kernel[0]: USBMSC Identifier (non-unique): 000000000820 0x5ac 0x8406 0x820, 3
Jul  1 11:49:29 authorMacBook-Pro symptomsd[215]: __73-[NetworkAnalyticsEngine observeValueForKeyPath:ofObject:change:context:]_block_invoke unexpected switch value 2
Jul  1 11:49:30 authorMacBook-Pro corecaptured[31480]: CCXPCService::setStreamEventHandler Registered for notification callback.
Jul  1 11:49:30 authorMacBook-Pro Dropbox[24019]: [0701/114930:WARNING:dns_config_service_posix.cc(306)] Failed to read DnsConfig.
Jul  1 11:49:35 calvisitor-10-105-160-95 cdpd[11807]: Saw change in network reachability (isReachable=2)
Jul  1 11:51:02 authorMacBook-Pro kernel[0]: [HID] [ATC] AppleDeviceManagementHIDEventService::processWakeReason Wake reason: Host (0x01)
[... 12 line(s) omitted ... ⟦tj:e1ac24ae345c45db26187da7916dc651⟧]
Jul  1 11:58:27 authorMacBook-Pro com.apple.cts[258]: com.apple.suggestions.harvest: scheduler_evaluate_activity told me to run this job; however, but the start time isn't for 15323 seconds.  Ignoring.
Jul  1 12:12:04 calvisitor-10-105-160-95 kernel[0]: AppleCamIn::systemWakeCall - messageType = 0xE0000340
Jul  1 12:12:21 calvisitor-10-105-160-95 secd[276]:  SOSAccountThisDeviceCanSyncWithCircle sync with device failure: Error Domain=com.apple.security.sos.error Code=1035 "Account identity not set" UserInfo={NSDescription=...
Jul  1 12:26:01 calvisitor-10-105-160-95 com.apple.cts[258]: com.apple.icloud.fmfd.heartbeat: scheduler_evaluate_activity told me to run this job; however, but the start time isn't for 427826 seconds.  Ignoring.

```

### `14-openssh`

- [Full input](cases/14-openssh/input.log)
- [Output with CCR](cases/14-openssh/output.log) - [diff](cases/14-openssh/compression.diff)
- [Output without CCR](cases/14-openssh/output-noccr.log) - [diff](cases/14-openssh/compression-noccr.diff)

Input excerpt:

```text
Dec 10 06:55:46 LabSZ sshd[24200]: reverse mapping checking getaddrinfo for ns.marryaldkfaczcz.com [173.234.31.186] failed - POSSIBLE BREAK-IN ATTEMPT!
Dec 10 06:55:46 LabSZ sshd[24200]: Invalid user webmaster from 173.234.31.186
Dec 10 06:55:46 LabSZ sshd[24200]: input_userauth_request: invalid user webmaster [preauth]
Dec 10 06:55:46 LabSZ sshd[24200]: pam_unix(sshd:auth): check pass; user unknown
Dec 10 06:55:46 LabSZ sshd[24200]: pam_unix(sshd:auth): authentication failure; logname= uid=0 euid=0 tty=ssh ruser= rhost=173.234.31.186 
Dec 10 06:55:48 LabSZ sshd[24200]: Failed password for invalid user webmaster from 173.234.31.186 port 38926 ssh2
Dec 10 06:55:48 LabSZ sshd[24200]: Connection closed by 173.234.31.186 [preauth]
Dec 10 07:02:47 LabSZ sshd[24203]: Connection closed by 212.47.254.145 [preauth]
Dec 10 07:07:38 LabSZ sshd[24206]: Invalid user test9 from 52.80.34.196
Dec 10 07:07:38 LabSZ sshd[24206]: input_userauth_request: invalid user test9 [preauth]
Dec 10 07:07:38 LabSZ sshd[24206]: pam_unix(sshd:auth): check pass; user unknown
Dec 10 07:07:38 LabSZ sshd[24206]: pam_unix(sshd:auth): authentication failure; logname= uid=0 euid=0 tty=ssh ruser= rhost=ec2-52-80-34-196.cn-north-1.compute.amazonaws.com.cn 
Dec 10 07:07:45 LabSZ sshd[24206]: Failed password for invalid user test9 from 52.80.34.196 port 36060 ssh2
Dec 10 07:07:45 LabSZ sshd[24206]: Received disconnect from 52.80.34.196: 11: Bye Bye [preauth]
Dec 10 07:08:28 LabSZ sshd[24208]: reverse mapping checking getaddrinfo for ns.marryaldkfaczcz.com [173.234.31.186] failed - POSSIBLE BREAK-IN ATTEMPT!
Dec 10 07:08:28 LabSZ sshd[24208]: Invalid user webmaster from 173.234.31.186
Dec 10 07:08:28 LabSZ sshd[24208]: input_userauth_request: invalid user webmaster [preauth]
Dec 10 07:08:28 LabSZ sshd[24208]: pam_unix(sshd:auth): check pass; user unknown
Dec 10 07:08:28 LabSZ sshd[24208]: pam_unix(sshd:auth): authentication failure; logname= uid=0 euid=0 tty=ssh ruser= rhost=173.234.31.186 
Dec 10 07:08:30 LabSZ sshd[24208]: Failed password for invalid user webmaster from 173.234.31.186 port 39257 ssh2
Dec 10 07:08:30 LabSZ sshd[24208]: Connection closed by 173.234.31.186 [preauth]
Dec 10 07:11:42 LabSZ sshd[24224]: Invalid user chen from 202.100.179.208
Dec 10 07:11:42 LabSZ sshd[24224]: input_userauth_request: invalid user chen [preauth]
Dec 10 07:11:42 LabSZ sshd[24224]: pam_unix(sshd:auth): check pass; user unknown
Dec 10 07:11:42 LabSZ sshd[24224]: pam_unix(sshd:auth): authentication failure; logname= uid=0 euid=0 tty=ssh ruser= rhost=202.100.179.208 
Dec 10 07:11:44 LabSZ sshd[24224]: Failed password for invalid user chen from 202.100.179.208 port 32484 ssh2
Dec 10 07:11:44 LabSZ sshd[24224]: Received disconnect from 202.100.179.208: 11: Bye Bye [preauth]
Dec 10 07:13:31 LabSZ sshd[24227]: pam_unix(sshd:auth): authentication failure; logname= uid=0 euid=0 tty=ssh ruser= rhost=5.36.59.76.dynamic-dsl-ip.omantel.net.om  user=root
Dec 10 07:13:43 LabSZ sshd[24227]: Failed password for root from 5.36.59.76 port 42393 ssh2
Dec 10 07:13:56 LabSZ sshd[24227]: message repeated 5 times: [ Failed password for root from 5.36.59.76 port 42393 ssh2]
Dec 10 07:13:56 LabSZ sshd[24227]: Disconnecting: Too many authentication failures for root [preauth]
Dec 10 07:13:56 LabSZ sshd[24227]: PAM 5 more authentication failures; logname= uid=0 euid=0 tty=ssh ruser= rhost=5.36.59.76.dynamic-dsl-ip.omantel.net.om  user=root
Dec 10 07:13:56 LabSZ sshd[24227]: PAM service(sshd) ignoring max retries; 6 > 3
Dec 10 07:27:50 LabSZ sshd[24235]: pam_unix(sshd:auth): authentication failure; logname= uid=0 euid=0 tty=ssh ruser= rhost=112.95.230.3  user=root
Dec 10 07:27:52 LabSZ sshd[24235]: Failed password for root from 112.95.230.3 port 45378 ssh2
Dec 10 07:27:52 LabSZ sshd[24235]: Received disconnect from 112.95.230.3: 11: Bye Bye [preauth]

```

Output excerpt:

```text
Dec 10 06:55:46 LabSZ sshd[24200]: reverse mapping checking getaddrinfo for ns.marryaldkfaczcz.com [173.234.31.186] failed - POSSIBLE BREAK-IN ATTEMPT!
Dec 10 06:55:46 LabSZ sshd[24200]: Invalid user webmaster from 173.234.31.186
Dec 10 06:55:46 LabSZ sshd[24200]: input_userauth_request: invalid user webmaster [preauth]
Dec 10 06:55:46 LabSZ sshd[24200]: pam_unix(sshd:auth): check pass; user unknown
Dec 10 06:55:46 LabSZ sshd[24200]: pam_unix(sshd:auth): authentication failure; logname= uid=0 euid=0 tty=ssh ruser= rhost=173.234.31.186 
Dec 10 06:55:48 LabSZ sshd[24200]: Failed password for invalid user webmaster from 173.234.31.186 port 38926 ssh2
Dec 10 06:55:48 LabSZ sshd[24200]: Connection closed by 173.234.31.186 [preauth]
Dec 10 07:02:47 LabSZ sshd[24203]: Connection closed by 212.47.254.145 [preauth]
Dec 10 07:07:38 LabSZ sshd[24206]: Invalid user test9 from 52.80.34.196
Dec 10 07:07:38 LabSZ sshd[24206]: input_userauth_request: invalid user test9 [preauth]
Dec 10 07:07:38 LabSZ sshd[24206]: pam_unix(sshd:auth): check pass; user unknown
Dec 10 07:07:38 LabSZ sshd[24206]: pam_unix(sshd:auth): authentication failure; logname= uid=0 euid=0 tty=ssh ruser= rhost=ec2-52-80-34-196.cn-north-1.compute.amazonaws.com.cn 
Dec 10 07:07:45 LabSZ sshd[24206]: Failed password for invalid user test9 from 52.80.34.196 port 36060 ssh2
Dec 10 07:07:45 LabSZ sshd[24206]: Received disconnect from 52.80.34.196: 11: Bye Bye [preauth]
Dec 10 07:08:28 LabSZ sshd[24208]: reverse mapping checking getaddrinfo for ns.marryaldkfaczcz.com [173.234.31.186] failed - POSSIBLE BREAK-IN ATTEMPT!
Dec 10 07:08:28 LabSZ sshd[24208]: Invalid user webmaster from 173.234.31.186
Dec 10 07:08:28 LabSZ sshd[24208]: input_userauth_request: invalid user webmaster [preauth]
Dec 10 07:08:28 LabSZ sshd[24208]: pam_unix(sshd:auth): check pass; user unknown
Dec 10 07:08:28 LabSZ sshd[24208]: pam_unix(sshd:auth): authentication failure; logname= uid=0 euid=0 tty=ssh ruser= rhost=173.234.31.186 
Dec 10 07:08:30 LabSZ sshd[24208]: Failed password for invalid user webmaster from 173.234.31.186 port 39257 ssh2
Dec 10 07:08:30 LabSZ sshd[24208]: Connection closed by 173.234.31.186 [preauth]
Dec 10 07:11:42 LabSZ sshd[24224]: Invalid user chen from 202.100.179.208
Dec 10 07:11:42 LabSZ sshd[24224]: input_userauth_request: invalid user chen [preauth]
Dec 10 07:11:42 LabSZ sshd[24224]: pam_unix(sshd:auth): check pass; user unknown
Dec 10 07:11:42 LabSZ sshd[24224]: pam_unix(sshd:auth): authentication failure; logname= uid=0 euid=0 tty=ssh ruser= rhost=202.100.179.208 
Dec 10 07:11:44 LabSZ sshd[24224]: Failed password for invalid user chen from 202.100.179.208 port 32484 ssh2
Dec 10 07:11:44 LabSZ sshd[24224]: Received disconnect from 202.100.179.208: 11: Bye Bye [preauth]
Dec 10 07:13:31 LabSZ sshd[24227]: pam_unix(sshd:auth): authentication failure; logname= uid=0 euid=0 tty=ssh ruser= rhost=5.36.59.76.dynamic-dsl-ip.omantel.net.om  user=root
Dec 10 07:13:43 LabSZ sshd[24227]: Failed password for root from 5.36.59.76 port 42393 ssh2
Dec 10 07:13:56 LabSZ sshd[24227]: message repeated 5 times: [ Failed password for root from 5.36.59.76 port 42393 ssh2]
Dec 10 07:13:56 LabSZ sshd[24227]: Disconnecting: Too many authentication failures for root [preauth]
Dec 10 07:13:56 LabSZ sshd[24227]: PAM 5 more authentication failures; logname= uid=0 euid=0 tty=ssh ruser= rhost=5.36.59.76.dynamic-dsl-ip.omantel.net.om  user=root
Dec 10 07:13:56 LabSZ sshd[24227]: PAM service(sshd) ignoring max retries; 6 > 3
Dec 10 07:27:50 LabSZ sshd[24235]: pam_unix(sshd:auth): authentication failure; logname= uid=0 euid=0 tty=ssh ruser= rhost=112.95.230.3  user=root
Dec 10 07:27:52 LabSZ sshd[24235]: Failed password for root from 112.95.230.3 port 45378 ssh2
Dec 10 07:27:52 LabSZ sshd[24235]: Received disconnect from 112.95.230.3: 11: Bye Bye [preauth]

```

### `08-windows`

- [Full input](cases/08-windows/input.log)
- [Output with CCR](cases/08-windows/output.log) - [diff](cases/08-windows/compression.diff)
- [Output without CCR](cases/08-windows/output-noccr.log) - [diff](cases/08-windows/compression-noccr.diff)

Input excerpt:

```text
2016-09-28 04:30:30, Info                  CBS    Loaded Servicing Stack v6.1.7601.23505 with Core: C:\Windows\winsxs\amd64_microsoft-windows-servicingstack_31bf3856ad364e35_6.1.7601.23505_none_681aa442f6fed7f0\cbscore.d...
2016-09-28 04:30:31, Info                  CSI    00000001@2016/9/27:20:30:31.455 WcpInitialize (wcp.dll version 0.0.0.6) called (stack @0x7fed806eb5d @0x7fef9fb9b6d @0x7fef9f8358f @0xff83e97c @0xff83d799 @0xff83db2f)
2016-09-28 04:30:31, Info                  CSI    00000002@2016/9/27:20:30:31.458 WcpInitialize (wcp.dll version 0.0.0.6) called (stack @0x7fed806eb5d @0x7fefa006ade @0x7fef9fd2984 @0x7fef9f83665 @0xff83e97c @0xff83d799)
2016-09-28 04:30:31, Info                  CSI    00000003@2016/9/27:20:30:31.458 WcpInitialize (wcp.dll version 0.0.0.6) called (stack @0x7fed806eb5d @0x7fefa1c8728 @0x7fefa1c8856 @0xff83e474 @0xff83d7de @0xff83db2f)
2016-09-28 04:30:31, Info                  CBS    Ending TrustedInstaller initialization.
2016-09-28 04:30:31, Info                  CBS    Starting the TrustedInstaller main loop.
2016-09-28 04:30:31, Info                  CBS    TrustedInstaller service starts successfully.
2016-09-28 04:30:31, Info                  CBS    SQM: Initializing online with Windows opt-in: False
2016-09-28 04:30:31, Info                  CBS    SQM: Cleaning up report files older than 10 days.
2016-09-28 04:30:31, Info                  CBS    SQM: Requesting upload of all unsent reports.
2016-09-28 04:30:31, Info                  CBS    SQM: Failed to start upload with file pattern: C:\Windows\servicing\sqm\*_std.sqm, flags: 0x2 [HRESULT = 0x80004005 - E_FAIL]
2016-09-28 04:30:31, Info                  CBS    SQM: Failed to start standard sample upload. [HRESULT = 0x80004005 - E_FAIL]
2016-09-28 04:30:31, Info                  CBS    SQM: Queued 0 file(s) for upload with pattern: C:\Windows\servicing\sqm\*_all.sqm, flags: 0x6
2016-09-28 04:30:31, Info                  CBS    SQM: Warning: Failed to upload all unsent reports. [HRESULT = 0x80004005 - E_FAIL]
2016-09-28 04:30:31, Info                  CBS    No startup processing required, TrustedInstaller service was not set as autostart, or else a reboot is still pending.
2016-09-28 04:30:31, Info                  CBS    NonStart: Checking to ensure startup processing was not required.
2016-09-28 04:30:31, Info                  CSI    00000004 IAdvancedInstallerAwareStore_ResolvePendingTransactions (call 1) (flags = 00000004, progress = NULL, phase = 0, pdwDisposition = @0xb6fd90
2016-09-28 04:30:31, Info                  CSI    00000005 Creating NT transaction (seq 1), objectname [6]"(null)"
2016-09-28 04:30:31, Info                  CSI    00000006 Created NT transaction (seq 1) result 0x00000000, handle @0x214
2016-09-28 04:30:31, Info                  CSI    00000007@2016/9/27:20:30:31.462 CSI perf trace:
2016-09-28 04:30:31, Info                  CBS    NonStart: Success, startup processing not required as expected.
2016-09-28 04:30:31, Info                  CBS    Startup processing thread terminated normally
2016-09-28 04:30:31, Info                  CSI    00000008 CSI Store 4991456 (0x00000000004c29e0) initialized
2016-09-28 04:30:31, Info                  CBS    Session: 30546173_4261722401 initialized by client WindowsUpdateAgent.
2016-09-28 04:30:31, Info                  CBS    Session: 30546173_4262462443 initialized by client WindowsUpdateAgent.
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.
2016-09-28 04:30:31, Info                  CBS    Expecting attribute name [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.
2016-09-28 04:30:31, Info                  CBS    Expecting attribute name [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.
2016-09-28 04:30:31, Info                  CBS    Expecting attribute name [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.
2016-09-28 04:30:31, Info                  CBS    Expecting attribute name [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]

```

Output excerpt:

```text
[... 8 line(s) omitted ... ⟦tj:9f54e7ad069923e6fb445671f06d52ac⟧]
2016-09-28 04:30:31, Info                  CBS    SQM: Cleaning up report files older than 10 days.
2016-09-28 04:30:31, Info                  CBS    SQM: Requesting upload of all unsent reports.
2016-09-28 04:30:31, Info                  CBS    SQM: Failed to start upload with file pattern: C:\Windows\servicing\sqm\*_std.sqm, flags: 0x2 [HRESULT = 0x80004005 - E_FAIL]
2016-09-28 04:30:31, Info                  CBS    SQM: Failed to start standard sample upload. [HRESULT = 0x80004005 - E_FAIL]
2016-09-28 04:30:31, Info                  CBS    SQM: Queued 0 file(s) for upload with pattern: C:\Windows\servicing\sqm\*_all.sqm, flags: 0x6
2016-09-28 04:30:31, Info                  CBS    SQM: Warning: Failed to upload all unsent reports. [HRESULT = 0x80004005 - E_FAIL]
2016-09-28 04:30:31, Info                  CBS    No startup processing required, TrustedInstaller service was not set as autostart, or else a reboot is still pending.
2016-09-28 04:30:31, Info                  CBS    NonStart: Checking to ensure startup processing was not required.
[... 9 line(s) omitted ... ⟦tj:eb027cf7e815ea5157343d5fd663af02⟧]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.  [×120 first 2016-09-28, last 2016-09-28]
2016-09-28 04:30:31, Info                  CBS    Expecting attribute name [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]  [×96 first 2016-09-28, last 2016-09-28]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.
2016-09-28 04:30:31, Info                  CBS    Expecting attribute name [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.
2016-09-28 04:30:31, Info                  CBS    Expecting attribute name [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.
2016-09-28 04:30:31, Info                  CBS    Expecting attribute name [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
[... 3 line(s) omitted ... ⟦tj:d7dbf7c5d52999b51edd437b8f46637e⟧]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.
2016-09-28 04:30:31, Info                  CBS    Expecting attribute name [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.
2016-09-28 04:30:31, Info                  CBS    Expecting attribute name [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.
2016-09-28 04:30:31, Info                  CBS    Expecting attribute name [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
[... 3 line(s) omitted ... ⟦tj:d7dbf7c5d52999b51edd437b8f46637e⟧]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.

```

### `02-hadoop`

- [Full input](cases/02-hadoop/input.log)
- [Output with CCR](cases/02-hadoop/output.log) - [diff](cases/02-hadoop/compression.diff)
- [Output without CCR](cases/02-hadoop/output-noccr.log) - [diff](cases/02-hadoop/compression-noccr.diff)

Input excerpt:

```text
2015-10-18 18:01:47,978 INFO [main] org.apache.hadoop.mapreduce.v2.app.MRAppMaster: Created MRAppMaster for application appattempt_1445144423722_0020_000001
2015-10-18 18:01:48,963 INFO [main] org.apache.hadoop.mapreduce.v2.app.MRAppMaster: Executing with tokens:
2015-10-18 18:01:48,963 INFO [main] org.apache.hadoop.mapreduce.v2.app.MRAppMaster: Kind: YARN_AM_RM_TOKEN, Service: , Ident: (appAttemptId { application_id { id: 20 cluster_timestamp: 1445144423722 } attemptId: 1 } keyI...
2015-10-18 18:01:49,228 INFO [main] org.apache.hadoop.mapreduce.v2.app.MRAppMaster: Using mapred newApiCommitter.
2015-10-18 18:01:50,353 INFO [main] org.apache.hadoop.mapreduce.v2.app.MRAppMaster: OutputCommitter set in config null
2015-10-18 18:01:50,509 INFO [main] org.apache.hadoop.mapreduce.v2.app.MRAppMaster: OutputCommitter is org.apache.hadoop.mapreduce.lib.output.FileOutputCommitter
2015-10-18 18:01:50,556 INFO [main] org.apache.hadoop.yarn.event.AsyncDispatcher: Registering class org.apache.hadoop.mapreduce.jobhistory.EventType for class org.apache.hadoop.mapreduce.jobhistory.JobHistoryEventHandler
2015-10-18 18:01:50,556 INFO [main] org.apache.hadoop.yarn.event.AsyncDispatcher: Registering class org.apache.hadoop.mapreduce.v2.app.job.event.JobEventType for class org.apache.hadoop.mapreduce.v2.app.MRAppMaster$JobEv...
2015-10-18 18:01:50,556 INFO [main] org.apache.hadoop.yarn.event.AsyncDispatcher: Registering class org.apache.hadoop.mapreduce.v2.app.job.event.TaskEventType for class org.apache.hadoop.mapreduce.v2.app.MRAppMaster$Task...
2015-10-18 18:01:50,556 INFO [main] org.apache.hadoop.yarn.event.AsyncDispatcher: Registering class org.apache.hadoop.mapreduce.v2.app.job.event.TaskAttemptEventType for class org.apache.hadoop.mapreduce.v2.app.MRAppMast...
2015-10-18 18:01:50,572 INFO [main] org.apache.hadoop.yarn.event.AsyncDispatcher: Registering class org.apache.hadoop.mapreduce.v2.app.commit.CommitterEventType for class org.apache.hadoop.mapreduce.v2.app.commit.Committ...
2015-10-18 18:01:50,572 INFO [main] org.apache.hadoop.yarn.event.AsyncDispatcher: Registering class org.apache.hadoop.mapreduce.v2.app.speculate.Speculator$EventType for class org.apache.hadoop.mapreduce.v2.app.MRAppMast...
2015-10-18 18:01:50,572 INFO [main] org.apache.hadoop.yarn.event.AsyncDispatcher: Registering class org.apache.hadoop.mapreduce.v2.app.rm.ContainerAllocator$EventType for class org.apache.hadoop.mapreduce.v2.app.MRAppMas...
2015-10-18 18:01:50,588 INFO [main] org.apache.hadoop.yarn.event.AsyncDispatcher: Registering class org.apache.hadoop.mapreduce.v2.app.launcher.ContainerLauncher$EventType for class org.apache.hadoop.mapreduce.v2.app.MRA...
2015-10-18 18:01:50,634 INFO [main] org.apache.hadoop.mapreduce.v2.jobhistory.JobHistoryUtils: Default file system [hdfs://msra-sa-41:9000]
2015-10-18 18:01:50,666 INFO [main] org.apache.hadoop.mapreduce.v2.jobhistory.JobHistoryUtils: Default file system [hdfs://msra-sa-41:9000]
2015-10-18 18:01:50,713 INFO [main] org.apache.hadoop.mapreduce.v2.jobhistory.JobHistoryUtils: Default file system [hdfs://msra-sa-41:9000]
2015-10-18 18:01:50,728 INFO [main] org.apache.hadoop.mapreduce.jobhistory.JobHistoryEventHandler: Emitting job history data to the timeline server is not enabled
2015-10-18 18:01:50,806 INFO [main] org.apache.hadoop.yarn.event.AsyncDispatcher: Registering class org.apache.hadoop.mapreduce.v2.app.job.event.JobFinishEvent$Type for class org.apache.hadoop.mapreduce.v2.app.MRAppMaste...
2015-10-18 18:01:51,197 INFO [main] org.apache.hadoop.metrics2.impl.MetricsConfig: loaded properties from hadoop-metrics2.properties
2015-10-18 18:01:51,306 INFO [main] org.apache.hadoop.metrics2.impl.MetricsSystemImpl: Scheduled snapshot period at 10 second(s).
2015-10-18 18:01:51,306 INFO [main] org.apache.hadoop.metrics2.impl.MetricsSystemImpl: MRAppMaster metrics system started
2015-10-18 18:01:51,322 INFO [main] org.apache.hadoop.mapreduce.v2.app.job.impl.JobImpl: Adding job token for job_1445144423722_0020 to jobTokenSecretManager
2015-10-18 18:01:51,619 INFO [main] org.apache.hadoop.mapreduce.v2.app.job.impl.JobImpl: Not uberizing job_1445144423722_0020 because: not enabled; too many maps; too much input;
2015-10-18 18:01:51,650 INFO [main] org.apache.hadoop.mapreduce.v2.app.job.impl.JobImpl: Input size for job job_1445144423722_0020 = 1256521728. Number of splits = 10
2015-10-18 18:01:51,650 INFO [main] org.apache.hadoop.mapreduce.v2.app.job.impl.JobImpl: Number of reduces for job job_1445144423722_0020 = 1
2015-10-18 18:01:51,650 INFO [main] org.apache.hadoop.mapreduce.v2.app.job.impl.JobImpl: job_1445144423722_0020Job Transitioned from NEW to INITED
2015-10-18 18:01:51,650 INFO [main] org.apache.hadoop.mapreduce.v2.app.MRAppMaster: MRAppMaster launching normal, non-uberized, multi-container job job_1445144423722_0020.
2015-10-18 18:01:51,713 INFO [main] org.apache.hadoop.ipc.CallQueueManager: Using callQueue class java.util.concurrent.LinkedBlockingQueue
2015-10-18 18:01:51,775 INFO [Socket Reader #1 for port 62260] org.apache.hadoop.ipc.Server: Starting Socket Reader #1 for port 62260
2015-10-18 18:01:51,791 INFO [main] org.apache.hadoop.yarn.factories.impl.pb.RpcServerFactoryPBImpl: Adding protocol org.apache.hadoop.mapreduce.v2.api.MRClientProtocolPB to the server
2015-10-18 18:01:51,791 INFO [main] org.apache.hadoop.mapreduce.v2.app.client.MRClientService: Instantiated MRClientService at MININT-FNANLI5.fareast.corp.microsoft.com/10.86.169.121:62260
2015-10-18 18:01:51,806 INFO [IPC Server Responder] org.apache.hadoop.ipc.Server: IPC Server Responder: starting
2015-10-18 18:01:51,806 INFO [IPC Server listener on 62260] org.apache.hadoop.ipc.Server: IPC Server listener on 62260: starting
2015-10-18 18:01:51,885 INFO [main] org.mortbay.log: Logging to org.slf4j.impl.Log4jLoggerAdapter(org.mortbay.log) via org.mortbay.log.Slf4jLog
2015-10-18 18:01:51,900 INFO [main] org.apache.hadoop.http.HttpRequestLog: Http request log for http.requests.mapreduce is not defined

```

Output excerpt:

```text
[... 665 line(s) omitted ... ⟦tj:a479d622f314c9731abf38fde532523a⟧]
2015-10-18 18:04:11,034 INFO [RMCommunicator Allocator] org.apache.hadoop.mapreduce.v2.app.rm.RMContainerRequestor: getResources() for application_1445144423722_0020: ask=0 release= 1 newContainers=0 finishedContainers=1...
2015-10-18 18:04:11,034 INFO [RMCommunicator Allocator] org.apache.hadoop.mapreduce.v2.app.rm.RMContainerAllocator: Received completed container container_1445144423722_0020_01_000012
2015-10-18 18:04:11,034 ERROR [RMCommunicator Allocator] org.apache.hadoop.mapreduce.v2.app.rm.RMContainerAllocator: Container complete event for unknown container id container_1445144423722_0020_01_000012
2015-10-18 18:04:11,034 INFO [RMCommunicator Allocator] org.apache.hadoop.mapreduce.v2.app.rm.RMContainerAllocator: Recalculating schedule, headroom=<memory:1024, vCores:-26>
2015-10-18 18:04:11,034 INFO [RMCommunicator Allocator] org.apache.hadoop.mapreduce.v2.app.rm.RMContainerAllocator: Reduce slow start threshold not met. completedMapsForReduceSlowstart 1
[... 177 line(s) omitted ... ⟦tj:3a12358835374311ff26072adbe1162f⟧]
2015-10-18 18:05:27,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.ipc.Client: Address change detected. Old: msra-sa-41/10.190.173.170:9000 New: msra-sa-41:9000  [×327 first 2015-10-18, last 2015-10-18]
2015-10-18 18:05:27,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 30 seconds.  Will retry shortly ...  [×326 first...
2015-10-18 18:05:28,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.ipc.Client: Address change detected. Old: msra-sa-41/10.190.173.170:9000 New: msra-sa-41:9000
2015-10-18 18:05:28,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 31 seconds.  Will retry shortly ...
2015-10-18 18:05:29,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.ipc.Client: Address change detected. Old: msra-sa-41/10.190.173.170:9000 New: msra-sa-41:9000
2015-10-18 18:05:29,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 32 seconds.  Will retry shortly ...
2015-10-18 18:05:30,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.ipc.Client: Address change detected. Old: msra-sa-41/10.190.173.170:9000 New: msra-sa-41:9000
2015-10-18 18:05:30,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 33 seconds.  Will retry shortly ...
2015-10-18 18:05:31,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.ipc.Client: Address change detected. Old: msra-sa-41/10.190.173.170:9000 New: msra-sa-41:9000
2015-10-18 18:05:31,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 34 seconds.  Will retry shortly ...
2015-10-18 18:05:32,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.ipc.Client: Address change detected. Old: msra-sa-41/10.190.173.170:9000 New: msra-sa-41:9000
2015-10-18 18:05:32,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 35 seconds.  Will retry shortly ...
2015-10-18 18:05:33,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.ipc.Client: Address change detected. Old: msra-sa-41/10.190.173.170:9000 New: msra-sa-41:9000
2015-10-18 18:05:33,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 36 seconds.  Will retry shortly ...
2015-10-18 18:05:34,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.ipc.Client: Address change detected. Old: msra-sa-41/10.190.173.170:9000 New: msra-sa-41:9000
2015-10-18 18:05:34,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 37 seconds.  Will retry shortly ...
2015-10-18 18:05:35,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.ipc.Client: Address change detected. Old: msra-sa-41/10.190.173.170:9000 New: msra-sa-41:9000
2015-10-18 18:05:35,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 38 seconds.  Will retry shortly ...
2015-10-18 18:05:36,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.ipc.Client: Address change detected. Old: msra-sa-41/10.190.173.170:9000 New: msra-sa-41:9000
2015-10-18 18:05:36,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 39 seconds.  Will retry shortly ...
2015-10-18 18:05:37,617 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.ipc.Client: Address change detected. Old: msra-sa-41/10.190.173.170:9000 New: msra-sa-41:9000
2015-10-18 18:05:37,617 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 40 seconds.  Will retry shortly ...
2015-10-18 18:05:38,617 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.ipc.Client: Address change detected. Old: msra-sa-41/10.190.173.170:9000 New: msra-sa-41:9000
2015-10-18 18:05:38,617 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 41 seconds.  Will retry shortly ...
2015-10-18 18:05:39,617 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.ipc.Client: Address change detected. Old: msra-sa-41/10.190.173.170:9000 New: msra-sa-41:9000
2015-10-18 18:05:39,617 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 42 seconds.  Will retry shortly ...
2015-10-18 18:05:40,617 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.ipc.Client: Address change detected. Old: msra-sa-41/10.190.173.170:9000 New: msra-sa-41:9000
2015-10-18 18:05:40,617 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 43 seconds.  Will retry shortly ...
2015-10-18 18:05:41,617 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.ipc.Client: Address change detected. Old: msra-sa-41/10.190.173.170:9000 New: msra-sa-41:9000

```

### `07-thunderbird`

- [Full input](cases/07-thunderbird/input.log)
- [Output with CCR](cases/07-thunderbird/output.log) - [diff](cases/07-thunderbird/compression.diff)
- [Output without CCR](cases/07-thunderbird/output-noccr.log) - [diff](cases/07-thunderbird/compression-noccr.diff)

Input excerpt:

```text
- 1131566461 2005.11.09 dn228 Nov 9 12:01:01 dn228/dn228 crond(pam_unix)[2915]: session closed for user root
- 1131566461 2005.11.09 dn228 Nov 9 12:01:01 dn228/dn228 crond(pam_unix)[2915]: session opened for user root by (uid=0)
- 1131566461 2005.11.09 dn228 Nov 9 12:01:01 dn228/dn228 crond[2916]: (root) CMD (run-parts /etc/cron.hourly)
- 1131566461 2005.11.09 dn261 Nov 9 12:01:01 dn261/dn261 crond(pam_unix)[2907]: session closed for user root
- 1131566461 2005.11.09 dn261 Nov 9 12:01:01 dn261/dn261 crond(pam_unix)[2907]: session opened for user root by (uid=0)
- 1131566461 2005.11.09 dn261 Nov 9 12:01:01 dn261/dn261 crond[2908]: (root) CMD (run-parts /etc/cron.hourly)
- 1131566461 2005.11.09 dn3 Nov 9 12:01:01 dn3/dn3 crond(pam_unix)[2907]: session closed for user root
- 1131566461 2005.11.09 dn3 Nov 9 12:01:01 dn3/dn3 crond(pam_unix)[2907]: session opened for user root by (uid=0)
- 1131566461 2005.11.09 dn3 Nov 9 12:01:01 dn3/dn3 crond[2908]: (root) CMD (run-parts /etc/cron.hourly)
- 1131566461 2005.11.09 dn596 Nov 9 12:01:01 dn596/dn596 crond(pam_unix)[2727]: session closed for user root
- 1131566461 2005.11.09 dn596 Nov 9 12:01:01 dn596/dn596 crond(pam_unix)[2727]: session opened for user root by (uid=0)
- 1131566461 2005.11.09 dn596 Nov 9 12:01:01 dn596/dn596 crond[2728]: (root) CMD (run-parts /etc/cron.hourly)
- 1131566461 2005.11.09 dn700 Nov 9 12:01:01 dn700/dn700 crond(pam_unix)[2912]: session closed for user root
- 1131566461 2005.11.09 dn700 Nov 9 12:01:01 dn700/dn700 crond(pam_unix)[2912]: session opened for user root by (uid=0)
- 1131566461 2005.11.09 dn700 Nov 9 12:01:01 dn700/dn700 crond[2913]: (root) CMD (run-parts /etc/cron.hourly)
- 1131566461 2005.11.09 dn73 Nov 9 12:01:01 dn73/dn73 crond(pam_unix)[2917]: session closed for user root
- 1131566461 2005.11.09 dn73 Nov 9 12:01:01 dn73/dn73 crond(pam_unix)[2917]: session opened for user root by (uid=0)
- 1131566461 2005.11.09 dn73 Nov 9 12:01:01 dn73/dn73 crond[2918]: (root) CMD (run-parts /etc/cron.hourly)
- 1131566461 2005.11.09 dn731 Nov 9 12:01:01 dn731/dn731 crond(pam_unix)[2916]: session closed for user root
- 1131566461 2005.11.09 dn731 Nov 9 12:01:01 dn731/dn731 crond(pam_unix)[2916]: session opened for user root by (uid=0)
- 1131566461 2005.11.09 dn731 Nov 9 12:01:01 dn731/dn731 crond[2917]: (root) CMD (run-parts /etc/cron.hourly)
- 1131566461 2005.11.09 dn754 Nov 9 12:01:01 dn754/dn754 crond(pam_unix)[2913]: session closed for user root
- 1131566461 2005.11.09 dn754 Nov 9 12:01:01 dn754/dn754 crond(pam_unix)[2913]: session opened for user root by (uid=0)
- 1131566461 2005.11.09 dn754 Nov 9 12:01:01 dn754/dn754 crond[2914]: (root) CMD (run-parts /etc/cron.hourly)
- 1131566461 2005.11.09 dn978 Nov 9 12:01:01 dn978/dn978 crond(pam_unix)[2920]: session closed for user root
- 1131566461 2005.11.09 dn978 Nov 9 12:01:01 dn978/dn978 crond(pam_unix)[2920]: session opened for user root by (uid=0)
- 1131566461 2005.11.09 dn978 Nov 9 12:01:01 dn978/dn978 crond[2921]: (root) CMD (run-parts /etc/cron.hourly)
- 1131566461 2005.11.09 eadmin1 Nov 9 12:01:01 src@eadmin1 crond(pam_unix)[4307]: session closed for user root
- 1131566461 2005.11.09 eadmin1 Nov 9 12:01:01 src@eadmin1 crond(pam_unix)[4307]: session opened for user root by (uid=0)
- 1131566461 2005.11.09 eadmin1 Nov 9 12:01:01 src@eadmin1 crond[4308]: (root) CMD (run-parts /etc/cron.hourly)
- 1131566461 2005.11.09 eadmin2 Nov 9 12:01:01 src@eadmin2 crond(pam_unix)[12636]: session closed for user root
- 1131566461 2005.11.09 eadmin2 Nov 9 12:01:01 src@eadmin2 crond(pam_unix)[12636]: session opened for user root by (uid=0)
- 1131566461 2005.11.09 eadmin2 Nov 9 12:01:01 src@eadmin2 crond[12637]: (root) CMD (run-parts /etc/cron.hourly)
- 1131566461 2005.11.09 en257 Nov 9 12:01:01 en257/en257 crond(pam_unix)[8950]: session closed for user root
- 1131566461 2005.11.09 en257 Nov 9 12:01:01 en257/en257 crond(pam_unix)[8950]: session opened for user root by (uid=0)
- 1131566461 2005.11.09 en257 Nov 9 12:01:01 en257/en257 crond[8951]: (root) CMD (run-parts /etc/cron.hourly)

```

Output excerpt:

```text
[... 1299 line(s) omitted ... ⟦tj:99c45489993dd6335892a3a86c4e5531⟧]
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 checking TSC synchronization across 4 CPUs: passed.
[... 8 line(s) omitted ... ⟦tj:c2a494633b17fc6a35f00ef95f8493a5⟧]
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 floppy0: no floppy controllers found
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 hw_random: RNG not detected
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 ide0: Wait for ready failed before probe !  [×6]
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 ide1: Wait for ready failed before probe !
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 ide2: Wait for ready failed before probe !
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 ide3: Wait for ready failed before probe !
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 ide4: Wait for ready failed before probe !
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 ide5: Wait for ready failed before probe !
[... 25 line(s) omitted ... ⟦tj:c94f52551492c47989be8ca8db23ca52⟧]
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 scsi[0]: scanning scsi channel 0 [Phy 0] for non-raid devices
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 scsi[0]: scanning scsi channel 1 [virtual] for logical drives
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 sda: asking for cache data failed
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 sda: assuming drive cache: write through
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 sda: sda1 sda2 sda3 sda4
[... 11 line(s) omitted ... ⟦tj:56d11eeab7f0aac8e59f1b4be8faafd0⟧]
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 usbcore: registered new driver usbfs
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 usbcore: registered new driver usbhid
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 vesafb: probe of vesafb0 failed with error -6
- 1131567044 2005.11.09 bn124 Nov 9 12:10:44 bn124/bn124 ntpd[22190]: synchronized to 10.100.22.250, stratum 3
- 1131567044 2005.11.09 tbird-admin1 Nov 9 12:10:44 local@tbird-admin1 /apps/x86_64/system/ganglia-3.0.1/sbin/gmetad[1682]: data_thread() got not answer from any [Thunderbird_C1] datasource
[... 19 line(s) omitted ... ⟦tj:e8adb754352b8c0ab963efacdbd12006⟧]
- 1131567052 2005.11.09 #8# Nov 9 12:10:52 #8#/#8# sshd[4718]: connection lost: 'Connection closed.'
- 1131567052 2005.11.09 tbird-admin1 Nov 9 12:10:52 local@tbird-admin1 netfs: Mounting NFS filesystems: succeeded
- 1131567052 2005.11.09 tbird-admin1 Nov 9 12:10:52 local@tbird-admin1 netfs: Mounting other filesystems: failed
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 /apps/x86_64/system/ganglia-3.0.1/sbin/gmetad[1682]: data_thread() got not answer from any [Thunderbird_A2] datasource
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 /apps/x86_64/system/ganglia-3.0.1/sbin/gmetad[1682]: data_thread() got not answer from any [Thunderbird_B5] datasource
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name an14 an15 an16 an17 an18 an19 an20 an21 an22 an23 an24 an25 an26 an27 an28 an29 an30 an31 an32...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name an14 an15 an16 an17 an18 an19 an20 an21 an22 an23 an24 an25 an26 an27 an28 an29 an30 an31 an32...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name an142 an143 an144 an145 an146 an147 an148 an149 an150 an151 an152 an153 an154 an155 an156 an15...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name an270 an271 an272 an273 an274 an275 an276 an277 an278 an279 an280 an281 an282 an283 an284 an28...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name an398 an399 an400 an401 an402 an403 an404 an405 an406 an407 an408 an409 an410 an411 an412 an41...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name an526 an527 an528 an529 an530 an531 an532 an533 an534 an535 an536 an537 an538 an539 an540 an54...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name an654 an655 an656 an657 an658 an659 an660 an661 an662 an663 an664 an665 an666 an667 an668 an66...

```

### `20-caddy-coraza-waf`

- [Full input](cases/20-caddy-coraza-waf/input.log)
- [Output with CCR](cases/20-caddy-coraza-waf/output.log) - [diff](cases/20-caddy-coraza-waf/compression.diff)
- [Output without CCR](cases/20-caddy-coraza-waf/output-noccr.log) - [diff](cases/20-caddy-coraza-waf/compression-noccr.diff)

Input excerpt:

```text
{"level":"warn","ts":1700758468.384019,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Restricted SQL Character Anomaly Detection (args): # of special characters exceeded (6) [file \"/rulese...
{"level":"warn","ts":1700758728.086958,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Restricted SQL Character Anomaly Detection (args): # of special characters exceeded (6) [file \"/rulese...
{"level":"warn","ts":1700759035.6624935,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Restricted SQL Character Anomaly Detection (args): # of special characters exceeded (6) [file \"/rules...
{"level":"info","ts":1700764841.4611049,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700764841.4622538,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700764841.4642208,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700764841.4663923,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700764841.4684355,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700764841.513978,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] [...
{"level":"info","ts":1700764841.528397,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] [...
{"level":"info","ts":1700764841.5307105,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700764841.5351427,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700764841.67431,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] [l...
{"level":"info","ts":1700764841.6831691,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700810574.5027742,"logger":"http.handlers.waf","msg":"[client \"192.168.1.4\"] Coraza: Warning. Request Missing an Accept Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.c...
{"level":"warn","ts":1700824916.1624498,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Restricted SQL Character Anomaly Detection (args): # of special characters exceeded (6) [file \"/rules...
{"level":"warn","ts":1700825111.5810397,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Restricted SQL Character Anomaly Detection (args): # of special characters exceeded (6) [file \"/rules...
{"level":"warn","ts":1700825382.6159449,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Restricted SQL Character Anomaly Detection (args): # of special characters exceeded (6) [file \"/rules...
{"level":"warn","ts":1700829528.0721524,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Restricted SQL Character Anomaly Detection (args): # of special characters exceeded (6) [file \"/rules...
{"level":"warn","ts":1700834231.5077024,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Meta-Character Anomaly Detection Alert - Repetitive Non-Word Characters [file \"/ruleset/coreruleset/r...
{"level":"info","ts":1700840763.8743727,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700840763.875109,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] [...
{"level":"info","ts":1700840763.8851051,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700840763.8871,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] [li...
{"level":"info","ts":1700840763.912266,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] [...
{"level":"info","ts":1700840763.9185588,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700840763.9311366,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700840763.9362268,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700840766.9606156,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700840766.9634898,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700840766.9641814,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700840766.9608645,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700840766.9609265,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700840766.960615,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] [...
{"level":"info","ts":1700840766.9647932,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700842573.101901,"logger":"http.handlers.waf","msg":"[client \"192.168.1.5\"] Coraza: Warning. Request Missing an Accept Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.co...

```

Output excerpt:

```text
{"level":"warn","ts":1700758468.384019,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Restricted SQL Character Anomaly Detection (args): # of special characters exceeded (6) [file \"/rulese...
{"level":"warn","ts":1700758728.086958,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Restricted SQL Character Anomaly Detection (args): # of special characters exceeded (6) [file \"/rulese...
{"level":"warn","ts":1700759035.6624935,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Restricted SQL Character Anomaly Detection (args): # of special characters exceeded (6) [file \"/rules...
{"level":"info","ts":1700764841.4611049,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700764841.4622538,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700764841.4642208,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700764841.4663923,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700764841.4684355,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] ...
{"level":"info","ts":1700764841.513978,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] [...
{"level":"info","ts":1700764841.528397,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] [...
[Template: <*> \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] [line \"3242\"] [id \"920320\"] [rev \"\"] [msg \"Missing User Agent ...
[Template: <*> \"192.168.1.1\"] Coraza: Warning. <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> <*> [severity \"warning\"] ...
[Template: <*> \"192.168.1.1\"] Coraza: Warning. Missing User Agent Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.conf\"] [line \"0\"] [id \"920320\"] [rev \"\"] [msg \"Missing User Agent Hea...
[... 25 line(s) omitted ... ⟦tj:90aff7d1fdc680fc4d6b3671c37762cd⟧]
{"level":"info","ts":1700842573.101901,"logger":"http.handlers.waf","msg":"[client \"192.168.1.5\"] Coraza: Warning. Request Missing an Accept Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.co...
{"level":"info","ts":1700842573.4644516,"logger":"http.handlers.waf","msg":"[client \"192.168.1.5\"] Coraza: Warning. Request Missing an Accept Header [file \"/ruleset/coreruleset/rules/REQUEST-920-PROTOCOL-ENFORCEMENT.c...
{"level":"error","ts":1700845155.7930043,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. HTTP Parameter Pollution (utf-8) [file \"/ruleset/coreruleset/rulesV4RC2/REQUEST-921-PROTOCOL-ATTACK....
{"level":"error","ts":1700845155.8713691,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Inbound Anomaly Score Exceeded (Total Score: 5) [file \"/ruleset/coreruleset/rulesV4RC2/REQUEST-949-B...
{"level":"error","ts":1700845155.8719003,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning.  [file \"/ruleset/coreruleset/rulesV4RC2/RESPONSE-980-CORRELATION.conf\"] [line \"0\"] [id \"980170\"...
{"level":"warn","ts":1700845303.6146033,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Restricted SQL Character Anomaly Detection (args): # of special characters exceeded (6) [file \"/rules...
{"level":"error","ts":1700845872.3002284,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. HTTP Parameter Pollution (utf-8) [file \"/ruleset/coreruleset/rulesV4RC2/REQUEST-921-PROTOCOL-ATTACK....
{"level":"error","ts":1700845872.3069482,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Remote Command Execution: Unix Shell Code Found in REQUEST_HEADERS [file \"/ruleset/coreruleset/rules...
{"level":"error","ts":1700845872.3356824,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Inbound Anomaly Score Exceeded (Total Score: 10) [file \"/ruleset/coreruleset/rulesV4RC2/REQUEST-949-...
{"level":"error","ts":1700845872.3361418,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning.  [file \"/ruleset/coreruleset/rulesV4RC2/RESPONSE-980-CORRELATION.conf\"] [line \"0\"] [id \"980170\"...
{"level":"error","ts":1700845885.0831904,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. HTTP Parameter Pollution (utf-8) [file \"/ruleset/coreruleset/rulesV4RC2/REQUEST-921-PROTOCOL-ATTACK....
{"level":"error","ts":1700845885.089673,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Remote Command Execution: Unix Shell Code Found in REQUEST_HEADERS [file \"/ruleset/coreruleset/rulesV...
{"level":"error","ts":1700845885.1165082,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Inbound Anomaly Score Exceeded (Total Score: 10) [file \"/ruleset/coreruleset/rulesV4RC2/REQUEST-949-...
{"level":"error","ts":1700845885.1170466,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning.  [file \"/ruleset/coreruleset/rulesV4RC2/RESPONSE-980-CORRELATION.conf\"] [line \"0\"] [id \"980170\"...
{"level":"error","ts":1700849331.8737118,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. HTTP Parameter Pollution (utf-8) [file \"/ruleset/coreruleset/rulesV4RC2/REQUEST-921-PROTOCOL-ATTACK....
{"level":"error","ts":1700849331.9081802,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Inbound Anomaly Score Exceeded (Total Score: 5) [file \"/ruleset/coreruleset/rulesV4RC2/REQUEST-949-B...
{"level":"error","ts":1700849331.908678,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning.  [file \"/ruleset/coreruleset/rulesV4RC2/RESPONSE-980-CORRELATION.conf\"] [line \"0\"] [id \"980170\"]...
{"level":"warn","ts":1700849441.1646814,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Restricted SQL Character Anomaly Detection (args): # of special characters exceeded (6) [file \"/rules...
{"level":"error","ts":1700849441.2226603,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. HTTP Parameter Pollution (utf-8) [file \"/ruleset/coreruleset/rulesV4RC2/REQUEST-921-PROTOCOL-ATTACK....
{"level":"error","ts":1700849441.2458975,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Inbound Anomaly Score Exceeded (Total Score: 5) [file \"/ruleset/coreruleset/rulesV4RC2/REQUEST-949-B...
{"level":"error","ts":1700849441.2465184,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning.  [file \"/ruleset/coreruleset/rulesV4RC2/RESPONSE-980-CORRELATION.conf\"] [line \"0\"] [id \"980170\"...
{"level":"error","ts":1700849594.2515044,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. HTTP Parameter Pollution (utf-8) [file \"/ruleset/coreruleset/rulesV4RC2/REQUEST-921-PROTOCOL-ATTACK....

```

### `33-postfix-mail`

- [Full input](cases/33-postfix-mail/input.log)
- [Output with CCR](cases/33-postfix-mail/output.log) - [diff](cases/33-postfix-mail/compression.diff)
- [Output without CCR](cases/33-postfix-mail/output-noccr.log) - [diff](cases/33-postfix-mail/compression-noccr.diff)

Input excerpt:

```text
# filterOptions: [{}, {"mode": "normal"}, {"mode": "aggressive"}]

# per https://github.com/fail2ban/fail2ban/issues/125
# and https://github.com/fail2ban/fail2ban/issues/126
# failJSON: { "time": "2005-02-21T09:21:54", "match": true , "host": "192.0.43.10" }
Feb 21 09:21:54 xxx postfix/smtpd[14398]: NOQUEUE: reject: RCPT from example.com[192.0.43.10]: 450 4.7.1 : Helo command rejected: Host not found; from=<> to=<> proto=ESMTP helo=
# failJSON: { "time": "2005-07-12T07:47:48", "match": true , "host": "1.2.3.4" }
Jul 12 07:47:48 saturn postfix/smtpd[8738]: NOQUEUE: reject: RCPT from 1-2-3-4-example.com[1.2.3.4]: 554 5.7.1 <smtp@example.com>: Relay access denied; from=<john@example.com> to=<smtp@example.org> proto=SMTP helo=<198.5...
# failJSON: { "time": "2005-07-18T23:12:56", "match": true , "host": "192.51.100.65" }
Jul 18 23:12:56 xxx postfix/smtpd[8738]: NOQUEUE: reject: RCPT from foo[192.51.100.65]: 554 5.7.1 <bad.domain>: Helo command rejected: match bad.domain; from=<foo@good.domain> to=<foo@porcupine.org> proto=SMTP helo=<bad....
# failJSON: { "time": "2005-07-18T23:12:56", "match": true , "host": "192.51.100.43" }
Jul 18 23:12:56 xxx postfix/smtpd[8738]: NOQUEUE: reject: RCPT from foo[192.51.100.43]: 554 5.7.1 <foo@bad.domain>: Sender address rejected: match bad.domain; from=<foo@bad.domain> to=<foo@porcupine.org> proto=SMTP helo=...
# failJSON: { "time": "2005-08-10T10:55:38", "match": true , "host": "72.53.132.234" }
Aug 10 10:55:38 f-vanier-bourgeois postfix/smtpd[2162]: NOQUEUE: reject: VRFY from 72-53-132-234.cpe.distributel.net[72.53.132.234]: 550 5.1.1 : Recipient address rejected: User unknown in local recipient tab
# failJSON: { "time": "2005-08-13T15:45:46", "match": true , "host": "192.0.2.1" }
Aug 13 15:45:46 server postfix/smtpd[13844]: 00ADB3C0899: reject: RCPT from example.com[192.0.2.1]: 550 5.1.1 <sales@server.com>: Recipient address rejected: User unknown in local recipient table; from=<xxxxxx@example.co...

# failJSON: { "time": "2005-05-19T00:00:30", "match": true , "host": "192.0.2.2", "desc": "undeliverable address (sender/recipient verification, gh-3039)" }
May 19 00:00:30 proxy2 postfix/smtpd[16123]: NOQUEUE: reject: RCPT from example.net[192.0.2.2]: 550 5.1.1 <user1@example.com>: Recipient address rejected: undeliverable address: verification failed; from=<user2@example.o...

# failJSON: { "time": "2005-01-12T11:07:49", "match": true , "host": "181.21.131.88" }
Jan 12 11:07:49 emf1pt2-2-35-70 postfix/smtpd[13767]: improper command pipelining after DATA from unknown[181.21.131.88]:

# failJSON: { "time": "2004-12-25T02:35:54", "match": true , "host": "173.10.140.217" }
Dec 25 02:35:54 platypus postfix/smtpd[9144]: improper command pipelining after RSET from 173-10-140-217-BusName-washingtonDC.hfc.comcastbusiness.net[173.10.140.217]

# failJSON: { "time": "2004-12-18T02:05:46", "match": true , "host": "216.245.198.245" }
Dec 18 02:05:46 platypus postfix/smtpd[16349]: improper command pipelining after NOOP from unknown[216.245.198.245]

# failJSON: { "time": "2004-12-21T21:17:29", "match": true , "host": "93.184.216.34" }
Dec 21 21:17:29 xxx postfix/smtpd[7150]: NOQUEUE: reject: RCPT from badserver.example.com[93.184.216.34]: 450 4.7.1 Client host rejected: cannot find your hostname, [93.184.216.34]; from=<badactor@example.com> to=<goodgu...
# failJSON: { "time": "2004-12-21T21:17:30", "match": true , "host": "93.184.216.34", "desc": "variable status code suffix, gh-2442" }
Dec 21 21:17:30 xxx postfix/smtpd[7150]: NOQUEUE: reject: RCPT from badserver.example.com[93.184.216.34]: 450 4.7.25 Client host rejected: cannot find your hostname, [93.184.216.34]; from=<badactor@example.com> to=<goodg...

# failJSON: { "time": "2004-11-22T22:33:44", "match": true , "host": "1.2.3.4" }
Nov 22 22:33:44 xxx postfix/smtpd[11111]: NOQUEUE: reject: RCPT from 1-2-3-4.example.com[1.2.3.4]: 450 4.1.8 <some@nonexistant.tld>: Sender address rejected: Domain not found; from=<some@nonexistant.tld> to=<goodguy@exam...

```

Output excerpt:

```text
[... 16 line(s) omitted ... ⟦tj:6c8729d8036a980fc8652101388d9a61⟧]

# failJSON: { "time": "2005-05-19T00:00:30", "match": true , "host": "192.0.2.2", "desc": "undeliverable address (sender/recipient verification, gh-3039)" }
May 19 00:00:30 proxy2 postfix/smtpd[16123]: NOQUEUE: reject: RCPT from example.net[192.0.2.2]: 550 5.1.1 <user1@example.com>: Recipient address rejected: undeliverable address: verification failed; from=<user2@example.o...

# failJSON: { "time": "2005-01-12T11:07:49", "match": true , "host": "181.21.131.88" }
[... 79 line(s) omitted ... ⟦tj:8031ffb79f3cee77288424def8acf947⟧]
Dec  2 22:24:22 hel postfix/smtpd[7676]: warning: 114-44-142-233.dynamic.hinet.net[114.44.142.233]: SASL CRAM-MD5 authentication failed: PDc3OTEwNTkyNTEyMzA2NDIuMTIyODI1MzA2MUBoZWw+
#2 Example from postfix from dbts #573314
# failJSON: { "time": "2005-03-10T13:33:30", "match": true , "host": "1.1.1.1" }
Mar 10 13:33:30 gandalf postfix/smtpd[3937]: warning: HOSTNAME[1.1.1.1]: SASL LOGIN authentication failed: authentication failure
[... 3 line(s) omitted ... ⟦tj:0de60b317642f8542e83ff986f38ac50⟧]
Sep  6 00:44:56 trianon postfix/submission/smtpd[11538]: warning: unknown[82.221.106.233]: SASL LOGIN authentication failed: UGFzc3dvcmQ6  [×2 first Sep 6 00:44:56, last Sep 6 00:44:57]
[... 3 line(s) omitted ... ⟦tj:1796d0ce2d91489b489701da4d9551ee⟧]
Sep  6 00:44:57 trianon postfix/submission/smtpd[11538]: warning: unknown[82.221.106.233]: SASL login authentication failed: UGFzc3dvcmQ6
[... 3 line(s) omitted ... ⟦tj:2f988e3da34dfed27d8f093b4429620f⟧]
Jan 29 08:11:45 mail postfix/smtpd[10752]: warning: unknown[1.1.1.1]: SASL LOGIN authentication failed: Password:

# failJSON: { "time": "2005-01-29T08:11:45", "match": true , "host": "1.1.1.1" }
Jan 29 08:11:45 mail postfix-incoming/smtpd[10752]: warning: unknown[1.1.1.1]: SASL LOGIN authentication failed: Password:

# failJSON: { "time": "2005-04-12T02:24:11", "match": true , "host": "62.138.2.143" }
Apr 12 02:24:11 xxx postfix/smtps/smtpd[42]: warning: astra4139.startdedicated.de[62.138.2.143]: SASL LOGIN authentication failed: UGFzc3dvcmQ6

# failJSON: { "time": "2005-08-03T15:30:49", "match": true , "host": "98.191.84.74" }
Aug 3 15:30:49 ksusha postfix/smtpd[17041]: warning: mail.foldsandwalker.com[98.191.84.74]: SASL Plain authentication failed:

# failJSON: { "time": "2005-08-04T16:47:52", "match": true , "host": "192.0.2.237", "desc": "cover optional port after host" }
Aug 4 16:47:52 mail3 postfix/smtpd[31152]: warning: unknown[192.0.2.237]:55729: SASL LOGIN authentication failed: authentication failure

# failJSON: { "time": "2004-11-04T09:11:01", "match": true , "host": "192.0.2.150", "desc": "without reason for fail, see gh-1245" }
Nov  4 09:11:01 mail postfix/submission/smtpd[27133]: warning: unknown[192.0.2.150]: SASL PLAIN authentication failed:

#6 Example to ignore because due to a failed attempt to connect to authentication service - no malicious activities whatsoever
# failJSON: { "match": false }
Feb  3 08:29:28 mail postfix/smtpd[21022]: warning: unknown[1.1.1.1]: SASL LOGIN authentication failed: Connection lost to authentication server

```

### `23-authelia-bf`

- [Full input](cases/23-authelia-bf/input.log)
- [Output with CCR](cases/23-authelia-bf/output.log) - [diff](cases/23-authelia-bf/compression.diff)
- [Output without CCR](cases/23-authelia-bf/output-noccr.log) - [diff](cases/23-authelia-bf/compression-noccr.diff)

Input excerpt:

```text
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser1': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.1 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser2': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.1 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser3': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.1 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser4': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.1 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser5': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.1 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser6': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.1 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser1@example.com': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.2 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser2@example.com': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.2 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser3@example.com': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.2 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser4@example.com': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.2 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser5@example.com': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.2 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser6@example.com': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.2 stack="longstacktrace"
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser' which usually indicates they do not exist" error="user not found" method=POST path=/api/firstfactor...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser' which usually indicates they do not exist" error="user not found" method=POST path=/api/firstfactor...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser' which usually indicates they do not exist" error="user not found" method=POST path=/api/firstfactor...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser' which usually indicates they do not exist" error="user not found" method=POST path=/api/firstfactor...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser' which usually indicates they do not exist" error="user not found" method=POST path=/api/firstfactor...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser' which usually indicates they do not exist" error="user not found" method=POST path=/api/firstfactor...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser@example.com' which usually indicates they do not exist" error="user not found" method=POST path=/api...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser@example.com' which usually indicates they do not exist" error="user not found" method=POST path=/api...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser@example.com' which usually indicates they do not exist" error="user not found" method=POST path=/api...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser@example.com' which usually indicates they do not exist" error="user not found" method=POST path=/api...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser@example.com' which usually indicates they do not exist" error="user not found" method=POST path=/api...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser@example.com' which usually indicates they do not exist" error="user not found" method=POST path=/api...
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser'" method=POST path=/api/firstfactor remote_ip=2.2.2.2 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser'" method=POST path=/api/firstfactor remote_ip=2.2.2.2 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser'" method=POST path=/api/firstfactor remote_ip=2.2.2.2 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser'" method=POST path=/api/firstfactor remote_ip=2.2.2.2 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser'" method=POST path=/api/firstfactor remote_ip=2.2.2.2 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser'" method=POST path=/api/firstfactor remote_ip=2.2.2.2 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser@example.com'" method=POST path=/api/firstfactor remote_ip=2.2.2.3 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser@example.com'" method=POST path=/api/firstfactor remote_ip=2.2.2.3 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser@example.com'" method=POST path=/api/firstfactor remote_ip=2.2.2.3 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser@example.com'" method=POST path=/api/firstfactor remote_ip=2.2.2.3 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser@example.com'" method=POST path=/api/firstfactor remote_ip=2.2.2.3 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser@example.com'" method=POST path=/api/firstfactor remote_ip=2.2.2.3 stack="longstacktrace"

```

Output excerpt:

```text
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser1': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.1 stack="longstacktrace"  [×6]
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser2': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.1 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser3': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.1 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser4': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.1 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser5': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.1 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser6': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.1 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser1@example.com': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.2 stack="longstacktrace"  [×6]
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser2@example.com': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.2 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser3@example.com': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.2 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser4@example.com': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.2 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser5@example.com': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.2 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser6@example.com': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.2 stack="longstacktrace"
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser' which usually indicates they do not exist" error="user not found" method=POST path=/api/firstfactor...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser' which usually indicates they do not exist" error="user not found" method=POST path=/api/firstfactor...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser' which usually indicates they do not exist" error="user not found" method=POST path=/api/firstfactor...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser' which usually indicates they do not exist" error="user not found" method=POST path=/api/firstfactor...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser' which usually indicates they do not exist" error="user not found" method=POST path=/api/firstfactor...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser' which usually indicates they do not exist" error="user not found" method=POST path=/api/firstfactor...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser@example.com' which usually indicates they do not exist" error="user not found" method=POST path=/api...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser@example.com' which usually indicates they do not exist" error="user not found" method=POST path=/api...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser@example.com' which usually indicates they do not exist" error="user not found" method=POST path=/api...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser@example.com' which usually indicates they do not exist" error="user not found" method=POST path=/api...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser@example.com' which usually indicates they do not exist" error="user not found" method=POST path=/api...
time="2025-03-13T14:01:02+02:00" level=error msg="Error occurred getting details for user with username input 'fakeuser@example.com' which usually indicates they do not exist" error="user not found" method=POST path=/api...
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser'" method=POST path=/api/firstfactor remote_ip=2.2.2.2 stack="longstacktrace"  [×6]
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser'" method=POST path=/api/firstfactor remote_ip=2.2.2.2 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser'" method=POST path=/api/firstfactor remote_ip=2.2.2.2 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser'" method=POST path=/api/firstfactor remote_ip=2.2.2.2 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser'" method=POST path=/api/firstfactor remote_ip=2.2.2.2 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser'" method=POST path=/api/firstfactor remote_ip=2.2.2.2 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser@example.com'" method=POST path=/api/firstfactor remote_ip=2.2.2.3 stack="longstacktrace"  [×6]
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser@example.com'" method=POST path=/api/firstfactor remote_ip=2.2.2.3 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser@example.com'" method=POST path=/api/firstfactor remote_ip=2.2.2.3 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser@example.com'" method=POST path=/api/firstfactor remote_ip=2.2.2.3 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser@example.com'" method=POST path=/api/firstfactor remote_ip=2.2.2.3 stack="longstacktrace"
time="2022-02-14T13:49:12+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'realuser@example.com'" method=POST path=/api/firstfactor remote_ip=2.2.2.3 stack="longstacktrace"

```

### `29-spark-eventlog`

- [Full input](cases/29-spark-eventlog/input.log)
- [Output with CCR](cases/29-spark-eventlog/output.log) - [diff](cases/29-spark-eventlog/compression.diff)
- [Output without CCR](cases/29-spark-eventlog/output-noccr.log) - [diff](cases/29-spark-eventlog/compression-noccr.diff)

Input excerpt:

```text
{"Event":"SparkListenerLogStart","Spark Version":"3.3.0-SNAPSHOT"}
{"Event":"SparkListenerResourceProfileAdded","Resource Profile Id":0,"Executor Resource Requests":{"cores":{"Resource Name":"cores","Amount":1,"Discovery Script":"","Vendor":""},"memory":{"Resource Name":"memory","Amount...
{"Event":"SparkListenerExecutorAdded","Timestamp":1642039451891,"Executor ID":"driver","Executor Info":{"Host":"172.22.200.52","Total Cores":8,"Log Urls":{},"Attributes":{},"Resources":{},"Resource Profile Id":0}}
{"Event":"SparkListenerBlockManagerAdded","Block Manager ID":{"Executor ID":"driver","Host":"172.22.200.52","Port":61039},"Maximum Memory":384093388,"Timestamp":1642039451909,"Maximum Onheap Memory":384093388,"Maximum Of...
{"Event":"SparkListenerEnvironmentUpdate","JVM Information":{"Java Home":"/Library/Java/JavaVirtualMachines/zulu-8.jdk/Contents/Home/jre","Java Version":"1.8.0_312 (Azul Systems, Inc.)","Scala Version":"version 2.12.15"}...
{"Event":"SparkListenerApplicationStart","App Name":"Spark shell","App ID":"local-1642039451826","Timestamp":1642039450519,"User":"lijunqing"}
{"Event":"org.apache.spark.sql.execution.ui.SparkListenerSQLExecutionStart","executionId":0,"description":"count at <console>:23","details":"org.apache.spark.sql.Dataset.count(Dataset.scala:3130)\n$line15.$read$$iw$$iw$$...
{"Event":"org.apache.spark.sql.execution.ui.SparkListenerSQLAdaptiveExecutionUpdate","executionId":0,"physicalPlanDescription":"== Physical Plan ==\nAdaptiveSparkPlan (12)\n+- == Current Plan ==\n   HashAggregate (7)\n  ...
{"Event":"org.apache.spark.sql.execution.ui.SparkListenerDriverAccumUpdates","executionId":0,"accumUpdates":[[62,10]]}
{"Event":"SparkListenerJobStart","Job ID":0,"Submission Time":1642039496191,"Stage Infos":[{"Stage ID":0,"Stage Attempt ID":0,"Stage Name":"count at <console>:23","Number of Tasks":8,"RDD Info":[{"RDD ID":4,"Name":"MapPa...
{"Event":"SparkListenerStageSubmitted","Stage Info":{"Stage ID":0,"Stage Attempt ID":0,"Stage Name":"count at <console>:23","Number of Tasks":8,"RDD Info":[{"RDD ID":4,"Name":"MapPartitionsRDD","Scope":"{\"id\":\"0\",\"n...
{"Event":"SparkListenerTaskStart","Stage ID":0,"Stage Attempt ID":0,"Task Info":{"Task ID":0,"Index":0,"Attempt":0,"Partition ID":0,"Launch Time":1642039496413,"Executor ID":"driver","Host":"172.22.200.52","Locality":"PR...
{"Event":"SparkListenerTaskStart","Stage ID":0,"Stage Attempt ID":0,"Task Info":{"Task ID":1,"Index":1,"Attempt":0,"Partition ID":1,"Launch Time":1642039496425,"Executor ID":"driver","Host":"172.22.200.52","Locality":"PR...
{"Event":"SparkListenerTaskStart","Stage ID":0,"Stage Attempt ID":0,"Task Info":{"Task ID":2,"Index":2,"Attempt":0,"Partition ID":2,"Launch Time":1642039496425,"Executor ID":"driver","Host":"172.22.200.52","Locality":"PR...
{"Event":"SparkListenerTaskStart","Stage ID":0,"Stage Attempt ID":0,"Task Info":{"Task ID":3,"Index":3,"Attempt":0,"Partition ID":3,"Launch Time":1642039496425,"Executor ID":"driver","Host":"172.22.200.52","Locality":"PR...
{"Event":"SparkListenerTaskStart","Stage ID":0,"Stage Attempt ID":0,"Task Info":{"Task ID":4,"Index":4,"Attempt":0,"Partition ID":4,"Launch Time":1642039496426,"Executor ID":"driver","Host":"172.22.200.52","Locality":"PR...
{"Event":"SparkListenerTaskStart","Stage ID":0,"Stage Attempt ID":0,"Task Info":{"Task ID":5,"Index":5,"Attempt":0,"Partition ID":5,"Launch Time":1642039496426,"Executor ID":"driver","Host":"172.22.200.52","Locality":"PR...
{"Event":"SparkListenerTaskStart","Stage ID":0,"Stage Attempt ID":0,"Task Info":{"Task ID":6,"Index":6,"Attempt":0,"Partition ID":6,"Launch Time":1642039496427,"Executor ID":"driver","Host":"172.22.200.52","Locality":"PR...
{"Event":"SparkListenerTaskStart","Stage ID":0,"Stage Attempt ID":0,"Task Info":{"Task ID":7,"Index":7,"Attempt":0,"Partition ID":7,"Launch Time":1642039496427,"Executor ID":"driver","Host":"172.22.200.52","Locality":"PR...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":6,"Index":6,"Attempt":0,"Partition ID":6,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":5,"Index":5,"Attempt":0,"Partition ID":5,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":4,"Index":4,"Attempt":0,"Partition ID":4,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":2,"Index":2,"Attempt":0,"Partition ID":2,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":0,"Index":0,"Attempt":0,"Partition ID":0,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":1,"Index":1,"Attempt":0,"Partition ID":1,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":7,"Index":7,"Attempt":0,"Partition ID":7,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":3,"Index":3,"Attempt":0,"Partition ID":3,"Launch Time":16420394...
{"Event":"SparkListenerStageCompleted","Stage Info":{"Stage ID":0,"Stage Attempt ID":0,"Stage Name":"count at <console>:23","Number of Tasks":8,"RDD Info":[{"RDD ID":4,"Name":"MapPartitionsRDD","Scope":"{\"id\":\"0\",\"n...
{"Event":"SparkListenerJobEnd","Job ID":0,"Completion Time":1642039496914,"Job Result":{"Result":"JobSucceeded"}}
{"Event":"org.apache.spark.sql.execution.ui.SparkListenerSQLAdaptiveExecutionUpdate","executionId":0,"physicalPlanDescription":"== Physical Plan ==\nAdaptiveSparkPlan (13)\n+- == Current Plan ==\n   HashAggregate (8)\n  ...
{"Event":"org.apache.spark.sql.execution.ui.SparkListenerDriverAccumUpdates","executionId":0,"accumUpdates":[[106,1]]}
{"Event":"SparkListenerJobStart","Job ID":1,"Submission Time":1642039497010,"Stage Infos":[{"Stage ID":1,"Stage Attempt ID":0,"Stage Name":"count at <console>:23","Number of Tasks":8,"RDD Info":[{"RDD ID":4,"Name":"MapPa...
{"Event":"SparkListenerStageSubmitted","Stage Info":{"Stage ID":2,"Stage Attempt ID":0,"Stage Name":"count at <console>:23","Number of Tasks":10,"RDD Info":[{"RDD ID":7,"Name":"MapPartitionsRDD","Scope":"{\"id\":\"4\",\"...
{"Event":"SparkListenerTaskStart","Stage ID":2,"Stage Attempt ID":0,"Task Info":{"Task ID":8,"Index":0,"Attempt":0,"Partition ID":0,"Launch Time":1642039497053,"Executor ID":"driver","Host":"172.22.200.52","Locality":"NO...
{"Event":"SparkListenerTaskStart","Stage ID":2,"Stage Attempt ID":0,"Task Info":{"Task ID":9,"Index":1,"Attempt":0,"Partition ID":1,"Launch Time":1642039497055,"Executor ID":"driver","Host":"172.22.200.52","Locality":"NO...
{"Event":"SparkListenerTaskStart","Stage ID":2,"Stage Attempt ID":0,"Task Info":{"Task ID":10,"Index":2,"Attempt":0,"Partition ID":2,"Launch Time":1642039497055,"Executor ID":"driver","Host":"172.22.200.52","Locality":"N...

```

Output excerpt:

```text
{"Event":"SparkListenerLogStart","Spark Version":"3.3.0-SNAPSHOT"}
{"Event":"SparkListenerResourceProfileAdded","Resource Profile Id":0,"Executor Resource Requests":{"cores":{"Resource Name":"cores","Amount":1,"Discovery Script":"","Vendor":""},"memory":{"Resource Name":"memory","Amount...
{"Event":"SparkListenerExecutorAdded","Timestamp":1642039451891,"Executor ID":"driver","Executor Info":{"Host":"172.22.200.52","Total Cores":8,"Log Urls":{},"Attributes":{},"Resources":{},"Resource Profile Id":0}}
{"Event":"SparkListenerBlockManagerAdded","Block Manager ID":{"Executor ID":"driver","Host":"172.22.200.52","Port":61039},"Maximum Memory":384093388,"Timestamp":1642039451909,"Maximum Onheap Memory":384093388,"Maximum Of...
{"Event":"SparkListenerEnvironmentUpdate","JVM Information":{"Java Home":"/Library/Java/JavaVirtualMachines/zulu-8.jdk/Contents/Home/jre","Java Version":"1.8.0_312 (Azul Systems, Inc.)","Scala Version":"version 2.12.15"}...
{"Event":"SparkListenerApplicationStart","App Name":"Spark shell","App ID":"local-1642039451826","Timestamp":1642039450519,"User":"lijunqing"}
{"Event":"org.apache.spark.sql.execution.ui.SparkListenerSQLExecutionStart","executionId":0,"description":"count at <console>:23","details":"org.apache.spark.sql.Dataset.count(Dataset.scala:3130)\n$line15.$read$$iw$$iw$$...
{"Event":"org.apache.spark.sql.execution.ui.SparkListenerSQLAdaptiveExecutionUpdate","executionId":0,"physicalPlanDescription":"== Physical Plan ==\nAdaptiveSparkPlan (12)\n+- == Current Plan ==\n   HashAggregate (7)\n  ...
{"Event":"org.apache.spark.sql.execution.ui.SparkListenerDriverAccumUpdates","executionId":0,"accumUpdates":[[62,10]]}
{"Event":"SparkListenerJobStart","Job ID":0,"Submission Time":1642039496191,"Stage Infos":[{"Stage ID":0,"Stage Attempt ID":0,"Stage Name":"count at <console>:23","Number of Tasks":8,"RDD Info":[{"RDD ID":4,"Name":"MapPa...
{"Event":"SparkListenerStageSubmitted","Stage Info":{"Stage ID":0,"Stage Attempt ID":0,"Stage Name":"count at <console>:23","Number of Tasks":8,"RDD Info":[{"RDD ID":4,"Name":"MapPartitionsRDD","Scope":"{\"id\":\"0\",\"n...
{"Event":"SparkListenerTaskStart","Stage ID":0,"Stage Attempt ID":0,"Task Info":{"Task ID":0,"Index":0,"Attempt":0,"Partition ID":0,"Launch Time":1642039496413,"Executor ID":"driver","Host":"172.22.200.52","Locality":"PR...
{"Event":"SparkListenerTaskStart","Stage ID":0,"Stage Attempt ID":0,"Task Info":{"Task ID":1,"Index":1,"Attempt":0,"Partition ID":1,"Launch Time":1642039496425,"Executor ID":"driver","Host":"172.22.200.52","Locality":"PR...
{"Event":"SparkListenerTaskStart","Stage ID":0,"Stage Attempt ID":0,"Task Info":{"Task ID":2,"Index":2,"Attempt":0,"Partition ID":2,"Launch Time":1642039496425,"Executor ID":"driver","Host":"172.22.200.52","Locality":"PR...
[... 3 line(s) omitted ... ⟦tj:1b02c45d0f1f109c6bf722b275a868f9⟧]
{"Event":"SparkListenerTaskStart","Stage ID":0,"Stage Attempt ID":0,"Task Info":{"Task ID":6,"Index":6,"Attempt":0,"Partition ID":6,"Launch Time":1642039496427,"Executor ID":"driver","Host":"172.22.200.52","Locality":"PR...
{"Event":"SparkListenerTaskStart","Stage ID":0,"Stage Attempt ID":0,"Task Info":{"Task ID":7,"Index":7,"Attempt":0,"Partition ID":7,"Launch Time":1642039496427,"Executor ID":"driver","Host":"172.22.200.52","Locality":"PR...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":6,"Index":6,"Attempt":0,"Partition ID":6,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":5,"Index":5,"Attempt":0,"Partition ID":5,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":4,"Index":4,"Attempt":0,"Partition ID":4,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":2,"Index":2,"Attempt":0,"Partition ID":2,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":0,"Index":0,"Attempt":0,"Partition ID":0,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":1,"Index":1,"Attempt":0,"Partition ID":1,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":7,"Index":7,"Attempt":0,"Partition ID":7,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":3,"Index":3,"Attempt":0,"Partition ID":3,"Launch Time":16420394...
{"Event":"SparkListenerStageCompleted","Stage Info":{"Stage ID":0,"Stage Attempt ID":0,"Stage Name":"count at <console>:23","Number of Tasks":8,"RDD Info":[{"RDD ID":4,"Name":"MapPartitionsRDD","Scope":"{\"id\":\"0\",\"n...
{"Event":"SparkListenerJobEnd","Job ID":0,"Completion Time":1642039496914,"Job Result":{"Result":"JobSucceeded"}}
{"Event":"org.apache.spark.sql.execution.ui.SparkListenerSQLAdaptiveExecutionUpdate","executionId":0,"physicalPlanDescription":"== Physical Plan ==\nAdaptiveSparkPlan (13)\n+- == Current Plan ==\n   HashAggregate (8)\n  ...
{"Event":"org.apache.spark.sql.execution.ui.SparkListenerDriverAccumUpdates","executionId":0,"accumUpdates":[[106,1]]}
{"Event":"SparkListenerJobStart","Job ID":1,"Submission Time":1642039497010,"Stage Infos":[{"Stage ID":1,"Stage Attempt ID":0,"Stage Name":"count at <console>:23","Number of Tasks":8,"RDD Info":[{"RDD ID":4,"Name":"MapPa...
{"Event":"SparkListenerStageSubmitted","Stage Info":{"Stage ID":2,"Stage Attempt ID":0,"Stage Name":"count at <console>:23","Number of Tasks":10,"RDD Info":[{"RDD ID":7,"Name":"MapPartitionsRDD","Scope":"{\"id\":\"4\",\"...
{"Event":"SparkListenerTaskStart","Stage ID":2,"Stage Attempt ID":0,"Task Info":{"Task ID":8,"Index":0,"Attempt":0,"Partition ID":0,"Launch Time":1642039497053,"Executor ID":"driver","Host":"172.22.200.52","Locality":"NO...
{"Event":"SparkListenerTaskStart","Stage ID":2,"Stage Attempt ID":0,"Task Info":{"Task ID":9,"Index":1,"Attempt":0,"Partition ID":1,"Launch Time":1642039497055,"Executor ID":"driver","Host":"172.22.200.52","Locality":"NO...
{"Event":"SparkListenerTaskStart","Stage ID":2,"Stage Attempt ID":0,"Task Info":{"Task ID":10,"Index":2,"Attempt":0,"Partition ID":2,"Launch Time":1642039497055,"Executor ID":"driver","Host":"172.22.200.52","Locality":"N...
[... 5 line(s) omitted ... ⟦tj:ec91309812eed646f17609ccf8bd4302⟧]
{"Event":"SparkListenerTaskStart","Stage ID":2,"Stage Attempt ID":0,"Task Info":{"Task ID":16,"Index":8,"Attempt":0,"Partition ID":8,"Launch Time":1642039497114,"Executor ID":"driver","Host":"172.22.200.52","Locality":"N...

```

### `32-w3c-iis`

- [Full input](cases/32-w3c-iis/input.log)
- [Output with CCR](cases/32-w3c-iis/output.log) - [diff](cases/32-w3c-iis/compression.diff)
- [Output without CCR](cases/32-w3c-iis/output-noccr.log) - [diff](cases/32-w3c-iis/compression-noccr.diff)

Input excerpt:

```text
#Software: Microsoft Internet Information Services 8.5
#Version: 1.0
#Date: 2015-01-13 00:32:17
#Fields: date time s-ip cs-method cs-uri-stem cs-uri-query s-port cs-username c-ip cs(User-Agent) cs(Referer) sc-status sc-substatus sc-win32-status sc-bytes cs-bytes time-taken
2015-01-13 00:32:17 100.79.192.81 GET /robots.txt - 80 - 157.55.39.146 Mozilla/5.0+(compatible;+bingbot/2.0;++http://www.bing.com/bingbot.htm) - 404 0 2 1405 242 283
2015-01-13 00:32:17 100.79.192.81 GET /robots.txt - 80 - 157.55.39.146 Mozilla/5.0+(compatible;+bingbot/2.0;++http://www.bing.com/bingbot.htm) - 404 0 2 1405 242 157
2015-01-13 00:32:17 100.79.192.81 GET /robots.txt - 80 - 157.55.39.146 Mozilla/5.0+(compatible;+bingbot/2.0;++http://www.bing.com/bingbot.htm) - 404 0 2 1405 242 152
2015-01-13 00:32:17 100.79.192.81 GET /robots.txt - 80 - 157.55.39.146 Mozilla/5.0+(compatible;+bingbot/2.0;++http://www.bing.com/bingbot.htm) - 404 0 2 1405 242 149
2015-01-13 00:32:17 100.79.192.81 GET /robots.txt - 80 - 157.55.39.146 Mozilla/5.0+(compatible;+bingbot/2.0;++http://www.bing.com/bingbot.htm) - 404 0 2 1405 242 137
2015-01-13 00:32:26 100.79.192.81 GET /p/eToken1.png - 80 - 207.46.13.64 Mozilla/5.0+(compatible;+bingbot/2.0;++http://www.bing.com/bingbot.htm) - 404 0 2 1405 245 157
#Software: Microsoft Internet Information Services 8.5
#Version: 1.0
#Date: 2015-01-13 02:05:40
#Fields: date time s-ip cs-method cs-uri-stem cs-uri-query s-port cs-username c-ip cs(User-Agent) cs(Referer) sc-status sc-substatus sc-win32-status sc-bytes cs-bytes time-taken
2015-01-13 02:05:40 100.79.192.81 GET /48672181611.html - 80 - 180.111.242.129 Mozilla/4.0+(compatible;+MSIE+8.0;+Windows+NT+5.1;+Trident/4.0) - 404 0 2 1405 141 2411
#Software: Microsoft Internet Information Services 8.5
#Version: 1.0
#Date: 2015-01-13 08:22:18
#Fields: date time s-ip cs-method cs-uri-stem cs-uri-query s-port cs-username c-ip cs(User-Agent) cs(Referer) sc-status sc-substatus sc-win32-status sc-bytes cs-bytes time-taken
2015-01-13 08:22:18 100.79.192.81 GET /robots.txt - 80 - 66.249.78.6 Mozilla/5.0+(compatible;+Googlebot/2.1;++http://www.google.com/bot.html) - 404 0 2 1405 250 156
2015-01-13 08:22:18 100.79.192.81 GET /contact/ - 80 - 66.249.64.36 Mozilla/5.0+AppleWebKit/537.36+(KHTML,+like+Gecko;+Google+Web+Preview+Analytics)+Chrome/27.0.1453+Safari/537.36+(compatible;+Googlebot/2.1;++http://www....
#Software: Microsoft Internet Information Services 8.5
#Version: 1.0
#Date: 2015-01-13 09:49:46
#Fields: date time s-ip cs-method cs-uri-stem cs-uri-query s-port cs-username c-ip cs(User-Agent) cs(Referer) sc-status sc-substatus sc-win32-status sc-bytes cs-bytes time-taken
2015-01-13 09:49:46 100.79.192.81 GET / - 80 - 188.120.253.124 Mozilla/5.0+(Windows;+U;+Windows+NT+5.1;+en-US)+AppleWebKit/533.4+(KHTML,+like+Gecko)+Chrome/5.0.375.99+Safari/533.4 http://example.com 200 0 0 960 205 468
#Software: Microsoft Internet Information Services 8.5
#Version: 1.0
#Date: 2015-01-13 10:16:10
#Fields: date time s-ip cs-method cs-uri-stem cs-uri-query s-port cs-username c-ip cs(User-Agent) cs(Referer) sc-status sc-substatus sc-win32-status sc-bytes cs-bytes time-taken
2015-01-13 10:16:10 100.79.192.81 GET /robots.txt - 80 - 37.59.20.217 Mozilla/5.0+(compatible;+MJ12bot/v1.4.5;+http://www.majestic12.co.uk/bot.php?+) - 404 0 2 1424 170 156
2015-01-13 10:16:15 100.79.192.81 GET / - 80 - 37.59.20.217 Mozilla/5.0+(compatible;+MJ12bot/v1.4.5;+http://www.majestic12.co.uk/bot.php?+) - 200 0 0 979 313 265
#Software: Microsoft Internet Information Services 8.5
#Version: 1.0
#Date: 2015-01-13 10:46:33
#Fields: date time s-ip cs-method cs-uri-stem cs-uri-query s-port cs-username c-ip cs(User-Agent) cs(Referer) sc-status sc-substatus sc-win32-status sc-bytes cs-bytes time-taken

```

Output excerpt:

```text
#Software: Microsoft Internet Information Services 8.5
#Version: 1.0
#Date: 2015-01-13 00:32:17
#Fields: date time s-ip cs-method cs-uri-stem cs-uri-query s-port cs-username c-ip cs(User-Agent) cs(Referer) sc-status sc-substatus sc-win32-status sc-bytes cs-bytes time-taken
2015-01-13 00:32:17 100.79.192.81 GET /robots.txt - 80 - 157.55.39.146 Mozilla/5.0+(compatible;+bingbot/2.0;++http://www.bing.com/bingbot.htm) - 404 0 2 1405 242 283
2015-01-13 00:32:17 100.79.192.81 GET /robots.txt - 80 - 157.55.39.146 Mozilla/5.0+(compatible;+bingbot/2.0;++http://www.bing.com/bingbot.htm) - 404 0 2 1405 242 157
2015-01-13 00:32:17 100.79.192.81 GET /robots.txt - 80 - 157.55.39.146 Mozilla/5.0+(compatible;+bingbot/2.0;++http://www.bing.com/bingbot.htm) - 404 0 2 1405 242 152
2015-01-13 00:32:17 100.79.192.81 GET /robots.txt - 80 - 157.55.39.146 Mozilla/5.0+(compatible;+bingbot/2.0;++http://www.bing.com/bingbot.htm) - 404 0 2 1405 242 149
2015-01-13 00:32:17 100.79.192.81 GET /robots.txt - 80 - 157.55.39.146 Mozilla/5.0+(compatible;+bingbot/2.0;++http://www.bing.com/bingbot.htm) - 404 0 2 1405 242 137
2015-01-13 00:32:26 100.79.192.81 GET /p/eToken1.png - 80 - 207.46.13.64 Mozilla/5.0+(compatible;+bingbot/2.0;++http://www.bing.com/bingbot.htm) - 404 0 2 1405 245 157
#Software: Microsoft Internet Information Services 8.5
#Version: 1.0
#Date: 2015-01-13 02:05:40
#Fields: date time s-ip cs-method cs-uri-stem cs-uri-query s-port cs-username c-ip cs(User-Agent) cs(Referer) sc-status sc-substatus sc-win32-status sc-bytes cs-bytes time-taken
2015-01-13 02:05:40 100.79.192.81 GET /48672181611.html - 80 - 180.111.242.129 Mozilla/4.0+(compatible;+MSIE+8.0;+Windows+NT+5.1;+Trident/4.0) - 404 0 2 1405 141 2411
#Software: Microsoft Internet Information Services 8.5
#Version: 1.0
#Date: 2015-01-13 08:22:18
#Fields: date time s-ip cs-method cs-uri-stem cs-uri-query s-port cs-username c-ip cs(User-Agent) cs(Referer) sc-status sc-substatus sc-win32-status sc-bytes cs-bytes time-taken
2015-01-13 08:22:18 100.79.192.81 GET /robots.txt - 80 - 66.249.78.6 Mozilla/5.0+(compatible;+Googlebot/2.1;++http://www.google.com/bot.html) - 404 0 2 1405 250 156
2015-01-13 08:22:18 100.79.192.81 GET /contact/ - 80 - 66.249.64.36 Mozilla/5.0+AppleWebKit/537.36+(KHTML,+like+Gecko;+Google+Web+Preview+Analytics)+Chrome/27.0.1453+Safari/537.36+(compatible;+Googlebot/2.1;++http://www....
#Software: Microsoft Internet Information Services 8.5
#Version: 1.0
#Date: 2015-01-13 09:49:46
#Fields: date time s-ip cs-method cs-uri-stem cs-uri-query s-port cs-username c-ip cs(User-Agent) cs(Referer) sc-status sc-substatus sc-win32-status sc-bytes cs-bytes time-taken
2015-01-13 09:49:46 100.79.192.81 GET / - 80 - 188.120.253.124 Mozilla/5.0+(Windows;+U;+Windows+NT+5.1;+en-US)+AppleWebKit/533.4+(KHTML,+like+Gecko)+Chrome/5.0.375.99+Safari/533.4 http://example.com 200 0 0 960 205 468
#Software: Microsoft Internet Information Services 8.5
#Version: 1.0
#Date: 2015-01-13 10:16:10
#Fields: date time s-ip cs-method cs-uri-stem cs-uri-query s-port cs-username c-ip cs(User-Agent) cs(Referer) sc-status sc-substatus sc-win32-status sc-bytes cs-bytes time-taken
2015-01-13 10:16:10 100.79.192.81 GET /robots.txt - 80 - 37.59.20.217 Mozilla/5.0+(compatible;+MJ12bot/v1.4.5;+http://www.majestic12.co.uk/bot.php?+) - 404 0 2 1424 170 156
2015-01-13 10:16:15 100.79.192.81 GET / - 80 - 37.59.20.217 Mozilla/5.0+(compatible;+MJ12bot/v1.4.5;+http://www.majestic12.co.uk/bot.php?+) - 200 0 0 979 313 265
#Software: Microsoft Internet Information Services 8.5
#Version: 1.0
#Date: 2015-01-13 10:46:33
#Fields: date time s-ip cs-method cs-uri-stem cs-uri-query s-port cs-username c-ip cs(User-Agent) cs(Referer) sc-status sc-substatus sc-win32-status sc-bytes cs-bytes time-taken

```

### `31-zeek-http`

- [Full input](cases/31-zeek-http/input.log)
- [Output with CCR](cases/31-zeek-http/output.log) - [diff](cases/31-zeek-http/compression.diff)
- [Output without CCR](cases/31-zeek-http/output-noccr.log) - [diff](cases/31-zeek-http/compression-noccr.diff)

Input excerpt:

```text
#separator \x09
#set_separator	,
#empty_field	(empty)
#unset_field	-
#path	http
#open	2017-04-16-21-36-10
#fields	ts	uid	id.orig_h	id.orig_p	id.resp_h	id.resp_p	trans_depth	method	host	uri	referrer	version	user_agent	request_body_len	response_body_len	status_code	status_msg	info_code	info_msg	tags	username	password	proxied	o...
#types	time	string	addr	port	addr	port	count	string	string	string	string	string	string	count	count	count	string	count	string	set[enum]	string	string	set[string]	vector[string]	vector[string]	vector[string]	vector[string]...
1320279566.452687	CwFs1P2UcUdlSxD2La	192.168.2.76	52026	132.235.215.119	80	1	GET	www.reddit.com	/	-	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/20100101 Firefox/7.0.1	0	109978	200	OK	-	-	(empty)	-	-	...
1320279566.831619	CJxSUgkInyKSHiju1	192.168.2.76	52030	72.21.211.173	80	1	GET	e.thumbs.redditmedia.com	/E-pbDbmiBclPkDaX.jpg	http://www.reddit.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/2010010...
1320279566.831563	CJwUi9bdB9c1lLW44	192.168.2.76	52029	72.21.211.173	80	1	GET	f.thumbs.redditmedia.com	/BP5bQfy4o-C7cF6A.jpg	http://www.reddit.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/2010010...
1320279566.831473	CoX7zA3OJKGUOSCBY2	192.168.2.76	52027	72.21.211.173	80	1	GET	e.thumbs.redditmedia.com	/SVUtep3Rhg5FTRn4.jpg	http://www.reddit.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/201001...
1320279566.831643	CT0JIh479jXIGt0Po1	192.168.2.76	52031	72.21.211.173	80	1	GET	f.thumbs.redditmedia.com	/uuy31444rLSyKdHS.jpg	http://www.reddit.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/201001...
1320279566.831666	C6Q4Vm14ZJIlZhsXqk	192.168.2.76	52032	72.21.211.173	80	1	GET	a.thumbs.redditmedia.com	/BoVp7eG0DUodTIfr.jpg	http://www.reddit.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/201001...
1320279566.831535	CdrfXZ1NOFPEawF218	192.168.2.76	52028	72.21.211.173	80	1	GET	c.thumbs.redditmedia.com	/IEeSI3Q47xHE0UEz.jpg	http://www.reddit.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/201001...
1320279567.211407	CdysLK1XpcrXOpVDuh	192.168.2.76	52034	174.129.249.33	80	1	GET	www.redditmedia.com	/ads/	http://www.reddit.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/20100101 Firefox/7.0.1	0	3...
1320279567.211031	CtgxRAqDLvrRUQdqe	192.168.2.76	52033	184.72.234.3	80	1	GET	pixel.redditmedia.com	/pixel/of_destiny.png?v=32tb6zakMbpImUZWtz+pksVc/8wYRc822cfKz091HT0oAKWHwZGxGpDcvvwUpyjwU8nJsyGc4cw=&r=296143927	http://w...
1320279567.296908	CwFs1P2UcUdlSxD2La	192.168.2.76	52026	132.235.215.119	80	2	GET	www.reddit.com	/static/bg-button-positive-unpressed.png	http://www.reddit.com/static/reddit.RZTLMiZ4gTk.css	1.1	Mozilla/5.0 (Macintosh; Int...
1320279567.451885	CtgxRAqDLvrRUQdqe	192.168.2.76	52033	184.72.234.3	80	2	GET	pixel.redditmedia.com	/fetch-trackers?callback=jQuery16107779853632052074_1320279566998&ids[]=t5_6&ids[]=t3_lsfmb&ids[]=t3_lsejk&_=132027956719...
1320279567.482546	C6nSoj1Qco9PGyslz6	192.168.2.76	52035	184.72.234.3	80	1	GET	pixel.redditmedia.com	/fetch-trackers?callback=jQuery16107779853632052074_1320279566999&ids[]=t5_6&ids[]=t3_lsfmb&ids[]=t3_lsejk&_=13202795671...
1320279567.536586	CN5hnY3x51j6Hr1v4	192.168.2.76	52036	74.125.225.78	80	1	GET	www.google-analytics.com	/__utm.gif?utmwv=5.2.0&utms=1&utmn=872724630&utmhn=www.reddit.com&utme=8(site*srpath*usertype*uitype)9( reddit.com* r...
1320279567.689996	CdZUPH2DKOE7zzCLE3	192.168.2.76	52038	132.235.215.119	80	1	GET	feeds.bbci.co.uk	/news/rss.xml?edition=int	-	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/20100101 Firefox/7.0.1	0	4484...
1320279567.680708	CtgxRAqDLvrRUQdqe	192.168.2.76	52033	184.72.234.3	80	3	GET	pixel.redditmedia.com	/pixel/of_doom.png?id=t5_6&hash=e962d119a7ff69901bb4ceaa7f3ba1224fd704b7&r=741109704	http://www.reddit.com/	1.1	Mozilla/5...
1320279567.683031	C6nSoj1Qco9PGyslz6	192.168.2.76	52035	184.72.234.3	80	2	GET	pixel.redditmedia.com	/pixel/of_doom.png?id=t3_lsfmb&hash=1c635ac04668546a1c33c2faf3c4814cd6c4f96a&r=1492956402	http://www.reddit.com/	1.1	Moz...
1320279567.690049	CmWpC33jXuKpXNLcie	192.168.2.76	52037	74.125.225.91	80	1	GET	ad.doubleclick.net	/adj/reddit.dart/reddit.com;kw=reddit.com;tile=1;sz=300x250;ord=5117434431991380?	http://www.redditmedia.com/ads/	1.1	Mozi...
1320279568.281910	CtgxRAqDLvrRUQdqe	192.168.2.76	52033	184.72.234.3	80	4	GET	pixel.redditmedia.com	/pixel/of_defenestration.png?hash=a8ababd2e4912c8b21d72252ad18ebb5d8e27ea3&id=dart_reddit.com&random=5012335803517919	htt...
1320279571.625521	CsBgiE1WmGP4Yo749h	192.168.2.76	52039	69.171.228.39	80	1	GET	www.facebook.com	/	-	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/20100101 Firefox/7.0.1	0	31379	200	OK	-	-	(empty)	-	-	-...
1320279571.883692	CYfHyC28tAhkLYkXB7	192.168.2.76	52040	132.235.215.117	80	1	GET	static.ak.fbcdn.net	/rsrc.php/v1/yt/r/svonORc8tTu.css	http://www.facebook.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) G...
1320279571.883724	CtANmVrHYMtkWqPE5	192.168.2.76	52041	132.235.215.117	80	1	GET	static.ak.fbcdn.net	/rsrc.php/v1/yZ/r/ejLIIb8vBQK.css	http://www.facebook.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Ge...
1320279571.884016	CSTH8n1O1nv0ztxNQd	192.168.2.76	52042	132.235.215.117	80	1	GET	static.ak.fbcdn.net	/rsrc.php/v1/yp/r/kk8dc2UJYJ4.png	http://www.facebook.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) G...
1320279571.884052	C4uDKU5tpeRU9Su19	192.168.2.76	52043	132.235.215.117	80	1	GET	static.ak.fbcdn.net	/rsrc.php/v1/yb/r/GsNJNwuI-UM.gif	http://www.facebook.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Ge...
1320279571.930335	CYfHyC28tAhkLYkXB7	192.168.2.76	52040	132.235.215.117	80	2	GET	static.ak.fbcdn.net	/rsrc.php/yi/r/q9U99v3_saj.ico	-	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/20100101 Firefox/7.0....
1320279572.530622	CYfHyC28tAhkLYkXB7	192.168.2.76	52040	132.235.215.117	80	3	GET	static.ak.fbcdn.net	/rsrc.php/v1/yB/r/TwAHgQi2ZPB.png	http://static.ak.fbcdn.net/rsrc.php/v1/yt/r/svonORc8tTu.css	1.1	Mozilla/5.0 (Macintos...
1320279572.541605	CYfHyC28tAhkLYkXB7	192.168.2.76	52040	132.235.215.117	80	4	GET	static.ak.fbcdn.net	/rsrc.php/v1/yu/r/O03OuHGGSjF.js	http://www.facebook.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Ge...
1320279572.531333	CSTH8n1O1nv0ztxNQd	192.168.2.76	52042	132.235.215.117	80	2	GET	static.ak.fbcdn.net	/rsrc.php/v1/yi/r/OBaVg52wtTZ.png	http://static.ak.fbcdn.net/rsrc.php/v1/yt/r/svonORc8tTu.css	1.1	Mozilla/5.0 (Macintos...
1320279577.475501	CEh6Ka2HInkNSH01L2	192.168.2.76	52044	216.34.181.48	80	1	GET	www.slashdot.org	/	-	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/20100101 Firefox/7.0.1	0	297	301	Moved Permanently	-	-	...

```

Output excerpt:

```text
#separator \x09
#set_separator	,
#empty_field	(empty)
#unset_field	-
#path	http
#open	2017-04-16-21-36-10
#fields	ts	uid	id.orig_h	id.orig_p	id.resp_h	id.resp_p	trans_depth	method	host	uri	referrer	version	user_agent	request_body_len	response_body_len	status_code	status_msg	info_code	info_msg	tags	username	password	proxied	o...
#types	time	string	addr	port	addr	port	count	string	string	string	string	string	string	count	count	count	string	count	string	set[enum]	string	string	set[string]	vector[string]	vector[string]	vector[string]	vector[string]...
1320279566.452687	CwFs1P2UcUdlSxD2La	192.168.2.76	52026	132.235.215.119	80	1	GET	www.reddit.com	/	-	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/20100101 Firefox/7.0.1	0	109978	200	OK	-	-	(empty)	-	-	...
1320279566.831619	CJxSUgkInyKSHiju1	192.168.2.76	52030	72.21.211.173	80	1	GET	e.thumbs.redditmedia.com	/E-pbDbmiBclPkDaX.jpg	http://www.reddit.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/2010010...
1320279566.831563	CJwUi9bdB9c1lLW44	192.168.2.76	52029	72.21.211.173	80	1	GET	f.thumbs.redditmedia.com	/BP5bQfy4o-C7cF6A.jpg	http://www.reddit.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/2010010...
1320279566.831473	CoX7zA3OJKGUOSCBY2	192.168.2.76	52027	72.21.211.173	80	1	GET	e.thumbs.redditmedia.com	/SVUtep3Rhg5FTRn4.jpg	http://www.reddit.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/201001...
1320279566.831643	CT0JIh479jXIGt0Po1	192.168.2.76	52031	72.21.211.173	80	1	GET	f.thumbs.redditmedia.com	/uuy31444rLSyKdHS.jpg	http://www.reddit.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/201001...
1320279566.831666	C6Q4Vm14ZJIlZhsXqk	192.168.2.76	52032	72.21.211.173	80	1	GET	a.thumbs.redditmedia.com	/BoVp7eG0DUodTIfr.jpg	http://www.reddit.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/201001...
1320279566.831535	CdrfXZ1NOFPEawF218	192.168.2.76	52028	72.21.211.173	80	1	GET	c.thumbs.redditmedia.com	/IEeSI3Q47xHE0UEz.jpg	http://www.reddit.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/201001...
1320279567.211407	CdysLK1XpcrXOpVDuh	192.168.2.76	52034	174.129.249.33	80	1	GET	www.redditmedia.com	/ads/	http://www.reddit.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/20100101 Firefox/7.0.1	0	3...
1320279567.211031	CtgxRAqDLvrRUQdqe	192.168.2.76	52033	184.72.234.3	80	1	GET	pixel.redditmedia.com	/pixel/of_destiny.png?v=32tb6zakMbpImUZWtz+pksVc/8wYRc822cfKz091HT0oAKWHwZGxGpDcvvwUpyjwU8nJsyGc4cw=&r=296143927	http://w...
1320279567.296908	CwFs1P2UcUdlSxD2La	192.168.2.76	52026	132.235.215.119	80	2	GET	www.reddit.com	/static/bg-button-positive-unpressed.png	http://www.reddit.com/static/reddit.RZTLMiZ4gTk.css	1.1	Mozilla/5.0 (Macintosh; Int...
1320279567.451885	CtgxRAqDLvrRUQdqe	192.168.2.76	52033	184.72.234.3	80	2	GET	pixel.redditmedia.com	/fetch-trackers?callback=jQuery16107779853632052074_1320279566998&ids[]=t5_6&ids[]=t3_lsfmb&ids[]=t3_lsejk&_=132027956719...
1320279567.482546	C6nSoj1Qco9PGyslz6	192.168.2.76	52035	184.72.234.3	80	1	GET	pixel.redditmedia.com	/fetch-trackers?callback=jQuery16107779853632052074_1320279566999&ids[]=t5_6&ids[]=t3_lsfmb&ids[]=t3_lsejk&_=13202795671...
1320279567.536586	CN5hnY3x51j6Hr1v4	192.168.2.76	52036	74.125.225.78	80	1	GET	www.google-analytics.com	/__utm.gif?utmwv=5.2.0&utms=1&utmn=872724630&utmhn=www.reddit.com&utme=8(site*srpath*usertype*uitype)9( reddit.com* r...
1320279567.689996	CdZUPH2DKOE7zzCLE3	192.168.2.76	52038	132.235.215.119	80	1	GET	feeds.bbci.co.uk	/news/rss.xml?edition=int	-	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/20100101 Firefox/7.0.1	0	4484...
1320279567.680708	CtgxRAqDLvrRUQdqe	192.168.2.76	52033	184.72.234.3	80	3	GET	pixel.redditmedia.com	/pixel/of_doom.png?id=t5_6&hash=e962d119a7ff69901bb4ceaa7f3ba1224fd704b7&r=741109704	http://www.reddit.com/	1.1	Mozilla/5...
1320279567.683031	C6nSoj1Qco9PGyslz6	192.168.2.76	52035	184.72.234.3	80	2	GET	pixel.redditmedia.com	/pixel/of_doom.png?id=t3_lsfmb&hash=1c635ac04668546a1c33c2faf3c4814cd6c4f96a&r=1492956402	http://www.reddit.com/	1.1	Moz...
1320279567.690049	CmWpC33jXuKpXNLcie	192.168.2.76	52037	74.125.225.91	80	1	GET	ad.doubleclick.net	/adj/reddit.dart/reddit.com;kw=reddit.com;tile=1;sz=300x250;ord=5117434431991380?	http://www.redditmedia.com/ads/	1.1	Mozi...
1320279568.281910	CtgxRAqDLvrRUQdqe	192.168.2.76	52033	184.72.234.3	80	4	GET	pixel.redditmedia.com	/pixel/of_defenestration.png?hash=a8ababd2e4912c8b21d72252ad18ebb5d8e27ea3&id=dart_reddit.com&random=5012335803517919	htt...
1320279571.625521	CsBgiE1WmGP4Yo749h	192.168.2.76	52039	69.171.228.39	80	1	GET	www.facebook.com	/	-	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/20100101 Firefox/7.0.1	0	31379	200	OK	-	-	(empty)	-	-	-...
1320279571.883692	CYfHyC28tAhkLYkXB7	192.168.2.76	52040	132.235.215.117	80	1	GET	static.ak.fbcdn.net	/rsrc.php/v1/yt/r/svonORc8tTu.css	http://www.facebook.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) G...
1320279571.883724	CtANmVrHYMtkWqPE5	192.168.2.76	52041	132.235.215.117	80	1	GET	static.ak.fbcdn.net	/rsrc.php/v1/yZ/r/ejLIIb8vBQK.css	http://www.facebook.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Ge...
1320279571.884016	CSTH8n1O1nv0ztxNQd	192.168.2.76	52042	132.235.215.117	80	1	GET	static.ak.fbcdn.net	/rsrc.php/v1/yp/r/kk8dc2UJYJ4.png	http://www.facebook.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) G...
1320279571.884052	C4uDKU5tpeRU9Su19	192.168.2.76	52043	132.235.215.117	80	1	GET	static.ak.fbcdn.net	/rsrc.php/v1/yb/r/GsNJNwuI-UM.gif	http://www.facebook.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Ge...
1320279571.930335	CYfHyC28tAhkLYkXB7	192.168.2.76	52040	132.235.215.117	80	2	GET	static.ak.fbcdn.net	/rsrc.php/yi/r/q9U99v3_saj.ico	-	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/20100101 Firefox/7.0....
1320279572.530622	CYfHyC28tAhkLYkXB7	192.168.2.76	52040	132.235.215.117	80	3	GET	static.ak.fbcdn.net	/rsrc.php/v1/yB/r/TwAHgQi2ZPB.png	http://static.ak.fbcdn.net/rsrc.php/v1/yt/r/svonORc8tTu.css	1.1	Mozilla/5.0 (Macintos...
1320279572.541605	CYfHyC28tAhkLYkXB7	192.168.2.76	52040	132.235.215.117	80	4	GET	static.ak.fbcdn.net	/rsrc.php/v1/yu/r/O03OuHGGSjF.js	http://www.facebook.com/	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Ge...
1320279572.531333	CSTH8n1O1nv0ztxNQd	192.168.2.76	52042	132.235.215.117	80	2	GET	static.ak.fbcdn.net	/rsrc.php/v1/yi/r/OBaVg52wtTZ.png	http://static.ak.fbcdn.net/rsrc.php/v1/yt/r/svonORc8tTu.css	1.1	Mozilla/5.0 (Macintos...
1320279577.475501	CEh6Ka2HInkNSH01L2	192.168.2.76	52044	216.34.181.48	80	1	GET	www.slashdot.org	/	-	1.1	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:7.0.1) Gecko/20100101 Firefox/7.0.1	0	297	301	Moved Permanently	-	-	...

```

### `27-suricata-eve`

- [Full input](cases/27-suricata-eve/input.log)
- [Output with CCR](cases/27-suricata-eve/output.log) - [diff](cases/27-suricata-eve/compression.diff)
- [Output without CCR](cases/27-suricata-eve/output-noccr.log) - [diff](cases/27-suricata-eve/compression-noccr.diff)

Input excerpt:

```text
{"timestamp":"2022-07-10T23:50:05.533101+0000","flow_id":1258832998001901,"in_iface":"bond0","event_type":"alert","src_ip":"1.2.3.4","src_port":52812,"dest_ip":"127.0.0.1","dest_port":80,"proto":"TCP","metadata":{"flowbi...
{"timestamp":"2022-07-10T23:52:31.097119+0000","flow_id":836311304881252,"in_iface":"bond0","event_type":"alert","src_ip":"1.2.3.5","src_port":53114,"dest_ip":"127.0.0.1","dest_port":80,"proto":"TCP","metadata":{"flowbit...
{"timestamp":"2022-07-11T05:52:18.378379+0000","flow_id":883709815679084,"in_iface":"eth0","event_type":"flow","src_ip":"1.2.3.6","src_port":56814,"dest_ip":"127.0.0.1","dest_port":2352,"proto":"TCP","flow":{"pkts_toserv...
{"timestamp":"2022-07-11T05:53:51.912203+0000","event_type":"stats","stats":{"uptime":510552,"capture":{"kernel_packets":373404,"kernel_drops":0,"errors":0},"decoder":{"pkts":373404,"bytes":75476420,"invalid":0,"ipv4":33...
{"timestamp":"2022-07-11T06:09:52.602489+0000","flow_id":1684596746908921,"in_iface":"eth0","event_type":"alert","src_ip":"1.2.3.7","src_port":36288,"dest_ip":"127.0.0.1","dest_port":80,"proto":"TCP","tx_id":0,"alert":{"...
{"timestamp":"2022-07-12T05:52:18.378379-0000","flow_id":883709815679084,"in_iface":"eth0","event_type":"flow","src_ip":"1.2.3.6","src_port":56814,"dest_ip":"127.0.0.1","dest_port":2352,"proto":"TCP","flow":{"pkts_toserv...
{"timestamp":"2022-07-12T05:53:51.912203-0000","event_type":"stats","stats":{"uptime":510552,"capture":{"kernel_packets":373404,"kernel_drops":0,"errors":0},"decoder":{"pkts":373404,"bytes":75476420,"invalid":0,"ipv4":33...
{"timestamp":"2022-07-12T06:09:52.602489-0000","flow_id":1684596746908921,"in_iface":"eth0","event_type":"alert","src_ip":"1.2.3.7","src_port":36288,"dest_ip":"127.0.0.1","dest_port":80,"proto":"TCP","tx_id":0,"alert":{"...

```

Output excerpt:

```text
{"timestamp":"2022-07-10T23:50:05.533101+0000","flow_id":1258832998001901,"in_iface":"bond0","event_type":"alert","src_ip":"1.2.3.4","src_port":52812,"dest_ip":"127.0.0.1","dest_port":80,"proto":"TCP","metadata":{"flowbi...
{"timestamp":"2022-07-10T23:52:31.097119+0000","flow_id":836311304881252,"in_iface":"bond0","event_type":"alert","src_ip":"1.2.3.5","src_port":53114,"dest_ip":"127.0.0.1","dest_port":80,"proto":"TCP","metadata":{"flowbit...
{"timestamp":"2022-07-11T05:52:18.378379+0000","flow_id":883709815679084,"in_iface":"eth0","event_type":"flow","src_ip":"1.2.3.6","src_port":56814,"dest_ip":"127.0.0.1","dest_port":2352,"proto":"TCP","flow":{"pkts_toserv...
{"timestamp":"2022-07-11T05:53:51.912203+0000","event_type":"stats","stats":{"uptime":510552,"capture":{"kernel_packets":373404,"kernel_drops":0,"errors":0},"decoder":{"pkts":373404,"bytes":75476420,"invalid":0,"ipv4":33...
{"timestamp":"2022-07-11T06:09:52.602489+0000","flow_id":1684596746908921,"in_iface":"eth0","event_type":"alert","src_ip":"1.2.3.7","src_port":36288,"dest_ip":"127.0.0.1","dest_port":80,"proto":"TCP","tx_id":0,"alert":{"...
{"timestamp":"2022-07-12T05:52:18.378379-0000","flow_id":883709815679084,"in_iface":"eth0","event_type":"flow","src_ip":"1.2.3.6","src_port":56814,"dest_ip":"127.0.0.1","dest_port":2352,"proto":"TCP","flow":{"pkts_toserv...
{"timestamp":"2022-07-12T05:53:51.912203-0000","event_type":"stats","stats":{"uptime":510552,"capture":{"kernel_packets":373404,"kernel_drops":0,"errors":0},"decoder":{"pkts":373404,"bytes":75476420,"invalid":0,"ipv4":33...
{"timestamp":"2022-07-12T06:09:52.602489-0000","flow_id":1684596746908921,"in_iface":"eth0","event_type":"alert","src_ip":"1.2.3.7","src_port":36288,"dest_ip":"127.0.0.1","dest_port":80,"proto":"TCP","tx_id":0,"alert":{"...

```

### `26-gitlab-bf`

- [Full input](cases/26-gitlab-bf/input.log)
- [Output with CCR](cases/26-gitlab-bf/output.log) - [diff](cases/26-gitlab-bf/compression.diff)
- [Output without CCR](cases/26-gitlab-bf/output-noccr.log) - [diff](cases/26-gitlab-bf/compression-noccr.diff)

Input excerpt:

```text
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T12:58:36.195Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T12:58:39.197Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T12:58:41.212Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T12:58:45.689Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":200,"time":"2022-04-15T12:58:50.060Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"ke...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":200,"time":"2022-04-15T12:58:52.054Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"ke...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T13:44:36.132Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T13:44:40.289Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T13:44:42.369Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T13:44:55.149Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T13:44:59.813Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T13:45:02.293Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/auth/ldapmain/callback","format":"html","controller":"OmniauthCallbacksController","action":"failure","status":302,"location":"https://gitlab.***/users/sign_in","time":"2025-05-22T04:26:59...
{"method":"POST","path":"/users/auth/ldapmain/callback","format":"html","controller":"OmniauthCallbacksController","action":"failure","status":302,"location":"https://gitlab.***/users/sign_in","time":"2025-05-22T04:27:01...
{"method":"POST","path":"/users/auth/ldapmain/callback","format":"html","controller":"OmniauthCallbacksController","action":"failure","status":302,"location":"https://gitlab.***/users/sign_in","time":"2025-05-22T04:27:03...
{"method":"POST","path":"/users/auth/ldapmain/callback","format":"html","controller":"OmniauthCallbacksController","action":"failure","status":302,"location":"https://gitlab.***/users/sign_in","time":"2025-05-22T04:27:06...
{"method":"POST","path":"/users/auth/ldapmain/callback","format":"html","controller":"OmniauthCallbacksController","action":"failure","status":302,"location":"https://gitlab.***/users/sign_in","time":"2025-05-22T04:27:09...
{"method":"POST","path":"/users/auth/ldapmain/callback","format":"html","controller":"OmniauthCallbacksController","action":"failure","status":302,"location":"https://gitlab.***/users/sign_in","time":"2025-05-22T04:27:11...

```

Output excerpt:

```text
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T12:58:36.195Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T12:58:39.197Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T12:58:41.212Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T12:58:45.689Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":200,"time":"2022-04-15T12:58:50.060Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"ke...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":200,"time":"2022-04-15T12:58:52.054Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"ke...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T13:44:36.132Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T13:44:40.289Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T13:44:42.369Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T13:44:55.149Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T13:44:59.813Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/sign_in","format":"html","controller":"SessionsController","action":"create","status":0,"time":"2022-04-15T13:45:02.293Z","params":[{"key":"authenticity_token","value":"[FILTERED]"},{"key"...
{"method":"POST","path":"/users/auth/ldapmain/callback","format":"html","controller":"OmniauthCallbacksController","action":"failure","status":302,"location":"https://gitlab.***/users/sign_in","time":"2025-05-22T04:26:59...
{"method":"POST","path":"/users/auth/ldapmain/callback","format":"html","controller":"OmniauthCallbacksController","action":"failure","status":302,"location":"https://gitlab.***/users/sign_in","time":"2025-05-22T04:27:01...
{"method":"POST","path":"/users/auth/ldapmain/callback","format":"html","controller":"OmniauthCallbacksController","action":"failure","status":302,"location":"https://gitlab.***/users/sign_in","time":"2025-05-22T04:27:03...
{"method":"POST","path":"/users/auth/ldapmain/callback","format":"html","controller":"OmniauthCallbacksController","action":"failure","status":302,"location":"https://gitlab.***/users/sign_in","time":"2025-05-22T04:27:06...
{"method":"POST","path":"/users/auth/ldapmain/callback","format":"html","controller":"OmniauthCallbacksController","action":"failure","status":302,"location":"https://gitlab.***/users/sign_in","time":"2025-05-22T04:27:09...
{"method":"POST","path":"/users/auth/ldapmain/callback","format":"html","controller":"OmniauthCallbacksController","action":"failure","status":302,"location":"https://gitlab.***/users/sign_in","time":"2025-05-22T04:27:11...

```

### `25-sshesame-honeypot`

- [Full input](cases/25-sshesame-honeypot/input.log)
- [Output with CCR](cases/25-sshesame-honeypot/output.log) - [diff](cases/25-sshesame-honeypot/compression.diff)
- [Output without CCR](cases/25-sshesame-honeypot/output-noccr.log) - [diff](cases/25-sshesame-honeypot/compression-noccr.diff)

Input excerpt:

```text
2022/05/06 04:53:57 [190.2.139.67:58629] [channel 106] direct TCP/IP forwarding from 127.0.0.1:22 to 142.93.136.142:80 requested
2022/05/06 04:53:57 [190.2.139.67:58629] [channel 106] input: "GET /?requestid=53219 HTTP/1.1\r\nHost: ip.bablosoft.com\r\nConnection: close\r\nAccept: */*\r\nConnection: close\r\n\r\n"
2022/05/06 04:53:57 [190.2.139.67:58629] [channel 106] closed
2022/05/06 04:58:33 [190.2.139.67:7117] [channel 63] direct TCP/IP forwarding from 127.0.0.1:22 to 142.93.136.142:80 requested
2022/05/06 04:58:33 [190.2.139.67:7117] [channel 63] input: "GET /?requestid=61619 HTTP/1.1\r\nHost: ip.bablosoft.com\r\nConnection: close\r\nAccept: */*\r\nConnection: close\r\n\r\n"
2022/05/06 04:58:33 [190.2.139.67:7117] [channel 63] closed
2022/05/06 05:10:03 [195.3.147.60:28696] authentication for user "admin" without credentials rejected
2022/05/06 05:10:03 [195.3.147.60:28696] authentication for user "admin" with password "aisadmin" accepted
2022/05/06 05:10:03 [195.3.147.60:28696] connection with client version "SSH-2.0-OpenSSH_5.9" established
2022/05/06 05:10:03 [195.3.147.60:28696] [channel 0] direct TCP/IP forwarding from 127.0.0.1:24161 to 74.125.205.113:80 requested
2022/05/06 05:10:03 [195.3.147.60:28696] [channel 0] input: "GET / HTTP/1.0\r\nHost: google.com\r\nConnection: close\r\n\r\n"
2022/05/06 05:10:03 [195.3.147.60:28696] [channel 0] closed
2022/05/06 05:10:03 [195.3.147.60:28696] [channel 1] direct TCP/IP forwarding from 127.0.0.1:14687 to [2a00:1450:4010:c02::8b]:80 requested
2022/05/06 05:10:03 [195.3.147.60:28696] [channel 1] input: "GET / HTTP/1.0\r\nHost: google.com\r\nConnection: close\r\n\r\n"
2022/05/06 05:10:03 [195.3.147.60:28696] [channel 1] closed
2022/05/06 05:10:03 [195.3.147.60:28696] connection closed
2022/05/06 05:11:00 [185.131.12.144:60273] authentication for user "default" without credentials rejected
2022/05/06 05:11:02 [185.131.12.144:60273] authentication for user "default" with password "1" accepted
2022/05/06 05:11:02 [185.131.12.144:60273] connection with client version "SSH-2.0-OpenSSH_7.4" established
2022/05/06 05:11:04 [185.131.12.144:60273] connection closed
2022/05/06 05:37:28 [165.232.183.156:55934] authentication for user "xuexiaoman" without credentials rejected
2022/05/06 05:37:28 [165.232.183.156:55934] authentication for user "xuexiaoman" with password "xuexiaoman" accepted
2022/05/06 05:37:28 [165.232.183.156:55934] connection with client version "SSH-2.0-Go" established
2022/05/06 05:37:28 [165.232.183.156:55934] [channel 0] session requested
2022/05/06 05:37:28 [165.232.183.156:55934] [channel 0] command "uname -s -v -n -r -m" requested
2022/05/06 05:37:29 [165.232.183.156:55934] [channel 0] closed
2022/05/06 05:40:00 [165.232.183.156:55934] connection closed
2022/05/06 05:40:30 [186.78.209.242:47338] authentication for user "pi" without credentials rejected
2022/05/06 05:40:30 [186.78.209.242:47338] authentication for user "pi" with password "raspberryraspberry993311" accepted
2022/05/06 05:40:30 [186.78.209.242:47338] connection with client version "SSH-2.0-OpenSSH_7.9p1 Raspbian-10+deb10u2+rpt1" established
2022/05/06 05:40:30 [186.78.209.242:47338] rejection of further session channels requested
2022/05/06 05:40:30 [186.78.209.242:47338] [channel 0] session requested
2022/05/06 05:40:31 [186.78.209.242:47338] [channel 0] environment variable "LANG" with value "en_GB.UTF-8" requested
2022/05/06 05:40:31 [186.78.209.242:47338] [channel 0] command "scp -t /tmp/taCiyiIF" requested
2022/05/06 05:40:31 [186.78.209.242:47338] [channel 0] closed
2022/05/06 05:40:31 [186.78.209.242:47338] connection closed

```

Output excerpt:

```text
2022/05/06 04:53:57 [190.2.139.67:58629] [channel 106] direct TCP/IP forwarding from 127.0.0.1:22 to 142.93.136.142:80 requested
2022/05/06 04:53:57 [190.2.139.67:58629] [channel 106] input: "GET /?requestid=53219 HTTP/1.1\r\nHost: ip.bablosoft.com\r\nConnection: close\r\nAccept: */*\r\nConnection: close\r\n\r\n"
2022/05/06 04:53:57 [190.2.139.67:58629] [channel 106] closed
2022/05/06 04:58:33 [190.2.139.67:7117] [channel 63] direct TCP/IP forwarding from 127.0.0.1:22 to 142.93.136.142:80 requested
2022/05/06 04:58:33 [190.2.139.67:7117] [channel 63] input: "GET /?requestid=61619 HTTP/1.1\r\nHost: ip.bablosoft.com\r\nConnection: close\r\nAccept: */*\r\nConnection: close\r\n\r\n"
2022/05/06 04:58:33 [190.2.139.67:7117] [channel 63] closed
2022/05/06 05:10:03 [195.3.147.60:28696] authentication for user "admin" without credentials rejected
2022/05/06 05:10:03 [195.3.147.60:28696] authentication for user "admin" with password "aisadmin" accepted
2022/05/06 05:10:03 [195.3.147.60:28696] connection with client version "SSH-2.0-OpenSSH_5.9" established
2022/05/06 05:10:03 [195.3.147.60:28696] [channel 0] direct TCP/IP forwarding from 127.0.0.1:24161 to 74.125.205.113:80 requested
2022/05/06 05:10:03 [195.3.147.60:28696] [channel 0] input: "GET / HTTP/1.0\r\nHost: google.com\r\nConnection: close\r\n\r\n"
2022/05/06 05:10:03 [195.3.147.60:28696] [channel 0] closed
2022/05/06 05:10:03 [195.3.147.60:28696] [channel 1] direct TCP/IP forwarding from 127.0.0.1:14687 to [2a00:1450:4010:c02::8b]:80 requested
2022/05/06 05:10:03 [195.3.147.60:28696] [channel 1] input: "GET / HTTP/1.0\r\nHost: google.com\r\nConnection: close\r\n\r\n"
2022/05/06 05:10:03 [195.3.147.60:28696] [channel 1] closed
2022/05/06 05:10:03 [195.3.147.60:28696] connection closed
2022/05/06 05:11:00 [185.131.12.144:60273] authentication for user "default" without credentials rejected
2022/05/06 05:11:02 [185.131.12.144:60273] authentication for user "default" with password "1" accepted
2022/05/06 05:11:02 [185.131.12.144:60273] connection with client version "SSH-2.0-OpenSSH_7.4" established
2022/05/06 05:11:04 [185.131.12.144:60273] connection closed
2022/05/06 05:37:28 [165.232.183.156:55934] authentication for user "xuexiaoman" without credentials rejected
2022/05/06 05:37:28 [165.232.183.156:55934] authentication for user "xuexiaoman" with password "xuexiaoman" accepted
2022/05/06 05:37:28 [165.232.183.156:55934] connection with client version "SSH-2.0-Go" established
2022/05/06 05:37:28 [165.232.183.156:55934] [channel 0] session requested
2022/05/06 05:37:28 [165.232.183.156:55934] [channel 0] command "uname -s -v -n -r -m" requested
2022/05/06 05:37:29 [165.232.183.156:55934] [channel 0] closed
2022/05/06 05:40:00 [165.232.183.156:55934] connection closed
2022/05/06 05:40:30 [186.78.209.242:47338] authentication for user "pi" without credentials rejected
2022/05/06 05:40:30 [186.78.209.242:47338] authentication for user "pi" with password "raspberryraspberry993311" accepted
2022/05/06 05:40:30 [186.78.209.242:47338] connection with client version "SSH-2.0-OpenSSH_7.9p1 Raspbian-10+deb10u2+rpt1" established
2022/05/06 05:40:30 [186.78.209.242:47338] rejection of further session channels requested
2022/05/06 05:40:30 [186.78.209.242:47338] [channel 0] session requested
2022/05/06 05:40:31 [186.78.209.242:47338] [channel 0] environment variable "LANG" with value "en_GB.UTF-8" requested
2022/05/06 05:40:31 [186.78.209.242:47338] [channel 0] command "scp -t /tmp/taCiyiIF" requested
2022/05/06 05:40:31 [186.78.209.242:47338] [channel 0] closed
2022/05/06 05:40:31 [186.78.209.242:47338] connection closed

```

### `24-http-dos`

- [Full input](cases/24-http-dos/input.log)
- [Output with CCR](cases/24-http-dos/output.log) - [diff](cases/24-http-dos/compression.diff)
- [Output without CCR](cases/24-http-dos/output-noccr.log) - [diff](cases/24-http-dos/compression-noccr.diff)

Input excerpt:

```text
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?8890121518 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?139153420466 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?121726773140 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?143226578870 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?46611161870 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?73603075199 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?177979831590 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?243568562324 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?155712433421 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?219770465784 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?115657720130 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.1.2.10 - - [10/Oct/2023:08:32:31 +0000] "GET /?29336676637 HTTP/1.1" 200 3437 "https://www.usatoday.com/search/results?q=localhost/" "Mozilla/5.0 (Windows 8) AppleWebKit/536.0 (KHTML, like Gecko) Chrome/24.08523.145 ...
127.1.2.10 - - [10/Oct/2023:08:32:31 +0000] "GET /?204692048474 HTTP/1.1" 200 3437 "https://www.usatoday.com/search/results?q=localhost/" "Mozilla/5.0 (Windows 8) AppleWebKit/536.0 (KHTML, like Gecko) Chrome/24.08523.145...
127.1.2.10 - - [10/Oct/2023:08:32:31 +0000] "GET /?254949042963 HTTP/1.1" 200 3437 "https://www.usatoday.com/search/results?q=localhost/" "Mozilla/5.0 (Windows 8) AppleWebKit/536.0 (KHTML, like Gecko) Chrome/24.08523.145...
127.1.2.10 - - [10/Oct/2023:08:32:31 +0000] "GET /?126399237686 HTTP/1.1" 200 3437 "https://www.usatoday.com/search/results?q=localhost/" "Mozilla/5.0 (Windows 8) AppleWebKit/536.0 (KHTML, like Gecko) Chrome/24.08523.145...
127.1.2.10 - - [10/Oct/2023:08:32:31 +0000] "GET /?224596106669 HTTP/1.1" 200 3437 "https://www.usatoday.com/search/results?q=localhost/" "Mozilla/5.0 (Windows 8) AppleWebKit/536.0 (KHTML, like Gecko) Chrome/24.08523.145...
127.1.2.10 - - [10/Oct/2023:08:32:31 +0000] "GET /?177097115359 HTTP/1.1" 200 3437 "https://www.usatoday.com/search/results?q=localhost/" "Mozilla/5.0 (Windows 8) AppleWebKit/536.0 (KHTML, like Gecko) Chrome/24.08523.145...
127.1.2.10 - - [10/Oct/2023:08:32:31 +0000] "GET /?187307234582 HTTP/1.1" 200 3437 "https://www.usatoday.com/search/results?q=localhost/" "Mozilla/5.0 (Windows 8) AppleWebKit/536.0 (KHTML, like Gecko) Chrome/24.08523.145...
127.1.2.10 - - [10/Oct/2023:08:32:31 +0000] "GET /?141787167776 HTTP/1.1" 200 3437 "https://www.usatoday.com/search/results?q=localhost/" "Mozilla/5.0 (Windows 8) AppleWebKit/536.0 (KHTML, like Gecko) Chrome/24.08523.145...
127.1.2.10 - - [10/Oct/2023:08:32:31 +0000] "GET /?256205938646 HTTP/1.1" 200 3437 "https://www.usatoday.com/search/results?q=localhost/" "Mozilla/5.0 (Windows 8) AppleWebKit/536.0 (KHTML, like Gecko) Chrome/24.08523.145...
127.1.1.6 - - [10/Oct/2023:08:32:31 +0000] "GET /?212726358119 HTTP/1.1" 200 10701 "https://www.google.com/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 42.0; Windows NT 6.1; Tablet PC; Trident/54.0)"
127.1.1.6 - - [10/Oct/2023:08:32:31 +0000] "GET /?91644935910 HTTP/1.1" 200 10701 "https://www.google.com/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 42.0; Windows NT 6.1; Tablet PC; Trident/54.0)"
127.3.2.1 - - [10/Oct/2023:08:32:31 +0000] "GET /?53436490197 HTTP/1.1" 200 10701 "https://www.google.com.af/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 19.0; 68K; Trident/27.0)"
127.3.2.1 - - [10/Oct/2023:08:32:31 +0000] "GET /?112190356440 HTTP/1.1" 200 10701 "https://www.google.com.af/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 19.0; 68K; Trident/27.0)"
127.3.2.1 - - [10/Oct/2023:08:32:31 +0000] "GET /?131999370343 HTTP/1.1" 200 10701 "https://www.google.com.af/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 19.0; 68K; Trident/27.0)"
127.3.2.1 - - [10/Oct/2023:08:32:31 +0000] "GET /?224894927745 HTTP/1.1" 200 10701 "https://www.google.com.af/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 19.0; 68K; Trident/27.0)"
127.2.2.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?139747440160 HTTP/1.1" 200 10701 "https://soda.demo.socrata.com/resource/4tka-6guv.json?$q=localhost/" "Mozilla/5.0 (Windows NT 6.0) AppleWebKit/524.0 (KHTML, like Gecko)...
127.2.2.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?235297870465 HTTP/1.1" 200 10701 "https://soda.demo.socrata.com/resource/4tka-6guv.json?$q=localhost/" "Mozilla/5.0 (Windows NT 6.0) AppleWebKit/524.0 (KHTML, like Gecko)...
127.2.2.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?258067560706 HTTP/1.1" 200 10701 "https://soda.demo.socrata.com/resource/4tka-6guv.json?$q=localhost/" "Mozilla/5.0 (Windows NT 6.0) AppleWebKit/524.0 (KHTML, like Gecko)...
127.1.1.6 - - [10/Oct/2023:08:32:31 +0000] "GET /?68531377541 HTTP/1.1" 200 10701 "https://www.google.com/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 42.0; Windows NT 6.1; Tablet PC; Trident/54.0)"
127.1.1.6 - - [10/Oct/2023:08:32:31 +0000] "GET /?137803466437 HTTP/1.1" 200 10701 "https://www.google.com/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 42.0; Windows NT 6.1; Tablet PC; Trident/54.0)"
127.1.1.6 - - [10/Oct/2023:08:32:31 +0000] "GET /?226999224294 HTTP/1.1" 200 10701 "https://www.google.com/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 42.0; Windows NT 6.1; Tablet PC; Trident/54.0)"
127.1.1.6 - - [10/Oct/2023:08:32:31 +0000] "GET /?13525042774 HTTP/1.1" 200 10701 "https://www.google.com/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 42.0; Windows NT 6.1; Tablet PC; Trident/54.0)"
127.1.1.6 - - [10/Oct/2023:08:32:31 +0000] "GET /?84776694018 HTTP/1.1" 200 10701 "https://www.google.com/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 42.0; Windows NT 6.1; Tablet PC; Trident/54.0)"
127.1.1.6 - - [10/Oct/2023:08:32:31 +0000] "GET /?239949447652 HTTP/1.1" 200 10701 "https://www.google.com/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 42.0; Windows NT 6.1; Tablet PC; Trident/54.0)"
127.1.1.6 - - [10/Oct/2023:08:32:31 +0000] "GET /?262337338535 HTTP/1.1" 200 9233 "https://www.google.com/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 42.0; Windows NT 6.1; Tablet PC; Trident/54.0)"

```

Output excerpt:

```text
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?8890121518 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?139153420466 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?121726773140 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?143226578870 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?46611161870 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?73603075199 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?177979831590 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?243568562324 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?155712433421 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?219770465784 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.4.3.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?115657720130 HTTP/1.1" 200 10701 "https://www.google.am/search?q=localhost/" "Mozilla/5.0 (Win 9x 4.90; rv:22.0) Gecko/20200928 Firefox/22.0"
127.1.2.10 - - [10/Oct/2023:08:32:31 +0000] "GET /?29336676637 HTTP/1.1" 200 3437 "https://www.usatoday.com/search/results?q=localhost/" "Mozilla/5.0 (Windows 8) AppleWebKit/536.0 (KHTML, like Gecko) Chrome/24.08523.145 ...
127.1.2.10 - - [10/Oct/2023:08:32:31 +0000] "GET /?204692048474 HTTP/1.1" 200 3437 "https://www.usatoday.com/search/results?q=localhost/" "Mozilla/5.0 (Windows 8) AppleWebKit/536.0 (KHTML, like Gecko) Chrome/24.08523.145...
127.1.2.10 - - [10/Oct/2023:08:32:31 +0000] "GET /?254949042963 HTTP/1.1" 200 3437 "https://www.usatoday.com/search/results?q=localhost/" "Mozilla/5.0 (Windows 8) AppleWebKit/536.0 (KHTML, like Gecko) Chrome/24.08523.145...
127.1.2.10 - - [10/Oct/2023:08:32:31 +0000] "GET /?126399237686 HTTP/1.1" 200 3437 "https://www.usatoday.com/search/results?q=localhost/" "Mozilla/5.0 (Windows 8) AppleWebKit/536.0 (KHTML, like Gecko) Chrome/24.08523.145...
127.1.2.10 - - [10/Oct/2023:08:32:31 +0000] "GET /?224596106669 HTTP/1.1" 200 3437 "https://www.usatoday.com/search/results?q=localhost/" "Mozilla/5.0 (Windows 8) AppleWebKit/536.0 (KHTML, like Gecko) Chrome/24.08523.145...
127.1.2.10 - - [10/Oct/2023:08:32:31 +0000] "GET /?177097115359 HTTP/1.1" 200 3437 "https://www.usatoday.com/search/results?q=localhost/" "Mozilla/5.0 (Windows 8) AppleWebKit/536.0 (KHTML, like Gecko) Chrome/24.08523.145...
127.1.2.10 - - [10/Oct/2023:08:32:31 +0000] "GET /?187307234582 HTTP/1.1" 200 3437 "https://www.usatoday.com/search/results?q=localhost/" "Mozilla/5.0 (Windows 8) AppleWebKit/536.0 (KHTML, like Gecko) Chrome/24.08523.145...
127.1.2.10 - - [10/Oct/2023:08:32:31 +0000] "GET /?141787167776 HTTP/1.1" 200 3437 "https://www.usatoday.com/search/results?q=localhost/" "Mozilla/5.0 (Windows 8) AppleWebKit/536.0 (KHTML, like Gecko) Chrome/24.08523.145...
127.1.2.10 - - [10/Oct/2023:08:32:31 +0000] "GET /?256205938646 HTTP/1.1" 200 3437 "https://www.usatoday.com/search/results?q=localhost/" "Mozilla/5.0 (Windows 8) AppleWebKit/536.0 (KHTML, like Gecko) Chrome/24.08523.145...
127.1.1.6 - - [10/Oct/2023:08:32:31 +0000] "GET /?212726358119 HTTP/1.1" 200 10701 "https://www.google.com/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 42.0; Windows NT 6.1; Tablet PC; Trident/54.0)"
127.1.1.6 - - [10/Oct/2023:08:32:31 +0000] "GET /?91644935910 HTTP/1.1" 200 10701 "https://www.google.com/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 42.0; Windows NT 6.1; Tablet PC; Trident/54.0)"
127.3.2.1 - - [10/Oct/2023:08:32:31 +0000] "GET /?53436490197 HTTP/1.1" 200 10701 "https://www.google.com.af/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 19.0; 68K; Trident/27.0)"
127.3.2.1 - - [10/Oct/2023:08:32:31 +0000] "GET /?112190356440 HTTP/1.1" 200 10701 "https://www.google.com.af/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 19.0; 68K; Trident/27.0)"
127.3.2.1 - - [10/Oct/2023:08:32:31 +0000] "GET /?131999370343 HTTP/1.1" 200 10701 "https://www.google.com.af/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 19.0; 68K; Trident/27.0)"
127.3.2.1 - - [10/Oct/2023:08:32:31 +0000] "GET /?224894927745 HTTP/1.1" 200 10701 "https://www.google.com.af/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 19.0; 68K; Trident/27.0)"
127.2.2.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?139747440160 HTTP/1.1" 200 10701 "https://soda.demo.socrata.com/resource/4tka-6guv.json?$q=localhost/" "Mozilla/5.0 (Windows NT 6.0) AppleWebKit/524.0 (KHTML, like Gecko)...
127.2.2.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?235297870465 HTTP/1.1" 200 10701 "https://soda.demo.socrata.com/resource/4tka-6guv.json?$q=localhost/" "Mozilla/5.0 (Windows NT 6.0) AppleWebKit/524.0 (KHTML, like Gecko)...
127.2.2.2 - - [10/Oct/2023:08:32:31 +0000] "GET /?258067560706 HTTP/1.1" 200 10701 "https://soda.demo.socrata.com/resource/4tka-6guv.json?$q=localhost/" "Mozilla/5.0 (Windows NT 6.0) AppleWebKit/524.0 (KHTML, like Gecko)...
127.1.1.6 - - [10/Oct/2023:08:32:31 +0000] "GET /?68531377541 HTTP/1.1" 200 10701 "https://www.google.com/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 42.0; Windows NT 6.1; Tablet PC; Trident/54.0)"
127.1.1.6 - - [10/Oct/2023:08:32:31 +0000] "GET /?137803466437 HTTP/1.1" 200 10701 "https://www.google.com/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 42.0; Windows NT 6.1; Tablet PC; Trident/54.0)"
127.1.1.6 - - [10/Oct/2023:08:32:31 +0000] "GET /?226999224294 HTTP/1.1" 200 10701 "https://www.google.com/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 42.0; Windows NT 6.1; Tablet PC; Trident/54.0)"
127.1.1.6 - - [10/Oct/2023:08:32:31 +0000] "GET /?13525042774 HTTP/1.1" 200 10701 "https://www.google.com/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 42.0; Windows NT 6.1; Tablet PC; Trident/54.0)"
127.1.1.6 - - [10/Oct/2023:08:32:31 +0000] "GET /?84776694018 HTTP/1.1" 200 10701 "https://www.google.com/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 42.0; Windows NT 6.1; Tablet PC; Trident/54.0)"
127.1.1.6 - - [10/Oct/2023:08:32:31 +0000] "GET /?239949447652 HTTP/1.1" 200 10701 "https://www.google.com/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 42.0; Windows NT 6.1; Tablet PC; Trident/54.0)"
127.1.1.6 - - [10/Oct/2023:08:32:31 +0000] "GET /?262337338535 HTTP/1.1" 200 9233 "https://www.google.com/search?q=localhost/" "Mozilla/5.0 (compatible; MSIE 42.0; Windows NT 6.1; Tablet PC; Trident/54.0)"

```

### `22-traefik-http`

- [Full input](cases/22-traefik-http/input.log)
- [Output with CCR](cases/22-traefik-http/output.log) - [diff](cases/22-traefik-http/compression.diff)
- [Output without CCR](cases/22-traefik-http/output-noccr.log) - [diff](cases/22-traefik-http/compression-noccr.diff)

Input excerpt:

```text
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":215563,"OriginContentSize":356,"OriginDuration":204708,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":221071,"OriginContentSize":356,"OriginDuration":208300,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":293192,"OriginContentSize":356,"OriginDuration":282825,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":358,"DownstreamStatus":404,"Duration":203756,"OriginContentSize":358,"OriginDuration":191877,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":236283,"OriginContentSize":356,"OriginDuration":222687,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":250311,"OriginContentSize":356,"OriginDuration":219964,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":358,"DownstreamStatus":404,"Duration":862057,"OriginContentSize":358,"OriginDuration":826299,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":360,"DownstreamStatus":404,"Duration":257280,"OriginContentSize":360,"OriginDuration":241167,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":357,"DownstreamStatus":404,"Duration":670681,"OriginContentSize":357,"OriginDuration":655388,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":388748,"OriginContentSize":356,"OriginDuration":368461,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":357,"DownstreamStatus":404,"Duration":371391,"OriginContentSize":357,"OriginDuration":340554,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":264620,"OriginContentSize":356,"OriginDuration":246002,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":363,"DownstreamStatus":404,"Duration":239035,"OriginContentSize":363,"OriginDuration":228234,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":243151,"OriginContentSize":356,"OriginDuration":228568,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":292339,"OriginContentSize":356,"OriginDuration":264921,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":396272,"OriginContentSize":356,"OriginDuration":374715,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":359,"DownstreamStatus":404,"Duration":971590,"OriginContentSize":359,"OriginDuration":910516,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":357,"DownstreamStatus":404,"Duration":346765,"OriginContentSize":357,"OriginDuration":327435,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":299828,"OriginContentSize":356,"OriginDuration":271004,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":359,"DownstreamStatus":404,"Duration":227517,"OriginContentSize":359,"OriginDuration":215492,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":355,"DownstreamStatus":404,"Duration":257161,"OriginContentSize":355,"OriginDuration":216910,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":201678,"OriginContentSize":356,"OriginDuration":192185,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":206623,"OriginContentSize":356,"OriginDuration":194507,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":359,"DownstreamStatus":404,"Duration":229777,"OriginContentSize":359,"OriginDuration":211769,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":359,"DownstreamStatus":404,"Duration":229777,"OriginContentSize":359,"OriginDuration":211769,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":362,"DownstreamStatus":404,"Duration":218407,"OriginContentSize":362,"OriginDuration":206047,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":357,"DownstreamStatus":404,"Duration":238777,"OriginContentSize":357,"OriginDuration":227532,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":365,"DownstreamStatus":404,"Duration":727038,"OriginContentSize":365,"OriginDuration":666613,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":357,"DownstreamStatus":404,"Duration":262803,"OriginContentSize":357,"OriginDuration":249790,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":360,"DownstreamStatus":404,"Duration":379585,"OriginContentSize":360,"OriginDuration":363568,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":240376,"OriginContentSize":356,"OriginDuration":227041,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":217874,"OriginContentSize":356,"OriginDuration":207637,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":359,"DownstreamStatus":404,"Duration":224377,"OriginContentSize":359,"OriginDuration":211392,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":361,"DownstreamStatus":404,"Duration":219219,"OriginContentSize":361,"OriginDuration":207830,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":355,"DownstreamStatus":404,"Duration":224740,"OriginContentSize":355,"OriginDuration":209491,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":204472,"OriginContentSize":356,"OriginDuration":192116,"O...

```

Output excerpt:

```text
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":215563,"OriginContentSize":356,"OriginDuration":204708,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":221071,"OriginContentSize":356,"OriginDuration":208300,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":293192,"OriginContentSize":356,"OriginDuration":282825,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":358,"DownstreamStatus":404,"Duration":203756,"OriginContentSize":358,"OriginDuration":191877,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":236283,"OriginContentSize":356,"OriginDuration":222687,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":250311,"OriginContentSize":356,"OriginDuration":219964,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":358,"DownstreamStatus":404,"Duration":862057,"OriginContentSize":358,"OriginDuration":826299,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":360,"DownstreamStatus":404,"Duration":257280,"OriginContentSize":360,"OriginDuration":241167,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":357,"DownstreamStatus":404,"Duration":670681,"OriginContentSize":357,"OriginDuration":655388,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":388748,"OriginContentSize":356,"OriginDuration":368461,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":357,"DownstreamStatus":404,"Duration":371391,"OriginContentSize":357,"OriginDuration":340554,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":264620,"OriginContentSize":356,"OriginDuration":246002,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":363,"DownstreamStatus":404,"Duration":239035,"OriginContentSize":363,"OriginDuration":228234,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":243151,"OriginContentSize":356,"OriginDuration":228568,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":292339,"OriginContentSize":356,"OriginDuration":264921,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":396272,"OriginContentSize":356,"OriginDuration":374715,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":359,"DownstreamStatus":404,"Duration":971590,"OriginContentSize":359,"OriginDuration":910516,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":357,"DownstreamStatus":404,"Duration":346765,"OriginContentSize":357,"OriginDuration":327435,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":299828,"OriginContentSize":356,"OriginDuration":271004,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":359,"DownstreamStatus":404,"Duration":227517,"OriginContentSize":359,"OriginDuration":215492,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":355,"DownstreamStatus":404,"Duration":257161,"OriginContentSize":355,"OriginDuration":216910,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":201678,"OriginContentSize":356,"OriginDuration":192185,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":206623,"OriginContentSize":356,"OriginDuration":194507,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":359,"DownstreamStatus":404,"Duration":229777,"OriginContentSize":359,"OriginDuration":211769,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":359,"DownstreamStatus":404,"Duration":229777,"OriginContentSize":359,"OriginDuration":211769,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":362,"DownstreamStatus":404,"Duration":218407,"OriginContentSize":362,"OriginDuration":206047,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":357,"DownstreamStatus":404,"Duration":238777,"OriginContentSize":357,"OriginDuration":227532,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":365,"DownstreamStatus":404,"Duration":727038,"OriginContentSize":365,"OriginDuration":666613,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":357,"DownstreamStatus":404,"Duration":262803,"OriginContentSize":357,"OriginDuration":249790,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":360,"DownstreamStatus":404,"Duration":379585,"OriginContentSize":360,"OriginDuration":363568,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":240376,"OriginContentSize":356,"OriginDuration":227041,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":217874,"OriginContentSize":356,"OriginDuration":207637,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":359,"DownstreamStatus":404,"Duration":224377,"OriginContentSize":359,"OriginDuration":211392,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":361,"DownstreamStatus":404,"Duration":219219,"OriginContentSize":361,"OriginDuration":207830,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":355,"DownstreamStatus":404,"Duration":224740,"OriginContentSize":355,"OriginDuration":209491,"O...
{"ClientAddr":"172.17.0.1:39490","ClientHost":"172.17.0.1","ClientPort":"39490","ClientUsername":"-","DownstreamContentSize":356,"DownstreamStatus":404,"Duration":204472,"OriginContentSize":356,"OriginDuration":192116,"O...

```

### `21-traefik-flood`

- [Full input](cases/21-traefik-flood/input.log)
- [Output with CCR](cases/21-traefik-flood/output.log) - [diff](cases/21-traefik-flood/compression.diff)
- [Output without CCR](cases/21-traefik-flood/output-noccr.log) - [diff](cases/21-traefik-flood/compression-noccr.diff)

Input excerpt:

```text
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling TCP connection from 1.2.3.4:35501 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"

```

Output excerpt:

```text
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling TCP connection from 1.2.3.4:35501 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"
time="2024-01-15T18:43:09Z" level=debug msg="Handling UDP stream from 1.2.3.44:34871 to 192.168.176.3:21116"

```

### `18-nginx-access`

- [Full input](cases/18-nginx-access/input.log)
- [Output with CCR](cases/18-nginx-access/output.log) - [diff](cases/18-nginx-access/compression.diff)
- [Output without CCR](cases/18-nginx-access/output-noccr.log) - [diff](cases/18-nginx-access/compression-noccr.diff)

Input excerpt:

```text
93.180.71.3 - - [17/May/2015:08:05:32 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.21)"
93.180.71.3 - - [17/May/2015:08:05:23 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.21)"
80.91.33.133 - - [17/May/2015:08:05:24 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.17)"
217.168.17.5 - - [17/May/2015:08:05:34 +0000] "GET /downloads/product_1 HTTP/1.1" 200 490 "-" "Debian APT-HTTP/1.3 (0.8.10.3)"
217.168.17.5 - - [17/May/2015:08:05:09 +0000] "GET /downloads/product_2 HTTP/1.1" 200 490 "-" "Debian APT-HTTP/1.3 (0.8.10.3)"
93.180.71.3 - - [17/May/2015:08:05:57 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.21)"
217.168.17.5 - - [17/May/2015:08:05:02 +0000] "GET /downloads/product_2 HTTP/1.1" 404 337 "-" "Debian APT-HTTP/1.3 (0.8.10.3)"
217.168.17.5 - - [17/May/2015:08:05:42 +0000] "GET /downloads/product_1 HTTP/1.1" 404 332 "-" "Debian APT-HTTP/1.3 (0.8.10.3)"
80.91.33.133 - - [17/May/2015:08:05:01 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.17)"
93.180.71.3 - - [17/May/2015:08:05:27 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.21)"
217.168.17.5 - - [17/May/2015:08:05:12 +0000] "GET /downloads/product_2 HTTP/1.1" 200 3316 "-" "-"
188.138.60.101 - - [17/May/2015:08:05:49 +0000] "GET /downloads/product_2 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.9.7.9)"
80.91.33.133 - - [17/May/2015:08:05:14 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.16)"
46.4.66.76 - - [17/May/2015:08:05:45 +0000] "GET /downloads/product_1 HTTP/1.1" 404 318 "-" "Debian APT-HTTP/1.3 (1.0.1ubuntu2)"
93.180.71.3 - - [17/May/2015:08:05:26 +0000] "GET /downloads/product_1 HTTP/1.1" 404 324 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.21)"
91.234.194.89 - - [17/May/2015:08:05:22 +0000] "GET /downloads/product_2 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.9.7.9)"
80.91.33.133 - - [17/May/2015:08:05:07 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.17)"
37.26.93.214 - - [17/May/2015:08:05:38 +0000] "GET /downloads/product_2 HTTP/1.1" 404 319 "-" "Go 1.1 package http"
188.138.60.101 - - [17/May/2015:08:05:25 +0000] "GET /downloads/product_2 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.9.7.9)"
93.180.71.3 - - [17/May/2015:08:05:11 +0000] "GET /downloads/product_1 HTTP/1.1" 404 340 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.21)"
46.4.66.76 - - [17/May/2015:08:05:02 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (1.0.1ubuntu2)"
62.75.198.179 - - [17/May/2015:08:05:06 +0000] "GET /downloads/product_2 HTTP/1.1" 200 490 "-" "Debian APT-HTTP/1.3 (0.9.7.9)"
80.91.33.133 - - [17/May/2015:08:05:55 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.16)"
173.203.139.108 - - [17/May/2015:08:05:53 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.9.7.9)"
210.245.80.75 - - [17/May/2015:08:05:32 +0000] "GET /downloads/product_2 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (1.0.1ubuntu2)"
46.4.83.163 - - [17/May/2015:08:05:52 +0000] "GET /downloads/product_2 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.9.7.9)"
91.234.194.89 - - [17/May/2015:08:05:18 +0000] "GET /downloads/product_2 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.9.7.9)"
31.22.86.126 - - [17/May/2015:08:05:24 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.16)"
217.168.17.5 - - [17/May/2015:08:05:25 +0000] "GET /downloads/product_1 HTTP/1.1" 200 3301 "-" "-"
80.91.33.133 - - [17/May/2015:08:05:50 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.22)"
173.203.139.108 - - [17/May/2015:08:05:03 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.9.7.9)"
80.91.33.133 - - [17/May/2015:08:05:35 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.16)"
5.83.131.103 - - [17/May/2015:08:05:51 +0000] "GET /downloads/product_1 HTTP/1.1" 200 490 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.22)"
80.91.33.133 - - [17/May/2015:08:05:59 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.17)"
200.6.73.40 - - [17/May/2015:08:05:42 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.9.7.9)"
80.91.33.133 - - [17/May/2015:08:05:48 +0000] "GET /downloads/product_1 HTTP/1.1" 404 324 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.17)"

```

Output excerpt:

```text
93.180.71.3 - - [17/May/2015:08:05:32 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.21)"
93.180.71.3 - - [17/May/2015:08:05:23 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.21)"
80.91.33.133 - - [17/May/2015:08:05:24 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.17)"
217.168.17.5 - - [17/May/2015:08:05:34 +0000] "GET /downloads/product_1 HTTP/1.1" 200 490 "-" "Debian APT-HTTP/1.3 (0.8.10.3)"
217.168.17.5 - - [17/May/2015:08:05:09 +0000] "GET /downloads/product_2 HTTP/1.1" 200 490 "-" "Debian APT-HTTP/1.3 (0.8.10.3)"
93.180.71.3 - - [17/May/2015:08:05:57 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.21)"
217.168.17.5 - - [17/May/2015:08:05:02 +0000] "GET /downloads/product_2 HTTP/1.1" 404 337 "-" "Debian APT-HTTP/1.3 (0.8.10.3)"
217.168.17.5 - - [17/May/2015:08:05:42 +0000] "GET /downloads/product_1 HTTP/1.1" 404 332 "-" "Debian APT-HTTP/1.3 (0.8.10.3)"
80.91.33.133 - - [17/May/2015:08:05:01 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.17)"
93.180.71.3 - - [17/May/2015:08:05:27 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.21)"
217.168.17.5 - - [17/May/2015:08:05:12 +0000] "GET /downloads/product_2 HTTP/1.1" 200 3316 "-" "-"
188.138.60.101 - - [17/May/2015:08:05:49 +0000] "GET /downloads/product_2 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.9.7.9)"
80.91.33.133 - - [17/May/2015:08:05:14 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.16)"
46.4.66.76 - - [17/May/2015:08:05:45 +0000] "GET /downloads/product_1 HTTP/1.1" 404 318 "-" "Debian APT-HTTP/1.3 (1.0.1ubuntu2)"
93.180.71.3 - - [17/May/2015:08:05:26 +0000] "GET /downloads/product_1 HTTP/1.1" 404 324 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.21)"
91.234.194.89 - - [17/May/2015:08:05:22 +0000] "GET /downloads/product_2 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.9.7.9)"
80.91.33.133 - - [17/May/2015:08:05:07 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.17)"
37.26.93.214 - - [17/May/2015:08:05:38 +0000] "GET /downloads/product_2 HTTP/1.1" 404 319 "-" "Go 1.1 package http"
188.138.60.101 - - [17/May/2015:08:05:25 +0000] "GET /downloads/product_2 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.9.7.9)"
93.180.71.3 - - [17/May/2015:08:05:11 +0000] "GET /downloads/product_1 HTTP/1.1" 404 340 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.21)"
46.4.66.76 - - [17/May/2015:08:05:02 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (1.0.1ubuntu2)"
62.75.198.179 - - [17/May/2015:08:05:06 +0000] "GET /downloads/product_2 HTTP/1.1" 200 490 "-" "Debian APT-HTTP/1.3 (0.9.7.9)"
80.91.33.133 - - [17/May/2015:08:05:55 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.16)"
173.203.139.108 - - [17/May/2015:08:05:53 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.9.7.9)"
210.245.80.75 - - [17/May/2015:08:05:32 +0000] "GET /downloads/product_2 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (1.0.1ubuntu2)"
46.4.83.163 - - [17/May/2015:08:05:52 +0000] "GET /downloads/product_2 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.9.7.9)"
91.234.194.89 - - [17/May/2015:08:05:18 +0000] "GET /downloads/product_2 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.9.7.9)"
31.22.86.126 - - [17/May/2015:08:05:24 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.16)"
217.168.17.5 - - [17/May/2015:08:05:25 +0000] "GET /downloads/product_1 HTTP/1.1" 200 3301 "-" "-"
80.91.33.133 - - [17/May/2015:08:05:50 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.22)"
173.203.139.108 - - [17/May/2015:08:05:03 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.9.7.9)"
80.91.33.133 - - [17/May/2015:08:05:35 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.16)"
5.83.131.103 - - [17/May/2015:08:05:51 +0000] "GET /downloads/product_1 HTTP/1.1" 200 490 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.22)"
80.91.33.133 - - [17/May/2015:08:05:59 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.17)"
200.6.73.40 - - [17/May/2015:08:05:42 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.9.7.9)"
80.91.33.133 - - [17/May/2015:08:05:48 +0000] "GET /downloads/product_1 HTTP/1.1" 404 324 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.17)"

```

### `10-android`

- [Full input](cases/10-android/input.log)
- [Output with CCR](cases/10-android/output.log) - [diff](cases/10-android/compression.diff)
- [Output without CCR](cases/10-android/output-noccr.log) - [diff](cases/10-android/compression-noccr.diff)

Input excerpt:

```text
03-17 16:13:38.811  1702  2395 D WindowManager: printFreezingDisplayLogsopening app wtoken = AppWindowToken{9f4ef63 token=Token{a64f992 ActivityRecord{de9231d u0 com.tencent.qt.qtl/.activity.info.NewsDetailXmlActivity t7...
03-17 16:13:38.819  1702  8671 D PowerManagerService: acquire lock=233570404, flags=0x1, tag="View Lock", name=com.android.systemui, ws=null, uid=10037, pid=2227
03-17 16:13:38.820  1702  8671 D PowerManagerService: ready=true,policy=3,wakefulness=1,wksummary=0x23,uasummary=0x1,bootcompleted=true,boostinprogress=false,waitmodeenable=false,mode=false,manual=38,auto=-1,adj=0.0userI...
03-17 16:13:38.839  1702  2113 V WindowManager: Skipping AppWindowToken{df0798e token=Token{78af589 ActivityRecord{3b04890 u0 com.tencent.qt.qtl/com.tencent.video.player.activity.PlayerActivity t761}}} -- going to hide
03-17 16:13:38.859  2227  2227 D TextView: visible is system.time.showampm
03-17 16:13:38.861  2227  2227 D TextView: mVisiblity.getValue is false
03-17 16:13:38.869  2227  2227 D TextView: visible is system.charge.show
03-17 16:13:38.871  2227  2227 D TextView: mVisiblity.getValue is false
03-17 16:13:38.875  2227  2227 D TextView: visible is system.call.count gt 0
03-17 16:13:38.877  2227  2227 D TextView: mVisiblity.getValue is false
03-17 16:13:38.881  2227  2227 D TextView: visible is system.message.count gt 0
03-17 16:13:38.882  2227  2227 D TextView: mVisiblity.getValue is false
03-17 16:13:38.887  2227  2227 D TextView: visible is system.ownerinfo.show
03-17 16:13:38.888  2227  2227 D TextView: mVisiblity.getValue is false
03-17 16:13:38.905  1702 10454 D PowerManagerService: release:lock=233570404, flg=0x0, tag="View Lock", name=com.android.systemui", ws=null, uid=10037, pid=2227
03-17 16:13:38.907  1702 10454 D PowerManagerService: ready=true,policy=3,wakefulness=1,wksummary=0x23,uasummary=0x1,bootcompleted=true,boostinprogress=false,waitmodeenable=false,mode=false,manual=38,auto=-1,adj=0.0userI...
03-17 16:13:38.915  1702  3693 V WindowManager: Skipping AppWindowToken{df0798e token=Token{78af589 ActivityRecord{3b04890 u0 com.tencent.qt.qtl/com.tencent.video.player.activity.PlayerActivity t761}}} -- going to hide
03-17 16:13:38.928  2227  2227 I StackScrollAlgorithm: updateClipping isOverlap:false, getTopPadding=333.0, Translation=-24.0
03-17 16:13:38.928  2227  2227 I StackScrollAlgorithm: updateDimmedActivatedHideSensitive overlap:false
03-17 16:13:38.935  1702  3697 W ActivityManager: getRunningAppProcesses: caller 10113 does not hold REAL_GET_TASKS; limiting output
03-17 16:13:38.936  1702 14638 D PowerManagerService: release:lock=189667585, flg=0x0, tag="*launch*", name=android", ws=WorkSource{10113}, uid=1000, pid=1702
03-17 16:13:38.938  1702 14638 D PowerManagerService: ready=true,policy=3,wakefulness=1,wksummary=0x23,uasummary=0x1,bootcompleted=true,boostinprogress=false,waitmodeenable=false,mode=false,manual=38,auto=-1,adj=0.0userI...
03-17 16:13:38.954  2227  2227 I PhoneStatusBar: setSystemUiVisibility vis=40000500 mask=ffffffff oldVal=508 newVal=40000500 diff=40000008 fullscreenStackVis=0 dockedStackVis=0, fullscreenStackBounds=Rect(0, 0 - 720, 128...
03-17 16:13:38.955  2227  2227 I PhoneStatusBar: cancelAutohide
03-17 16:13:38.955  2227  2227 I PhoneStatusBar: notifyUiVisibilityChanged:vis=0x40000500, SystemUiVisibility=0x40000500
03-17 16:13:38.994  1702 27365 I WindowManager: Destroying surface Surface(name=SurfaceView - com.tencent.qt.qtl/com.tencent.video.player.activity.PlayerActivity) called by com.android.server.wm.WindowStateAnimator.destr...
03-17 16:13:39.006  1702  2639 I WindowManager: Destroying surface Surface(name=com.tencent.qt.qtl/com.tencent.video.player.activity.PlayerActivity) called by com.android.server.wm.WindowStateAnimator.destroySurface:2060...
03-17 16:13:39.010  1702  2639 D PowerManagerService: release:lock=62617001, flg=0x0, tag="WindowManager", name=android", ws=WorkSource{10113}, uid=1000, pid=1702
03-17 16:13:39.011  1702  2639 D PowerManagerService: userActivityNoUpdateLocked: eventTime=261843648, event=0, flags=0x1, uid=1000
03-17 16:13:39.011  1702  2639 D PowerManagerService: ready=true,policy=3,wakefulness=1,wksummary=0x1,uasummary=0x1,bootcompleted=true,boostinprogress=false,waitmodeenable=false,mode=false,manual=38,auto=-1,adj=0.0userId...
03-17 16:13:39.069  1702  1815 I WindowManager: orientation change is complete, call stopFreezingDisplayLocked
03-17 16:13:39.070  1702  1815 I WindowManager: Screen frozen for +1s0ms due to Window{ca98d5 u0 com.tencent.qt.qtl/com.tencent.qt.qtl.activity.info.NewsDetailXmlActivity}
03-17 16:13:39.070  1702  1815 D WindowManager: startAnimation begin
03-17 16:13:39.079  1702  1815 D WindowManager: startAnimation end
03-17 16:13:39.080  1702  1815 D PowerManagerService: release:lock=226887582, flg=0x0, tag="SCREEN_FROZEN", name=android", ws=null, uid=1000, pid=1702
03-17 16:13:39.080  1702  1815 D PowerManagerService: ready=true,policy=3,wakefulness=1,wksummary=0x1,uasummary=0x1,bootcompleted=true,boostinprogress=false,waitmodeenable=false,mode=false,manual=38,auto=-1,adj=0.0userId...

```

Output excerpt:

```text
03-17 16:13:38.811  1702  2395 D WindowManager: printFreezingDisplayLogsopening app wtoken = AppWindowToken{9f4ef63 token=Token{a64f992 ActivityRecord{de9231d u0 com.tencent.qt.qtl/.activity.info.NewsDetailXmlActivity t7...
03-17 16:13:38.819  1702  8671 D PowerManagerService: acquire lock=233570404, flags=0x1, tag="View Lock", name=com.android.systemui, ws=null, uid=10037, pid=2227
03-17 16:13:38.820  1702  8671 D PowerManagerService: ready=true,policy=3,wakefulness=1,wksummary=0x23,uasummary=0x1,bootcompleted=true,boostinprogress=false,waitmodeenable=false,mode=false,manual=38,auto=-1,adj=0.0userI...
03-17 16:13:38.839  1702  2113 V WindowManager: Skipping AppWindowToken{df0798e token=Token{78af589 ActivityRecord{3b04890 u0 com.tencent.qt.qtl/com.tencent.video.player.activity.PlayerActivity t761}}} -- going to hide
03-17 16:13:38.859  2227  2227 D TextView: visible is system.time.showampm
03-17 16:13:38.861  2227  2227 D TextView: mVisiblity.getValue is false
03-17 16:13:38.869  2227  2227 D TextView: visible is system.charge.show
03-17 16:13:38.871  2227  2227 D TextView: mVisiblity.getValue is false
03-17 16:13:38.875  2227  2227 D TextView: visible is system.call.count gt 0
03-17 16:13:38.877  2227  2227 D TextView: mVisiblity.getValue is false
03-17 16:13:38.881  2227  2227 D TextView: visible is system.message.count gt 0
03-17 16:13:38.882  2227  2227 D TextView: mVisiblity.getValue is false
03-17 16:13:38.887  2227  2227 D TextView: visible is system.ownerinfo.show
03-17 16:13:38.888  2227  2227 D TextView: mVisiblity.getValue is false
03-17 16:13:38.905  1702 10454 D PowerManagerService: release:lock=233570404, flg=0x0, tag="View Lock", name=com.android.systemui", ws=null, uid=10037, pid=2227
03-17 16:13:38.907  1702 10454 D PowerManagerService: ready=true,policy=3,wakefulness=1,wksummary=0x23,uasummary=0x1,bootcompleted=true,boostinprogress=false,waitmodeenable=false,mode=false,manual=38,auto=-1,adj=0.0userI...
03-17 16:13:38.915  1702  3693 V WindowManager: Skipping AppWindowToken{df0798e token=Token{78af589 ActivityRecord{3b04890 u0 com.tencent.qt.qtl/com.tencent.video.player.activity.PlayerActivity t761}}} -- going to hide
03-17 16:13:38.928  2227  2227 I StackScrollAlgorithm: updateClipping isOverlap:false, getTopPadding=333.0, Translation=-24.0
03-17 16:13:38.928  2227  2227 I StackScrollAlgorithm: updateDimmedActivatedHideSensitive overlap:false
03-17 16:13:38.935  1702  3697 W ActivityManager: getRunningAppProcesses: caller 10113 does not hold REAL_GET_TASKS; limiting output
03-17 16:13:38.936  1702 14638 D PowerManagerService: release:lock=189667585, flg=0x0, tag="*launch*", name=android", ws=WorkSource{10113}, uid=1000, pid=1702
03-17 16:13:38.938  1702 14638 D PowerManagerService: ready=true,policy=3,wakefulness=1,wksummary=0x23,uasummary=0x1,bootcompleted=true,boostinprogress=false,waitmodeenable=false,mode=false,manual=38,auto=-1,adj=0.0userI...
03-17 16:13:38.954  2227  2227 I PhoneStatusBar: setSystemUiVisibility vis=40000500 mask=ffffffff oldVal=508 newVal=40000500 diff=40000008 fullscreenStackVis=0 dockedStackVis=0, fullscreenStackBounds=Rect(0, 0 - 720, 128...
03-17 16:13:38.955  2227  2227 I PhoneStatusBar: cancelAutohide
03-17 16:13:38.955  2227  2227 I PhoneStatusBar: notifyUiVisibilityChanged:vis=0x40000500, SystemUiVisibility=0x40000500
03-17 16:13:38.994  1702 27365 I WindowManager: Destroying surface Surface(name=SurfaceView - com.tencent.qt.qtl/com.tencent.video.player.activity.PlayerActivity) called by com.android.server.wm.WindowStateAnimator.destr...
03-17 16:13:39.006  1702  2639 I WindowManager: Destroying surface Surface(name=com.tencent.qt.qtl/com.tencent.video.player.activity.PlayerActivity) called by com.android.server.wm.WindowStateAnimator.destroySurface:2060...
03-17 16:13:39.010  1702  2639 D PowerManagerService: release:lock=62617001, flg=0x0, tag="WindowManager", name=android", ws=WorkSource{10113}, uid=1000, pid=1702
03-17 16:13:39.011  1702  2639 D PowerManagerService: userActivityNoUpdateLocked: eventTime=261843648, event=0, flags=0x1, uid=1000
03-17 16:13:39.011  1702  2639 D PowerManagerService: ready=true,policy=3,wakefulness=1,wksummary=0x1,uasummary=0x1,bootcompleted=true,boostinprogress=false,waitmodeenable=false,mode=false,manual=38,auto=-1,adj=0.0userId...
03-17 16:13:39.069  1702  1815 I WindowManager: orientation change is complete, call stopFreezingDisplayLocked
03-17 16:13:39.070  1702  1815 I WindowManager: Screen frozen for +1s0ms due to Window{ca98d5 u0 com.tencent.qt.qtl/com.tencent.qt.qtl.activity.info.NewsDetailXmlActivity}
03-17 16:13:39.070  1702  1815 D WindowManager: startAnimation begin
03-17 16:13:39.079  1702  1815 D WindowManager: startAnimation end
03-17 16:13:39.080  1702  1815 D PowerManagerService: release:lock=226887582, flg=0x0, tag="SCREEN_FROZEN", name=android", ws=null, uid=1000, pid=1702
03-17 16:13:39.080  1702  1815 D PowerManagerService: ready=true,policy=3,wakefulness=1,wksummary=0x1,uasummary=0x1,bootcompleted=true,boostinprogress=false,waitmodeenable=false,mode=false,manual=38,auto=-1,adj=0.0userId...

```

### `03-spark`

- [Full input](cases/03-spark/input.log)
- [Output with CCR](cases/03-spark/output.log) - [diff](cases/03-spark/compression.diff)
- [Output without CCR](cases/03-spark/output-noccr.log) - [diff](cases/03-spark/compression-noccr.diff)

Input excerpt:

```text
17/06/09 20:10:40 INFO executor.CoarseGrainedExecutorBackend: Registered signal handlers for [TERM, HUP, INT]
17/06/09 20:10:40 INFO spark.SecurityManager: Changing view acls to: yarn,curi
17/06/09 20:10:40 INFO spark.SecurityManager: Changing modify acls to: yarn,curi
17/06/09 20:10:40 INFO spark.SecurityManager: SecurityManager: authentication disabled; ui acls disabled; users with view permissions: Set(yarn, curi); users with modify permissions: Set(yarn, curi)
17/06/09 20:10:41 INFO spark.SecurityManager: Changing view acls to: yarn,curi
17/06/09 20:10:41 INFO spark.SecurityManager: Changing modify acls to: yarn,curi
17/06/09 20:10:41 INFO spark.SecurityManager: SecurityManager: authentication disabled; ui acls disabled; users with view permissions: Set(yarn, curi); users with modify permissions: Set(yarn, curi)
17/06/09 20:10:41 INFO slf4j.Slf4jLogger: Slf4jLogger started
17/06/09 20:10:41 INFO Remoting: Starting remoting
17/06/09 20:10:41 INFO Remoting: Remoting started; listening on addresses :[akka.tcp://sparkExecutorActorSystem@mesos-slave-07:55904]
17/06/09 20:10:41 INFO util.Utils: Successfully started service 'sparkExecutorActorSystem' on port 55904.
17/06/09 20:10:41 INFO storage.DiskBlockManager: Created local directory at /opt/hdfs/nodemanager/usercache/curi/appcache/application_1485248649253_0147/blockmgr-70293f72-844a-4b39-9ad6-fb0ad7e364e4
17/06/09 20:10:41 INFO storage.MemoryStore: MemoryStore started with capacity 17.7 GB
17/06/09 20:10:42 INFO executor.CoarseGrainedExecutorBackend: Connecting to driver: spark://CoarseGrainedScheduler@10.10.34.11:48069
17/06/09 20:10:42 INFO executor.CoarseGrainedExecutorBackend: Successfully registered with driver
17/06/09 20:10:42 INFO executor.Executor: Starting executor ID 5 on host mesos-slave-07
17/06/09 20:10:42 INFO util.Utils: Successfully started service 'org.apache.spark.network.netty.NettyBlockTransferService' on port 40984.
17/06/09 20:10:42 INFO netty.NettyBlockTransferService: Server created on 40984
17/06/09 20:10:42 INFO storage.BlockManagerMaster: Trying to register BlockManager
17/06/09 20:10:42 INFO storage.BlockManagerMaster: Registered BlockManager
17/06/09 20:10:45 INFO executor.CoarseGrainedExecutorBackend: Got assigned task 0
17/06/09 20:10:45 INFO executor.CoarseGrainedExecutorBackend: Got assigned task 1
17/06/09 20:10:45 INFO executor.CoarseGrainedExecutorBackend: Got assigned task 2
17/06/09 20:10:45 INFO executor.CoarseGrainedExecutorBackend: Got assigned task 3
17/06/09 20:10:45 INFO executor.Executor: Running task 0.0 in stage 0.0 (TID 0)
17/06/09 20:10:45 INFO executor.Executor: Running task 2.0 in stage 0.0 (TID 2)
17/06/09 20:10:45 INFO executor.Executor: Running task 1.0 in stage 0.0 (TID 1)
17/06/09 20:10:45 INFO executor.Executor: Running task 3.0 in stage 0.0 (TID 3)
17/06/09 20:10:45 INFO executor.CoarseGrainedExecutorBackend: Got assigned task 4
17/06/09 20:10:45 INFO executor.Executor: Running task 4.0 in stage 0.0 (TID 4)
17/06/09 20:10:45 INFO broadcast.TorrentBroadcast: Started reading broadcast variable 9
17/06/09 20:10:45 INFO storage.MemoryStore: Block broadcast_9_piece0 stored as bytes in memory (estimated size 5.2 KB, free 5.2 KB)
17/06/09 20:10:45 INFO broadcast.TorrentBroadcast: Reading broadcast variable 9 took 160 ms
17/06/09 20:10:46 INFO storage.MemoryStore: Block broadcast_9 stored as values in memory (estimated size 8.8 KB, free 14.0 KB)
17/06/09 20:10:46 INFO spark.CacheManager: Partition rdd_2_1 not found, computing it
17/06/09 20:10:46 INFO spark.CacheManager: Partition rdd_2_3 not found, computing it

```

Output excerpt:

```text
17/06/09 20:10:40 INFO executor.CoarseGrainedExecutorBackend: Registered signal handlers for [TERM, HUP, INT]
17/06/09 20:10:40 INFO spark.SecurityManager: Changing view acls to: yarn,curi
17/06/09 20:10:40 INFO spark.SecurityManager: Changing modify acls to: yarn,curi
17/06/09 20:10:40 INFO spark.SecurityManager: SecurityManager: authentication disabled; ui acls disabled; users with view permissions: Set(yarn, curi); users with modify permissions: Set(yarn, curi)
17/06/09 20:10:41 INFO spark.SecurityManager: Changing view acls to: yarn,curi
17/06/09 20:10:41 INFO spark.SecurityManager: Changing modify acls to: yarn,curi
17/06/09 20:10:41 INFO spark.SecurityManager: SecurityManager: authentication disabled; ui acls disabled; users with view permissions: Set(yarn, curi); users with modify permissions: Set(yarn, curi)
17/06/09 20:10:41 INFO slf4j.Slf4jLogger: Slf4jLogger started
17/06/09 20:10:41 INFO Remoting: Starting remoting
17/06/09 20:10:41 INFO Remoting: Remoting started; listening on addresses :[akka.tcp://sparkExecutorActorSystem@mesos-slave-07:55904]
17/06/09 20:10:41 INFO util.Utils: Successfully started service 'sparkExecutorActorSystem' on port 55904.
17/06/09 20:10:41 INFO storage.DiskBlockManager: Created local directory at /opt/hdfs/nodemanager/usercache/curi/appcache/application_1485248649253_0147/blockmgr-70293f72-844a-4b39-9ad6-fb0ad7e364e4
17/06/09 20:10:41 INFO storage.MemoryStore: MemoryStore started with capacity 17.7 GB
17/06/09 20:10:42 INFO executor.CoarseGrainedExecutorBackend: Connecting to driver: spark://CoarseGrainedScheduler@10.10.34.11:48069
17/06/09 20:10:42 INFO executor.CoarseGrainedExecutorBackend: Successfully registered with driver
17/06/09 20:10:42 INFO executor.Executor: Starting executor ID 5 on host mesos-slave-07
17/06/09 20:10:42 INFO util.Utils: Successfully started service 'org.apache.spark.network.netty.NettyBlockTransferService' on port 40984.
17/06/09 20:10:42 INFO netty.NettyBlockTransferService: Server created on 40984
17/06/09 20:10:42 INFO storage.BlockManagerMaster: Trying to register BlockManager
17/06/09 20:10:42 INFO storage.BlockManagerMaster: Registered BlockManager
17/06/09 20:10:45 INFO executor.CoarseGrainedExecutorBackend: Got assigned task 0
17/06/09 20:10:45 INFO executor.CoarseGrainedExecutorBackend: Got assigned task 1
17/06/09 20:10:45 INFO executor.CoarseGrainedExecutorBackend: Got assigned task 2
17/06/09 20:10:45 INFO executor.CoarseGrainedExecutorBackend: Got assigned task 3
17/06/09 20:10:45 INFO executor.Executor: Running task 0.0 in stage 0.0 (TID 0)
17/06/09 20:10:45 INFO executor.Executor: Running task 2.0 in stage 0.0 (TID 2)
17/06/09 20:10:45 INFO executor.Executor: Running task 1.0 in stage 0.0 (TID 1)
17/06/09 20:10:45 INFO executor.Executor: Running task 3.0 in stage 0.0 (TID 3)
17/06/09 20:10:45 INFO executor.CoarseGrainedExecutorBackend: Got assigned task 4
17/06/09 20:10:45 INFO executor.Executor: Running task 4.0 in stage 0.0 (TID 4)
17/06/09 20:10:45 INFO broadcast.TorrentBroadcast: Started reading broadcast variable 9
17/06/09 20:10:45 INFO storage.MemoryStore: Block broadcast_9_piece0 stored as bytes in memory (estimated size 5.2 KB, free 5.2 KB)
17/06/09 20:10:45 INFO broadcast.TorrentBroadcast: Reading broadcast variable 9 took 160 ms
17/06/09 20:10:46 INFO storage.MemoryStore: Block broadcast_9 stored as values in memory (estimated size 8.8 KB, free 14.0 KB)
17/06/09 20:10:46 INFO spark.CacheManager: Partition rdd_2_1 not found, computing it
17/06/09 20:10:46 INFO spark.CacheManager: Partition rdd_2_3 not found, computing it

```

