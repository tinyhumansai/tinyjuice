# Service And Docker Logs

Real OpenHuman runtime crash-log slices, with live Docker OpenHuman logs used first when a container is available. The log compressor keeps incident signals and collapses repeated low-value lines.

Each row links to the full raw input and both compacted outputs. Percentages are **token reduction: higher is better**; 0% means pass-through. `Bytes` shows the raw input size -> compressor-only output size and its byte reduction. `Pass 1` disables CCR and is **lossless by construction**: faithful reshapes (JSON tables/minify, HTML->text) still ship because nothing is lost, but anything that *drops* detail (log lines, diff context, search matches, code bodies, sampled JSON rows) passes the original through untouched, since without the cache it could not be recovered. `Pass 2` enables CCR, so information-dropping compression is allowed — every dropped block is offloaded behind a retrieval token. Faithful reshapes (HTML->text) are identical in both passes (Pass 2 is marginally lower only for the recovery footer); pure information-dropping categories are 0% in Pass 1 and compress only in Pass 2; JSON is a hybrid — Pass 1 renders the full lossless markdown table (all rows), and Pass 2 additionally samples the middle away behind retrieval tokens. Each pass links its own output and its own diff against the input.

## Cases

Every case links to the raw input; each pass column carries its percentage plus that pass's exact output and a unified diff against the input.

| Case | Input | Bytes | Pass 1: no CCR | Pass 2: with CCR | Avg latency |
| --- | --- | ---: | ---: | ---: | ---: |
| `08-openhuman-crash-slice-8` | [input](cases/08-openhuman-crash-slice-8/input.log) | 41.0 KB -> 916 B (-98%) | 0.0%<br>[output](cases/08-openhuman-crash-slice-8/output-noccr.log) - [diff](cases/08-openhuman-crash-slice-8/compression-noccr.diff) | 97.5%<br>[output](cases/08-openhuman-crash-slice-8/output.log) - [diff](cases/08-openhuman-crash-slice-8/compression.diff) | 0.933 ms |
| `06-openhuman-crash-slice-6` | [input](cases/06-openhuman-crash-slice-6/input.log) | 41.0 KB -> 987 B (-98%) | 0.0%<br>[output](cases/06-openhuman-crash-slice-6/output-noccr.log) - [diff](cases/06-openhuman-crash-slice-6/compression-noccr.diff) | 97.3%<br>[output](cases/06-openhuman-crash-slice-6/output.log) - [diff](cases/06-openhuman-crash-slice-6/compression.diff) | 0.931 ms |
| `07-openhuman-crash-slice-7` | [input](cases/07-openhuman-crash-slice-7/input.log) | 40.9 KB -> 987 B (-98%) | 0.0%<br>[output](cases/07-openhuman-crash-slice-7/output-noccr.log) - [diff](cases/07-openhuman-crash-slice-7/compression-noccr.diff) | 97.3%<br>[output](cases/07-openhuman-crash-slice-7/output.log) - [diff](cases/07-openhuman-crash-slice-7/compression.diff) | 0.932 ms |
| `09-openhuman-crash-slice-9` | [input](cases/09-openhuman-crash-slice-9/input.log) | 38.1 KB -> 1.3 KB (-97%) | 0.0%<br>[output](cases/09-openhuman-crash-slice-9/output-noccr.log) - [diff](cases/09-openhuman-crash-slice-9/compression-noccr.diff) | 96.3%<br>[output](cases/09-openhuman-crash-slice-9/output.log) - [diff](cases/09-openhuman-crash-slice-9/compression.diff) | 0.955 ms |
| `03-openhuman-crash-slice-3` | [input](cases/03-openhuman-crash-slice-3/input.log) | 45.9 KB -> 2.1 KB (-95%) | 0.0%<br>[output](cases/03-openhuman-crash-slice-3/output-noccr.log) - [diff](cases/03-openhuman-crash-slice-3/compression-noccr.diff) | 95.2%<br>[output](cases/03-openhuman-crash-slice-3/output.log) - [diff](cases/03-openhuman-crash-slice-3/compression.diff) | 1.114 ms |
| `01-openhuman-crash-slice-1` | [input](cases/01-openhuman-crash-slice-1/input.log) | 26.6 KB -> 1.4 KB (-95%) | 0.0%<br>[output](cases/01-openhuman-crash-slice-1/output-noccr.log) - [diff](cases/01-openhuman-crash-slice-1/compression-noccr.diff) | 94.2%<br>[output](cases/01-openhuman-crash-slice-1/output.log) - [diff](cases/01-openhuman-crash-slice-1/compression.diff) | 1.010 ms |
| `04-openhuman-crash-slice-4` | [input](cases/04-openhuman-crash-slice-4/input.log) | 47.0 KB -> 3.2 KB (-93%) | 0.0%<br>[output](cases/04-openhuman-crash-slice-4/output-noccr.log) - [diff](cases/04-openhuman-crash-slice-4/compression-noccr.diff) | 92.9%<br>[output](cases/04-openhuman-crash-slice-4/output.log) - [diff](cases/04-openhuman-crash-slice-4/compression.diff) | 1.134 ms |
| `02-openhuman-crash-slice-2` | [input](cases/02-openhuman-crash-slice-2/input.log) | 29.7 KB -> 2.1 KB (-93%) | 0.0%<br>[output](cases/02-openhuman-crash-slice-2/output-noccr.log) - [diff](cases/02-openhuman-crash-slice-2/compression-noccr.diff) | 92.6%<br>[output](cases/02-openhuman-crash-slice-2/output.log) - [diff](cases/02-openhuman-crash-slice-2/compression.diff) | 0.890 ms |
| `05-openhuman-crash-slice-5` | [input](cases/05-openhuman-crash-slice-5/input.log) | 44.5 KB -> 3.2 KB (-93%) | 0.0%<br>[output](cases/05-openhuman-crash-slice-5/output-noccr.log) - [diff](cases/05-openhuman-crash-slice-5/compression-noccr.diff) | 92.5%<br>[output](cases/05-openhuman-crash-slice-5/output.log) - [diff](cases/05-openhuman-crash-slice-5/compression.diff) | 1.058 ms |
| `10-openhuman-crash-slice-10` | [input](cases/10-openhuman-crash-slice-10/input.log) | 493.4 KB -> 479.4 KB (-3%) | 0.0%<br>[output](cases/10-openhuman-crash-slice-10/output-noccr.log) - [diff](cases/10-openhuman-crash-slice-10/compression-noccr.diff) | 2.8%<br>[output](cases/10-openhuman-crash-slice-10/output.log) - [diff](cases/10-openhuman-crash-slice-10/compression.diff) | 3.587 ms |

## What TinyJuice Is Doing

The log path scores lines by signal. Errors, warnings, exception metadata, stack frames, and summaries are favored; repetitive routine lines are collapsed behind omission markers.

## Syntax-Aware Samples

### `08-openhuman-crash-slice-8`

- [Full input](cases/08-openhuman-crash-slice-8/input.log)
- [Output with CCR](cases/08-openhuman-crash-slice-8/output.log) - [diff](cases/08-openhuman-crash-slice-8/compression.diff)
- [Output without CCR](cases/08-openhuman-crash-slice-8/output-noccr.log) - [diff](cases/08-openhuman-crash-slice-8/compression-noccr.diff)

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
10  OpenHuman                     	       0x10523d088 std::sys::backtrace::__rust_begin_short_backtrace::hdb34a205259453e8 + 16
11  OpenHuman                     	       0x10522c42c std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h05989c1143d288cd + 116
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44  [×16]
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
14  OpenHuman                     	       0x105232e84 __rust_try + 32
[... 344 line(s) omitted ... ⟦tj:5b2084ac76c34f02c4ce515ba988a3da⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (40969 bytes): call tinyjuice_retrieve with token "7aa04dead4a225a1cf760c5752e89302"]

```

### `06-openhuman-crash-slice-6`

- [Full input](cases/06-openhuman-crash-slice-6/input.log)
- [Output with CCR](cases/06-openhuman-crash-slice-6/output.log) - [diff](cases/06-openhuman-crash-slice-6/compression.diff)
- [Output without CCR](cases/06-openhuman-crash-slice-6/output-noccr.log) - [diff](cases/06-openhuman-crash-slice-6/compression-noccr.diff)

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
[... 16 line(s) omitted ... ⟦tj:912ce6e5519da266f429e3aff464c8fd⟧]
10  OpenHuman                     	       0x10523d088 std::sys::backtrace::__rust_begin_short_backtrace::hdb34a205259453e8 + 16
11  OpenHuman                     	       0x10522c42c std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h05989c1143d288cd + 116
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44  [×16]
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
14  OpenHuman                     	       0x105232e84 __rust_try + 32
[... 328 line(s) omitted ... ⟦tj:bd8cd5f3009cf88b89c43ae5f41558d8⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (41041 bytes): call tinyjuice_retrieve with token "2ad9146dc43ecec3eaadc280722eb802"]

```

### `07-openhuman-crash-slice-7`

- [Full input](cases/07-openhuman-crash-slice-7/input.log)
- [Output with CCR](cases/07-openhuman-crash-slice-7/output.log) - [diff](cases/07-openhuman-crash-slice-7/compression.diff)
- [Output without CCR](cases/07-openhuman-crash-slice-7/output-noccr.log) - [diff](cases/07-openhuman-crash-slice-7/compression-noccr.diff)

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
[... 19 line(s) omitted ... ⟦tj:fb862d455ea2991cb47092bfffcbcc3d⟧]
10  OpenHuman                     	       0x10523d088 std::sys::backtrace::__rust_begin_short_backtrace::hdb34a205259453e8 + 16
11  OpenHuman                     	       0x10522c42c std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h05989c1143d288cd + 116
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44  [×15]
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
14  OpenHuman                     	       0x105232e84 __rust_try + 32
[... 325 line(s) omitted ... ⟦tj:6ddd9f356431ac7ba5968f12397b3ea8⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (40864 bytes): call tinyjuice_retrieve with token "86568fb7c4af4732716fbb38d5d077dc"]

```

### `09-openhuman-crash-slice-9`

- [Full input](cases/09-openhuman-crash-slice-9/input.log)
- [Output with CCR](cases/09-openhuman-crash-slice-9/output.log) - [diff](cases/09-openhuman-crash-slice-9/compression.diff)
- [Output without CCR](cases/09-openhuman-crash-slice-9/output-noccr.log) - [diff](cases/09-openhuman-crash-slice-9/compression-noccr.diff)

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
[... 3 line(s) omitted ... ⟦tj:638d7a3d69a2307b1f2f225752ef2d98⟧]
10  OpenHuman                     	       0x10523d088 std::sys::backtrace::__rust_begin_short_backtrace::hdb34a205259453e8 + 16
11  OpenHuman                     	       0x10522c42c std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h05989c1143d288cd + 116
12  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44  [×12]
13  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
14  OpenHuman                     	       0x105232e84 __rust_try + 32
[... 319 line(s) omitted ... ⟦tj:9a0c679d7bd4417182661d48d9d35b95⟧]
   x28: 0x0000000000000000   fp: 0x0000000302854480   lr: 0x000000010575a2e0
    sp: 0x00000003028534f0   pc: 0x00000001057554a0 cpsr: 0x20000000
   far: 0x00000003028534f0  esr: 0x92000047 (Data Abort) byte write Translation fault

Binary Images:
[... 17 line(s) omitted ... ⟦tj:f439ec7deac1ca58495748b87b2be525⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (38092 bytes): call tinyjuice_retrieve with token "6f9fb0f1edabd33489a2e8931a812b81"]

```

### `03-openhuman-crash-slice-3`

- [Full input](cases/03-openhuman-crash-slice-3/input.log)
- [Output with CCR](cases/03-openhuman-crash-slice-3/output.log) - [diff](cases/03-openhuman-crash-slice-3/compression.diff)
- [Output without CCR](cases/03-openhuman-crash-slice-3/output-noccr.log) - [diff](cases/03-openhuman-crash-slice-3/compression-noccr.diff)

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
[... 28 line(s) omitted ... ⟦tj:dac040076c8aef51bf760ae4fe1cd067⟧]
26  OpenHuman                     	       0x1052256c8 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h3c393af5c9cccfb5 + 72
27  OpenHuman                     	       0x1052142b8 tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hbb7135acba10cb1b + 64
28  OpenHuman                     	       0x10523cae4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::hd44b258fb813656b + 44  [×6]
29  OpenHuman                     	       0x1052a64b0 std::panicking::catch_unwind::do_call::h72377314c177ff75 + 64
30  OpenHuman                     	       0x1052965a4 __rust_try + 32
31  OpenHuman                     	       0x105291df0 std::panic::catch_unwind::hfd000fe64f5efc68 + 96  [×6]
32  OpenHuman                     	       0x105211138 tokio::runtime::task::harness::poll_future::h13c71e773a7b909e + 96
33  OpenHuman                     	       0x105215998 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h8d4c47df8e07256b + 172
[... 7 line(s) omitted ... ⟦tj:2ed0bc214763cab87cba05b7ba7365c5⟧]
41  OpenHuman                     	       0x10523d088 std::sys::backtrace::__rust_begin_short_backtrace::hdb34a205259453e8 + 16
42  OpenHuman                     	       0x10522c42c std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h05989c1143d288cd + 116
43  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44  [×6]
44  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
45  OpenHuman                     	       0x105232e84 __rust_try + 32
[... 301 line(s) omitted ... ⟦tj:6b13a9c891c475a92272f3f6e780bc63⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (45921 bytes): call tinyjuice_retrieve with token "30afbaeb44296119cf1f6a9da564a843"]

```

### `01-openhuman-crash-slice-1`

- [Full input](cases/01-openhuman-crash-slice-1/input.log)
- [Output with CCR](cases/01-openhuman-crash-slice-1/output.log) - [diff](cases/01-openhuman-crash-slice-1/compression.diff)
- [Output without CCR](cases/01-openhuman-crash-slice-1/output-noccr.log) - [diff](cases/01-openhuman-crash-slice-1/compression-noccr.diff)

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
[... 30 line(s) omitted ... ⟦tj:ad8f694445bc092da907dd089d52a2a6⟧]
Triggered by Thread: 46  tokio-rt-worker

Exception Type:    EXC_BAD_ACCESS (SIGBUS)
Exception Subtype: KERN_PROTECTION_FAILURE at 0x00000003028534f0
Exception Message: Could not determine thread index for stack guard region
Exception Codes:   0x0000000000000002, 0x00000003028534f0

Termination Reason:  Namespace SIGNAL, Code 10, Bus error: 10
Terminating Process: exc handler [90825]

[... 57 line(s) omitted ... ⟦tj:5f7973ee520da26f28f99ffb84251e3b⟧]
14  OpenHuman                     	       0x10177388c std::sys::backtrace::__rust_begin_short_backtrace::h9059a573f3ca8f30 + 16
15  OpenHuman                     	       0x101a79868 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc1ad5879cbb72eea + 124
16  OpenHuman                     	       0x1029a57bc _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h0abea6da5fd65be4 + 48
17  OpenHuman                     	       0x1024a5688 std::panicking::catch_unwind::do_call::h34447d5ada7f4206 + 56
18  OpenHuman                     	       0x101a7e64c __rust_try + 32
[... 247 line(s) omitted ... ⟦tj:065cf2f0a93ff544ced9ea3182409cd4⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (26592 bytes): call tinyjuice_retrieve with token "c87f7d774040584706d28f7d9035b8d5"]

```

### `04-openhuman-crash-slice-4`

- [Full input](cases/04-openhuman-crash-slice-4/input.log)
- [Output with CCR](cases/04-openhuman-crash-slice-4/output.log) - [diff](cases/04-openhuman-crash-slice-4/compression.diff)
- [Output without CCR](cases/04-openhuman-crash-slice-4/output-noccr.log) - [diff](cases/04-openhuman-crash-slice-4/compression-noccr.diff)

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
[... 26 line(s) omitted ... ⟦tj:d9b3973b7fb4a25ee616bdc170c99162⟧]
50  OpenHuman                     	       0x102b1762c tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h0b0bd6fe76c2b4df + 72
51  OpenHuman                     	       0x101d4dd34 tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::h76be7172bf1e86bc + 64
52  OpenHuman                     	       0x1029a4d30 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h04422aac1d5d487b + 44
53  OpenHuman                     	       0x1024b9fbc std::panicking::catch_unwind::do_call::hac75a22b40c0b34f + 64
54  OpenHuman                     	       0x101704b0c __rust_try + 32
55  OpenHuman                     	       0x1016b202c std::panic::catch_unwind::h272ebb504f50c3c9 + 96
56  OpenHuman                     	       0x101d3c9c0 tokio::runtime::task::harness::poll_future::hf88896021c669225 + 96
57  OpenHuman                     	       0x101d64228 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h006ddb55c96c1fee + 172
[... 19 line(s) omitted ... ⟦tj:302fdcf1664670954d676d344717ff5c⟧]
77  OpenHuman                     	       0x1052256c8 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h3c393af5c9cccfb5 + 72
78  OpenHuman                     	       0x1052142b8 tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hbb7135acba10cb1b + 64
79  OpenHuman                     	       0x10523cae4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::hd44b258fb813656b + 44  [×6]
80  OpenHuman                     	       0x1052a64b0 std::panicking::catch_unwind::do_call::h72377314c177ff75 + 64
81  OpenHuman                     	       0x1052965a4 __rust_try + 32
82  OpenHuman                     	       0x105291df0 std::panic::catch_unwind::hfd000fe64f5efc68 + 96  [×6]
83  OpenHuman                     	       0x105211138 tokio::runtime::task::harness::poll_future::h13c71e773a7b909e + 96
84  OpenHuman                     	       0x105215998 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h8d4c47df8e07256b + 172
[... 7 line(s) omitted ... ⟦tj:4b6fadd1276de519104d6b64e71e1a24⟧]
92  OpenHuman                     	       0x10523d088 std::sys::backtrace::__rust_begin_short_backtrace::hdb34a205259453e8 + 16
93  OpenHuman                     	       0x10522c42c std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h05989c1143d288cd + 116
94  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44  [×6]
95  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
96  OpenHuman                     	       0x105232e84 __rust_try + 32
[... 276 line(s) omitted ... ⟦tj:1526df8325ae83fde535d55f65d26137⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (46952 bytes): call tinyjuice_retrieve with token "68101d9a95ec3ff37b0d77c3cbc60a20"]

```

### `02-openhuman-crash-slice-2`

- [Full input](cases/02-openhuman-crash-slice-2/input.log)
- [Output with CCR](cases/02-openhuman-crash-slice-2/output.log) - [diff](cases/02-openhuman-crash-slice-2/compression.diff)
- [Output without CCR](cases/02-openhuman-crash-slice-2/output-noccr.log) - [diff](cases/02-openhuman-crash-slice-2/compression-noccr.diff)

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
[... 324 line(s) omitted ... ⟦tj:bcb99c5f052759cf6055d655d6e73e11⟧]
26  OpenHuman                     	       0x1052256c8 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h3c393af5c9cccfb5 + 72
27  OpenHuman                     	       0x1052142b8 tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hbb7135acba10cb1b + 64
28  OpenHuman                     	       0x10523cae4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::hd44b258fb813656b + 44
29  OpenHuman                     	       0x1052a64b0 std::panicking::catch_unwind::do_call::h72377314c177ff75 + 64
30  OpenHuman                     	       0x1052965a4 __rust_try + 32
31  OpenHuman                     	       0x105291df0 std::panic::catch_unwind::hfd000fe64f5efc68 + 96
32  OpenHuman                     	       0x105211138 tokio::runtime::task::harness::poll_future::h13c71e773a7b909e + 96
33  OpenHuman                     	       0x105215998 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h8d4c47df8e07256b + 172
[... 7 line(s) omitted ... ⟦tj:2ed0bc214763cab87cba05b7ba7365c5⟧]
41  OpenHuman                     	       0x10523d088 std::sys::backtrace::__rust_begin_short_backtrace::hdb34a205259453e8 + 16
42  OpenHuman                     	       0x10522c42c std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h05989c1143d288cd + 116
43  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44
44  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
45  OpenHuman                     	       0x105232e84 __rust_try + 32
[... 5 line(s) omitted ... ⟦tj:9e882b305f6417078d0d49f096c0b1d1⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (29717 bytes): call tinyjuice_retrieve with token "b2a16dcca94e32f601f4c27a80236f09"]

```

### `05-openhuman-crash-slice-5`

- [Full input](cases/05-openhuman-crash-slice-5/input.log)
- [Output with CCR](cases/05-openhuman-crash-slice-5/output.log) - [diff](cases/05-openhuman-crash-slice-5/compression.diff)
- [Output without CCR](cases/05-openhuman-crash-slice-5/output-noccr.log) - [diff](cases/05-openhuman-crash-slice-5/compression-noccr.diff)

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
[... 22 line(s) omitted ... ⟦tj:49e2b53c44cc5990003bed1d582f4336⟧]
26  OpenHuman                     	       0x1052256c8 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h3c393af5c9cccfb5 + 72
27  OpenHuman                     	       0x1052142b8 tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hbb7135acba10cb1b + 64
28  OpenHuman                     	       0x10523cae4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::hd44b258fb813656b + 44  [×5]
29  OpenHuman                     	       0x1052a64b0 std::panicking::catch_unwind::do_call::h72377314c177ff75 + 64
30  OpenHuman                     	       0x1052965a4 __rust_try + 32
31  OpenHuman                     	       0x105291df0 std::panic::catch_unwind::hfd000fe64f5efc68 + 96  [×5]
32  OpenHuman                     	       0x105211138 tokio::runtime::task::harness::poll_future::h13c71e773a7b909e + 96
33  OpenHuman                     	       0x105215998 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h8d4c47df8e07256b + 172
[... 7 line(s) omitted ... ⟦tj:2ed0bc214763cab87cba05b7ba7365c5⟧]
41  OpenHuman                     	       0x10523d088 std::sys::backtrace::__rust_begin_short_backtrace::hdb34a205259453e8 + 16
42  OpenHuman                     	       0x10522c42c std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h05989c1143d288cd + 116
43  OpenHuman                     	       0x10523c3b4 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h4c64399e54c61e3d + 44  [×9]
44  OpenHuman                     	       0x1052a62c4 std::panicking::catch_unwind::do_call::h6438cac469ea6444 + 52
45  OpenHuman                     	       0x105232e84 __rust_try + 32
[... 171 line(s) omitted ... ⟦tj:9cb8d408b083c5b5977e6934b13ce074⟧]
5   OpenHuman                     	       0x100e061d0 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h946b1d55626c17ed + 72
6   OpenHuman                     	       0x101019918 tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::h08b97b811664958b + 64
7   OpenHuman                     	       0x100f7c4cc _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::hccaeeb6618423b77 + 44
8   OpenHuman                     	       0x100ad5e90 std::panicking::catch_unwind::do_call::ha5f1569f211f9f27 + 64
9   OpenHuman                     	       0x101228fc8 __rust_try + 32
10  OpenHuman                     	       0x1012182dc std::panic::catch_unwind::h84bfe213f5791cd8 + 96
11  OpenHuman                     	       0x101008b78 tokio::runtime::task::harness::poll_future::h3476347798f06359 + 96
12  OpenHuman                     	       0x10102decc tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h7c360d239b2884fc + 172
[... 128 line(s) omitted ... ⟦tj:1aa190fac1fecf95b477de2bb6b56c5c⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (44535 bytes): call tinyjuice_retrieve with token "e4ac41e578f1d783c441d7b17c420ad6"]

```

### `10-openhuman-crash-slice-10`

- [Full input](cases/10-openhuman-crash-slice-10/input.log)
- [Output with CCR](cases/10-openhuman-crash-slice-10/output.log) - [diff](cases/10-openhuman-crash-slice-10/compression.diff)
- [Output without CCR](cases/10-openhuman-crash-slice-10/output-noccr.log) - [diff](cases/10-openhuman-crash-slice-10/compression-noccr.diff)

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
[... 113 line(s) omitted ... ⟦tj:a68fbc035877309605c925535a723e32⟧]
  "sip" : "enabled",
  "vmRegionInfo" : "0x3028534f0 is in 0x302850000-0x302854000;  bytes after start: 13552  bytes before end: 2831\n      REGION TYPE                    START - END         [ VSIZE] PRT\/MAX SHRMOD  REGION DETAIL\n      St...
  "exception" : {"codes":"0x0000000000000002, 0x00000003028534f0","message":"Could not determine thread index for stack guard region","rawCodes":[2,12927186160],"type":"EXC_BAD_ACCESS","signal":"SIGBUS","subtype":"KERN_P...
  "termination" : {"flags":0,"code":10,"namespace":"SIGNAL","indicator":"Bus error: 10","byProc":"exc handler","byPid":90825},
  "vmregioninfo" : "0x3028534f0 is in 0x302850000-0x302854000;  bytes after start: 13552  bytes before end: 2831\n      REGION TYPE                    START - END         [ VSIZE] PRT\/MAX SHRMOD  REGION DETAIL\n      St...
  "extMods" : {"caller":{"thread_create":0,"thread_set_state":0,"task_for_pid":0},"system":{"thread_create":0,"thread_set_state":0,"task_for_pid":0},"targeted":{"thread_create":0,"thread_set_state":0,"task_for_pid":0},"w...
  "faultingThread" : 46,
  "threads" : [{"threadState":{"x":[{"value":268451845},{"value":21592279046},{"value":8589934592},{"value":28600187224064},{"value":0},{"value":28600187224064},{"value":2},{"value":4294967295},{"value":0},{"value":17179...
  "usedImages" : [
  {
[... 217 line(s) omitted ... ⟦tj:8d84aa87c05fa9dc1eb372eea2023b87⟧]

[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (493358 bytes): call tinyjuice_retrieve with token "23c47ce58e1cf696d3aa8b0f9e9e54ce"]

```

