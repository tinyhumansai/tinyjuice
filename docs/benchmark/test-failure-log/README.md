# Test Failure Logs

Real OpenHuman Vitest command logs. The command-aware reducer keeps failure summaries and drops repetitive success or setup noise.

Each row links to the full raw input and both compacted outputs. Percentages are **token reduction: higher is better**; 0% means pass-through. `Bytes` shows the raw input size -> compressor-only output size and its byte reduction. `Pass 1` disables CCR and is **lossless by construction**: faithful reshapes (JSON tables/minify, HTML->text) still ship because nothing is lost, but anything that *drops* detail (log lines, diff context, search matches, code bodies, sampled JSON rows) passes the original through untouched, since without the cache it could not be recovered. `Pass 2` enables CCR, so information-dropping compression is allowed — every dropped block is offloaded behind a retrieval token. Faithful reshapes (HTML->text) are identical in both passes (Pass 2 is marginally lower only for the recovery footer); pure information-dropping categories are 0% in Pass 1 and compress only in Pass 2; JSON is a hybrid — Pass 1 renders the full lossless markdown table (all rows), and Pass 2 additionally samples the middle away behind retrieval tokens. Each pass links its own output and its own diff against the input.

## Cases

Every case links to the raw input; each pass column carries its percentage plus that pass's exact output and a unified diff against the input.

| Case | Input | Bytes | Pass 1: no CCR | Pass 2: with CCR | Avg latency |
| --- | --- | ---: | ---: | ---: | ---: |
| `07-vitest-excerpt-7` | [input](cases/07-vitest-excerpt-7/input.log) | 5.6 KB -> 1.2 KB (-78%) | 0.0%<br>[output](cases/07-vitest-excerpt-7/output-noccr.log) - [diff](cases/07-vitest-excerpt-7/compression-noccr.diff) | 77.3%<br>[output](cases/07-vitest-excerpt-7/output.log) - [diff](cases/07-vitest-excerpt-7/compression.diff) | 0.230 ms |
| `01-vitest-unit-20260704-234941` | [input](cases/01-vitest-unit-20260704-234941/input.log) | 2.8 KB -> 792 B (-71%) | 0.0%<br>[output](cases/01-vitest-unit-20260704-234941/output-noccr.log) - [diff](cases/01-vitest-unit-20260704-234941/compression-noccr.diff) | 69.2%<br>[output](cases/01-vitest-unit-20260704-234941/output.log) - [diff](cases/01-vitest-unit-20260704-234941/compression.diff) | 0.140 ms |
| `10-vitest-excerpt-10` | [input](cases/10-vitest-excerpt-10/input.log) | 1.9 KB -> 1.9 KB (-0%) | 0.0%<br>[output](cases/10-vitest-excerpt-10/output-noccr.log) - [diff](cases/10-vitest-excerpt-10/compression-noccr.diff) | 72.7%<br>[output](cases/10-vitest-excerpt-10/output.log) - [diff](cases/10-vitest-excerpt-10/compression.diff) | 0.080 ms |
| `09-vitest-excerpt-9` | [input](cases/09-vitest-excerpt-9/input.log) | 1.9 KB -> 1.9 KB (-0%) | 0.0%<br>[output](cases/09-vitest-excerpt-9/output-noccr.log) - [diff](cases/09-vitest-excerpt-9/compression-noccr.diff) | 72.7%<br>[output](cases/09-vitest-excerpt-9/output.log) - [diff](cases/09-vitest-excerpt-9/compression.diff) | 0.080 ms |
| `08-vitest-excerpt-8` | [input](cases/08-vitest-excerpt-8/input.log) | 1.9 KB -> 1.9 KB (-0%) | 0.0%<br>[output](cases/08-vitest-excerpt-8/output-noccr.log) - [diff](cases/08-vitest-excerpt-8/compression-noccr.diff) | 72.6%<br>[output](cases/08-vitest-excerpt-8/output.log) - [diff](cases/08-vitest-excerpt-8/compression.diff) | 0.084 ms |
| `04-vitest-unit-20260704-235125` | [input](cases/04-vitest-unit-20260704-235125/input.log) | 967 B -> 967 B (-0%) | 0.0%<br>[output](cases/04-vitest-unit-20260704-235125/output-noccr.log) - [diff](cases/04-vitest-unit-20260704-235125/compression-noccr.diff) | 66.5%<br>[output](cases/04-vitest-unit-20260704-235125/output.log) - [diff](cases/04-vitest-unit-20260704-235125/compression.diff) | 0.046 ms |
| `03-vitest-unit-20260704-235052` | [input](cases/03-vitest-unit-20260704-235052/input.log) | 967 B -> 967 B (-0%) | 0.0%<br>[output](cases/03-vitest-unit-20260704-235052/output-noccr.log) - [diff](cases/03-vitest-unit-20260704-235052/compression-noccr.diff) | 66.5%<br>[output](cases/03-vitest-unit-20260704-235052/output.log) - [diff](cases/03-vitest-unit-20260704-235052/compression.diff) | 0.047 ms |
| `06-vitest-unit-20260704-235240` | [input](cases/06-vitest-unit-20260704-235240/input.log) | 971 B -> 971 B (-0%) | 0.0%<br>[output](cases/06-vitest-unit-20260704-235240/output-noccr.log) - [diff](cases/06-vitest-unit-20260704-235240/compression-noccr.diff) | 66.3%<br>[output](cases/06-vitest-unit-20260704-235240/output.log) - [diff](cases/06-vitest-unit-20260704-235240/compression.diff) | 0.045 ms |
| `05-vitest-unit-20260704-235231` | [input](cases/05-vitest-unit-20260704-235231/input.log) | 970 B -> 970 B (-0%) | 0.0%<br>[output](cases/05-vitest-unit-20260704-235231/output-noccr.log) - [diff](cases/05-vitest-unit-20260704-235231/compression-noccr.diff) | 66.3%<br>[output](cases/05-vitest-unit-20260704-235231/output.log) - [diff](cases/05-vitest-unit-20260704-235231/compression.diff) | 0.046 ms |
| `02-vitest-unit-20260704-234958` | [input](cases/02-vitest-unit-20260704-234958/input.log) | 969 B -> 969 B (-0%) | 0.0%<br>[output](cases/02-vitest-unit-20260704-234958/output-noccr.log) - [diff](cases/02-vitest-unit-20260704-234958/compression-noccr.diff) | 66.3%<br>[output](cases/02-vitest-unit-20260704-234958/output.log) - [diff](cases/02-vitest-unit-20260704-234958/compression.diff) | 0.050 ms |

## What TinyJuice Is Doing

The command context routes these logs through the Vitest rule. Setup chatter and repeated passing output are removed, while failure blocks, summaries, and locations remain visible.

## Syntax-Aware Samples

### `07-vitest-excerpt-7`

- [Full input](cases/07-vitest-excerpt-7/input.log)
- [Output with CCR](cases/07-vitest-excerpt-7/output.log) - [diff](cases/07-vitest-excerpt-7/compression.diff)
- [Output without CCR](cases/07-vitest-excerpt-7/output-noccr.log) - [diff](cases/07-vitest-excerpt-7/compression-noccr.diff)

Input excerpt:

```text
11:49:42 PM [vite] warning: `esbuild` option was specified by "vite-plugin-node-polyfills" plugin. This option is deprecated, please use `oxc` instead.
Both esbuild and oxc options were set. oxc options will be used and esbuild options will be ignored. The following esbuild options were set: `{
  banner: "import __buffer_polyfill from 'vite-plugin-node-polyfills/shims/buffer'\n" +
    'globalThis.Buffer = globalThis.Buffer || __buffer_polyfill\n' +
    "import __global_polyfill from 'vite-plugin-node-polyfills/shims/global'\n" +
    'globalThis.global = globalThis.global || __global_polyfill\n' +
    "import __process_polyfill from 'vite-plugin-node-polyfills/shims/process'\n" +
    'globalThis.process = globalThis.process || __process_polyfill\n'
}`

 RUN  v4.1.5 <OPENHUMAN_ROOT>/app

stdout | src/components/intelligence/__tests__/IntelligenceSubconsciousTab.test.tsx
[MockServer] Listening on http://127.0.0.1:5005

stdout | src/components/intelligence/__tests__/IntelligenceSubconsciousTab.test.tsx
[MockServer] Stopped

 ❯ src/components/intelligence/__tests__/IntelligenceSubconsciousTab.test.tsx (9 tests | 1 failed) 80ms
     × per-kind spinner: only the triggering kind spins 8ms

⎯⎯⎯⎯⎯⎯⎯ Failed Tests 1 ⎯⎯⎯⎯⎯⎯⎯

 FAIL  src/components/intelligence/__tests__/IntelligenceSubconsciousTab.test.tsx > IntelligenceSubconsciousTab > per-kind spinner: only the triggering kind spins
Error: expect(element).not.toBeInTheDocument()

expected document not to contain element, found <button
  class="inline-flex items-center justify-center gap-2 font-medium transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:opacity-40 disabled:pointer-events-no...
  disabled=""
  type="button"
>
  <div
    class="w-3 h-3 border border-stone-400 border-t-transparent rounded-full animate-spin"
  />
  Run review now
</button> instead

```

Output excerpt:

```text
exit 101
4 failed suites, 8 failures
RUN  v4.1.5 <OPENHUMAN_ROOT>/app
 ❯ src/components/intelligence/__tests__/IntelligenceSubconsciousTab.test.tsx (9 tests | 1 failed) 80ms
⎯⎯⎯⎯⎯⎯⎯ Failed Tests 1 ⎯⎯⎯⎯⎯⎯⎯
 FAIL  src/components/intelligence/__tests__/IntelligenceSubconsciousTab.test.tsx > IntelligenceSubconsciousTab > per-kind spinner: only the triggering kind spins
Error: expect(element).not.toBeInTheDocument()
 ❯ src/components/intelligence/__tests__/IntelligenceSubconsciousTab.test.tsx:144:54
⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[1/1]⎯
 Test Files  1 failed (1)
      Tests  1 failed | 8 passed (9)
   Start at  23:49:42
   Duration  2.19s (transform 1.29s, setup 307ms, import 1.32s, tests 80ms, environment 403ms)
 RUN  v4.1.5 <OPENHUMAN_ROOT>/app
... omitted ...
Error: expect(element).not.toBeInTheDocument()
 ❯ src/components/intelligence/__tests__/IntelligenceSubconsciousTab.test.tsx:144:54
⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[1/1]⎯
 Test Files  1 failed (1)
      Tests  1 failed | 8 passed (9)
   Start at  23:49:42
   Duration  2.19s (transform 1.29s, setup 307ms, import 1.32s, tests 80ms, environment 403ms)

[PARTIAL view — full original (5554 bytes): call tinyjuice_retrieve with token "6662f8a36798fc5c20e4a63b3b70c5fe"]

```

### `10-vitest-excerpt-10`

- [Full input](cases/10-vitest-excerpt-10/input.log)
- [Output with CCR](cases/10-vitest-excerpt-10/output.log) - [diff](cases/10-vitest-excerpt-10/compression.diff)
- [Output without CCR](cases/10-vitest-excerpt-10/output-noccr.log) - [diff](cases/10-vitest-excerpt-10/compression-noccr.diff)

Input excerpt:

```text
11:51:25 PM [vite] warning: `esbuild` option was specified by "vite-plugin-node-polyfills" plugin. This option is deprecated, please use `oxc` instead.
Both esbuild and oxc options were set. oxc options will be used and esbuild options will be ignored. The following esbuild options were set: `{
  banner: "import __buffer_polyfill from 'vite-plugin-node-polyfills/shims/buffer'\n" +
    'globalThis.Buffer = globalThis.Buffer || __buffer_polyfill\n' +
    "import __global_polyfill from 'vite-plugin-node-polyfills/shims/global'\n" +
    'globalThis.global = globalThis.global || __global_polyfill\n' +
    "import __process_polyfill from 'vite-plugin-node-polyfills/shims/process'\n" +
    'globalThis.process = globalThis.process || __process_polyfill\n'
}`

 RUN  v4.1.5 <OPENHUMAN_ROOT>/app


 Test Files  1 passed (1)
      Tests  3 passed (3)
   Start at  23:51:26
   Duration  713ms (transform 116ms, setup 273ms, import 11ms, tests 10ms, environment 348ms)

11:51:25 PM [vite] warning: `esbuild` option was specified by "vite-plugin-node-polyfills" plugin. This option is deprecated, please use `oxc` instead.
Both esbuild and oxc options were set. oxc options will be used and esbuild options will be ignored. The following esbuild options were set: `{
  banner: "import __buffer_polyfill from 'vite-plugin-node-polyfills/shims/buffer'\n" +
    'globalThis.Buffer = globalThis.Buffer || __buffer_polyfill\n' +
    "import __global_polyfill from 'vite-plugin-node-polyfills/shims/global'\n" +
    'globalThis.global = globalThis.global || __global_polyfill\n' +
    "import __process_polyfill from 'vite-plugin-node-polyfills/shims/process'\n" +
    'globalThis.process = globalThis.process || __process_polyfill\n'
}`

 RUN  v4.1.5 <OPENHUMAN_ROOT>/app


 Test Files  1 passed (1)
      Tests  3 passed (3)
   Start at  23:51:26
   Duration  713ms (transform 116ms, setup 273ms, import 11ms, tests 10ms, environment 348ms)


```

Output excerpt:

```text
exit 101
RUN  v4.1.5 <OPENHUMAN_ROOT>/app
 Test Files  1 passed (1)
      Tests  3 passed (3)
   Start at  23:51:26
   Duration  713ms (transform 116ms, setup 273ms, import 11ms, tests 10ms, environment 348ms)
 RUN  v4.1.5 <OPENHUMAN_ROOT>/app
 Test Files  1 passed (1)
      Tests  3 passed (3)
   Start at  23:51:26
   Duration  713ms (transform 116ms, setup 273ms, import 11ms, tests 10ms, environment 348ms)

[PARTIAL view — full original (1934 bytes): call tinyjuice_retrieve with token "13daad6e3388bc546954e3f132c0fbac"]

```

### `09-vitest-excerpt-9`

- [Full input](cases/09-vitest-excerpt-9/input.log)
- [Output with CCR](cases/09-vitest-excerpt-9/output.log) - [diff](cases/09-vitest-excerpt-9/compression.diff)
- [Output without CCR](cases/09-vitest-excerpt-9/output-noccr.log) - [diff](cases/09-vitest-excerpt-9/compression-noccr.diff)

Input excerpt:

```text
11:50:52 PM [vite] warning: `esbuild` option was specified by "vite-plugin-node-polyfills" plugin. This option is deprecated, please use `oxc` instead.
Both esbuild and oxc options were set. oxc options will be used and esbuild options will be ignored. The following esbuild options were set: `{
  banner: "import __buffer_polyfill from 'vite-plugin-node-polyfills/shims/buffer'\n" +
    'globalThis.Buffer = globalThis.Buffer || __buffer_polyfill\n' +
    "import __global_polyfill from 'vite-plugin-node-polyfills/shims/global'\n" +
    'globalThis.global = globalThis.global || __global_polyfill\n' +
    "import __process_polyfill from 'vite-plugin-node-polyfills/shims/process'\n" +
    'globalThis.process = globalThis.process || __process_polyfill\n'
}`

 RUN  v4.1.5 <OPENHUMAN_ROOT>/app


 Test Files  1 passed (1)
      Tests  6 passed (6)
   Start at  23:50:52
   Duration  1.28s (transform 363ms, setup 598ms, import 28ms, tests 57ms, environment 491ms)

11:50:52 PM [vite] warning: `esbuild` option was specified by "vite-plugin-node-polyfills" plugin. This option is deprecated, please use `oxc` instead.
Both esbuild and oxc options were set. oxc options will be used and esbuild options will be ignored. The following esbuild options were set: `{
  banner: "import __buffer_polyfill from 'vite-plugin-node-polyfills/shims/buffer'\n" +
    'globalThis.Buffer = globalThis.Buffer || __buffer_polyfill\n' +
    "import __global_polyfill from 'vite-plugin-node-polyfills/shims/global'\n" +
    'globalThis.global = globalThis.global || __global_polyfill\n' +
    "import __process_polyfill from 'vite-plugin-node-polyfills/shims/process'\n" +
    'globalThis.process = globalThis.process || __process_polyfill\n'
}`

 RUN  v4.1.5 <OPENHUMAN_ROOT>/app


 Test Files  1 passed (1)
      Tests  6 passed (6)
   Start at  23:50:52
   Duration  1.28s (transform 363ms, setup 598ms, import 28ms, tests 57ms, environment 491ms)


```

Output excerpt:

```text
exit 101
RUN  v4.1.5 <OPENHUMAN_ROOT>/app
 Test Files  1 passed (1)
      Tests  6 passed (6)
   Start at  23:50:52
   Duration  1.28s (transform 363ms, setup 598ms, import 28ms, tests 57ms, environment 491ms)
 RUN  v4.1.5 <OPENHUMAN_ROOT>/app
 Test Files  1 passed (1)
      Tests  6 passed (6)
   Start at  23:50:52
   Duration  1.28s (transform 363ms, setup 598ms, import 28ms, tests 57ms, environment 491ms)

[PARTIAL view — full original (1934 bytes): call tinyjuice_retrieve with token "c741b45480ff49d2da14c8f5ebe4e264"]

```

### `08-vitest-excerpt-8`

- [Full input](cases/08-vitest-excerpt-8/input.log)
- [Output with CCR](cases/08-vitest-excerpt-8/output.log) - [diff](cases/08-vitest-excerpt-8/compression.diff)
- [Output without CCR](cases/08-vitest-excerpt-8/output-noccr.log) - [diff](cases/08-vitest-excerpt-8/compression-noccr.diff)

Input excerpt:

```text
11:49:59 PM [vite] warning: `esbuild` option was specified by "vite-plugin-node-polyfills" plugin. This option is deprecated, please use `oxc` instead.
Both esbuild and oxc options were set. oxc options will be used and esbuild options will be ignored. The following esbuild options were set: `{
  banner: "import __buffer_polyfill from 'vite-plugin-node-polyfills/shims/buffer'\n" +
    'globalThis.Buffer = globalThis.Buffer || __buffer_polyfill\n' +
    "import __global_polyfill from 'vite-plugin-node-polyfills/shims/global'\n" +
    'globalThis.global = globalThis.global || __global_polyfill\n' +
    "import __process_polyfill from 'vite-plugin-node-polyfills/shims/process'\n" +
    'globalThis.process = globalThis.process || __process_polyfill\n'
}`

 RUN  v4.1.5 <OPENHUMAN_ROOT>/app


 Test Files  1 passed (1)
      Tests  9 passed (9)
   Start at  23:49:59
   Duration  2.33s (transform 1.41s, setup 263ms, import 1.43s, tests 116ms, environment 302ms)

11:49:59 PM [vite] warning: `esbuild` option was specified by "vite-plugin-node-polyfills" plugin. This option is deprecated, please use `oxc` instead.
Both esbuild and oxc options were set. oxc options will be used and esbuild options will be ignored. The following esbuild options were set: `{
  banner: "import __buffer_polyfill from 'vite-plugin-node-polyfills/shims/buffer'\n" +
    'globalThis.Buffer = globalThis.Buffer || __buffer_polyfill\n' +
    "import __global_polyfill from 'vite-plugin-node-polyfills/shims/global'\n" +
    'globalThis.global = globalThis.global || __global_polyfill\n' +
    "import __process_polyfill from 'vite-plugin-node-polyfills/shims/process'\n" +
    'globalThis.process = globalThis.process || __process_polyfill\n'
}`

 RUN  v4.1.5 <OPENHUMAN_ROOT>/app


 Test Files  1 passed (1)
      Tests  9 passed (9)
   Start at  23:49:59
   Duration  2.33s (transform 1.41s, setup 263ms, import 1.43s, tests 116ms, environment 302ms)


```

Output excerpt:

```text
exit 101
RUN  v4.1.5 <OPENHUMAN_ROOT>/app
 Test Files  1 passed (1)
      Tests  9 passed (9)
   Start at  23:49:59
   Duration  2.33s (transform 1.41s, setup 263ms, import 1.43s, tests 116ms, environment 302ms)
 RUN  v4.1.5 <OPENHUMAN_ROOT>/app
 Test Files  1 passed (1)
      Tests  9 passed (9)
   Start at  23:49:59
   Duration  2.33s (transform 1.41s, setup 263ms, import 1.43s, tests 116ms, environment 302ms)

[PARTIAL view — full original (1938 bytes): call tinyjuice_retrieve with token "8e829b53a3eba9d8d543876623d5fd81"]

```

### `01-vitest-unit-20260704-234941`

- [Full input](cases/01-vitest-unit-20260704-234941/input.log)
- [Output with CCR](cases/01-vitest-unit-20260704-234941/output.log) - [diff](cases/01-vitest-unit-20260704-234941/compression.diff)
- [Output without CCR](cases/01-vitest-unit-20260704-234941/output-noccr.log) - [diff](cases/01-vitest-unit-20260704-234941/compression-noccr.diff)

Input excerpt:

```text
11:49:42 PM [vite] warning: `esbuild` option was specified by "vite-plugin-node-polyfills" plugin. This option is deprecated, please use `oxc` instead.
Both esbuild and oxc options were set. oxc options will be used and esbuild options will be ignored. The following esbuild options were set: `{
  banner: "import __buffer_polyfill from 'vite-plugin-node-polyfills/shims/buffer'\n" +
    'globalThis.Buffer = globalThis.Buffer || __buffer_polyfill\n' +
    "import __global_polyfill from 'vite-plugin-node-polyfills/shims/global'\n" +
    'globalThis.global = globalThis.global || __global_polyfill\n' +
    "import __process_polyfill from 'vite-plugin-node-polyfills/shims/process'\n" +
    'globalThis.process = globalThis.process || __process_polyfill\n'
}`

 RUN  v4.1.5 <OPENHUMAN_ROOT>/app

stdout | src/components/intelligence/__tests__/IntelligenceSubconsciousTab.test.tsx
[MockServer] Listening on http://127.0.0.1:5005

stdout | src/components/intelligence/__tests__/IntelligenceSubconsciousTab.test.tsx
[MockServer] Stopped

 ❯ src/components/intelligence/__tests__/IntelligenceSubconsciousTab.test.tsx (9 tests | 1 failed) 80ms
     × per-kind spinner: only the triggering kind spins 8ms

⎯⎯⎯⎯⎯⎯⎯ Failed Tests 1 ⎯⎯⎯⎯⎯⎯⎯

 FAIL  src/components/intelligence/__tests__/IntelligenceSubconsciousTab.test.tsx > IntelligenceSubconsciousTab > per-kind spinner: only the triggering kind spins
Error: expect(element).not.toBeInTheDocument()

expected document not to contain element, found <button
  class="inline-flex items-center justify-center gap-2 font-medium transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:opacity-40 disabled:pointer-events-no...
  disabled=""
  type="button"
>
  <div
    class="w-3 h-3 border border-stone-400 border-t-transparent rounded-full animate-spin"
  />
  Run review now
</button> instead

```

Output excerpt:

```text
exit 101
2 failed suites, 4 failures
RUN  v4.1.5 <OPENHUMAN_ROOT>/app
 ❯ src/components/intelligence/__tests__/IntelligenceSubconsciousTab.test.tsx (9 tests | 1 failed) 80ms
⎯⎯⎯⎯⎯⎯⎯ Failed Tests 1 ⎯⎯⎯⎯⎯⎯⎯
 FAIL  src/components/intelligence/__tests__/IntelligenceSubconsciousTab.test.tsx > IntelligenceSubconsciousTab > per-kind spinner: only the triggering kind spins
Error: expect(element).not.toBeInTheDocument()
 ❯ src/components/intelligence/__tests__/IntelligenceSubconsciousTab.test.tsx:144:54
⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[1/1]⎯
 Test Files  1 failed (1)
      Tests  1 failed | 8 passed (9)
   Start at  23:49:42
   Duration  2.19s (transform 1.29s, setup 307ms, import 1.32s, tests 80ms, environment 403ms)

[PARTIAL view — full original (2777 bytes): call tinyjuice_retrieve with token "86eb51aa134ea50382aa89d95827b1b5"]

```

### `04-vitest-unit-20260704-235125`

- [Full input](cases/04-vitest-unit-20260704-235125/input.log)
- [Output with CCR](cases/04-vitest-unit-20260704-235125/output.log) - [diff](cases/04-vitest-unit-20260704-235125/compression.diff)
- [Output without CCR](cases/04-vitest-unit-20260704-235125/output-noccr.log) - [diff](cases/04-vitest-unit-20260704-235125/compression-noccr.diff)

Input excerpt:

```text
11:51:25 PM [vite] warning: `esbuild` option was specified by "vite-plugin-node-polyfills" plugin. This option is deprecated, please use `oxc` instead.
Both esbuild and oxc options were set. oxc options will be used and esbuild options will be ignored. The following esbuild options were set: `{
  banner: "import __buffer_polyfill from 'vite-plugin-node-polyfills/shims/buffer'\n" +
    'globalThis.Buffer = globalThis.Buffer || __buffer_polyfill\n' +
    "import __global_polyfill from 'vite-plugin-node-polyfills/shims/global'\n" +
    'globalThis.global = globalThis.global || __global_polyfill\n' +
    "import __process_polyfill from 'vite-plugin-node-polyfills/shims/process'\n" +
    'globalThis.process = globalThis.process || __process_polyfill\n'
}`

 RUN  v4.1.5 <OPENHUMAN_ROOT>/app


 Test Files  1 passed (1)
      Tests  3 passed (3)
   Start at  23:51:26
   Duration  713ms (transform 116ms, setup 273ms, import 11ms, tests 10ms, environment 348ms)


```

Output excerpt:

```text
exit 101
RUN  v4.1.5 <OPENHUMAN_ROOT>/app
 Test Files  1 passed (1)
      Tests  3 passed (3)
   Start at  23:51:26
   Duration  713ms (transform 116ms, setup 273ms, import 11ms, tests 10ms, environment 348ms)

[PARTIAL view — full original (967 bytes): call tinyjuice_retrieve with token "cfa1a1d241b616e925c4cfe672c6e5ba"]

```

### `03-vitest-unit-20260704-235052`

- [Full input](cases/03-vitest-unit-20260704-235052/input.log)
- [Output with CCR](cases/03-vitest-unit-20260704-235052/output.log) - [diff](cases/03-vitest-unit-20260704-235052/compression.diff)
- [Output without CCR](cases/03-vitest-unit-20260704-235052/output-noccr.log) - [diff](cases/03-vitest-unit-20260704-235052/compression-noccr.diff)

Input excerpt:

```text
11:50:52 PM [vite] warning: `esbuild` option was specified by "vite-plugin-node-polyfills" plugin. This option is deprecated, please use `oxc` instead.
Both esbuild and oxc options were set. oxc options will be used and esbuild options will be ignored. The following esbuild options were set: `{
  banner: "import __buffer_polyfill from 'vite-plugin-node-polyfills/shims/buffer'\n" +
    'globalThis.Buffer = globalThis.Buffer || __buffer_polyfill\n' +
    "import __global_polyfill from 'vite-plugin-node-polyfills/shims/global'\n" +
    'globalThis.global = globalThis.global || __global_polyfill\n' +
    "import __process_polyfill from 'vite-plugin-node-polyfills/shims/process'\n" +
    'globalThis.process = globalThis.process || __process_polyfill\n'
}`

 RUN  v4.1.5 <OPENHUMAN_ROOT>/app


 Test Files  1 passed (1)
      Tests  6 passed (6)
   Start at  23:50:52
   Duration  1.28s (transform 363ms, setup 598ms, import 28ms, tests 57ms, environment 491ms)


```

Output excerpt:

```text
exit 101
RUN  v4.1.5 <OPENHUMAN_ROOT>/app
 Test Files  1 passed (1)
      Tests  6 passed (6)
   Start at  23:50:52
   Duration  1.28s (transform 363ms, setup 598ms, import 28ms, tests 57ms, environment 491ms)

[PARTIAL view — full original (967 bytes): call tinyjuice_retrieve with token "4a03c2794bc9e773675b9a4c162b5954"]

```

### `06-vitest-unit-20260704-235240`

- [Full input](cases/06-vitest-unit-20260704-235240/input.log)
- [Output with CCR](cases/06-vitest-unit-20260704-235240/output.log) - [diff](cases/06-vitest-unit-20260704-235240/compression.diff)
- [Output without CCR](cases/06-vitest-unit-20260704-235240/output-noccr.log) - [diff](cases/06-vitest-unit-20260704-235240/compression-noccr.diff)

Input excerpt:

```text
11:52:41 PM [vite] warning: `esbuild` option was specified by "vite-plugin-node-polyfills" plugin. This option is deprecated, please use `oxc` instead.
Both esbuild and oxc options were set. oxc options will be used and esbuild options will be ignored. The following esbuild options were set: `{
  banner: "import __buffer_polyfill from 'vite-plugin-node-polyfills/shims/buffer'\n" +
    'globalThis.Buffer = globalThis.Buffer || __buffer_polyfill\n' +
    "import __global_polyfill from 'vite-plugin-node-polyfills/shims/global'\n" +
    'globalThis.global = globalThis.global || __global_polyfill\n' +
    "import __process_polyfill from 'vite-plugin-node-polyfills/shims/process'\n" +
    'globalThis.process = globalThis.process || __process_polyfill\n'
}`

 RUN  v4.1.5 <OPENHUMAN_ROOT>/app


 Test Files  8 passed (8)
      Tests  69 passed (69)
   Start at  23:52:41
   Duration  5.07s (transform 1.45s, setup 763ms, import 1.64s, tests 600ms, environment 1.53s)


```

Output excerpt:

```text
exit 101
RUN  v4.1.5 <OPENHUMAN_ROOT>/app
 Test Files  8 passed (8)
      Tests  69 passed (69)
   Start at  23:52:41
   Duration  5.07s (transform 1.45s, setup 763ms, import 1.64s, tests 600ms, environment 1.53s)

[PARTIAL view — full original (971 bytes): call tinyjuice_retrieve with token "2dae148f0f15a4aa336dc8e45977a35f"]

```

### `05-vitest-unit-20260704-235231`

- [Full input](cases/05-vitest-unit-20260704-235231/input.log)
- [Output with CCR](cases/05-vitest-unit-20260704-235231/output.log) - [diff](cases/05-vitest-unit-20260704-235231/compression.diff)
- [Output without CCR](cases/05-vitest-unit-20260704-235231/output-noccr.log) - [diff](cases/05-vitest-unit-20260704-235231/compression-noccr.diff)

Input excerpt:

```text
11:52:31 PM [vite] warning: `esbuild` option was specified by "vite-plugin-node-polyfills" plugin. This option is deprecated, please use `oxc` instead.
Both esbuild and oxc options were set. oxc options will be used and esbuild options will be ignored. The following esbuild options were set: `{
  banner: "import __buffer_polyfill from 'vite-plugin-node-polyfills/shims/buffer'\n" +
    'globalThis.Buffer = globalThis.Buffer || __buffer_polyfill\n' +
    "import __global_polyfill from 'vite-plugin-node-polyfills/shims/global'\n" +
    'globalThis.global = globalThis.global || __global_polyfill\n' +
    "import __process_polyfill from 'vite-plugin-node-polyfills/shims/process'\n" +
    'globalThis.process = globalThis.process || __process_polyfill\n'
}`

 RUN  v4.1.5 <OPENHUMAN_ROOT>/app


 Test Files  1 passed (1)
      Tests  13 passed (13)
   Start at  23:52:32
   Duration  848ms (transform 151ms, setup 235ms, import 77ms, tests 154ms, environment 317ms)


```

Output excerpt:

```text
exit 101
RUN  v4.1.5 <OPENHUMAN_ROOT>/app
 Test Files  1 passed (1)
      Tests  13 passed (13)
   Start at  23:52:32
   Duration  848ms (transform 151ms, setup 235ms, import 77ms, tests 154ms, environment 317ms)

[PARTIAL view — full original (970 bytes): call tinyjuice_retrieve with token "ddb0cfd662fa34d5f4bba410ef1e402f"]

```

### `02-vitest-unit-20260704-234958`

- [Full input](cases/02-vitest-unit-20260704-234958/input.log)
- [Output with CCR](cases/02-vitest-unit-20260704-234958/output.log) - [diff](cases/02-vitest-unit-20260704-234958/compression.diff)
- [Output without CCR](cases/02-vitest-unit-20260704-234958/output-noccr.log) - [diff](cases/02-vitest-unit-20260704-234958/compression-noccr.diff)

Input excerpt:

```text
11:49:59 PM [vite] warning: `esbuild` option was specified by "vite-plugin-node-polyfills" plugin. This option is deprecated, please use `oxc` instead.
Both esbuild and oxc options were set. oxc options will be used and esbuild options will be ignored. The following esbuild options were set: `{
  banner: "import __buffer_polyfill from 'vite-plugin-node-polyfills/shims/buffer'\n" +
    'globalThis.Buffer = globalThis.Buffer || __buffer_polyfill\n' +
    "import __global_polyfill from 'vite-plugin-node-polyfills/shims/global'\n" +
    'globalThis.global = globalThis.global || __global_polyfill\n' +
    "import __process_polyfill from 'vite-plugin-node-polyfills/shims/process'\n" +
    'globalThis.process = globalThis.process || __process_polyfill\n'
}`

 RUN  v4.1.5 <OPENHUMAN_ROOT>/app


 Test Files  1 passed (1)
      Tests  9 passed (9)
   Start at  23:49:59
   Duration  2.33s (transform 1.41s, setup 263ms, import 1.43s, tests 116ms, environment 302ms)


```

Output excerpt:

```text
exit 101
RUN  v4.1.5 <OPENHUMAN_ROOT>/app
 Test Files  1 passed (1)
      Tests  9 passed (9)
   Start at  23:49:59
   Duration  2.33s (transform 1.41s, setup 263ms, import 1.43s, tests 116ms, environment 302ms)

[PARTIAL view — full original (969 bytes): call tinyjuice_retrieve with token "33584aa40077e45faaf15e86ed05b667"]

```

