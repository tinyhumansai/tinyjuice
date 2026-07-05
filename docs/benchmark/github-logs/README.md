# GitHub Log Files

Real log files from public GitHub repositories: the loghub corpus (HDFS, Hadoop, Spark, Zookeeper, BGL, HPC, Thunderbird, Windows, Linux, Android, HealthApp, Apache, Proxifier, OpenSSH, OpenStack, macOS), Elastic example datasets (Apache/nginx access logs, auth.log), and CrowdSec parser test fixtures (WAF, Traefik, Authelia, GitLab, Suricata). Sources and licenses in ATTRIBUTION.md.

Each row links to the full raw input and the exact compacted output used by the benchmark. Percentages are **token reduction: higher is better**; 0% means pass-through. `Algorithm` is the compressor-only reduction. `Pass 1` disables CCR (compressed with omission markers, no recovery footer). `Pass 2` is the final model-facing result with CCR enabled — it reads marginally *lower* than Pass 1 only because the recovery footer adds a few dozen bytes to the output.

## Cases

Every case links to the raw input, the exact model-facing output (with the CCR recovery footer), and a unified diff between the two.

| Case | Input | Output (after CCR) | Diff | Original | Algorithm | Pass 1: no CCR | Pass 2: with CCR | Avg latency | CCR |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| `34-jvm-gc` | [input](cases/34-jvm-gc/input.log) | [output](cases/34-jvm-gc/output.log) | [diff](cases/34-jvm-gc/compression.diff) | 235.6 KB | 99.7% | 99.8% | 99.6% | 1.240 ms | true |
| `17-apache-access` | [input](cases/17-apache-access/input.log) | [output](cases/17-apache-access/output.log) | [diff](cases/17-apache-access/compression.diff) | 578.0 KB | 99.7% | 99.7% | 99.6% | 1.646 ms | true |
| `10-android` | [input](cases/10-android/input.log) | [output](cases/10-android/output.log) | [diff](cases/10-android/compression.diff) | 279.1 KB | 99.6% | 99.8% | 99.6% | 0.882 ms | true |
| `03-spark` | [input](cases/03-spark/input.log) | [output](cases/03-spark/output.log) | [diff](cases/03-spark/compression.diff) | 196.3 KB | 99.6% | 99.7% | 99.5% | 0.512 ms | true |
| `11-healthapp` | [input](cases/11-healthapp/input.log) | [output](cases/11-healthapp/output.log) | [diff](cases/11-healthapp/compression.diff) | 187.5 KB | 99.3% | 99.6% | 99.2% | 0.673 ms | true |
| `15-openstack` | [input](cases/15-openstack/input.log) | [output](cases/15-openstack/output.log) | [diff](cases/15-openstack/compression.diff) | 595.1 KB | 99.3% | 99.4% | 99.3% | 1.571 ms | true |
| `01-hdfs` | [input](cases/01-hdfs/input.log) | [output](cases/01-hdfs/output.log) | [diff](cases/01-hdfs/compression.diff) | 287.8 KB | 99.3% | 99.4% | 99.2% | 0.917 ms | true |
| `04-zookeeper` | [input](cases/04-zookeeper/input.log) | [output](cases/04-zookeeper/output.log) | [diff](cases/04-zookeeper/compression.diff) | 279.9 KB | 99.2% | 99.4% | 99.1% | 0.794 ms | true |
| `12-apache-error` | [input](cases/12-apache-error/input.log) | [output](cases/12-apache-error/output.log) | [diff](cases/12-apache-error/compression.diff) | 171.2 KB | 99.1% | 99.4% | 99.0% | 0.543 ms | true |
| `06-hpc` | [input](cases/06-hpc/input.log) | [output](cases/06-hpc/output.log) | [diff](cases/06-hpc/compression.diff) | 151.2 KB | 98.8% | 99.1% | 98.7% | 0.581 ms | true |
| `13-proxifier` | [input](cases/13-proxifier/input.log) | [output](cases/13-proxifier/output.log) | [diff](cases/13-proxifier/compression.diff) | 237.0 KB | 97.1% | 97.3% | 97.0% | 0.789 ms | true |
| `30-laravel-app` | [input](cases/30-laravel-app/input.log) | [output](cases/30-laravel-app/output.log) | [diff](cases/30-laravel-app/compression.diff) | 104.5 KB | 97.1% | 97.6% | 96.8% | 0.501 ms | true |
| `09-linux` | [input](cases/09-linux/input.log) | [output](cases/09-linux/output.log) | [diff](cases/09-linux/compression.diff) | 216.5 KB | 96.9% | 97.1% | 96.8% | 0.662 ms | true |
| `23-authelia-bf` | [input](cases/23-authelia-bf/input.log) | [output](cases/23-authelia-bf/output.log) | [diff](cases/23-authelia-bf/compression.diff) | 82.7 KB | 96.8% | 97.0% | 96.6% | 0.116 ms | true |
| `05-bgl` | [input](cases/05-bgl/input.log) | [output](cases/05-bgl/output.log) | [diff](cases/05-bgl/compression.diff) | 317.1 KB | 96.5% | 96.9% | 96.4% | 0.757 ms | true |
| `20-caddy-coraza-waf` | [input](cases/20-caddy-coraza-waf/input.log) | [output](cases/20-caddy-coraza-waf/output.log) | [diff](cases/20-caddy-coraza-waf/compression.diff) | 427.0 KB | 96.4% | 96.4% | 96.3% | 0.642 ms | true |
| `19-auth-log` | [input](cases/19-auth-log/input.log) | [output](cases/19-auth-log/output.log) | [diff](cases/19-auth-log/compression.diff) | 282.2 KB | 95.6% | 96.5% | 95.5% | 0.947 ms | true |
| `08-windows` | [input](cases/08-windows/input.log) | [output](cases/08-windows/output.log) | [diff](cases/08-windows/compression.diff) | 285.4 KB | 93.1% | 94.4% | 93.0% | 0.926 ms | true |
| `02-hadoop` | [input](cases/02-hadoop/input.log) | [output](cases/02-hadoop/output.log) | [diff](cases/02-hadoop/compression.diff) | 384.9 KB | 92.9% | 93.9% | 92.8% | 1.077 ms | true |
| `14-openssh` | [input](cases/14-openssh/input.log) | [output](cases/14-openssh/output.log) | [diff](cases/14-openssh/compression.diff) | 225.2 KB | 92.6% | 94.2% | 92.5% | 0.702 ms | true |
| `16-mac` | [input](cases/16-mac/input.log) | [output](cases/16-mac/output.log) | [diff](cases/16-mac/compression.diff) | 319.4 KB | 91.3% | 92.3% | 91.3% | 0.986 ms | true |
| `07-thunderbird` | [input](cases/07-thunderbird/input.log) | [output](cases/07-thunderbird/output.log) | [diff](cases/07-thunderbird/compression.diff) | 325.2 KB | 91.1% | 91.3% | 91.0% | 0.942 ms | true |
| `33-postfix-mail` | [input](cases/33-postfix-mail/input.log) | [output](cases/33-postfix-mail/output.log) | [diff](cases/33-postfix-mail/compression.diff) | 16.3 KB | 74.6% | 80.1% | 73.2% | 0.074 ms | true |
| `29-spark-eventlog` | [input](cases/29-spark-eventlog/input.log) | [output](cases/29-spark-eventlog/output.log) | [diff](cases/29-spark-eventlog/compression.diff) | 412.7 KB | 34.1% | 34.2% | 34.0% | 0.679 ms | true |
| `32-w3c-iis` | [input](cases/32-w3c-iis/input.log) | [output](cases/32-w3c-iis/output.log) | [diff](cases/32-w3c-iis/compression.diff) | 47.4 KB | 0.0% | 0.0% | 0.0% | 0.091 ms | n/a |
| `31-zeek-http` | [input](cases/31-zeek-http/input.log) | [output](cases/31-zeek-http/output.log) | [diff](cases/31-zeek-http/compression.diff) | 71.2 KB | 0.0% | 0.0% | 0.0% | 0.116 ms | n/a |
| `27-suricata-eve` | [input](cases/27-suricata-eve/input.log) | [output](cases/27-suricata-eve/output.log) | [diff](cases/27-suricata-eve/compression.diff) | 19.4 KB | 0.0% | 0.0% | 0.0% | 0.002 ms | n/a |
| `26-gitlab-bf` | [input](cases/26-gitlab-bf/input.log) | [output](cases/26-gitlab-bf/output.log) | [diff](cases/26-gitlab-bf/compression.diff) | 40.7 KB | 0.0% | 0.0% | 0.0% | 0.004 ms | n/a |
| `25-sshesame-honeypot` | [input](cases/25-sshesame-honeypot/input.log) | [output](cases/25-sshesame-honeypot/output.log) | [diff](cases/25-sshesame-honeypot/compression.diff) | 42.1 KB | 0.0% | 0.0% | 0.0% | 0.103 ms | n/a |
| `24-http-dos` | [input](cases/24-http-dos/input.log) | [output](cases/24-http-dos/output.log) | [diff](cases/24-http-dos/compression.diff) | 73.0 KB | 0.0% | 0.0% | 0.0% | 0.125 ms | n/a |
| `22-traefik-http` | [input](cases/22-traefik-http/input.log) | [output](cases/22-traefik-http/output.log) | [diff](cases/22-traefik-http/compression.diff) | 116.4 KB | 0.0% | 0.0% | 0.0% | 0.011 ms | n/a |
| `21-traefik-flood` | [input](cases/21-traefik-flood/input.log) | [output](cases/21-traefik-flood/output.log) | [diff](cases/21-traefik-flood/compression.diff) | 133.6 KB | 0.0% | 0.0% | 0.0% | 0.259 ms | n/a |
| `18-nginx-access` | [input](cases/18-nginx-access/input.log) | [output](cases/18-nginx-access/output.log) | [diff](cases/18-nginx-access/compression.diff) | 337.5 KB | 0.0% | 0.0% | 0.0% | 0.696 ms | n/a |

## What TinyJuice Is Doing

The signal-based log path keeps errors, warnings, stack frames, and summaries and collapses the rest behind per-gap retrieval tokens. Logs with no failure signal (pure access logs) are deliberately passed through rather than blindly truncated.

## Syntax-Aware Samples

### `34-jvm-gc`

- [Full input](cases/34-jvm-gc/input.log)
- [Full output](cases/34-jvm-gc/output.log)
- [Input vs output diff](cases/34-jvm-gc/compression.diff)

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
[... 762 line(s) omitted ... ⟦tj:0946aaca4ddab79ce8e7587155f72bdd⟧]
15.987: [GC concurrent-mark-abort]
[... 314 line(s) omitted ... ⟦tj:a34d19015aff28fdf790be816dbb563f⟧]
22.980: [GC concurrent-mark-abort]
[... 106 line(s) omitted ... ⟦tj:198f2f38810f015e82aaf3ba417fb61f⟧]
25.792: [GC concurrent-mark-abort]
[... 538 line(s) omitted ... ⟦tj:a5aae823de5d33c1662f0aeecf1816f3⟧]
37.369: [GC concurrent-mark-abort]
[... 1245 line(s) omitted ... ⟦tj:e5e909f8b7961a42689b0fb4cbab9db2⟧]
77.417: [GC concurrent-mark-abort]
[... 1491 line(s) omitted ... ⟦tj:816bc2d8766343c9211a87ddeb9096d6⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (235561 bytes) is available by calling tinyjuice_retrieve with token "d8744977a631986bd19845f464437fa3" (marker ⟦tj:d8744977a631986bd19845f464437fa3�...

```

### `10-android`

- [Full input](cases/10-android/input.log)
- [Full output](cases/10-android/output.log)
- [Input vs output diff](cases/10-android/compression.diff)

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
[... 90 line(s) omitted ... ⟦tj:e6a134371088b452b7d09ee85c9237fb⟧]
03-17 16:13:45.466  1702 17632 W ActivityManager: java.lang.ClassCastException: android.os.BinderProxy cannot be cast to com.android.server.am.ActivityRecord$Token
[... 109 line(s) omitted ... ⟦tj:4d403f145498d58d3b3530e7fe02e426⟧]
03-17 16:13:46.765  2227  2794 W KeyguardUpdateMonitor: android.util.AndroidRuntimeException: Must execute in UI
[... 2 line(s) omitted ... ⟦tj:8b148736f578461fdccfa2cca5eb645d⟧]
03-17 16:13:46.765  2227  2794 W KeyguardUpdateMonitor: android.util.AndroidRuntimeException: Must execute in UI
[... 140 line(s) omitted ... ⟦tj:72b9f6d7a68cb15ecc72b0a0afc1420f⟧]
03-17 16:13:47.743  2227  2794 W KeyguardUpdateMonitor: android.util.AndroidRuntimeException: Must execute in UI
[... 1655 line(s) omitted ... ⟦tj:0b5c3e82b025cc2af22e2144a468559f⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (279076 bytes) is available by calling tinyjuice_retrieve with token "47641549915e662ff590291df266a45f" (marker ⟦tj:47641549915e662ff590291df266a45f�...

```

### `17-apache-access`

- [Full input](cases/17-apache-access/input.log)
- [Full output](cases/17-apache-access/output.log)
- [Input vs output diff](cases/17-apache-access/compression.diff)

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
[... 550 line(s) omitted ... ⟦tj:8401add0c62700e49243079c3751dab7⟧]
65.55.213.74 - - [17/May/2015:15:05:32 +0000] "GET /blog/tags/installer%20failure HTTP/1.1" 200 8948 "-" "msnbot/2.0b (+http://search.msn.com/msnbot.htm)"
[... 1220 line(s) omitted ... ⟦tj:faabb95d2537d23043c71ae604c03013⟧]
66.249.73.185 - - [18/May/2015:01:05:47 +0000] "GET /files/pam_logfailure/ HTTP/1.1" 200 992 "-" "SAMSUNG-SGH-E250/1.0 Profile/MIDP-2.0 Configuration/CLDC-1.1 UP.Browser/6.2.3.3.c.1.101 (GUI) MMP/2.0 (compatible; Googleb...
[... 7 line(s) omitted ... ⟦tj:24d9484f78eb23f2eb3f12f1355e9aba⟧]
66.249.73.135 - - [18/May/2015:01:05:29 +0000] "GET /blog/tags/installer%20failure HTTP/1.1" 200 8948 "-" "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)"
[... 516 line(s) omitted ... ⟦tj:b78f68d47a94fe65934cdecb8579d1b7⟧]
66.249.73.185 - - [18/May/2015:05:05:35 +0000] "GET /files/pam_logfailure/ HTTP/1.1" 200 992 "-" "DoCoMo/2.0 N905i(c100;TB;W24H16) (compatible; Googlebot-Mobile/2.1; +http://www.google.com/bot.html)"
66.249.73.135 - - [18/May/2015:05:05:49 +0000] "GET /blog/geekery/ie-javascript-error.html HTTP/1.1" 200 8600 "-" "Mozilla/5.0 (iPhone; CPU iPhone OS 6_0 like Mac OS X) AppleWebKit/536.26 (KHTML, like Gecko) Version/6.0 ...
[... 10 line(s) omitted ... ⟦tj:2f6ddb8744f58a8e2ac964488dee4dc1⟧]
66.249.73.135 - - [18/May/2015:05:05:08 +0000] "GET /blog/geekery/vim-function-to-make-errors-readable.html HTTP/1.1" 200 10260 "-" "Mozilla/5.0 (iPhone; CPU iPhone OS 6_0 like Mac OS X) AppleWebKit/536.26 (KHTML, like G...
[... 191 line(s) omitted ... ⟦tj:b494b8bb717ec2cadee491ad3239cc8a⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (577981 bytes) is available by calling tinyjuice_retrieve with token "4bdb559eee69e9cd66b1271b4079d3bf" (marker ⟦tj:4bdb559eee69e9cd66b1271b4079d3bf�...

```

### `03-spark`

- [Full input](cases/03-spark/input.log)
- [Full output](cases/03-spark/output.log)
- [Input vs output diff](cases/03-spark/compression.diff)

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
[... 48 line(s) omitted ... ⟦tj:ad33c588dcc862c168be3fd2818e4068⟧]
17/06/09 20:10:47 INFO Configuration.deprecation: mapred.tip.id is deprecated. Instead, use mapreduce.task.id
17/06/09 20:10:47 INFO Configuration.deprecation: mapred.task.id is deprecated. Instead, use mapreduce.task.attempt.id
17/06/09 20:10:47 INFO Configuration.deprecation: mapred.task.is.map is deprecated. Instead, use mapreduce.task.ismap
17/06/09 20:10:47 INFO Configuration.deprecation: mapred.job.id is deprecated. Instead, use mapreduce.job.id
17/06/09 20:10:47 INFO Configuration.deprecation: mapred.task.partition is deprecated. Instead, use mapreduce.task.partition
[... 1947 line(s) omitted ... ⟦tj:667883944011647dc0f89a063f0b3056⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (196268 bytes) is available by calling tinyjuice_retrieve with token "2e8b9a37fc5c238253e0b8e18a8bd5e4" (marker ⟦tj:2e8b9a37fc5c238253e0b8e18a8bd5e4�...

```

### `11-healthapp`

- [Full input](cases/11-healthapp/input.log)
- [Full output](cases/11-healthapp/output.log)
- [Input vs output diff](cases/11-healthapp/compression.diff)

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
[... 729 line(s) omitted ... ⟦tj:9c68d125e0b984e1eeae9df06e55c4fc⟧]
20171223-22:19:58:411|HiH_HiHealthBinder|30002312|insertHiHealthData() bulkSaveDetailHiHealthData fail errorCode = 4,errorMessage = ERR_DATA_INSERT 
[... 2 line(s) omitted ... ⟦tj:b0dcaa3e496c7ca21648be452bc7ed05⟧]
20171223-22:19:58:411|Step_LSC|30002312|uploadStaticsToDB failed message=true
[... 21 line(s) omitted ... ⟦tj:14eda5086a471bac4b68de94e1216a04⟧]
20171223-22:19:58:428|HiH_HiBroadcastUtil|30002312|sendSyncFailedBroadcast
[... 39 line(s) omitted ... ⟦tj:d3e700c135d7de6df7cf3d27caed940a⟧]
20171223-22:19:58:518|HiH_HiBroadcastUtil|30002312|sendSyncFailedBroadcast
[... 219 line(s) omitted ... ⟦tj:10f0ca145a14bd6cf18455ee0d1de70e⟧]
20171223-22:32:28:801|HiH_HiBroadcastUtil|30002312|sendSyncFailedBroadcast
[... 529 line(s) omitted ... ⟦tj:08c91842b3c0c199f30f7407e05ed7ba⟧]
20171223-23:32:28:796|HiH_HiBroadcastUtil|30002312|sendSyncFailedBroadcast
[... 437 line(s) omitted ... ⟦tj:2631f8f5d1ef0ad76e1b515a7febce47⟧]
20171224-0:32:28:806|HiH_HiBroadcastUtil|30002312|sendSyncFailedBroadcast
[... 17 line(s) omitted ... ⟦tj:d5951eaee28d71190672d1526a6945bd⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (187456 bytes) is available by calling tinyjuice_retrieve with token "95ec36322f5db1e6faaab764c568b670" (marker ⟦tj:95ec36322f5db1e6faaab764c568b670�...

```

### `01-hdfs`

- [Full input](cases/01-hdfs/input.log)
- [Full output](cases/01-hdfs/output.log)
- [Input vs output diff](cases/01-hdfs/compression.diff)

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
081109 214043 2561 WARN dfs.DataNode$DataXceiver: 10.251.30.85:50010:Got exception while serving blk_-2918118818249673980 to /10.251.90.64:
081109 214402 2677 WARN dfs.DataNode$DataXceiver: 10.251.126.255:50010:Got exception while serving blk_8376667364205250596 to /10.251.91.159:
[... 1 line(s) omitted ... ⟦tj:aa768ccd6f8649887c8dd6998cf6e023⟧]
081109 214529 2747 WARN dfs.DataNode$DataXceiver: 10.251.123.132:50010:Got exception while serving blk_3763728533434719668 to /10.251.38.214:
081109 214910 2848 WARN dfs.DataNode$DataXceiver: 10.250.13.188:50010:Got exception while serving blk_6241141267506413726 to /10.251.194.245:
[... 1 line(s) omitted ... ⟦tj:fe001978517c233f627f8a3cd467b277⟧]
081109 215136 2868 WARN dfs.DataNode$DataXceiver: 10.251.199.19:50010:Got exception while serving blk_8466246428293623262 to /10.251.106.37:
[... 1029 line(s) omitted ... ⟦tj:0aed3df73a59f6b6a76c7a9120b7eee8⟧]
081110 233954 17191 WARN dfs.DataNode$DataXceiver: 10.250.7.230:50010:Got exception while serving blk_-7029628814943626474 to /10.251.38.197:
[... 5 line(s) omitted ... ⟦tj:93c51bed5081231c46db6585ffabfbcb⟧]
081111 004102 17259 WARN dfs.DataNode$DataXceiver: 10.251.214.175:50010:Got exception while serving blk_481857539063371482 to /10.251.105.189:
[... 1 line(s) omitted ... ⟦tj:2dd4005418c9288af48a818ce485c499⟧]
081111 011254 17716 WARN dfs.DataNode$DataXceiver: 10.251.39.144:50010:Got exception while serving blk_-8083036675630459841 to /10.251.39.209:
081111 012254 17517 WARN dfs.DataNode$DataXceiver: 10.250.7.32:50010:Got exception while serving blk_-1508527605812345693 to /10.251.74.192:
[... 3 line(s) omitted ... ⟦tj:f14bc5473560a3661cf95d0229c5ce62⟧]
081111 014431 17416 WARN dfs.DataNode$DataXceiver: 10.251.107.98:50010:Got exception while serving blk_-3140031507252212554 to /10.250.7.244:
[... 873 line(s) omitted ... ⟦tj:73c6b3b2dda0627c72e61000739b4afb⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (287848 bytes) is available by calling tinyjuice_retrieve with token "7c967000980c086ed55fa6544ba4f05f" (marker ⟦tj:7c967000980c086ed55fa6544ba4f05f�...

```

### `15-openstack`

- [Full input](cases/15-openstack/input.log)
- [Full output](cases/15-openstack/output.log)
- [Input vs output diff](cases/15-openstack/compression.diff)

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
[... 2 line(s) omitted ... ⟦tj:5cc774c3e1cd3e19efae1d95b087d147⟧]
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:00:21.067 25746 INFO nova.api.openstack.wsgi [req-0b851395-2895-44b9-8265-a27d0bb52910 f7b8d1f1d4d44643b07fa10ca7d021fb e9746973ac574c6b8a9e8857f56a7608 - - -] HTTP except...
[... 90 line(s) omitted ... ⟦tj:722ba84ad96053f7b225144c009ae8fe⟧]
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:01:03.114 25746 INFO nova.api.openstack.wsgi [req-fff6fe1a-cbb6-4b38-806a-afee069d7c13 f7b8d1f1d4d44643b07fa10ca7d021fb e9746973ac574c6b8a9e8857f56a7608 - - -] HTTP except...
[... 84 line(s) omitted ... ⟦tj:135d15768d6b1bd81443061aa1a97e68⟧]
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:01:43.495 25746 INFO nova.api.openstack.wsgi [req-a0893d5a-dc60-4c49-82e5-b9f7dfc2f6ab f7b8d1f1d4d44643b07fa10ca7d021fb e9746973ac574c6b8a9e8857f56a7608 - - -] HTTP except...
[... 93 line(s) omitted ... ⟦tj:da3406d8e839f26fc729a6395a7b6665⟧]
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:02:25.212 25746 INFO nova.api.openstack.wsgi [req-3fa8a45e-031e-4f3b-b327-b9cd4210f3ba f7b8d1f1d4d44643b07fa10ca7d021fb e9746973ac574c6b8a9e8857f56a7608 - - -] HTTP except...
[... 92 line(s) omitted ... ⟦tj:fffac6dfc9dd3c0da727b63bf775cceb⟧]
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:03:07.021 25746 INFO nova.api.openstack.wsgi [req-c4d0c20c-cfe8-4e66-b280-b083419d4967 f7b8d1f1d4d44643b07fa10ca7d021fb e9746973ac574c6b8a9e8857f56a7608 - - -] HTTP except...
[... 873 line(s) omitted ... ⟦tj:4ec584a785a17eeb64617d4cfb03799b⟧]
nova-compute.log.1.2017-05-16_13:55:31 2017-05-16 00:09:41.850 2931 WARNING nova.compute.manager [req-addc1839-2ed5-4778-b57e-5854eb7b8b09 - - - - -] While synchronizing instance power states, found 1 instances in the da...
[... 232 line(s) omitted ... ⟦tj:f09b1e0a9894a467d9cf2108491a64ae⟧]
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:11:23.292 25746 INFO nova.api.openstack.wsgi [req-033d97b9-69e4-4acd-9029-f0d7b9370645 f7b8d1f1d4d44643b07fa10ca7d021fb e9746973ac574c6b8a9e8857f56a7608 - - -] HTTP except...
[... 104 line(s) omitted ... ⟦tj:1db2d9641d38a6ef7f6e41adde1c6b08⟧]
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:12:05.112 25746 INFO nova.api.openstack.wsgi [req-d6e9cfb8-d914-48c3-b677-72bc73329c69 f7b8d1f1d4d44643b07fa10ca7d021fb e9746973ac574c6b8a9e8857f56a7608 - - -] HTTP except...
[... 93 line(s) omitted ... ⟦tj:e119580769fb5fc6c7defbb923665317⟧]
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:12:45.461 25746 INFO nova.api.openstack.wsgi [req-ab2e766f-eee9-4e84-8171-8ba713a28f9f f7b8d1f1d4d44643b07fa10ca7d021fb e9746973ac574c6b8a9e8857f56a7608 - - -] HTTP except...
[... 90 line(s) omitted ... ⟦tj:643aeef1b0fe17af4b08230ed036e191⟧]
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:13:27.251 25746 INFO nova.api.openstack.wsgi [req-4beefba4-a928-45a5-90f6-6246e77bc2ce f7b8d1f1d4d44643b07fa10ca7d021fb e9746973ac574c6b8a9e8857f56a7608 - - -] HTTP except...
[... 87 line(s) omitted ... ⟦tj:599b4202e2aefdf20037592948af523b⟧]
nova-api.log.1.2017-05-16_13:53:08 2017-05-16 00:14:09.186 25746 INFO nova.api.openstack.wsgi [req-8a5b19ff-20d8-40e7-94d3-29b89f9b6987 f7b8d1f1d4d44643b07fa10ca7d021fb e9746973ac574c6b8a9e8857f56a7608 - - -] HTTP except...
[... 92 line(s) omitted ... ⟦tj:e1875ed5f9c34a9a287c8857d828d203⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (595119 bytes) is available by calling tinyjuice_retrieve with token "025a1bc64ff5b2ef4a4bda6c4ad5c5c5" (marker ⟦tj:025a1bc64ff5b2ef4a4bda6c4ad5c5c5�...

```

### `04-zookeeper`

- [Full input](cases/04-zookeeper/input.log)
- [Full output](cases/04-zookeeper/output.log)
- [Input vs output diff](cases/04-zookeeper/compression.diff)

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
[... 5 line(s) omitted ... ⟦tj:c872a0c89d808f2a493b48f15a630652⟧]
2015-07-29 19:13:24,282 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@762] - Connection broken for id 188978561024, my id = 1, error = 
[... 1 line(s) omitted ... ⟦tj:76ee3eb69ea90ef0ffcbb7b52ea80681⟧]
2015-07-29 19:13:27,721 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@762] - Connection broken for id 188978561024, my id = 1, error = 
[... 3 line(s) omitted ... ⟦tj:3d0ac8fc734768fe574aa8bfea3cb303⟧]
2015-07-29 19:13:47,731 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@762] - Connection broken for id 188978561024, my id = 1, error = 
[... 1 line(s) omitted ... ⟦tj:4c2e9d9e8bf48166e6c1bee77cb4278a⟧]
2015-07-29 19:13:54,399 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@762] - Connection broken for id 188978561024, my id = 1, error = 
[... 17 line(s) omitted ... ⟦tj:b82106d73c48db7a2873ed46b0c4b391⟧]
2015-07-29 19:16:24,348 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@762] - Connection broken for id 188978561024, my id = 1, error = 
[... 1914 line(s) omitted ... ⟦tj:053df1b92e5a52bdc7ea1df770848377⟧]
2015-07-30 16:21:13,810 - WARN  [NIOServerCxn.Factory:0.0.0.0/0.0.0.0:2181:NIOServerCnxn@349] - caught end of stream exception
[... 7 line(s) omitted ... ⟦tj:35355ddc4a2ec3c549cb682237b34750⟧]
2015-07-30 17:36:52,812 - WARN  [NIOServerCxn.Factory:0.0.0.0/0.0.0.0:2181:NIOServerCnxn@349] - caught end of stream exception
2015-07-30 17:40:45,765 - WARN  [RecvWorker:1:QuorumCnxManager$RecvWorker@762] - Connection broken for id 1, my id = 3, error = 
[... 17 line(s) omitted ... ⟦tj:c89b56349786a3ff416f3f552ba5a4b3⟧]
2015-07-30 23:03:42,330 - WARN  [NIOServerCxn.Factory:0.0.0.0/0.0.0.0:2181:NIOServerCnxn@349] - caught end of stream exception
[... 7 line(s) omitted ... ⟦tj:4abb0e6b27de556dfaac3ab8a4255127⟧]
2015-07-31 00:16:12,152 - WARN  [NIOServerCxn.Factory:0.0.0.0/0.0.0.0:2181:NIOServerCnxn@349] - caught end of stream exception
[... 18 line(s) omitted ... ⟦tj:308905979430fc8694348571832e2a63⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (279891 bytes) is available by calling tinyjuice_retrieve with token "e40e0af5ef9eb6e4097200f260b9d1f6" (marker ⟦tj:e40e0af5ef9eb6e4097200f260b9d1f6�...

```

### `12-apache-error`

- [Full input](cases/12-apache-error/input.log)
- [Full output](cases/12-apache-error/output.log)
- [Input vs output diff](cases/12-apache-error/compression.diff)

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
[... 1 line(s) omitted ... ⟦tj:5a68782b23ba362d588b43fec8fd90ab⟧]
[Sun Dec 04 04:47:44 2005] [error] mod_jk child workerEnv in error state 6
[... 6 line(s) omitted ... ⟦tj:13b7297934b4beb67c8b648d95bf094d⟧]
[Sun Dec 04 04:51:18 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:51:18 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:51:18 2005] [error] mod_jk child workerEnv in error state 6
[... 5 line(s) omitted ... ⟦tj:88d86852a47e586fec32405fc9478cb9⟧]
[Sun Dec 04 04:51:55 2005] [error] mod_jk child workerEnv in error state 6
[... 1971 line(s) omitted ... ⟦tj:a316609f77c9ff718ada11c8b3923bf3⟧]
[Mon Dec 05 19:00:56 2005] [error] [client 68.228.3.15] Directory index forbidden by rule: /var/www/html/
[... 2 line(s) omitted ... ⟦tj:01ac9d0a4564900b79e264664ae932b2⟧]
[Mon Dec 05 19:11:04 2005] [error] mod_jk child workerEnv in error state 6
[... 1 line(s) omitted ... ⟦tj:9433d6c97e90b9f60c823c88bf2ef870⟧]
[Mon Dec 05 19:14:09 2005] [error] [client 61.220.139.68] Directory index forbidden by rule: /var/www/html/
[... 1 line(s) omitted ... ⟦tj:67a7a206dfe64028b4563c30769649d2⟧]
[Mon Dec 05 19:14:11 2005] [error] mod_jk child workerEnv in error state 6
[... 3 line(s) omitted ... ⟦tj:0acde7f1f4c2653e6c9889d0f30bd586⟧]
[Mon Dec 05 19:15:57 2005] [error] mod_jk child workerEnv in error state 6

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (171239 bytes) is available by calling tinyjuice_retrieve with token "c7efa3eb686e3a96bd2f8f4457b2a788" (marker ⟦tj:c7efa3eb686e3a96bd2f8f4457b2a788�...

```

### `06-hpc`

- [Full input](cases/06-hpc/input.log)
- [Full output](cases/06-hpc/output.log)
- [Input vs output diff](cases/06-hpc/compression.diff)

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
[... 172 line(s) omitted ... ⟦tj:8da68e68f0ce5433b4c0eb9752adcda9⟧]
51338 node-3 node psu 1106496000 1 psu failure\ ambient=28
191898 node-238 node psu 1131240275 1 psu failure\ ambient=28
236618 node-104 node psu 1132434391 1 psu failure\ ambient=28
341834 node-118 node psu 1140312091 1 psu failure\ ambient=28
347972 node-118 node psu 1140430530 1 psu failure\ ambient=31
147394 Interconnect-0N00 switch_module temphigh 1129812510 1 Temperature (41C) exceeds warning threshold
[... 33 line(s) omitted ... ⟦tj:748e84d28b4a40c3f2fa4baa274beee7⟧]
365140 node-69 unix.hw net.niff.down 1085075228 1 NIFF: node node-69 detected a failed network connection on network 5.5.224.0 via interface alt0
401608 node-162 unix.hw net.niff.down 1142550442 1 NIFF: node node-162 detected a failed network connection on network 5.5.224.0 via interface alt0
[... 499 line(s) omitted ... ⟦tj:4b688617163ceea8210bc9abc82a0ed0⟧]
2556843 gige5 gige temperature 1072878243 1 warning
[... 1127 line(s) omitted ... ⟦tj:c0c73b5ac05403c614131673b1d3dd16⟧]
72725 3398 boot_cmd error 1108648264 1 Failed subcommands 3406
[... 63 line(s) omitted ... ⟦tj:82917ae7aefd6e5e2dba49bd0985003a⟧]
414569 Interconnect-1N02 switch_module error 1142843014 1 link errors remain current
414493 Interconnect-1N02 switch_module error 1142837578 1 link errors remain current
414390 Interconnect-1N00 switch_module error 1142825359 1 link errors remain current
443580 Interconnect-1T01 switch_module error 1144142493 1 link errors remain current
479886 Interconnect-1T02 switch_module error 1146011009 1 link errors remain current
[... 91 line(s) omitted ... ⟦tj:360e4f8780a8f84fe8d37b7ac183e794⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (151178 bytes) is available by calling tinyjuice_retrieve with token "826e5957b461e65780a8bda5c186c2fc" (marker ⟦tj:826e5957b461e65780a8bda5c186c2fc�...

```

### `30-laravel-app`

- [Full input](cases/30-laravel-app/input.log)
- [Full output](cases/30-laravel-app/output.log)
- [Input vs output diff](cases/30-laravel-app/compression.diff)

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
[... 15 line(s) omitted ... ⟦tj:f069b65eaba8d04b3f722df60a85f427⟧]
[previous exception] [object] (ReflectionException(code: -1): Class \"Fruitcake\\Cors\\HandleCors\" does not exist at /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Container/Container.php:91...
[... 16 line(s) omitted ... ⟦tj:aee74597e4def51014cf4aeba6c2c2da⟧]
[2023-05-15 18:19:33] local.ERROR: Target class [Fruitcake\Cors\HandleCors] does not exist. {"exception":"[object] (Illuminate\\Contracts\\Container\\BindingResolutionException(code: 0): Target class [Fruitcake\\Cors\\Ha...
[... 11 line(s) omitted ... ⟦tj:0d383449e69e8da97a4a44b5ca2acd6d⟧]
[previous exception] [object] (ReflectionException(code: -1): Class \"Fruitcake\\Cors\\HandleCors\" does not exist at /Users/zhebaoting/Sites/laravel/crm/vendor/laravel/framework/src/Illuminate/Container/Container.php:91...
[... 12 line(s) omitted ... ⟦tj:ffd2902c99e67f158241f214bda645f6⟧]
[2023-05-15 18:19:49] local.ERROR: Uncaught ReflectionException: Class "App\Providers\App\Policies\CustomerPolicy" does not exist in Command line code:1
[... 1315 line(s) omitted ... ⟦tj:13d2badc2e579b9e842ce29cd8655e86⟧]
  thrown {"exception":"[object] (Symfony\\Component\\ErrorHandler\\Error\\FatalError(code: 0): Uncaught ReflectionException: Class \"App\\Providers\\App\\Policies\\CustomerPolicy\" does not exist in Command line code:1
[... 9 line(s) omitted ... ⟦tj:ff7a4298d055dad0727deaec53929bc3⟧]
[2023-05-15 20:01:50] local.ERROR: Uncaught ReflectionException: Class "App\Providers\App\Policies\CustomerPolicy" does not exist in Command line code:1
[... 5 line(s) omitted ... ⟦tj:91b74a9854daf1c77d1c508e4deb84e1⟧]
  thrown {"exception":"[object] (Symfony\\Component\\ErrorHandler\\Error\\FatalError(code: 0): Uncaught ReflectionException: Class \"App\\Providers\\App\\Policies\\CustomerPolicy\" does not exist in Command line code:1
[... 9 line(s) omitted ... ⟦tj:ff7a4298d055dad0727deaec53929bc3⟧]
[2023-05-15 20:02:50] local.ERROR: Uncaught ReflectionException: Class "App\Providers\App\Policies\CustomerPolicy" does not exist in Command line code:1
[... 5 line(s) omitted ... ⟦tj:91b74a9854daf1c77d1c508e4deb84e1⟧]
  thrown {"exception":"[object] (Symfony\\Component\\ErrorHandler\\Error\\FatalError(code: 0): Uncaught ReflectionException: Class \"App\\Providers\\App\\Policies\\CustomerPolicy\" does not exist in Command line code:1
[... 9 line(s) omitted ... ⟦tj:ff7a4298d055dad0727deaec53929bc3⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (104500 bytes) is available by calling tinyjuice_retrieve with token "873b367bf87a3e07b2e64482312beb8b" (marker ⟦tj:873b367bf87a3e07b2e64482312beb8b�...

```

### `13-proxifier`

- [Full input](cases/13-proxifier/input.log)
- [Full output](cases/13-proxifier/output.log)
- [Input vs output diff](cases/13-proxifier/compression.diff)

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
[... 252 line(s) omitted ... ⟦tj:9a59553424d0eeb8db0fb08be810ac22⟧]
[10.30 17:15:42] QQ.exe - tcpconn6.tencent.com:443 error : A connection request was canceled before the completion.
[10.30 17:16:10] QQ.exe - tcpconn4.tencent.com:80 error : Could not connect through proxy proxy.cse.cuhk.edu.hk:5070 - Proxy closed the connection unexpectedly. 
[... 173 line(s) omitted ... ⟦tj:cf1f44176398766e921f131cd9e5b270⟧]
[10.30 17:57:12] YodaoDict.exe - oimagec7.ydstatic.com:80 error : A connection request was canceled before the completion. 
[... 13 line(s) omitted ... ⟦tj:9ebb8258b4ee5462ed6e9837c9ae39ff⟧]
[10.30 18:00:12] FlashPlayerPlugin_18_0_0_209.exe - formi.baidu.com:843 error : Could not connect through proxy proxy.cse.cuhk.edu.hk:5070 - Proxy server cannot establish a connection with the target, status code 403
[... 39 line(s) omitted ... ⟦tj:c438ec65a72d1bdd37627a0deae3eb61⟧]
[10.30 18:04:14] Skype.exe - 86.99.222.235:443 error : Could not connect through proxy proxy.cse.cuhk.edu.hk:5070 - Proxy closed the connection unexpectedly.
[... 1317 line(s) omitted ... ⟦tj:f4dae9bcb3f17ec82c87c45a280c8310⟧]
[07.27 05:05:50] chrome.exe *64 - qa.sockets.stackexchange.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:05:51] chrome.exe *64 - 13.client-channel.google.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:05:52] chrome.exe *64 - 13.client-channel.google.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:05:52] chrome.exe *64 - 13.client-channel.google.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:05:53] chrome.exe *64 - www.google.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:05:53] Dropbox.exe - client-cf.dropbox.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:05:54] chrome.exe *64 - qa.sockets.stackexchange.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:05:54] chrome.exe *64 - 16.client-channel.google.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:05:56] Dropbox.exe - bolt.dropbox.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:06:01] Dropbox.exe - bolt.dropbox.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:06:08] chrome.exe *64 - www.google.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:06:09] Dropbox.exe - bolt.dropbox.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:06:13] chrome.exe *64 - 16.client-channel.google.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:06:15] chrome.exe *64 - www.google.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:06:22] chrome.exe *64 - mtalk.google.com:5228 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:06:32] chrome.exe *64 - www.google.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:06:40] Dropbox.exe - client-lb.dropbox.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:07:02] Dropbox.exe - bolt.dropbox.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:07:06] chrome.exe *64 - 16.client-channel.google.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:08:27] Dropbox.exe - bolt.dropbox.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:08:29] Dropbox.exe - bolt.dropbox.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:09:52] Dropbox.exe - bolt.dropbox.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:10:59] Dropbox.exe - bolt.dropbox.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:11:29] Dropbox.exe - bolt.dropbox.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:11:34] chrome.exe *64 - 16.client-channel.google.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061
[07.27 05:12:56] Dropbox.exe - bolt.dropbox.com:443 error : Could not connect to proxy proxy.cse.cuhk.edu.hk:5070 - connection attempt failed with error 10061

```

### `09-linux`

- [Full input](cases/09-linux/input.log)
- [Full output](cases/09-linux/output.log)
- [Input vs output diff](cases/09-linux/compression.diff)

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
[... 1 line(s) omitted ... ⟦tj:ec82dddff7e657485704bf88ff382470⟧]
Jun 14 15:16:02 combo sshd(pam_unix)[19937]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=218.188.2.4 
Jun 15 02:04:59 combo sshd(pam_unix)[20882]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20884]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20883]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
[... 532 line(s) omitted ... ⟦tj:993d266eb971e6bf2df3861ba2a0a71c⟧]
Jun 30 20:53:04 combo klogind[19272]: Authentication failed from 163.27.187.39 (163.27.187.39): Permission denied in replay cache code 
Jun 30 20:53:04 combo klogind[19272]: Kerberos authentication failed 
Jun 30 20:53:04 combo klogind[19287]: Authentication failed from 163.27.187.39 (163.27.187.39): Permission denied in replay cache code 
Jun 30 20:53:04 combo klogind[19287]: Kerberos authentication failed 
Jun 30 20:53:04 combo klogind[19286]: Authentication failed from 163.27.187.39 (163.27.187.39): Permission denied in replay cache code 
Jun 30 20:53:04 combo klogind[19286]: Kerberos authentication failed 
Jun 30 20:53:04 combo klogind[19271]: Authentication failed from 163.27.187.39 (163.27.187.39): Permission denied in replay cache code 
Jun 30 20:53:04 combo klogind[19271]: Kerberos authentication failed 
Jun 30 20:53:04 combo klogind[19270]: Authentication failed from 163.27.187.39 (163.27.187.39): Permission denied in replay cache code 
Jun 30 20:53:04 combo klogind[19270]: Kerberos authentication failed 
Jun 30 20:53:04 combo klogind[19269]: Authentication failed from 163.27.187.39 (163.27.187.39): Permission denied in replay cache code 
Jun 30 20:53:04 combo klogind[19269]: Kerberos authentication failed 
Jun 30 20:53:04 combo klogind[19268]: Authentication failed from 163.27.187.39 (163.27.187.39): Permission denied in replay cache code 
Jun 30 20:53:04 combo klogind[19288]: Authentication failed from 163.27.187.39 (163.27.187.39): Permission denied in replay cache code 
Jun 30 20:53:04 combo klogind[19274]: Authentication failed from 163.27.187.39 (163.27.187.39): Software caused connection abort 
Jun 30 20:53:04 combo klogind[19268]: Kerberos authentication failed 
Jun 30 20:53:04 combo klogind[19288]: Kerberos authentication failed 
Jun 30 20:53:04 combo klogind[19274]: Kerberos authentication failed 
Jun 30 20:53:06 combo klogind[19266]: Authentication failed from 163.27.187.39 (163.27.187.39): Software caused connection abort 
Jun 30 20:53:06 combo klogind[19266]: Kerberos authentication failed 
Jun 30 20:53:06 combo klogind[19267]: Authentication failed from 163.27.187.39 (163.27.187.39): Software caused connection abort 
Jun 30 20:53:06 combo klogind[19267]: Kerberos authentication failed 
Jun 30 20:53:06 combo klogind[19278]: Authentication failed from 163.27.187.39 (163.27.187.39): Software caused connection abort 
Jun 30 20:53:06 combo klogind[19278]: Kerberos authentication failed 
Jun 30 20:53:06 combo klogind[19273]: Authentication failed from 163.27.187.39 (163.27.187.39): Software caused connection abort 
Jun 30 20:53:06 combo klogind[19273]: Kerberos authentication failed 
Jun 30 20:53:06 combo klogind[19276]: Authentication failed from 163.27.187.39 (163.27.187.39): Software caused connection abort 
Jun 30 20:53:06 combo klogind[19276]: Kerberos authentication failed 
Jun 30 20:53:06 combo klogind[19275]: Authentication failed from 163.27.187.39 (163.27.187.39): Software caused connection abort 

```

### `23-authelia-bf`

- [Full input](cases/23-authelia-bf/input.log)
- [Full output](cases/23-authelia-bf/output.log)
- [Input vs output diff](cases/23-authelia-bf/compression.diff)

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
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser1': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.1 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser2': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.1 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser3': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.1 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser4': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.1 stack="longstacktrace"
time="2022-02-14T13:47:54+02:00" level=error msg="Unsuccessful 1FA authentication attempt by user 'fakeuser5': user not found" method=POST path=/api/firstfactor remote_ip=1.1.1.1 stack="longstacktrace"
[... 134 line(s) omitted ... ⟦tj:4ce831f2e8c788c4183d06b461263e6d⟧]
{"error":"duo auth result: deny, status: deny, message: Login request denied.","level":"error","method":"POST","msg":"Unsuccessful Duo authentication attempt by user 'realuser@example.com'","path":"/api/secondfactor/duo"...
{"error":"duo auth result: deny, status: deny, message: Login request denied.","level":"error","method":"POST","msg":"Unsuccessful Duo authentication attempt by user 'realuser@example.com'","path":"/api/secondfactor/duo"...
{"error":"duo auth result: deny, status: deny, message: Login request denied.","level":"error","method":"POST","msg":"Unsuccessful Duo authentication attempt by user 'realuser@example.com'","path":"/api/secondfactor/duo"...
{"error":"duo auth result: deny, status: deny, message: Login request denied.","level":"error","method":"POST","msg":"Unsuccessful Duo authentication attempt by user 'realuser@example.com'","path":"/api/secondfactor/duo"...
{"error":"duo auth result: deny, status: deny, message: Login request denied.","level":"error","method":"POST","msg":"Unsuccessful Duo authentication attempt by user 'realuser@example.com'","path":"/api/secondfactor/duo"...

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (82668 bytes) is available by calling tinyjuice_retrieve with token "fd5b95737b66c5e69f3945b5bebd5f26" (marker ⟦tj:fd5b95737b66c5e69f3945b5bebd5f26�...

```

### `05-bgl`

- [Full input](cases/05-bgl/input.log)
- [Full output](cases/05-bgl/output.log)
- [Input vs output diff](cases/05-bgl/compression.diff)

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
- 1117838570 2005.06.03 R02-M1-N0-C:J12-U11 2005-06-03-15.42.50.675872 R02-M1-N0-C:J12-U11 RAS KERNEL INFO instruction cache parity error corrected
- 1117838573 2005.06.03 R02-M1-N0-C:J12-U11 2005-06-03-15.42.53.276129 R02-M1-N0-C:J12-U11 RAS KERNEL INFO instruction cache parity error corrected
- 1117838976 2005.06.03 R02-M1-N0-C:J12-U11 2005-06-03-15.49.36.156884 R02-M1-N0-C:J12-U11 RAS KERNEL INFO instruction cache parity error corrected
- 1117838978 2005.06.03 R02-M1-N0-C:J12-U11 2005-06-03-15.49.38.026704 R02-M1-N0-C:J12-U11 RAS KERNEL INFO instruction cache parity error corrected
- 1117842440 2005.06.03 R23-M0-NE-C:J05-U01 2005-06-03-16.47.20.730545 R23-M0-NE-C:J05-U01 RAS KERNEL INFO 63543 double-hummer alignment exceptions
[... 3 line(s) omitted ... ⟦tj:3e4d1b2d5f613fccc5bdd7e3f9a6ad45⟧]
APPREAD 1117869872 2005.06.04 R04-M1-N4-I:J18-U11 2005-06-04-00.24.32.432192 R04-M1-N4-I:J18-U11 RAS APP FATAL ciod: failed to read message prefix on control stream (CioStream socket to 172.16.96.116:33569
APPREAD 1117869876 2005.06.04 R27-M1-N4-I:J18-U01 2005-06-04-00.24.36.222560 R27-M1-N4-I:J18-U01 RAS APP FATAL ciod: failed to read message prefix on control stream (CioStream socket to 172.16.96.116:33370
[... 74 line(s) omitted ... ⟦tj:90ca2ff66b849f0a4f32d93975a452b9⟧]
- 1118193358 2005.06.07 R11-M0-NC-I:J18-U01 2005-06-07-18.15.58.583443 R11-M0-NC-I:J18-U01 RAS APP FATAL ciod: LOGIN chdir(/p/gb2/glosli/8M_5000K/t800) failed: No such file or directory
[... 260 line(s) omitted ... ⟦tj:44a0c0d9c8c10affbadc41d1232e75f2⟧]
APPREAD 1118958902 2005.06.16 R32-M1-N8-I:J18-U01 2005-06-16-14.55.02.074340 R32-M1-N8-I:J18-U01 RAS APP FATAL ciod: failed to read message prefix on control stream (CioStream socket to 172.16.96.116:51706
[... 111 line(s) omitted ... ⟦tj:094940c9f1b03d120d4468fd221bb219⟧]
- 1119977619 2005.06.28 R02-M1-NE 2005-06-28-09.53.39.479164 R02-M1-NE NULL DISCOVERY WARNING Node card is not fully functional
[... 162 line(s) omitted ... ⟦tj:0e61b65f92b1213d70326f67d5f738d9⟧]
- 1120889703 2005.07.08 R05-M0-N2 2005-07-08-23.15.03.798449 R05-M0-N2 NULL DISCOVERY WARNING Node card is not fully functional
[... 597 line(s) omitted ... ⟦tj:91a482e4b9d5004021edfe22dccd9b09⟧]
- 1123178302 2005.08.04 R06-M1-ND 2005-08-04-10.58.22.010246 R06-M1-ND NULL DISCOVERY WARNING Node card is not fully functional
[... 4 line(s) omitted ... ⟦tj:80c5fc4420c0b865d403495296124844⟧]
- 1123262593 2005.08.05 R25-M1-N2 2005-08-05-10.23.13.029850 R25-M1-N2 NULL HARDWARE WARNING PrepareForService shutting down NodeCard(mLctn(R25-M1-N2), mCardSernum(203231503833343000000000594c31304b34333431303158), mLp(F...
[... 7 line(s) omitted ... ⟦tj:de4f2051687387c5737fac614d54c296⟧]
- 1123614572 2005.08.09 R00-M0-N6-C:J03-U11 2005-08-09-12.09.32.252222 R00-M0-N6-C:J03-U11 RAS KERNEL FATAL rts tree/torus link training failed: wanted: B C X+ X- Y+ Y- Z+ Z- got: B C X- Y- Z+ Z-
[... 21 line(s) omitted ... ⟦tj:75d7ea035ee89e41b2617392bcbd0501⟧]
- 1123966456 2005.08.13 R27-M1-NC-I:J18-U11 2005-08-13-13.54.16.386775 R27-M1-NC-I:J18-U11 RAS APP FATAL ciod: LOGIN chdir(pwd) failed: No such file or directory
- 1123966497 2005.08.13 R61-M1-NC-I:J18-U01 2005-08-13-13.54.57.564471 R61-M1-NC-I:J18-U01 RAS APP FATAL ciod: LOGIN chdir(pwd) failed: No such file or directory
[... 7 line(s) omitted ... ⟦tj:975776259c0fcd16448fb330dc98994c⟧]
- 1124306491 2005.08.17 R12-M0-N4-I:J18-U01 2005-08-17-12.21.31.579167 R12-M0-N4-I:J18-U01 RAS APP FATAL ciod: LOGIN chdir(/p/bg1/da) failed: No such file or directory
[... 8 line(s) omitted ... ⟦tj:a8b898e4ad0b3626136c63ad4c39d384⟧]
- 1124722015 2005.08.22 R24-M1-N4-I:J18-U11 2005-08-22-07.46.55.044922 R24-M1-N4-I:J18-U11 RAS APP FATAL ciod: LOGIN chdir(/home/germann2/SPaSM_static) failed: No such file or directory
[... 6 line(s) omitted ... ⟦tj:9b5cf90e0cc88ca571ef71e6fa8034c8⟧]
- 1124798068 2005.08.23 R50-M1-N0-I:J18-U11 2005-08-23-04.54.28.532357 R50-M1-N0-I:J18-U11 RAS APP FATAL ciod: LOGIN chdir(/home/germann2/BGL-demo) failed: No such file or directory
[... 91 line(s) omitted ... ⟦tj:76c5b22b5df9c0049d16277978fb81e8⟧]
- 1125225358 2005.08.28 R54-M0-NC-I:J18-U01 2005-08-28-03.35.58.673640 R54-M0-NC-I:J18-U01 RAS KERNEL INFO NFS Mount failed on bglio716, slept 15 seconds, retrying (1)
- 1125225361 2005.08.28 R05-M0-NC-I:J18-U11 2005-08-28-03.36.01.417658 R05-M0-NC-I:J18-U11 RAS KERNEL INFO NFS Mount failed on bglio91, slept 15 seconds, retrying (1)
[... 4 line(s) omitted ... ⟦tj:111b74fc2230bb4cfd83a5bdf668bdae⟧]
KERNMNTF 1125552593 2005.08.31 R42-M1-NC-I:J18-U11 2005-08-31-22.29.53.058583 R42-M1-NC-I:J18-U11 RAS KERNEL FATAL Lustre mount FAILED : bglio559 : point /p/gb1

```

### `19-auth-log`

- [Full input](cases/19-auth-log/input.log)
- [Full output](cases/19-auth-log/output.log)
- [Input vs output diff](cases/19-auth-log/compression.diff)

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
[... 85 line(s) omitted ... ⟦tj:d4dad37d3377a96329fe2dca3bea3d5b⟧]
Mar 27 14:01:39 ip-10-77-20-248 sshd[2938]: error: maximum authentication attempts exceeded for root from 122.176.37.221 port 37107 ssh2 [preauth]
Mar 27 14:01:39 ip-10-77-20-248 sshd[2938]: Disconnecting: Too many authentication failures [preauth]
[... 10 line(s) omitted ... ⟦tj:1e2bd1d8d558dc141ae2d6a6fde96d24⟧]
Mar 27 14:54:58 ip-10-77-20-248 sshd[2967]: error: maximum authentication attempts exceeded for invalid user support from 95.152.57.58 port 53679 ssh2 [preauth]
Mar 27 14:54:58 ip-10-77-20-248 sshd[2967]: Disconnecting: Too many authentication failures [preauth]
[... 3 line(s) omitted ... ⟦tj:c933c7df599f7b4c9b6270aaa30e8bc6⟧]
Mar 27 15:46:53 ip-10-77-20-248 sshd[2996]: error: maximum authentication attempts exceeded for root from 90.144.183.19 port 57648 ssh2 [preauth]
[... 1432 line(s) omitted ... ⟦tj:b3b3239150cf7fe56d0e7e223927b1a5⟧]
Mar 29 10:36:52 ip-10-77-20-248 useradd[852]: failed adding user 'elastic_user_0', data deleted
[... 1 line(s) omitted ... ⟦tj:121f3508915b429b0b93d2762fe10001⟧]
Mar 29 10:36:52 ip-10-77-20-248 useradd[858]: failed adding user 'elastic_user_1', data deleted
[... 1 line(s) omitted ... ⟦tj:530c05ab87b0202cbfd9650c878a880e⟧]
Mar 29 10:36:52 ip-10-77-20-248 useradd[864]: failed adding user 'elastic_user_2', data deleted
[... 1 line(s) omitted ... ⟦tj:1dd2d3698fbd6ca6ddd4a7e84e4f3826⟧]
Mar 29 10:36:52 ip-10-77-20-248 useradd[870]: failed adding user 'elastic_user_3', data deleted
[... 1 line(s) omitted ... ⟦tj:6d48f0d0f4ccae42ff2344fedc48552f⟧]
Mar 29 10:36:52 ip-10-77-20-248 useradd[876]: failed adding user 'elastic_user_4', data deleted
[... 1 line(s) omitted ... ⟦tj:a8ddbff853ae641304738df7f4482f5e⟧]
Mar 29 10:36:52 ip-10-77-20-248 useradd[882]: failed adding user 'elastic_user_5', data deleted
[... 1 line(s) omitted ... ⟦tj:6c4da33c1379722ead4a79d9e1a23741⟧]
Mar 29 10:36:52 ip-10-77-20-248 useradd[888]: failed adding user 'elastic_user_6', data deleted
[... 1 line(s) omitted ... ⟦tj:20c48589eb00c01c033674dd210cb9aa⟧]
Mar 29 10:36:52 ip-10-77-20-248 useradd[894]: failed adding user 'elastic_user_7', data deleted
[... 1 line(s) omitted ... ⟦tj:739cb7e94bc662d23f8f54bc3611b6fe⟧]
Mar 29 10:36:52 ip-10-77-20-248 useradd[900]: failed adding user 'elastic_user_8', data deleted
[... 1 line(s) omitted ... ⟦tj:04195edea25db16e503e561c14599524⟧]
Mar 29 10:36:52 ip-10-77-20-248 useradd[906]: failed adding user 'elastic_user_9', data deleted
[... 4 line(s) omitted ... ⟦tj:cc54bc2cbe1eb0e3e4cf38fb636492da⟧]
Mar 29 10:37:34 ip-10-77-20-248 useradd[916]: failed adding user 'elastic_user_0', data deleted
[... 1 line(s) omitted ... ⟦tj:5bf0c5a8bb8c100b4901f27b92b7a638⟧]
Mar 29 10:37:34 ip-10-77-20-248 useradd[922]: failed adding user 'elastic_user_1', data deleted
[... 1 line(s) omitted ... ⟦tj:30a3afc112002fba8b79d2cebbacffd0⟧]
Mar 29 10:37:34 ip-10-77-20-248 useradd[929]: failed adding user 'elastic_user_2', data deleted
[... 1 line(s) omitted ... ⟦tj:2cf9d72431d850681838fe4b337f2cc8⟧]
Mar 29 10:37:34 ip-10-77-20-248 useradd[935]: failed adding user 'elastic_user_3', data deleted

```

### `20-caddy-coraza-waf`

- [Full input](cases/20-caddy-coraza-waf/input.log)
- [Full output](cases/20-caddy-coraza-waf/output.log)
- [Input vs output diff](cases/20-caddy-coraza-waf/compression.diff)

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
[... 32 line(s) omitted ... ⟦tj:61c3bbe463e13b1f8106d0d5f4b914ed⟧]
{"level":"error","ts":1700845155.7930043,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. HTTP Parameter Pollution (utf-8) [file \"/ruleset/coreruleset/rulesV4RC2/REQUEST-921-PROTOCOL-ATTACK....
{"level":"error","ts":1700845155.8713691,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Inbound Anomaly Score Exceeded (Total Score: 5) [file \"/ruleset/coreruleset/rulesV4RC2/REQUEST-949-B...
{"level":"error","ts":1700845155.8719003,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning.  [file \"/ruleset/coreruleset/rulesV4RC2/RESPONSE-980-CORRELATION.conf\"] [line \"0\"] [id \"980170\"...
[... 1 line(s) omitted ... ⟦tj:fff17de318995867c7ef23acd4d899f1⟧]
{"level":"error","ts":1700845872.3002284,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. HTTP Parameter Pollution (utf-8) [file \"/ruleset/coreruleset/rulesV4RC2/REQUEST-921-PROTOCOL-ATTACK....
{"level":"error","ts":1700845872.3069482,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Remote Command Execution: Unix Shell Code Found in REQUEST_HEADERS [file \"/ruleset/coreruleset/rules...
[... 426 line(s) omitted ... ⟦tj:9e09b6218cc3ceab3d68464275c6ba67⟧]
{"level":"error","ts":1701376729.062896,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Inbound Anomaly Score Exceeded (Total Score: 5) [file \"/ruleset/coreruleset/rulesV4RC2/REQUEST-949-BL...
{"level":"error","ts":1701376729.063486,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning.  [file \"/ruleset/coreruleset/rulesV4RC2/RESPONSE-980-CORRELATION.conf\"] [line \"0\"] [id \"980170\"]...
{"level":"error","ts":1701377065.6526973,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. HTTP Parameter Pollution (utf-8) [file \"/ruleset/coreruleset/rulesV4RC2/REQUEST-921-PROTOCOL-ATTACK....
{"level":"error","ts":1701377065.729784,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning. Inbound Anomaly Score Exceeded (Total Score: 5) [file \"/ruleset/coreruleset/rulesV4RC2/REQUEST-949-BL...
{"level":"error","ts":1701377065.7302442,"logger":"http.handlers.waf","msg":"[client \"192.168.1.1\"] Coraza: Warning.  [file \"/ruleset/coreruleset/rulesV4RC2/RESPONSE-980-CORRELATION.conf\"] [line \"0\"] [id \"980170\"...
[... 1 line(s) omitted ... ⟦tj:918f75d0bb7e7736c74045610315e22e⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (426962 bytes) is available by calling tinyjuice_retrieve with token "898db299de64de3d7facec1a431d44af" (marker ⟦tj:898db299de64de3d7facec1a431d44af�...

```

### `08-windows`

- [Full input](cases/08-windows/input.log)
- [Full output](cases/08-windows/output.log)
- [Input vs output diff](cases/08-windows/compression.diff)

Input excerpt:

```text
2016-09-28 04:30:30, Info                  CBS    Loaded Servicing Stack v6.1.7601.23505 with Core: C:\Windows\winsxs\amd64_microsoft-windows-servicingstack_31bf3856ad364e35_6.1.7601.23505_none_681aa442f6fed7f0\cbscore.d...
2016-09-28 04:30:31, Info                  CSI    00000001@2016/9/27:20:30:31.455 WcpInitialize (wcp.dll version 0.0.0.6) called (stack @0x7fed806eb5d @0x7fef9fb9b6d @0x7fef9f8358f @0xff83e97c @0xff83d799 @0xff83db2f)
2016-09-28 04:30:31, Info                  CSI    00000002@2016/9/27:20:30:31.458 WcpInitialize (wcp.dll version 0.0.0.6) called (stack @0x7fed806eb5d @0x7fefa006ade @0x7fef9fd2984 @0x7fef9f83665 @0xff83e97c @0xff83d799)...
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
[... 10 line(s) omitted ... ⟦tj:2563d608a0a2931b41db40f2d8f716cc⟧]
2016-09-28 04:30:31, Info                  CBS    SQM: Failed to start upload with file pattern: C:\Windows\servicing\sqm\*_std.sqm, flags: 0x2 [HRESULT = 0x80004005 - E_FAIL]
2016-09-28 04:30:31, Info                  CBS    SQM: Failed to start standard sample upload. [HRESULT = 0x80004005 - E_FAIL]
[... 1 line(s) omitted ... ⟦tj:7e93bb2ea82a45dc7c2ab7f7486ab89e⟧]
2016-09-28 04:30:31, Info                  CBS    SQM: Warning: Failed to upload all unsent reports. [HRESULT = 0x80004005 - E_FAIL]
[... 11 line(s) omitted ... ⟦tj:eb1463cb1f6e42b724414b2c9a02ffb5⟧]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.
[... 1 line(s) omitted ... ⟦tj:b746301d1459a5eda579fea46d247ae3⟧]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
[... 2 line(s) omitted ... ⟦tj:474a84323bf5ff2bf7febe65da466c4a⟧]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
[... 2 line(s) omitted ... ⟦tj:474a84323bf5ff2bf7febe65da466c4a⟧]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
[... 2 line(s) omitted ... ⟦tj:474a84323bf5ff2bf7febe65da466c4a⟧]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
[... 3 line(s) omitted ... ⟦tj:d7dbf7c5d52999b51edd437b8f46637e⟧]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
[... 2 line(s) omitted ... ⟦tj:474a84323bf5ff2bf7febe65da466c4a⟧]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
[... 2 line(s) omitted ... ⟦tj:474a84323bf5ff2bf7febe65da466c4a⟧]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
[... 2 line(s) omitted ... ⟦tj:474a84323bf5ff2bf7febe65da466c4a⟧]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
[... 3 line(s) omitted ... ⟦tj:d7dbf7c5d52999b51edd437b8f46637e⟧]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
[... 2 line(s) omitted ... ⟦tj:474a84323bf5ff2bf7febe65da466c4a⟧]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
[... 2 line(s) omitted ... ⟦tj:474a84323bf5ff2bf7febe65da466c4a⟧]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
[... 2 line(s) omitted ... ⟦tj:474a84323bf5ff2bf7febe65da466c4a⟧]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
[... 3 line(s) omitted ... ⟦tj:d7dbf7c5d52999b51edd437b8f46637e⟧]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
[... 2 line(s) omitted ... ⟦tj:474a84323bf5ff2bf7febe65da466c4a⟧]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
[... 2 line(s) omitted ... ⟦tj:474a84323bf5ff2bf7febe65da466c4a⟧]

```

### `14-openssh`

- [Full input](cases/14-openssh/input.log)
- [Full output](cases/14-openssh/output.log)
- [Input vs output diff](cases/14-openssh/compression.diff)

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
[... 3 line(s) omitted ... ⟦tj:c6db44b5673fcbeb531498f017678a33⟧]
Dec 10 06:55:46 LabSZ sshd[24200]: pam_unix(sshd:auth): authentication failure; logname= uid=0 euid=0 tty=ssh ruser= rhost=173.234.31.186 
Dec 10 06:55:48 LabSZ sshd[24200]: Failed password for invalid user webmaster from 173.234.31.186 port 38926 ssh2
[... 5 line(s) omitted ... ⟦tj:b99b6cf498735e88819ff995bc20ee30⟧]
Dec 10 07:07:38 LabSZ sshd[24206]: pam_unix(sshd:auth): authentication failure; logname= uid=0 euid=0 tty=ssh ruser= rhost=ec2-52-80-34-196.cn-north-1.compute.amazonaws.com.cn 
Dec 10 07:07:45 LabSZ sshd[24206]: Failed password for invalid user test9 from 52.80.34.196 port 36060 ssh2
[... 1 line(s) omitted ... ⟦tj:b5e67dcebe6b0e4b306a4890b40bde97⟧]
Dec 10 07:08:28 LabSZ sshd[24208]: reverse mapping checking getaddrinfo for ns.marryaldkfaczcz.com [173.234.31.186] failed - POSSIBLE BREAK-IN ATTEMPT!
[... 4 line(s) omitted ... ⟦tj:ba59e4ce0b45b1e7a2dd40574b67afd6⟧]
Dec 10 07:08:30 LabSZ sshd[24208]: Failed password for invalid user webmaster from 173.234.31.186 port 39257 ssh2
[... 5 line(s) omitted ... ⟦tj:98c4c8ebaccaf80fce4bb83db125b7fb⟧]
Dec 10 07:11:44 LabSZ sshd[24224]: Failed password for invalid user chen from 202.100.179.208 port 32484 ssh2
[... 2 line(s) omitted ... ⟦tj:cfa7faeaa8f5fb9e2b1b3bfaaba956ce⟧]
Dec 10 07:13:43 LabSZ sshd[24227]: Failed password for root from 5.36.59.76 port 42393 ssh2
Dec 10 07:13:56 LabSZ sshd[24227]: message repeated 5 times: [ Failed password for root from 5.36.59.76 port 42393 ssh2]
[... 4 line(s) omitted ... ⟦tj:e23cc67f63c538c2a8499898b90bb3d0⟧]
Dec 10 07:27:52 LabSZ sshd[24235]: Failed password for root from 112.95.230.3 port 45378 ssh2
[... 2 line(s) omitted ... ⟦tj:8381c7e8c8f506e05f65a652ce10a998⟧]
Dec 10 07:27:55 LabSZ sshd[24237]: Failed password for root from 112.95.230.3 port 47068 ssh2
[... 2 line(s) omitted ... ⟦tj:8f935a686680ecb1d402e741f8c23ed9⟧]
Dec 10 07:27:58 LabSZ sshd[24239]: Failed password for root from 112.95.230.3 port 49188 ssh2
[... 2 line(s) omitted ... ⟦tj:8c155bc32370ba3b8eb37f13382a3c21⟧]
Dec 10 07:28:00 LabSZ sshd[24241]: Failed password for root from 112.95.230.3 port 50999 ssh2
[... 2 line(s) omitted ... ⟦tj:0ebbde99fbc5a2dbd711ff6fceb7f26f⟧]
Dec 10 07:28:03 LabSZ sshd[24243]: Failed password for root from 112.95.230.3 port 52660 ssh2
[... 5 line(s) omitted ... ⟦tj:12723f8e09f6672acac2ca8d87e52716⟧]
Dec 10 07:28:05 LabSZ sshd[24245]: Failed password for invalid user pgadmin from 112.95.230.3 port 54087 ssh2
[... 2 line(s) omitted ... ⟦tj:6b84c87fcec5bca06ce7c8dc1b3e6ae4⟧]
Dec 10 07:28:08 LabSZ sshd[24247]: Failed password for root from 112.95.230.3 port 55618 ssh2
[... 2 line(s) omitted ... ⟦tj:9ae143a4d92f37b0bffa14ab300706d6⟧]
Dec 10 07:28:10 LabSZ sshd[24249]: Failed password for root from 112.95.230.3 port 57138 ssh2
[... 2 line(s) omitted ... ⟦tj:48b4ea7bc85c11a707b1d8eeb6ac0df8⟧]
Dec 10 07:28:12 LabSZ sshd[24251]: Failed password for root from 112.95.230.3 port 58304 ssh2
[... 2 line(s) omitted ... ⟦tj:2ea7de9d0cd6e8daf2f8ba7cbe6be36e⟧]
Dec 10 07:28:14 LabSZ sshd[24253]: Failed password for root from 112.95.230.3 port 59849 ssh2

```

### `02-hadoop`

- [Full input](cases/02-hadoop/input.log)
- [Full output](cases/02-hadoop/output.log)
- [Input vs output diff](cases/02-hadoop/compression.diff)

Input excerpt:

```text
2015-10-18 18:01:47,978 INFO [main] org.apache.hadoop.mapreduce.v2.app.MRAppMaster: Created MRAppMaster for application appattempt_1445144423722_0020_000001
2015-10-18 18:01:48,963 INFO [main] org.apache.hadoop.mapreduce.v2.app.MRAppMaster: Executing with tokens:
2015-10-18 18:01:48,963 INFO [main] org.apache.hadoop.mapreduce.v2.app.MRAppMaster: Kind: YARN_AM_RM_TOKEN, Service: , Ident: (appAttemptId { application_id { id: 20 cluster_timestamp: 1445144423722 } attemptId: 1 } keyI...
2015-10-18 18:01:49,228 INFO [main] org.apache.hadoop.mapreduce.v2.app.MRAppMaster: Using mapred newApiCommitter.
2015-10-18 18:01:50,353 INFO [main] org.apache.hadoop.mapreduce.v2.app.MRAppMaster: OutputCommitter set in config null
2015-10-18 18:01:50,509 INFO [main] org.apache.hadoop.mapreduce.v2.app.MRAppMaster: OutputCommitter is org.apache.hadoop.mapreduce.lib.output.FileOutputCommitter
2015-10-18 18:01:50,556 INFO [main] org.apache.hadoop.yarn.event.AsyncDispatcher: Registering class org.apache.hadoop.mapreduce.jobhistory.EventType for class org.apache.hadoop.mapreduce.jobhistory.JobHistoryEventHandler...
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
[... 50 line(s) omitted ... ⟦tj:6b0c1d21b07a4aa8916ae2780ada6b80⟧]
2015-10-18 18:01:53,447 INFO [main] org.apache.hadoop.mapreduce.v2.app.rm.RMContainerRequestor: maxTaskFailuresPerNode is 3
[... 616 line(s) omitted ... ⟦tj:61d496a43b82bb9b202ba6ebb9dbb106⟧]
2015-10-18 18:04:11,034 ERROR [RMCommunicator Allocator] org.apache.hadoop.mapreduce.v2.app.rm.RMContainerAllocator: Container complete event for unknown container id container_1445144423722_0020_01_000012
[... 180 line(s) omitted ... ⟦tj:52f74630b62b27be19b4a36c21e1b8ad⟧]
2015-10-18 18:05:27,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 30 seconds.  Will retry shortly ...
[... 1 line(s) omitted ... ⟦tj:16437f4433b62c519ec5d73313745313⟧]
2015-10-18 18:05:28,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 31 seconds.  Will retry shortly ...
[... 1 line(s) omitted ... ⟦tj:2f7b14ddc75804e887c3c31ec16f6441⟧]
2015-10-18 18:05:29,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 32 seconds.  Will retry shortly ...
[... 1 line(s) omitted ... ⟦tj:fa13ededb8addfcca844028cfb0075a9⟧]
2015-10-18 18:05:30,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 33 seconds.  Will retry shortly ...
[... 1 line(s) omitted ... ⟦tj:6c09f8b6414438dc00ac235cadc678fb⟧]
2015-10-18 18:05:31,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 34 seconds.  Will retry shortly ...
[... 1 line(s) omitted ... ⟦tj:a8e2e29d8b7862b19b62932d743b5ff5⟧]
2015-10-18 18:05:32,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 35 seconds.  Will retry shortly ...
[... 1 line(s) omitted ... ⟦tj:abeaacedfa04715b18130f3ade6a2e48⟧]
2015-10-18 18:05:33,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 36 seconds.  Will retry shortly ...
[... 1 line(s) omitted ... ⟦tj:e74cfda177d85ae45aa1535ebf146960⟧]
2015-10-18 18:05:34,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 37 seconds.  Will retry shortly ...
[... 1 line(s) omitted ... ⟦tj:2046f4c30741a4e9cd2b307aef4c17e6⟧]
2015-10-18 18:05:35,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 38 seconds.  Will retry shortly ...
[... 1 line(s) omitted ... ⟦tj:513ab407e0fc11fe01594e93c0a9ae6b⟧]
2015-10-18 18:05:36,570 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 39 seconds.  Will retry shortly ...
[... 1 line(s) omitted ... ⟦tj:303b2b32564b1371f461a278fa1abe5b⟧]
2015-10-18 18:05:37,617 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 40 seconds.  Will retry shortly ...
[... 1 line(s) omitted ... ⟦tj:52f3f101c8e3778424ecfb7d71df9360⟧]
2015-10-18 18:05:38,617 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 41 seconds.  Will retry shortly ...
[... 1 line(s) omitted ... ⟦tj:0226a630c93a3a09af06dd663629b1b6⟧]
2015-10-18 18:05:39,617 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 42 seconds.  Will retry shortly ...
[... 1 line(s) omitted ... ⟦tj:5660cef33b49a448bb6446f02ce83b49⟧]
2015-10-18 18:05:40,617 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 43 seconds.  Will retry shortly ...
[... 1 line(s) omitted ... ⟦tj:8e393f7f107507edb5dbfb7680b3ee35⟧]
2015-10-18 18:05:41,617 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 44 seconds.  Will retry shortly ...
[... 1 line(s) omitted ... ⟦tj:7e34388abbb19bf4962c7af9bdf47f6e⟧]
2015-10-18 18:05:42,633 WARN [LeaseRenewer:msrabi@msra-sa-41:9000] org.apache.hadoop.hdfs.LeaseRenewer: Failed to renew lease for [DFSClient_NONMAPREDUCE_1537864556_1] for 45 seconds.  Will retry shortly ...

```

### `16-mac`

- [Full input](cases/16-mac/input.log)
- [Full output](cases/16-mac/output.log)
- [Input vs output diff](cases/16-mac/compression.diff)

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
[... 83 line(s) omitted ... ⟦tj:aa91113a3564a6ecd7b821a14d16519d⟧]
Jul  1 10:13:39 calvisitor-10-105-160-95 secd[276]:  SOSAccountThisDeviceCanSyncWithCircle sync with device failure: Error Domain=com.apple.security.sos.error Code=1035 "Account identity not set" UserInfo={NSDescription=...
Jul  1 10:13:43 calvisitor-10-105-160-95 SpotlightNetHelper[352]: CFPasteboardRef CFPasteboardCreate(CFAllocatorRef, CFStringRef) : failed to create global data
[... 7 line(s) omitted ... ⟦tj:f852b4da3514616e851109ba9eadf1b5⟧]
Jul  1 11:24:45 calvisitor-10-105-160-95 secd[276]:  securityd_xpc_dictionary_handler cloudd[326] copy_matching Error Domain=NSOSStatusErrorDomain Code=-50 "query missing class name" (paramErr: error in user parameter li...
[... 16 line(s) omitted ... ⟦tj:80d94dc811edc9cfef58890146c5caef⟧]
Jul  1 11:46:16 calvisitor-10-105-160-95 symptomsd[215]: -[NetworkAnalyticsEngine _writeJournalRecord:fromCellFingerprint:key:atLOI:ofKind:lqm:isFaulty:] Hashing of the primary key failed. Dropping the journal record.
[... 8 line(s) omitted ... ⟦tj:066377b982bbf816b1d9b51a1b4b5102⟧]
Jul  1 11:49:29 calvisitor-10-105-160-95 QQ[10018]: tcp_connection_destination_perform_socket_connect 19110 connectx to 183.57.48.75:80@0 failed: [50] Network is down
[... 5 line(s) omitted ... ⟦tj:04f1e76c4a4597e56666934c8f536474⟧]
Jul  1 11:49:30 authorMacBook-Pro Dropbox[24019]: [0701/114930:WARNING:dns_config_service_posix.cc(306)] Failed to read DnsConfig.
[... 51 line(s) omitted ... ⟦tj:6dfb32e2f8efdc811229d81e7c13aeb1⟧]
Jul  1 15:05:51 calvisitor-10-105-160-95 com.apple.AddressBook.InternetAccountsBridge[31654]: dnssd_clientstub ConnectToServer: connect() failed path:/var/run/mDNSResponder Socket:4 Err:-1 Errno:1 Operation not permitted
[... 6 line(s) omitted ... ⟦tj:c277f765af4cddddde9252f9fad573ae⟧]
Jul  1 19:46:42 calvisitor-10-105-163-202 iconservicesagent[328]: -[ISGenerateImageOp generateImageWithCompletion:] Failed to composit image for descriptor <ISBindingImageDescriptor: 0x7f9950712080>.
Jul  1 19:46:42 calvisitor-10-105-163-202 iconservicesagent[328]: -[ISGenerateImageOp generateImageWithCompletion:] Failed to composit image for descriptor <ISBindingImageDescriptor: 0x7f9951303310>.
[... 2 line(s) omitted ... ⟦tj:3f1686d80b63b7ff4c49bf23521525b5⟧]
Jul  1 20:17:07 calvisitor-10-105-163-202 iconservicesagent[328]: -[ISGenerateImageOp generateImageWithCompletion:] Failed to composit image for descriptor <ISBindingImageDescriptor: 0x7f9951105bf0>.
[... 2 line(s) omitted ... ⟦tj:fa8eef71b88aacee78cd3fc989cc1768⟧]
Jul  1 21:03:00 calvisitor-10-105-163-202 WindowServer[184]: send_datagram_available_ping: pid 445 failed to act on a ping it dequeued before timing out.
Jul  1 21:10:19 calvisitor-10-105-163-202 Preview[11512]: WARNING: Type1 font data isn't in the correct format required by the Adobe Type 1 Font Format specification.
Jul  1 21:17:32 calvisitor-10-105-163-202 WindowServer[184]: send_datagram_available_ping: pid 445 failed to act on a ping it dequeued before timing out.
[... 1 line(s) omitted ... ⟦tj:2170d360d8c22ae959334369627efda2⟧]
Jul  1 21:18:10 calvisitor-10-105-163-202 iconservicesagent[328]: -[ISGenerateImageOp generateImageWithCompletion:] Failed to composit image for descriptor <ISBindingImageDescriptor: 0x7f9950712080>.
[... 1 line(s) omitted ... ⟦tj:ece3433e57f33cc9fbc46a671d3832ac⟧]
Jul  1 21:21:33 calvisitor-10-105-163-202 iconservicesagent[328]: -[ISGenerateImageOp generateImageWithCompletion:] Failed to composit image for descriptor <ISBindingImageDescriptor: 0x7f9950712080>.
Jul  1 21:24:38 calvisitor-10-105-163-202 iconservicesagent[328]: -[ISGenerateImageOp generateImageWithCompletion:] Failed to composit image for descriptor <ISBindingImageDescriptor: 0x7f9951005740>.
Jul  1 21:33:23 calvisitor-10-105-163-202 iconservicesagent[328]: -[ISGenerateImageOp generateImageWithCompletion:] Failed to composit image for descriptor <ISBindingImageDescriptor: 0x7f9950606790>.
[... 2 line(s) omitted ... ⟦tj:6c1b3c1dfc54622872d6c7141af76bd2⟧]
Jul  1 22:12:41 calvisitor-10-105-163-202 iconservicesagent[328]: -[ISGenerateImageOp generateImageWithCompletion:] Failed to composit image for descriptor <ISBindingImageDescriptor: 0x7f9950606790>.
Jul  1 22:13:49 calvisitor-10-105-163-202 WindowServer[184]: send_datagram_available_ping: pid 445 failed to act on a ping it dequeued before timing out.
Jul  1 22:19:34 calvisitor-10-105-163-202 iconservicesagent[328]: -[ISGenerateImageOp generateImageWithCompletion:] Failed to composit image for descriptor <ISBindingImageDescriptor: 0x7f9950606790>.
[... 1 line(s) omitted ... ⟦tj:1db2fb06e8d419703615d582bcd841de⟧]
Jul  1 22:20:57 calvisitor-10-105-163-202 iconservicesagent[328]: -[ISGenerateImageOp generateImageWithCompletion:] Failed to composit image for descriptor <ISBindingImageDescriptor: 0x7f9950712080>.
[... 12 line(s) omitted ... ⟦tj:7b4f902e41e39d57a4c545be71d8e6bd⟧]
Jul  2 02:19:03 calvisitor-10-105-163-202 com.apple.AddressBook.InternetAccountsBridge[31953]: dnssd_clientstub ConnectToServer: connect() failed path:/var/run/mDNSResponder Socket:4 Err:-1 Errno:1 Operation not permitte...

```

### `07-thunderbird`

- [Full input](cases/07-thunderbird/input.log)
- [Full output](cases/07-thunderbird/output.log)
- [Input vs output diff](cases/07-thunderbird/compression.diff)

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
[... 10 line(s) omitted ... ⟦tj:7e82e8e61e97565c9960132543ca114a⟧]
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 ide0: Wait for ready failed before probe !
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 ide1: Wait for ready failed before probe !
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 ide2: Wait for ready failed before probe !
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 ide3: Wait for ready failed before probe !
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 ide4: Wait for ready failed before probe !
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 ide5: Wait for ready failed before probe !
[... 27 line(s) omitted ... ⟦tj:70c33ed26bbbd5686079242dd32057e7⟧]
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 sda: asking for cache data failed
[... 15 line(s) omitted ... ⟦tj:903a7d907af23d227a233dd91160e327⟧]
- 1131567043 2005.11.09 tbird-admin1 Nov 9 12:10:43 local@tbird-admin1 vesafb: probe of vesafb0 failed with error -6
[... 23 line(s) omitted ... ⟦tj:e57177c209824103adf757f151530af8⟧]
- 1131567052 2005.11.09 tbird-admin1 Nov 9 12:10:52 local@tbird-admin1 netfs: Mounting other filesystems: failed
[... 2 line(s) omitted ... ⟦tj:cf5f6c1de6094388e9dedb1140b7444f⟧]
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name an14 an15 an16 an17 an18 an19 an20 an21 an22 an23 an24 an25 an26 an27 an28 an29 an30 an31 an32...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name an14 an15 an16 an17 an18 an19 an20 an21 an22 an23 an24 an25 an26 an27 an28 an29 an30 an31 an32...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name an142 an143 an144 an145 an146 an147 an148 an149 an150 an151 an152 an153 an154 an155 an156 an15...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name an270 an271 an272 an273 an274 an275 an276 an277 an278 an279 an280 an281 an282 an283 an284 an28...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name an398 an399 an400 an401 an402 an403 an404 an405 an406 an407 an408 an409 an410 an411 an412 an41...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name an526 an527 an528 an529 an530 an531 an532 an533 an534 an535 an536 an537 an538 an539 an540 an54...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name an654 an655 an656 an657 an658 an659 an660 an661 an662 an663 an664 an665 an666 an667 an668 an66...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name an782 an783 an784 an785 an786 an787 an788 an789 an790 an791 an792 an793 an794 an795 an796 an79...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name an910 an911 an912 an913 an914 an915 an916 an917 an918 an919 an920 an921 an922 an923 an924 an92...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name bn14 bn15 bn16 bn17 bn18 bn19 bn20 bn21 bn22 bn23 bn24 bn25 bn26 bn27 bn28 bn29 bn30 bn31 bn32...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name bn142 bn143 bn144 bn145 bn146 bn147 bn148 bn149 bn150 bn151 bn152 bn153 bn154 bn155 bn156 bn15...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name bn270 bn271 bn272 bn273 bn274 bn275 bn276 bn277 bn278 bn279 bn280 bn281 bn282 bn283 bn284 bn28...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name bn398 bn399 bn400 bn401 bn402 bn403 bn404 bn405 bn406 bn407 bn408 bn409 bn410 bn411 bn412 bn41...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name bn526 bn527 bn528 bn529 bn530 bn531 bn532 bn533 bn534 bn535 bn536 bn537 bn538 bn539 bn540 bn54...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name bn654 bn655 bn656 bn657 bn658 bn659 bn660 bn661 bn662 bn663 bn664 bn665 bn666 bn667 bn668 bn66...
- 1131567053 2005.11.09 tbird-admin1 Nov 9 12:10:53 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name dadmin2 dadmin3 dadmin4
[... 33 line(s) omitted ... ⟦tj:17d434827e5d3a7a6c05759b443dc64a⟧]
- 1131567054 2005.11.09 tbird-admin1 Nov 9 12:10:54 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name bn782 bn783 bn784 bn785 bn786 bn787 bn788 bn789 bn790 bn791 bn792 bn793 bn794 bn795 bn796 bn79...
- 1131567054 2005.11.09 tbird-admin1 Nov 9 12:10:54 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name bn910 bn911 bn912 bn913 bn914 bn915 bn916 bn917 bn918 bn919 bn920 bn921 bn922 bn923 bn924 bn92...
- 1131567054 2005.11.09 tbird-admin1 Nov 9 12:10:54 local@tbird-admin1 gmetad: Warning: we failed to resolve data source name cn14 cn15 cn16 cn17 cn18 cn19 cn20 cn21 cn22 cn23 cn24 cn25 cn26 cn27 cn28 cn29 cn30 cn31 cn32...

```

### `33-postfix-mail`

- [Full input](cases/33-postfix-mail/input.log)
- [Full output](cases/33-postfix-mail/output.log)
- [Input vs output diff](cases/33-postfix-mail/compression.diff)

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
[... 18 line(s) omitted ... ⟦tj:455b0b00de1f4ce16d011970743de818⟧]
May 19 00:00:30 proxy2 postfix/smtpd[16123]: NOQUEUE: reject: RCPT from example.net[192.0.2.2]: 550 5.1.1 <user1@example.com>: Recipient address rejected: undeliverable address: verification failed; from=<user2@example.o...
[... 49 line(s) omitted ... ⟦tj:e6f8082c656c20812fcf836e13256e0e⟧]
# failJSON: { "time": "2005-06-03T06:25:43", "match": true , "host": "192.0.2.11", "desc": "too many errors / gh-2439" }
Jun  3 06:25:43 srv postfix/smtpd[29306]: too many errors after RCPT from example.com[192.0.2.11]
[... 1 line(s) omitted ... ⟦tj:e3b0c44298fc1c149afbf4c8996fb924⟧]
# filterOptions: [{"mode": "errors"}]
[... 1 line(s) omitted ... ⟦tj:e3b0c44298fc1c149afbf4c8996fb924⟧]
# failJSON: { "match": false, "desc": "ignore normal messages, jail for too many errors only" }
[... 26 line(s) omitted ... ⟦tj:49117de44b0633e0e83b1a06bbee2bb2⟧]
Dec  2 22:24:22 hel postfix/smtpd[7676]: warning: 114-44-142-233.dynamic.hinet.net[114.44.142.233]: SASL CRAM-MD5 authentication failed: PDc3OTEwNTkyNTEyMzA2NDIuMTIyODI1MzA2MUBoZWw+
[... 2 line(s) omitted ... ⟦tj:5660b22e1d5666683849044b9479f7cf⟧]
Mar 10 13:33:30 gandalf postfix/smtpd[3937]: warning: HOSTNAME[1.1.1.1]: SASL LOGIN authentication failed: authentication failure
[... 3 line(s) omitted ... ⟦tj:0de60b317642f8542e83ff986f38ac50⟧]
Sep  6 00:44:56 trianon postfix/submission/smtpd[11538]: warning: unknown[82.221.106.233]: SASL LOGIN authentication failed: UGFzc3dvcmQ6
[... 3 line(s) omitted ... ⟦tj:1796d0ce2d91489b489701da4d9551ee⟧]
Sep  6 00:44:57 trianon postfix/submission/smtpd[11538]: warning: unknown[82.221.106.233]: SASL login authentication failed: UGFzc3dvcmQ6
[... 3 line(s) omitted ... ⟦tj:2f988e3da34dfed27d8f093b4429620f⟧]
Jan 29 08:11:45 mail postfix/smtpd[10752]: warning: unknown[1.1.1.1]: SASL LOGIN authentication failed: Password:
[... 2 line(s) omitted ... ⟦tj:a2d14e1a52f4127f37ac46102fabd08c⟧]
Jan 29 08:11:45 mail postfix-incoming/smtpd[10752]: warning: unknown[1.1.1.1]: SASL LOGIN authentication failed: Password:
[... 2 line(s) omitted ... ⟦tj:e18c8b69c9adc12e09265956d74ed8f7⟧]
Apr 12 02:24:11 xxx postfix/smtps/smtpd[42]: warning: astra4139.startdedicated.de[62.138.2.143]: SASL LOGIN authentication failed: UGFzc3dvcmQ6
[... 2 line(s) omitted ... ⟦tj:a7e5a1712adef8b294382054c8ba9601⟧]
Aug 3 15:30:49 ksusha postfix/smtpd[17041]: warning: mail.foldsandwalker.com[98.191.84.74]: SASL Plain authentication failed:
[... 2 line(s) omitted ... ⟦tj:06daf077d7189b8902e63604975acc5d⟧]
Aug 4 16:47:52 mail3 postfix/smtpd[31152]: warning: unknown[192.0.2.237]:55729: SASL LOGIN authentication failed: authentication failure
[... 2 line(s) omitted ... ⟦tj:66b529e473658b5ec1ac89588a37bafb⟧]
Nov  4 09:11:01 mail postfix/submission/smtpd[27133]: warning: unknown[192.0.2.150]: SASL PLAIN authentication failed:
[... 1 line(s) omitted ... ⟦tj:e3b0c44298fc1c149afbf4c8996fb924⟧]
#6 Example to ignore because due to a failed attempt to connect to authentication service - no malicious activities whatsoever
[... 1 line(s) omitted ... ⟦tj:81808d126fbdc04c017102292ce7e415⟧]
Feb  3 08:29:28 mail postfix/smtpd[21022]: warning: unknown[1.1.1.1]: SASL LOGIN authentication failed: Connection lost to authentication server
[... 4 line(s) omitted ... ⟦tj:c11dcc7141b90266bbd4bf0236d235be⟧]
Jan 14 16:18:16 xxx postfix/smtpd[14933]: warning: host[192.0.2.5]: SASL CRAM-MD5 authentication failed: Invalid authentication mechanism
[... 4 line(s) omitted ... ⟦tj:db2f7ee749084a32e3fca1fa954599d6⟧]

```

### `29-spark-eventlog`

- [Full input](cases/29-spark-eventlog/input.log)
- [Full output](cases/29-spark-eventlog/output.log)
- [Input vs output diff](cases/29-spark-eventlog/compression.diff)

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
[... 4 line(s) omitted ... ⟦tj:4f12c89f9f5255f8e39d70eb6f10fe48⟧]
{"Event":"SparkListenerEnvironmentUpdate","JVM Information":{"Java Home":"/Library/Java/JavaVirtualMachines/zulu-8.jdk/Contents/Home/jre","Java Version":"1.8.0_312 (Azul Systems, Inc.)","Scala Version":"version 2.12.15"}...
[... 6 line(s) omitted ... ⟦tj:a256b10761704289fbd343fd176c36c3⟧]
{"Event":"SparkListenerTaskStart","Stage ID":0,"Stage Attempt ID":0,"Task Info":{"Task ID":0,"Index":0,"Attempt":0,"Partition ID":0,"Launch Time":1642039496413,"Executor ID":"driver","Host":"172.22.200.52","Locality":"PR...
{"Event":"SparkListenerTaskStart","Stage ID":0,"Stage Attempt ID":0,"Task Info":{"Task ID":1,"Index":1,"Attempt":0,"Partition ID":1,"Launch Time":1642039496425,"Executor ID":"driver","Host":"172.22.200.52","Locality":"PR...
{"Event":"SparkListenerTaskStart","Stage ID":0,"Stage Attempt ID":0,"Task Info":{"Task ID":2,"Index":2,"Attempt":0,"Partition ID":2,"Launch Time":1642039496425,"Executor ID":"driver","Host":"172.22.200.52","Locality":"PR...
{"Event":"SparkListenerTaskStart","Stage ID":0,"Stage Attempt ID":0,"Task Info":{"Task ID":3,"Index":3,"Attempt":0,"Partition ID":3,"Launch Time":1642039496425,"Executor ID":"driver","Host":"172.22.200.52","Locality":"PR...
[... 4 line(s) omitted ... ⟦tj:1d3429a8e740739d98d6ac60548597b3⟧]
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":6,"Index":6,"Attempt":0,"Partition ID":6,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":5,"Index":5,"Attempt":0,"Partition ID":5,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":4,"Index":4,"Attempt":0,"Partition ID":4,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":2,"Index":2,"Attempt":0,"Partition ID":2,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":0,"Index":0,"Attempt":0,"Partition ID":0,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":1,"Index":1,"Attempt":0,"Partition ID":1,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":7,"Index":7,"Attempt":0,"Partition ID":7,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":0,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":3,"Index":3,"Attempt":0,"Partition ID":3,"Launch Time":16420394...
{"Event":"SparkListenerStageCompleted","Stage Info":{"Stage ID":0,"Stage Attempt ID":0,"Stage Name":"count at <console>:23","Number of Tasks":8,"RDD Info":[{"RDD ID":4,"Name":"MapPartitionsRDD","Scope":"{\"id\":\"0\",\"n...
[... 15 line(s) omitted ... ⟦tj:6bfaf2589f66f824689557e4a3e4e3f5⟧]
{"Event":"SparkListenerTaskEnd","Stage ID":2,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":14,"Index":6,"Attempt":0,"Partition ID":6,"Launch Time":1642039...
{"Event":"SparkListenerTaskEnd","Stage ID":2,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":10,"Index":2,"Attempt":0,"Partition ID":2,"Launch Time":1642039...
{"Event":"SparkListenerTaskEnd","Stage ID":2,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":15,"Index":7,"Attempt":0,"Partition ID":7,"Launch Time":1642039...
{"Event":"SparkListenerTaskEnd","Stage ID":2,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":12,"Index":4,"Attempt":0,"Partition ID":4,"Launch Time":1642039...
{"Event":"SparkListenerTaskEnd","Stage ID":2,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":11,"Index":3,"Attempt":0,"Partition ID":3,"Launch Time":1642039...
{"Event":"SparkListenerTaskEnd","Stage ID":2,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":9,"Index":1,"Attempt":0,"Partition ID":1,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":2,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":13,"Index":5,"Attempt":0,"Partition ID":5,"Launch Time":1642039...
{"Event":"SparkListenerTaskEnd","Stage ID":2,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":8,"Index":0,"Attempt":0,"Partition ID":0,"Launch Time":16420394...
{"Event":"SparkListenerTaskEnd","Stage ID":2,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":17,"Index":9,"Attempt":0,"Partition ID":9,"Launch Time":1642039...
{"Event":"SparkListenerTaskEnd","Stage ID":2,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":16,"Index":8,"Attempt":0,"Partition ID":8,"Launch Time":1642039...
{"Event":"SparkListenerStageCompleted","Stage Info":{"Stage ID":2,"Stage Attempt ID":0,"Stage Name":"count at <console>:23","Number of Tasks":10,"RDD Info":[{"RDD ID":7,"Name":"MapPartitionsRDD","Scope":"{\"id\":\"4\",\"...
[... 5 line(s) omitted ... ⟦tj:91bb41d497ee785b9b4e46f62436eb78⟧]
{"Event":"SparkListenerTaskEnd","Stage ID":5,"Stage Attempt ID":0,"Task Type":"ResultTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":18,"Index":0,"Attempt":0,"Partition ID":0,"Launch Time":16420394971...
{"Event":"SparkListenerStageCompleted","Stage Info":{"Stage ID":5,"Stage Attempt ID":0,"Stage Name":"count at <console>:23","Number of Tasks":1,"RDD Info":[{"RDD ID":10,"Name":"MapPartitionsRDD","Scope":"{\"id\":\"15\",\...
[... 12 line(s) omitted ... ⟦tj:43450163b3c0556ff7b3008c6ec9aef2⟧]
{"Event":"SparkListenerTaskEnd","Stage ID":6,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":19,"Index":0,"Attempt":0,"Partition ID":0,"Launch Time":1642039...
{"Event":"SparkListenerTaskEnd","Stage ID":6,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":23,"Index":4,"Attempt":0,"Partition ID":4,"Launch Time":1642039...
{"Event":"SparkListenerTaskEnd","Stage ID":6,"Stage Attempt ID":0,"Task Type":"ShuffleMapTask","Task End Reason":{"Reason":"Success"},"Task Info":{"Task ID":20,"Index":1,"Attempt":0,"Partition ID":1,"Launch Time":1642039...

```

### `32-w3c-iis`

- [Full input](cases/32-w3c-iis/input.log)
- [Full output](cases/32-w3c-iis/output.log)
- [Input vs output diff](cases/32-w3c-iis/compression.diff)

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
- [Full output](cases/31-zeek-http/output.log)
- [Input vs output diff](cases/31-zeek-http/compression.diff)

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
- [Full output](cases/27-suricata-eve/output.log)
- [Input vs output diff](cases/27-suricata-eve/compression.diff)

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
- [Full output](cases/26-gitlab-bf/output.log)
- [Input vs output diff](cases/26-gitlab-bf/compression.diff)

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
- [Full output](cases/25-sshesame-honeypot/output.log)
- [Input vs output diff](cases/25-sshesame-honeypot/compression.diff)

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
- [Full output](cases/24-http-dos/output.log)
- [Input vs output diff](cases/24-http-dos/compression.diff)

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
- [Full output](cases/22-traefik-http/output.log)
- [Input vs output diff](cases/22-traefik-http/compression.diff)

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
- [Full output](cases/21-traefik-flood/output.log)
- [Input vs output diff](cases/21-traefik-flood/compression.diff)

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
- [Full output](cases/18-nginx-access/output.log)
- [Input vs output diff](cases/18-nginx-access/compression.diff)

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

