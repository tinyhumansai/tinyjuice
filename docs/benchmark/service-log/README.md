# Service And Docker Logs

Real OpenHuman runtime crash-log slices, with live Docker OpenHuman logs used first when a container is available. The log compressor keeps incident signals and collapses repeated low-value lines.

Each row links to the full raw input and the exact compacted output used by the benchmark. Percentages are **token reduction: higher is better**; 0% means pass-through. `Bytes` shows the raw input size -> compressor-only output size and its byte reduction. `Pass 1` disables CCR (compressed with omission markers, no recovery footer). `Pass 2` is the final model-facing result with CCR enabled — it reads marginally *lower* than Pass 1 only because the recovery footer adds a few dozen bytes to the output.

## Cases

Every case links to the raw input, the exact model-facing output (with the CCR recovery footer), and a unified diff between the two.

| Case | Input | Output (after CCR) | Diff | Bytes | Pass 1: no CCR | Pass 2: with CCR | Avg latency |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: |
| `02-openhuman-crash-slice-2` | [input](cases/02-openhuman-crash-slice-2/input.log) | [output](cases/02-openhuman-crash-slice-2/output.log) | [diff](cases/02-openhuman-crash-slice-2/compression.diff) | 29.7 KB -> 1.2 KB (-96%) | 97.1% | 95.4% | 0.119 ms |
| `01-openhuman-crash-slice-1` | [input](cases/01-openhuman-crash-slice-1/input.log) | [output](cases/01-openhuman-crash-slice-1/output.log) | [diff](cases/01-openhuman-crash-slice-1/compression.diff) | 26.6 KB -> 1.0 KB (-96%) | 97.2% | 95.3% | 0.118 ms |
| `04-openhuman-crash-slice-4` | [input](cases/04-openhuman-crash-slice-4/input.log) | [output](cases/04-openhuman-crash-slice-4/output.log) | [diff](cases/04-openhuman-crash-slice-4/compression.diff) | 47.0 KB -> 2.1 KB (-95%) | 96.4% | 95.1% | 0.151 ms |
| `03-openhuman-crash-slice-3` | [input](cases/03-openhuman-crash-slice-3/input.log) | [output](cases/03-openhuman-crash-slice-3/output.log) | [diff](cases/03-openhuman-crash-slice-3/compression.diff) | 45.9 KB -> 2.1 KB (-95%) | 96.3% | 95.0% | 0.153 ms |
| `05-openhuman-crash-slice-5` | [input](cases/05-openhuman-crash-slice-5/input.log) | [output](cases/05-openhuman-crash-slice-5/output.log) | [diff](cases/05-openhuman-crash-slice-5/compression.diff) | 44.5 KB -> 2.1 KB (-95%) | 96.2% | 94.8% | 0.145 ms |
| `07-openhuman-crash-slice-7` | [input](cases/07-openhuman-crash-slice-7/input.log) | [output](cases/07-openhuman-crash-slice-7/output.log) | [diff](cases/07-openhuman-crash-slice-7/compression.diff) | 40.9 KB -> 2.1 KB (-95%) | 95.9% | 94.4% | 0.140 ms |
| `08-openhuman-crash-slice-8` | [input](cases/08-openhuman-crash-slice-8/input.log) | [output](cases/08-openhuman-crash-slice-8/output.log) | [diff](cases/08-openhuman-crash-slice-8/compression.diff) | 41.0 KB -> 2.2 KB (-95%) | 95.6% | 94.1% | 0.136 ms |
| `06-openhuman-crash-slice-6` | [input](cases/06-openhuman-crash-slice-6/input.log) | [output](cases/06-openhuman-crash-slice-6/output.log) | [diff](cases/06-openhuman-crash-slice-6/compression.diff) | 41.0 KB -> 2.3 KB (-94%) | 95.4% | 94.0% | 0.136 ms |
| `09-openhuman-crash-slice-9` | [input](cases/09-openhuman-crash-slice-9/input.log) | [output](cases/09-openhuman-crash-slice-9/output.log) | [diff](cases/09-openhuman-crash-slice-9/compression.diff) | 38.1 KB -> 2.2 KB (-94%) | 95.3% | 93.7% | 0.136 ms |
| `10-openhuman-crash-slice-10` | [input](cases/10-openhuman-crash-slice-10/input.log) | [output](cases/10-openhuman-crash-slice-10/output.log) | [diff](cases/10-openhuman-crash-slice-10/compression.diff) | 493.4 KB -> 478.5 KB (-3%) | 3.1% | 3.0% | 0.721 ms |

## What TinyJuice Is Doing

The log path scores lines by signal. Errors, warnings, exception metadata, stack frames, and summaries are favored; repetitive routine lines are collapsed behind omission markers.

## Syntax-Aware Samples

### `01-openhuman-crash-slice-1`

- [Full input](cases/01-openhuman-crash-slice-1/input.log)
- [Full output](cases/01-openhuman-crash-slice-1/output.log)
- [Input vs output diff](cases/01-openhuman-crash-slice-1/compression.diff)

Input excerpt:

```text
-------------------------------------
Translated Report (Full Report Below)
-------------------------------------
Process:             OpenHuman [90825]
Path:                /Users/USER/*/OpenHuman.app/Contents/MacOS/OpenHuman
Identifier:          com.openhuman.app
Version:             0.53.49 (0.53.49)
Code Type:           ARM-64 (Native)
Role:                Foreground
Parent Process:      cargo-tauri [90125]
Coalition:           com.mitchellh.ghostty [1140]
Responsible Process: ghostty [1684]
User ID:             501

Date/Time:           2026-05-17 22:22:21.2525 -0700
Launch Time:         2026-05-17 22:21:08.3067 -0700
Hardware Model:      Mac17,9
OS Version:          macOS 26.5 (25F71)
Release Type:        User

Crash Reporter Key:  54C8B0B1-AAA7-FB80-382A-B9EF705FB713
Incident Identifier: 296E9C85-2978-4CF1-99AF-8344D328256F

Sleep/Wake UUID:       88CACA1E-ECCE-4B52-8067-80E6D6970B4C

Time Awake Since Boot: 250000 seconds
Time Since Wake:       15790 seconds

System Integrity Protection: enabled

Triggered by Thread: 46  tokio-rt-worker

Exception Type:    EXC_BAD_ACCESS (SIGBUS)
Exception Subtype: KERN_PROTECTION_FAILURE at 0x00000003028534f0
Exception Message: Could not determine thread index for stack guard region
Exception Codes:   0x0000000000000002, 0x00000003028534f0

```

Output excerpt:

```text
[... 32 line(s) omitted ... ⟦tj:0155c2cddf478d13f916badfee0141ff⟧]
Exception Type:    EXC_BAD_ACCESS (SIGBUS)
Exception Subtype: KERN_PROTECTION_FAILURE at 0x00000003028534f0
Exception Message: Could not determine thread index for stack guard region
Exception Codes:   0x0000000000000002, 0x00000003028534f0
[... 1 line(s) omitted ... ⟦tj:e3b0c44298fc1c149afbf4c8996fb924⟧]
Termination Reason:  Namespace SIGNAL, Code 10, Bus error: 10
[... 61 line(s) omitted ... ⟦tj:5187f676d6221360bf3ea70691c78c7f⟧]
16  OpenHuman                     	       0x1029a57bc _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h0abea6da5fd65be4 + 48
17  OpenHuman                     	       0x1024a5688 std::panicking::catch_unwind::do_call::h34447d5ada7f4206 + 56
[... 248 line(s) omitted ... ⟦tj:03c5b8d1785f717c5f218dbdf098c066⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (26592 bytes) is available by calling tinyjuice_retrieve with token "c87f7d774040584706d28f7d9035b8d5" (marker ⟦tj:c87f7d774040584706d28f7d9035b8d5⟧)]

```

### `02-openhuman-crash-slice-2`

- [Full input](cases/02-openhuman-crash-slice-2/input.log)
- [Full output](cases/02-openhuman-crash-slice-2/output.log)
- [Input vs output diff](cases/02-openhuman-crash-slice-2/compression.diff)

Input excerpt:

```text
Thread 19:: ThreadPoolForegroundWorker
0   libsystem_kernel.dylib        	       0x18ac09c34 mach_msg2_trap + 8
1   libsystem_kernel.dylib        	       0x18ac1c574 mach_msg2_internal + 76
2   libsystem_kernel.dylib        	       0x18ac129c0 mach_msg_overwrite + 480
3   libsystem_kernel.dylib        	       0x18ac09fc0 mach_msg + 24
4   Chromium Embedded Framework   	       0x12350e064 ChromeWebAppShortcutCopierMain + 5532228
5   Chromium Embedded Framework   	       0x123496984 ChromeWebAppShortcutCopierMain + 5043044
6   Chromium Embedded Framework   	       0x1234d2fe4 ChromeWebAppShortcutCopierMain + 5290436
7   Chromium Embedded Framework   	       0x1234d40e0 ChromeWebAppShortcutCopierMain + 5294784
8   Chromium Embedded Framework   	       0x1234d3ac4 ChromeWebAppShortcutCopierMain + 5293220
9   Chromium Embedded Framework   	       0x1234d399c ChromeWebAppShortcutCopierMain + 5292924
10  Chromium Embedded Framework   	       0x1234f1678 ChromeWebAppShortcutCopierMain + 5415000
11  libsystem_pthread.dylib       	       0x18ac4dc58 _pthread_start + 136
12  libsystem_pthread.dylib       	       0x18ac48c1c thread_start + 8

Thread 20:: ThreadPoolForegroundWorker
0   libsystem_kernel.dylib        	       0x18ac09c34 mach_msg2_trap + 8
1   libsystem_kernel.dylib        	       0x18ac1c574 mach_msg2_internal + 76
2   libsystem_kernel.dylib        	       0x18ac129c0 mach_msg_overwrite + 480
3   libsystem_kernel.dylib        	       0x18ac09fc0 mach_msg + 24
4   Chromium Embedded Framework   	       0x12350e064 ChromeWebAppShortcutCopierMain + 5532228
5   Chromium Embedded Framework   	       0x123496984 ChromeWebAppShortcutCopierMain + 5043044
6   Chromium Embedded Framework   	       0x1234d2fe4 ChromeWebAppShortcutCopierMain + 5290436
7   Chromium Embedded Framework   	       0x1234d40e0 ChromeWebAppShortcutCopierMain + 5294784
8   Chromium Embedded Framework   	       0x1234d3ac4 ChromeWebAppShortcutCopierMain + 5293220
9   Chromium Embedded Framework   	       0x1234d399c ChromeWebAppShortcutCopierMain + 5292924
10  Chromium Embedded Framework   	       0x1234f1678 ChromeWebAppShortcutCopierMain + 5415000
11  libsystem_pthread.dylib       	       0x18ac4dc58 _pthread_start + 136
12  libsystem_pthread.dylib       	       0x18ac48c1c thread_start + 8

Thread 21:: NetworkNotificationThreadMac
0   libsystem_kernel.dylib        	       0x18ac09c34 mach_msg2_trap + 8
1   libsystem_kernel.dylib        	       0x18ac1c574 mach_msg2_internal + 76
2   libsystem_kernel.dylib        	       0x18ac129c0 mach_msg_overwrite + 480
3   libsystem_kernel.dylib        	       0x18ac09fc0 mach_msg + 24
4   CoreFoundation                	       0x18ad0b0d8 __CFRunLoopServiceMachPort + 160

```

Output excerpt:

```text
[... 326 line(s) omitted ... ⟦tj:41c0df8e237b5a865c5a84ec1c556bf2⟧]
28  OpenHuman                     	       0x10523cae4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::hd44b258fb813656b + 44
29  OpenHuman                     	       0x1052a64b0 std::panicking::catch_unwind::do_call::h72377314c177ff75 + 64
[... 1 line(s) omitted ... ⟦tj:e3c14a382aa2b9c278e169961948fc41⟧]
31  OpenHuman                     	       0x105291df0 std::panic::catch_unwind::hfd000fe64f5efc68 + 96
[... 11 line(s) omitted ... ⟦tj:081d124183c37af5d30eeffc7f8f5da6⟧]
43  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
44  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 6 line(s) omitted ... ⟦tj:ce1a656f51dfc526a938a5f6f7cb6df2⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (29717 bytes) is available by calling tinyjuice_retrieve with token "b2a16dcca94e32f601f4c27a80236f09" (marker ⟦tj:b2a16dcca94e32f601f4c27a80236f09⟧)]

```

### `04-openhuman-crash-slice-4`

- [Full input](cases/04-openhuman-crash-slice-4/input.log)
- [Full output](cases/04-openhuman-crash-slice-4/output.log)
- [Input vs output diff](cases/04-openhuman-crash-slice-4/compression.diff)

Input excerpt:

```text
24  OpenHuman                     	       0x10385bf40 tokio::task::task_local::LocalKey$LT$T$GT$::scope_inner::hd93b1a3de1b566f7 + 312
25  OpenHuman                     	       0x10384c810 _$LT$tokio..task..task_local..TaskLocalFuture$LT$T$C$F$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb22659504f5fc861 + 80
26  OpenHuman                     	       0x10288aad8 openhuman_core::openhuman::agent::harness::sandbox_context::with_current_sandbox_mode::_$u7b$$u7b$closure$u7d$$u7d$::hddf6baac2f8be787 + 536
27  OpenHuman                     	       0x101badd0c openhuman_core::openhuman::agent::harness::subagent_runner::ops::run_subagent::_$u7b$$u7b$closure$u7d$$u7d$::h2850db6a85715cee + 2168
28  OpenHuman                     	       0x103880df0 openhuman_core::openhuman::tools::implementations::agent::dispatch::dispatch_subagent::_$u7b$$u7b$closure$u7d$$u7d$::h4a8d2e5e9e99494b + 2900
29  OpenHuman                     	       0x1017402b0 _$LT$openhuman_core..openhuman..tools..implementations..agent..skill_delegation..SkillDelegationTool$u20$as$u20$openhuman_core..openhuman..tools..traits..Tool$GT$::ex...
30  OpenHuman                     	       0x10195f184 _$LT$core..pin..Pin$LT$P$GT$$u20$as$u20$core..future..future..Future$GT$::poll::ha81329e0a770dcaa + 80
31  OpenHuman                     	       0x102a3766c openhuman_core::openhuman::tools::traits::Tool::execute_with_options::_$u7b$$u7b$closure$u7d$$u7d$::hc26a3b04e83c866d + 588
32  OpenHuman                     	       0x10195f184 _$LT$core..pin..Pin$LT$P$GT$$u20$as$u20$core..future..future..Future$GT$::poll::ha81329e0a770dcaa + 80
33  OpenHuman                     	       0x1022f5f2c openhuman_core::openhuman::agent::harness::session::turn::_$LT$impl$u20$openhuman_core..openhuman..agent..harness..session..types..Agent$GT$::execute_tool_call::_$u7b...
34  OpenHuman                     	       0x1022f50d0 openhuman_core::openhuman::agent::harness::session::turn::_$LT$impl$u20$openhuman_core..openhuman..agent..harness..session..types..Agent$GT$::execute_tools::_$u7b$$u7...
35  OpenHuman                     	       0x10230000c openhuman_core::openhuman::agent::harness::session::turn::_$LT$impl$u20$openhuman_core..openhuman..agent..harness..session..types..Agent$GT$::turn::_$u7b$$u7b$closure...
36  OpenHuman                     	       0x10384d224 _$LT$tokio..task..task_local..TaskLocalFuture$LT$T$C$F$GT$$u20$as$u20$core..future..future..Future$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hc28f1de0b64d5d5c + 116
37  OpenHuman                     	       0x10385b484 tokio::task::task_local::LocalKey$LT$T$GT$::scope_inner::h87a516f3c968981e + 288
38  OpenHuman                     	       0x10384c324 _$LT$tokio..task..task_local..TaskLocalFuture$LT$T$C$F$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h0fe125e9bc0e90ad + 80
39  OpenHuman                     	       0x1021bc510 openhuman_core::openhuman::agent::harness::fork_context::with_parent_context::_$u7b$$u7b$closure$u7d$$u7d$::he54ba2eb22262a15 + 472
40  OpenHuman                     	       0x1022ff7f8 openhuman_core::openhuman::agent::harness::session::turn::_$LT$impl$u20$openhuman_core..openhuman..agent..harness..session..types..Agent$GT$::turn::_$u7b$$u7b$closure...
41  OpenHuman                     	       0x10228faf8 openhuman_core::openhuman::agent::harness::session::runtime::_$LT$impl$u20$openhuman_core..openhuman..agent..harness..session..types..Agent$GT$::run_single::_$u7b$$u7...
42  OpenHuman                     	       0x10384c95c _$LT$tokio..task..task_local..TaskLocalFuture$LT$T$C$F$GT$$u20$as$u20$core..future..future..Future$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::h0ccf050da9632c37 + 160
43  OpenHuman                     	       0x10385ba00 tokio::task::task_local::LocalKey$LT$T$GT$::scope_inner::hc22046fa47ef5122 + 288
44  OpenHuman                     	       0x10384c714 _$LT$tokio..task..task_local..TaskLocalFuture$LT$T$C$F$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h6db3fcedb0bb809e + 80
45  OpenHuman                     	       0x1022861e0 openhuman_core::openhuman::inference::provider::thread_context::with_thread_id::_$u7b$$u7b$closure$u7d$$u7d$::h5b13f32409d45a5f + 1428
46  OpenHuman                     	       0x1025374e4 openhuman_core::openhuman::channels::providers::web::run_chat_task::_$u7b$$u7b$closure$u7d$$u7d$::h9bf9c51a4c5500dc + 9128
47  OpenHuman                     	       0x102532a84 openhuman_core::openhuman::channels::providers::web::start_chat::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha707d5c1096bafbc + 748
48  OpenHuman                     	       0x10195db00 _$LT$core..pin..Pin$LT$P$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h53218f396abd2124 + 56
49  OpenHuman                     	       0x102b37bb0 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::h0af618666ef30b6a + 192
50  OpenHuman                     	       0x102b1762c tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h0b0bd6fe76c2b4df + 72
51  OpenHuman                     	       0x101d4dd34 tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::h76be7172bf1e86bc + 64
52  OpenHuman                     	       0x1029a4d30 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h04422aac1d5d487b + 44
53  OpenHuman                     	       0x1024b9fbc std::panicking::catch_unwind::do_call::hac75a22b40c0b34f + 64
54  OpenHuman                     	       0x101704b0c __rust_try + 32
55  OpenHuman                     	       0x1016b202c std::panic::catch_unwind::h272ebb504f50c3c9 + 96
56  OpenHuman                     	       0x101d3c9c0 tokio::runtime::task::harness::poll_future::hf88896021c669225 + 96
57  OpenHuman                     	       0x101d64228 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h006ddb55c96c1fee + 172
58  OpenHuman                     	       0x101de6b90 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h8f9970206615fb41 + 28
59  OpenHuman                     	       0x102cee530 tokio::runtime::task::raw::poll::hccd3e3c31ead7352 + 36

```

Output excerpt:

```text
[... 28 line(s) omitted ... ⟦tj:f49f7e99ea18c6606d85d93d7a042a9e⟧]
52  OpenHuman                     	       0x1029a4d30 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h04422aac1d5d487b + 44
53  OpenHuman                     	       0x1024b9fbc std::panicking::catch_unwind::do_call::hac75a22b40c0b34f + 64
[... 1 line(s) omitted ... ⟦tj:3394d7bdebebad412ce6219dad806caf⟧]
55  OpenHuman                     	       0x1016b202c std::panic::catch_unwind::h272ebb504f50c3c9 + 96
[... 23 line(s) omitted ... ⟦tj:410b8b1db7dfe322bca3f105f2336aa5⟧]
79  OpenHuman                     	       0x10523cae4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::hd44b258fb813656b + 44
80  OpenHuman                     	       0x1052a64b0 std::panicking::catch_unwind::do_call::h72377314c177ff75 + 64
[... 263 line(s) omitted ... ⟦tj:1211510cc08bec6f16a3a217cb02ff0d⟧]
28  OpenHuman                     	       0x10523cae4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::hd44b258fb813656b + 44
29  OpenHuman                     	       0x1052a64b0 std::panicking::catch_unwind::do_call::h72377314c177ff75 + 64
[... 1 line(s) omitted ... ⟦tj:e3c14a382aa2b9c278e169961948fc41⟧]
31  OpenHuman                     	       0x105291df0 std::panic::catch_unwind::hfd000fe64f5efc68 + 96
[... 11 line(s) omitted ... ⟦tj:081d124183c37af5d30eeffc7f8f5da6⟧]
43  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
44  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 12 line(s) omitted ... ⟦tj:f0b5ff07bbd5b7dbb87d674f08619065⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (46952 bytes) is available by calling tinyjuice_retrieve with token "68101d9a95ec3ff37b0d77c3cbc60a20" (marker ⟦tj:68101d9a95ec3ff37b0d77c3cbc60a20⟧)]

```

### `03-openhuman-crash-slice-3`

- [Full input](cases/03-openhuman-crash-slice-3/input.log)
- [Full output](cases/03-openhuman-crash-slice-3/output.log)
- [Input vs output diff](cases/03-openhuman-crash-slice-3/compression.diff)

Input excerpt:

```text

Thread 40:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x18ac0d50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x18ac4e128 _pthread_cond_wait + 980
2   OpenHuman                     	       0x1056e2ffc _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h0a7c3433fc43bc25 + 228
3   OpenHuman                     	       0x1056e1c94 parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hc2da40aa73586843 + 748
4   OpenHuman                     	       0x1056dfd6c parking_lot_core::parking_lot::park::h964bd64c3ea232d0 + 296
5   OpenHuman                     	       0x1056e36c8 parking_lot::condvar::Condvar::wait_until_internal::hb2630d95a9855c15 + 160
6   OpenHuman                     	       0x105282284 parking_lot::condvar::Condvar::wait::hb3c1ad4200a27b21 + 68
7   OpenHuman                     	       0x10526841c tokio::loom::std::parking_lot::Condvar::wait::hb9507be274b09c59 + 36
8   OpenHuman                     	       0x105244574 tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h45bbf62bf2df50a2 + 412
9   OpenHuman                     	       0x105244c50 tokio::runtime::scheduler::multi_thread::park::Inner::park::h0a85db492e901412 + 180
10  OpenHuman                     	       0x1052450e0 tokio::runtime::scheduler::multi_thread::park::Parker::park::h2c0f996c9186cbc5 + 40
11  OpenHuman                     	       0x1052881e4 tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h16ea2e1c3f7b1259 + 884
12  OpenHuman                     	       0x10528924c tokio::runtime::scheduler::multi_thread::worker::Context::park::h15c486eee2be666e + 968
13  OpenHuman                     	       0x105288d48 tokio::runtime::scheduler::multi_thread::worker::Context::run::ha450fd319292bedf + 1764
14  OpenHuman                     	       0x105285d60 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h374e273d28c1b535 + 104
15  OpenHuman                     	       0x10526a5c0 tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::h7e26ff9cdb4392fd + 148
16  OpenHuman                     	       0x10529352c tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h27e77eb90cca057a + 40
17  OpenHuman                     	       0x105251a94 std::thread::local::LocalKey$LT$T$GT$::try_with::hd0792127a34961ac + 168
18  OpenHuman                     	       0x10524ff20 std::thread::local::LocalKey$LT$T$GT$::with::h35b743a0638d66cc + 24
19  OpenHuman                     	       0x1052934b8 tokio::runtime::context::set_scheduler::hb888aec89c60bb61 + 68
20  OpenHuman                     	       0x105285c84 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::hc2de7fb5a1fdde38 + 248
21  OpenHuman                     	       0x1052aec9c tokio::runtime::context::runtime::enter_runtime::h6a4d022345791cb8 + 176
22  OpenHuman                     	       0x105285b2c tokio::runtime::scheduler::multi_thread::worker::run::he2ad276f3d9c73ea + 600
23  OpenHuman                     	       0x105286d10 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hacf90a62dc5a56bb + 24
24  OpenHuman                     	       0x105233654 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h6db8103896276dde + 136
25  OpenHuman                     	       0x1052266b0 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::h2d485835559c1639 + 192
26  OpenHuman                     	       0x1052256c8 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h3c393af5c9cccfb5 + 72
27  OpenHuman                     	       0x1052142b8 tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hbb7135acba10cb1b + 64
28  OpenHuman                     	       0x10523cae4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::hd44b258fb813656b + 44
29  OpenHuman                     	       0x1052a64b0 std::panicking::catch_unwind::do_call::h72377314c177ff75 + 64
30  OpenHuman                     	       0x1052965a4 __rust_try + 32
31  OpenHuman                     	       0x105291df0 std::panic::catch_unwind::hfd000fe64f5efc68 + 96
32  OpenHuman                     	       0x105211138 tokio::runtime::task::harness::poll_future::h13c71e773a7b909e + 96
33  OpenHuman                     	       0x105215998 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h8d4c47df8e07256b + 172

```

Output excerpt:

```text
[... 30 line(s) omitted ... ⟦tj:3e5ccfb58b14e352d860b7a500ba6668⟧]
28  OpenHuman                     	       0x10523cae4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::hd44b258fb813656b + 44
29  OpenHuman                     	       0x1052a64b0 std::panicking::catch_unwind::do_call::h72377314c177ff75 + 64
[... 1 line(s) omitted ... ⟦tj:e3c14a382aa2b9c278e169961948fc41⟧]
31  OpenHuman                     	       0x105291df0 std::panic::catch_unwind::hfd000fe64f5efc68 + 96
[... 11 line(s) omitted ... ⟦tj:081d124183c37af5d30eeffc7f8f5da6⟧]
43  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
44  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 253 line(s) omitted ... ⟦tj:a361d5788d1bd88b0fe301fee8f2cc2a⟧]
33  OpenHuman                     	       0x10523cae4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::hd44b258fb813656b + 44
34  OpenHuman                     	       0x1052a64b0 std::panicking::catch_unwind::do_call::h72377314c177ff75 + 64
[... 1 line(s) omitted ... ⟦tj:dda347add71b691573a64ee900deae26⟧]
36  OpenHuman                     	       0x105291df0 std::panic::catch_unwind::hfd000fe64f5efc68 + 96
[... 11 line(s) omitted ... ⟦tj:1c0af347417e1a7c791e8a99ffd27986⟧]
48  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
49  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 32 line(s) omitted ... ⟦tj:1f5c89ae2b31e842c1bd418aa0e612b0⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (45921 bytes) is available by calling tinyjuice_retrieve with token "30afbaeb44296119cf1f6a9da564a843" (marker ⟦tj:30afbaeb44296119cf1f6a9da564a843⟧)]

```

### `05-openhuman-crash-slice-5`

- [Full input](cases/05-openhuman-crash-slice-5/input.log)
- [Full output](cases/05-openhuman-crash-slice-5/output.log)
- [Input vs output diff](cases/05-openhuman-crash-slice-5/compression.diff)

Input excerpt:

```text
4   OpenHuman                     	       0x1056dfd6c parking_lot_core::parking_lot::park::h964bd64c3ea232d0 + 296
5   OpenHuman                     	       0x1056e36c8 parking_lot::condvar::Condvar::wait_until_internal::hb2630d95a9855c15 + 160
6   OpenHuman                     	       0x105282284 parking_lot::condvar::Condvar::wait::hb3c1ad4200a27b21 + 68
7   OpenHuman                     	       0x10526841c tokio::loom::std::parking_lot::Condvar::wait::hb9507be274b09c59 + 36
8   OpenHuman                     	       0x105244574 tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h45bbf62bf2df50a2 + 412
9   OpenHuman                     	       0x105244c50 tokio::runtime::scheduler::multi_thread::park::Inner::park::h0a85db492e901412 + 180
10  OpenHuman                     	       0x1052450e0 tokio::runtime::scheduler::multi_thread::park::Parker::park::h2c0f996c9186cbc5 + 40
11  OpenHuman                     	       0x1052881e4 tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h16ea2e1c3f7b1259 + 884
12  OpenHuman                     	       0x10528924c tokio::runtime::scheduler::multi_thread::worker::Context::park::h15c486eee2be666e + 968
13  OpenHuman                     	       0x105288d48 tokio::runtime::scheduler::multi_thread::worker::Context::run::ha450fd319292bedf + 1764
14  OpenHuman                     	       0x105285d60 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h374e273d28c1b535 + 104
15  OpenHuman                     	       0x10526a5c0 tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::h7e26ff9cdb4392fd + 148
16  OpenHuman                     	       0x10529352c tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h27e77eb90cca057a + 40
17  OpenHuman                     	       0x105251a94 std::thread::local::LocalKey$LT$T$GT$::try_with::hd0792127a34961ac + 168
18  OpenHuman                     	       0x10524ff20 std::thread::local::LocalKey$LT$T$GT$::with::h35b743a0638d66cc + 24
19  OpenHuman                     	       0x1052934b8 tokio::runtime::context::set_scheduler::hb888aec89c60bb61 + 68
20  OpenHuman                     	       0x105285c84 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::hc2de7fb5a1fdde38 + 248
21  OpenHuman                     	       0x1052aec9c tokio::runtime::context::runtime::enter_runtime::h6a4d022345791cb8 + 176
22  OpenHuman                     	       0x105285b2c tokio::runtime::scheduler::multi_thread::worker::run::he2ad276f3d9c73ea + 600
23  OpenHuman                     	       0x105286d10 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hacf90a62dc5a56bb + 24
24  OpenHuman                     	       0x105233654 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h6db8103896276dde + 136
25  OpenHuman                     	       0x1052266b0 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::h2d485835559c1639 + 192
26  OpenHuman                     	       0x1052256c8 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h3c393af5c9cccfb5 + 72
27  OpenHuman                     	       0x1052142b8 tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hbb7135acba10cb1b + 64
28  OpenHuman                     	       0x10523cae4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::hd44b258fb813656b + 44
29  OpenHuman                     	       0x1052a64b0 std::panicking::catch_unwind::do_call::h72377314c177ff75 + 64
30  OpenHuman                     	       0x1052965a4 __rust_try + 32
31  OpenHuman                     	       0x105291df0 std::panic::catch_unwind::hfd000fe64f5efc68 + 96
32  OpenHuman                     	       0x105211138 tokio::runtime::task::harness::poll_future::h13c71e773a7b909e + 96
33  OpenHuman                     	       0x105215998 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h8d4c47df8e07256b + 172
34  OpenHuman                     	       0x105218efc tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h52a69113cb330ce2 + 28
35  OpenHuman                     	       0x1052ac2a4 tokio::runtime::task::raw::poll::h361c3293cb39ab07 + 36
36  OpenHuman                     	       0x1052adfac tokio::runtime::task::raw::RawTask::poll::hd80bf3a96acb22db + 52
37  OpenHuman                     	       0x10524a6b8 tokio::runtime::task::UnownedTask$LT$S$GT$::run::hf5f2edbbe3cb6426 + 64
38  OpenHuman                     	       0x1052b5884 tokio::runtime::blocking::pool::Task::run::hbbdc75f7f091115b + 28
39  OpenHuman                     	       0x1052b5a98 tokio::runtime::blocking::pool::Inner::run::h01e32a9f9b2701bd + 520

```

Output excerpt:

```text
[... 24 line(s) omitted ... ⟦tj:56aced5d0aacb7099bd99a83538b7306⟧]
28  OpenHuman                     	       0x10523cae4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::hd44b258fb813656b + 44
29  OpenHuman                     	       0x1052a64b0 std::panicking::catch_unwind::do_call::h72377314c177ff75 + 64
[... 1 line(s) omitted ... ⟦tj:e3c14a382aa2b9c278e169961948fc41⟧]
31  OpenHuman                     	       0x105291df0 std::panic::catch_unwind::hfd000fe64f5efc68 + 96
[... 11 line(s) omitted ... ⟦tj:081d124183c37af5d30eeffc7f8f5da6⟧]
43  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
44  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 261 line(s) omitted ... ⟦tj:6a7ef65429a2fa955b40c8906098fa00⟧]
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 20 line(s) omitted ... ⟦tj:9435bdb67567537f94d81d98547a4fc6⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 20 line(s) omitted ... ⟦tj:18cbcd5b8ebea43db00ab8cbf3e14e77⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 2 line(s) omitted ... ⟦tj:07ee23edce71048f44562e4e1da7ad98⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (44535 bytes) is available by calling tinyjuice_retrieve with token "e4ac41e578f1d783c441d7b17c420ad6" (marker ⟦tj:e4ac41e578f1d783c441d7b17c420ad6⟧)]

```

### `07-openhuman-crash-slice-7`

- [Full input](cases/07-openhuman-crash-slice-7/input.log)
- [Full output](cases/07-openhuman-crash-slice-7/output.log)
- [Input vs output diff](cases/07-openhuman-crash-slice-7/compression.diff)

Input excerpt:

```text
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
14  OpenHuman                     	       0x105232e84 __rust_try + 32
15  OpenHuman                     	       0x10522c2ac std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::hf215137585e369ce + 224
16  OpenHuman                     	       0x10525acc0 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h123778ac55eae71d + 24
17  OpenHuman                     	       0x1057e9fdc std::sys::thread::unix::Thread::new::thread_start::hd42ed9e591878587 + 408
18  libsystem_pthread.dylib       	       0x18ac4dc58 _pthread_start + 136
19  libsystem_pthread.dylib       	       0x18ac48c1c thread_start + 8

Thread 77:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x18ac0d50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x18ac4e128 _pthread_cond_wait + 980
2   OpenHuman                     	       0x1056e2cec _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park_until::h3312b73289c83185 + 472
3   OpenHuman                     	       0x1056e1c84 parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hc2da40aa73586843 + 732
4   OpenHuman                     	       0x1056dfd6c parking_lot_core::parking_lot::park::h964bd64c3ea232d0 + 296
5   OpenHuman                     	       0x1056e36c8 parking_lot::condvar::Condvar::wait_until_internal::hb2630d95a9855c15 + 160
6   OpenHuman                     	       0x1052822fc parking_lot::condvar::Condvar::wait_for::h969488ffa1e51882 + 108
7   OpenHuman                     	       0x1052682e4 tokio::loom::std::parking_lot::Condvar::wait_timeout::h222f196a0b832f05 + 48
8   OpenHuman                     	       0x1052b5b28 tokio::runtime::blocking::pool::Inner::run::h01e32a9f9b2701bd + 664
9   OpenHuman                     	       0x1052b6974 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6de928fa040b7fb7 + 144
10  OpenHuman                     	       0x10523d088 std::sys::backtrace::__rust_begin_short_backtrace::hdb34a205259453e8 + 16
11  OpenHuman                     	       0x10522c42c std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h05989c1143d288cd + 116
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
14  OpenHuman                     	       0x105232e84 __rust_try + 32
15  OpenHuman                     	       0x10522c2ac std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::hf215137585e369ce + 224
16  OpenHuman                     	       0x10525acc0 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h123778ac55eae71d + 24
17  OpenHuman                     	       0x1057e9fdc std::sys::thread::unix::Thread::new::thread_start::hd42ed9e591878587 + 408
18  libsystem_pthread.dylib       	       0x18ac4dc58 _pthread_start + 136
19  libsystem_pthread.dylib       	       0x18ac48c1c thread_start + 8

Thread 78:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x18ac0d50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x18ac4e128 _pthread_cond_wait + 980
2   OpenHuman                     	       0x1056e2cec _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park_until::h3312b73289c83185 + 472
3   OpenHuman                     	       0x1056e1c84 parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hc2da40aa73586843 + 732
4   OpenHuman                     	       0x1056dfd6c parking_lot_core::parking_lot::park::h964bd64c3ea232d0 + 296

```

Output excerpt:

```text
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 20 line(s) omitted ... ⟦tj:ad50f67da295922ba4a81c30cb5f3d12⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 20 line(s) omitted ... ⟦tj:2f19fa575a1cf642997bb2a6b1a68e28⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 241 line(s) omitted ... ⟦tj:d1dea337fa5c48ae9ef0ae094e57c37d⟧]
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 20 line(s) omitted ... ⟦tj:d7efa2c04219b8b69c7bc69d476a510b⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 20 line(s) omitted ... ⟦tj:ad35e9ecccacba61e1dd8aa9d904093d⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 18 line(s) omitted ... ⟦tj:c898675dd1bd4024440d43ba9c27a0f7⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (40864 bytes) is available by calling tinyjuice_retrieve with token "86568fb7c4af4732716fbb38d5d077dc" (marker ⟦tj:86568fb7c4af4732716fbb38d5d077dc⟧)]

```

### `08-openhuman-crash-slice-8`

- [Full input](cases/08-openhuman-crash-slice-8/input.log)
- [Full output](cases/08-openhuman-crash-slice-8/output.log)
- [Input vs output diff](cases/08-openhuman-crash-slice-8/compression.diff)

Input excerpt:

```text
10  OpenHuman                     	       0x10523d088 std::sys::backtrace::__rust_begin_short_backtrace::hdb34a205259453e8 + 16
11  OpenHuman                     	       0x10522c42c std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h05989c1143d288cd + 116
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
14  OpenHuman                     	       0x105232e84 __rust_try + 32
15  OpenHuman                     	       0x10522c2ac std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::hf215137585e369ce + 224
16  OpenHuman                     	       0x10525acc0 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h123778ac55eae71d + 24
17  OpenHuman                     	       0x1057e9fdc std::sys::thread::unix::Thread::new::thread_start::hd42ed9e591878587 + 408
18  libsystem_pthread.dylib       	       0x18ac4dc58 _pthread_start + 136
19  libsystem_pthread.dylib       	       0x18ac48c1c thread_start + 8

Thread 93:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x18ac0d50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x18ac4e128 _pthread_cond_wait + 980
2   OpenHuman                     	       0x1056e2cec _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park_until::h3312b73289c83185 + 472
3   OpenHuman                     	       0x1056e1c84 parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hc2da40aa73586843 + 732
4   OpenHuman                     	       0x1056dfd6c parking_lot_core::parking_lot::park::h964bd64c3ea232d0 + 296
5   OpenHuman                     	       0x1056e36c8 parking_lot::condvar::Condvar::wait_until_internal::hb2630d95a9855c15 + 160
6   OpenHuman                     	       0x1052822fc parking_lot::condvar::Condvar::wait_for::h969488ffa1e51882 + 108
7   OpenHuman                     	       0x1052682e4 tokio::loom::std::parking_lot::Condvar::wait_timeout::h222f196a0b832f05 + 48
8   OpenHuman                     	       0x1052b5b28 tokio::runtime::blocking::pool::Inner::run::h01e32a9f9b2701bd + 664
9   OpenHuman                     	       0x1052b6974 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6de928fa040b7fb7 + 144
10  OpenHuman                     	       0x10523d088 std::sys::backtrace::__rust_begin_short_backtrace::hdb34a205259453e8 + 16
11  OpenHuman                     	       0x10522c42c std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h05989c1143d288cd + 116
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
14  OpenHuman                     	       0x105232e84 __rust_try + 32
15  OpenHuman                     	       0x10522c2ac std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::hf215137585e369ce + 224
16  OpenHuman                     	       0x10525acc0 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h123778ac55eae71d + 24
17  OpenHuman                     	       0x1057e9fdc std::sys::thread::unix::Thread::new::thread_start::hd42ed9e591878587 + 408
18  libsystem_pthread.dylib       	       0x18ac4dc58 _pthread_start + 136
19  libsystem_pthread.dylib       	       0x18ac48c1c thread_start + 8

Thread 94:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x18ac0d50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x18ac4e128 _pthread_cond_wait + 980

```

Output excerpt:

```text
[... 2 line(s) omitted ... ⟦tj:c5bda4cd31ffd7eb499659f8def693af⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 20 line(s) omitted ... ⟦tj:7e09a628d6f86c1d8d91e42dbeaf1e80⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 20 line(s) omitted ... ⟦tj:dfed89db860f683957c87e382f3f5df2⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
[... 242 line(s) omitted ... ⟦tj:7bbd5bb0d68c1edea5a3a6d442d03901⟧]
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 20 line(s) omitted ... ⟦tj:73a563b1f24659dbf93ad86c32c9b352⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 20 line(s) omitted ... ⟦tj:f14ea8d31db9cc27fe7407f5f465b57f⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 15 line(s) omitted ... ⟦tj:fa0fecc5aa274b8f0c2ded17d5c344f4⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (40969 bytes) is available by calling tinyjuice_retrieve with token "7aa04dead4a225a1cf760c5752e89302" (marker ⟦tj:7aa04dead4a225a1cf760c5752e89302⟧)]

```

### `06-openhuman-crash-slice-6`

- [Full input](cases/06-openhuman-crash-slice-6/input.log)
- [Full output](cases/06-openhuman-crash-slice-6/output.log)
- [Input vs output diff](cases/06-openhuman-crash-slice-6/compression.diff)

Input excerpt:

```text
16  OpenHuman                     	       0x10525acc0 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h123778ac55eae71d + 24
17  OpenHuman                     	       0x1057e9fdc std::sys::thread::unix::Thread::new::thread_start::hd42ed9e591878587 + 408
18  libsystem_pthread.dylib       	       0x18ac4dc58 _pthread_start + 136
19  libsystem_pthread.dylib       	       0x18ac48c1c thread_start + 8

Thread 61:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x18ac0d50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x18ac4e128 _pthread_cond_wait + 980
2   OpenHuman                     	       0x1056e2cec _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park_until::h3312b73289c83185 + 472
3   OpenHuman                     	       0x1056e1c84 parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hc2da40aa73586843 + 732
4   OpenHuman                     	       0x1056dfd6c parking_lot_core::parking_lot::park::h964bd64c3ea232d0 + 296
5   OpenHuman                     	       0x1056e36c8 parking_lot::condvar::Condvar::wait_until_internal::hb2630d95a9855c15 + 160
6   OpenHuman                     	       0x1052822fc parking_lot::condvar::Condvar::wait_for::h969488ffa1e51882 + 108
7   OpenHuman                     	       0x1052682e4 tokio::loom::std::parking_lot::Condvar::wait_timeout::h222f196a0b832f05 + 48
8   OpenHuman                     	       0x1052b5b28 tokio::runtime::blocking::pool::Inner::run::h01e32a9f9b2701bd + 664
9   OpenHuman                     	       0x1052b6974 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6de928fa040b7fb7 + 144
10  OpenHuman                     	       0x10523d088 std::sys::backtrace::__rust_begin_short_backtrace::hdb34a205259453e8 + 16
11  OpenHuman                     	       0x10522c42c std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h05989c1143d288cd + 116
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
14  OpenHuman                     	       0x105232e84 __rust_try + 32
15  OpenHuman                     	       0x10522c2ac std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::hf215137585e369ce + 224
16  OpenHuman                     	       0x10525acc0 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h123778ac55eae71d + 24
17  OpenHuman                     	       0x1057e9fdc std::sys::thread::unix::Thread::new::thread_start::hd42ed9e591878587 + 408
18  libsystem_pthread.dylib       	       0x18ac4dc58 _pthread_start + 136
19  libsystem_pthread.dylib       	       0x18ac48c1c thread_start + 8

Thread 62:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x18ac0d50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x18ac4e128 _pthread_cond_wait + 980
2   OpenHuman                     	       0x1056e2cec _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park_until::h3312b73289c83185 + 472
3   OpenHuman                     	       0x1056e1c84 parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hc2da40aa73586843 + 732
4   OpenHuman                     	       0x1056dfd6c parking_lot_core::parking_lot::park::h964bd64c3ea232d0 + 296
5   OpenHuman                     	       0x1056e36c8 parking_lot::condvar::Condvar::wait_until_internal::hb2630d95a9855c15 + 160
6   OpenHuman                     	       0x1052822fc parking_lot::condvar::Condvar::wait_for::h969488ffa1e51882 + 108
7   OpenHuman                     	       0x1052682e4 tokio::loom::std::parking_lot::Condvar::wait_timeout::h222f196a0b832f05 + 48

```

Output excerpt:

```text
[... 18 line(s) omitted ... ⟦tj:e9ae820d5971905a02614e30e34a3b06⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 20 line(s) omitted ... ⟦tj:2d77eb76f2debc4804b7d534f90fb336⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 20 line(s) omitted ... ⟦tj:7f1c466727227e62337239b540362669⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
[... 241 line(s) omitted ... ⟦tj:b420640daa84a9cb6a5ba9daa4cb4e84⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 20 line(s) omitted ... ⟦tj:61d1efb6eab84267a89085537d31e7f4⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 20 line(s) omitted ... ⟦tj:c7b8b061b696686bc774e5275dba2f2d⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (41041 bytes) is available by calling tinyjuice_retrieve with token "2ad9146dc43ecec3eaadc280722eb802" (marker ⟦tj:2ad9146dc43ecec3eaadc280722eb802⟧)]

```

### `09-openhuman-crash-slice-9`

- [Full input](cases/09-openhuman-crash-slice-9/input.log)
- [Full output](cases/09-openhuman-crash-slice-9/output.log)
- [Input vs output diff](cases/09-openhuman-crash-slice-9/compression.diff)

Input excerpt:

```text
7   OpenHuman                     	       0x1052682e4 tokio::loom::std::parking_lot::Condvar::wait_timeout::h222f196a0b832f05 + 48
8   OpenHuman                     	       0x1052b5b28 tokio::runtime::blocking::pool::Inner::run::h01e32a9f9b2701bd + 664
9   OpenHuman                     	       0x1052b6974 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6de928fa040b7fb7 + 144
10  OpenHuman                     	       0x10523d088 std::sys::backtrace::__rust_begin_short_backtrace::hdb34a205259453e8 + 16
11  OpenHuman                     	       0x10522c42c std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h05989c1143d288cd + 116
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
14  OpenHuman                     	       0x105232e84 __rust_try + 32
15  OpenHuman                     	       0x10522c2ac std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::hf215137585e369ce + 224
16  OpenHuman                     	       0x10525acc0 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h123778ac55eae71d + 24
17  OpenHuman                     	       0x1057e9fdc std::sys::thread::unix::Thread::new::thread_start::hd42ed9e591878587 + 408
18  libsystem_pthread.dylib       	       0x18ac4dc58 _pthread_start + 136
19  libsystem_pthread.dylib       	       0x18ac48c1c thread_start + 8

Thread 109:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x18ac0d50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x18ac4e128 _pthread_cond_wait + 980
2   OpenHuman                     	       0x1056e2cec _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park_until::h3312b73289c83185 + 472
3   OpenHuman                     	       0x1056e1c84 parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hc2da40aa73586843 + 732
4   OpenHuman                     	       0x1056dfd6c parking_lot_core::parking_lot::park::h964bd64c3ea232d0 + 296
5   OpenHuman                     	       0x1056e36c8 parking_lot::condvar::Condvar::wait_until_internal::hb2630d95a9855c15 + 160
6   OpenHuman                     	       0x1052822fc parking_lot::condvar::Condvar::wait_for::h969488ffa1e51882 + 108
7   OpenHuman                     	       0x1052682e4 tokio::loom::std::parking_lot::Condvar::wait_timeout::h222f196a0b832f05 + 48
8   OpenHuman                     	       0x1052b5b28 tokio::runtime::blocking::pool::Inner::run::h01e32a9f9b2701bd + 664
9   OpenHuman                     	       0x1052b6974 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6de928fa040b7fb7 + 144
10  OpenHuman                     	       0x10523d088 std::sys::backtrace::__rust_begin_short_backtrace::hdb34a205259453e8 + 16
11  OpenHuman                     	       0x10522c42c std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h05989c1143d288cd + 116
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
14  OpenHuman                     	       0x105232e84 __rust_try + 32
15  OpenHuman                     	       0x10522c2ac std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::hf215137585e369ce + 224
16  OpenHuman                     	       0x10525acc0 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h123778ac55eae71d + 24
17  OpenHuman                     	       0x1057e9fdc std::sys::thread::unix::Thread::new::thread_start::hd42ed9e591878587 + 408
18  libsystem_pthread.dylib       	       0x18ac4dc58 _pthread_start + 136
19  libsystem_pthread.dylib       	       0x18ac48c1c thread_start + 8


```

Output excerpt:

```text
[... 5 line(s) omitted ... ⟦tj:d2d76866f5bf437644fc39865742f76c⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 20 line(s) omitted ... ⟦tj:f39456300a8ce2274653fb2eea3bd6c0⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 20 line(s) omitted ... ⟦tj:09c4396646cf034b7953ac8ae7d96ad4⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
[... 175 line(s) omitted ... ⟦tj:b6f61c89925a85d833e32a3abfeb5bab⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 20 line(s) omitted ... ⟦tj:b6d4636051c9e19b166c73bf2ee7fae2⟧]
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
[... 80 line(s) omitted ... ⟦tj:bb9b204926cc34e943ea4c53300df8a1⟧]
   far: 0x00000003028534f0  esr: 0x92000047 (Data Abort) byte write Translation fault
[... 19 line(s) omitted ... ⟦tj:81544969cf456620f12493b3576c63f5⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (38092 bytes) is available by calling tinyjuice_retrieve with token "6f9fb0f1edabd33489a2e8931a812b81" (marker ⟦tj:6f9fb0f1edabd33489a2e8931a812b81⟧)]

```

### `10-openhuman-crash-slice-10`

- [Full input](cases/10-openhuman-crash-slice-10/input.log)
- [Full output](cases/10-openhuman-crash-slice-10/output.log)
- [Input vs output diff](cases/10-openhuman-crash-slice-10/compression.diff)

Input excerpt:

```text
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
    thread_set_state: 0
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
    thread_set_state: 0
  Calls made by all processes on this machine:
    task_for_pid: 0
    thread_create: 0
    thread_set_state: 0

VM Region Summary:
ReadOnly portion of Libraries: Total=2.2G resident=0K(0%) swapped_out_or_unallocated=2.2G(100%)
Writable regions: Total=756.0M written=1394K(0%) resident=1394K(0%) swapped_out=0K(0%) unallocated=754.7M(100%)

                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Accelerate framework               128K        1 
Activity Tracing                   256K        1 
AttributeGraph Data               1024K        1 
CG image                            48K        2 
ColorSync                           16K        1 
CoreAnimation                      368K       23 
CoreGraphics                        32K        2 
CoreServices                       288K        2 
CoreUI image data                  256K        2 
Foundation                          48K        2 
Image IO                            96K        6 
Kernel Alloc Once                   48K        2 
MALLOC                            37.7M       18 
MALLOC guard page                 3248K        4 
Mach message                       256K       10 

```

Output excerpt:

```text
[... 115 line(s) omitted ... ⟦tj:183581b05a611dba4de35233ce131bc6⟧]
  "exception" : {"codes":"0x0000000000000002, 0x00000003028534f0","message":"Could not determine thread index for stack guard region","rawCodes":[2,12927186160],"type":"EXC_BAD_ACCESS","signal":"SIGBUS","subtype":"KERN_P...
  "termination" : {"flags":0,"code":10,"namespace":"SIGNAL","indicator":"Bus error: 10","byProc":"exc handler","byPid":90825},
[... 1 line(s) omitted ... ⟦tj:0f962b960ec39ff2e3e86caad50ba2b5⟧]
  "extMods" : {"caller":{"thread_create":0,"thread_set_state":0,"task_for_pid":0},"system":{"thread_create":0,"thread_set_state":0,"task_for_pid":0},"targeted":{"thread_create":0,"thread_set_state":0,"task_for_pid":0},"w...
[... 1 line(s) omitted ... ⟦tj:484e1a5af23a1f1c3165bc88139edca6⟧]
  "threads" : [{"threadState":{"x":[{"value":268451845},{"value":21592279046},{"value":8589934592},{"value":28600187224064},{"value":0},{"value":28600187224064},{"value":2},{"value":4294967295},{"value":0},{"value":17179...
[... 219 line(s) omitted ... ⟦tj:a97ab27f84d373a498e463ee5b106b6c⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (493358 bytes) is available by calling tinyjuice_retrieve with token "23c47ce58e1cf696d3aa8b0f9e9e54ce" (marker ⟦tj:23c47ce58e1cf696d3aa8b0f9e9e54ce⟧)]

```

