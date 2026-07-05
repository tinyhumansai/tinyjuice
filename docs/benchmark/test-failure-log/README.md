# Test Failure Logs

Real OpenHuman Vitest command logs. The command-aware reducer keeps failure summaries and drops repetitive success or setup noise.

Each row links to the full raw input and the exact compacted output used by the benchmark.

## Cases

| Case | Input | Output | Original | Compacted | Est. token reduction | Avg latency | CCR |
| --- | --- | --- | ---: | ---: | ---: | ---: | --- |
| `01-vitest-unit-20260704-234941` | [input](cases/01-vitest-unit-20260704-234941/input.log) | [output](cases/01-vitest-unit-20260704-234941/output.log) | 2.8 KB | 1.0 KB | 65.4% | 0.115 ms | true |
| `02-vitest-unit-20260704-234958` | [input](cases/02-vitest-unit-20260704-234958/input.log) | [output](cases/02-vitest-unit-20260704-234958/output.log) | 969 B | 969 B | 0.0% | 0.000 ms | n/a |
| `03-vitest-unit-20260704-235052` | [input](cases/03-vitest-unit-20260704-235052/input.log) | [output](cases/03-vitest-unit-20260704-235052/output.log) | 967 B | 967 B | 0.0% | 0.000 ms | n/a |
| `04-vitest-unit-20260704-235125` | [input](cases/04-vitest-unit-20260704-235125/input.log) | [output](cases/04-vitest-unit-20260704-235125/output.log) | 967 B | 967 B | 0.0% | 0.000 ms | n/a |
| `05-vitest-unit-20260704-235231` | [input](cases/05-vitest-unit-20260704-235231/input.log) | [output](cases/05-vitest-unit-20260704-235231/output.log) | 970 B | 970 B | 0.0% | 0.000 ms | n/a |
| `06-vitest-unit-20260704-235240` | [input](cases/06-vitest-unit-20260704-235240/input.log) | [output](cases/06-vitest-unit-20260704-235240/output.log) | 971 B | 971 B | 0.0% | 0.000 ms | n/a |
| `07-vitest-excerpt-7` | [input](cases/07-vitest-excerpt-7/input.log) | [output](cases/07-vitest-excerpt-7/output.log) | 5.6 KB | 1.5 KB | 75.4% | 0.231 ms | true |
| `08-vitest-excerpt-8` | [input](cases/08-vitest-excerpt-8/input.log) | [output](cases/08-vitest-excerpt-8/output.log) | 1.9 KB | 1.9 KB | 0.0% | 0.000 ms | n/a |
| `09-vitest-excerpt-9` | [input](cases/09-vitest-excerpt-9/input.log) | [output](cases/09-vitest-excerpt-9/output.log) | 1.9 KB | 1.9 KB | 0.0% | 0.000 ms | n/a |
| `10-vitest-excerpt-10` | [input](cases/10-vitest-excerpt-10/input.log) | [output](cases/10-vitest-excerpt-10/output.log) | 1.9 KB | 1.9 KB | 0.0% | 0.000 ms | n/a |

## What TinyJuice Is Doing

The command context routes these logs through the Vitest rule. Setup chatter and repeated passing output are removed, while failure blocks, summaries, and locations remain visible.

## Syntax-Aware Samples

### `01-vitest-unit-20260704-234941`

- [Full input](cases/01-vitest-unit-20260704-234941/input.log)
- [Full output](cases/01-vitest-unit-20260704-234941/output.log)

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
Error: [2mexpect([22m[31melement[39m[2m).not.toBeInTheDocument()[22m

[31mexpected document not to contain element, found <button
  class="inline-flex items-center justify-center gap-2 font-medium transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:opacity-40 disabled:pointer-events-no...
  disabled=""
  type="button"
>
  <div
    class="w-3 h-3 border border-stone-400 border-t-transparent rounded-full animate-spin"
  />
  Run review now
</button> instead[39m

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

[compacted tool output — this is a PARTIAL view; the full original (2777 bytes) is available by calling tokenjuice_retrieve with token "86eb51aa134ea50382aa89d95827b1b5" (marker ⟦tj:86eb51aa134ea50382aa89d95827b1b5�...

```

### `02-vitest-unit-20260704-234958`

- [Full input](cases/02-vitest-unit-20260704-234958/input.log)
- [Full output](cases/02-vitest-unit-20260704-234958/output.log)

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

### `03-vitest-unit-20260704-235052`

- [Full input](cases/03-vitest-unit-20260704-235052/input.log)
- [Full output](cases/03-vitest-unit-20260704-235052/output.log)

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

### `04-vitest-unit-20260704-235125`

- [Full input](cases/04-vitest-unit-20260704-235125/input.log)
- [Full output](cases/04-vitest-unit-20260704-235125/output.log)

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

### `05-vitest-unit-20260704-235231`

- [Full input](cases/05-vitest-unit-20260704-235231/input.log)
- [Full output](cases/05-vitest-unit-20260704-235231/output.log)

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

### `06-vitest-unit-20260704-235240`

- [Full input](cases/06-vitest-unit-20260704-235240/input.log)
- [Full output](cases/06-vitest-unit-20260704-235240/output.log)

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

### `07-vitest-excerpt-7`

- [Full input](cases/07-vitest-excerpt-7/input.log)
- [Full output](cases/07-vitest-excerpt-7/output.log)

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
Error: [2mexpect([22m[31melement[39m[2m).not.toBeInTheDocument()[22m

[31mexpected document not to contain element, found <button
  class="inline-flex items-center justify-center gap-2 font-medium transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:opacity-40 disabled:pointer-events-no...
  disabled=""
  type="button"
>
  <div
    class="w-3 h-3 border border-stone-400 border-t-transparent rounded-full animate-spin"
  />
  Run review now
</button> instead[39m

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

[compacted tool output — this is a PARTIAL view; the full original (5554 bytes) is available by calling tokenjuice_retrieve with token "6662f8a36798fc5c20e4a63b3b70c5fe" (marker ⟦tj:6662f8a36798fc5c20e4a63b3b70c5fe�...

```

### `08-vitest-excerpt-8`

- [Full input](cases/08-vitest-excerpt-8/input.log)
- [Full output](cases/08-vitest-excerpt-8/output.log)

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

### `09-vitest-excerpt-9`

- [Full input](cases/09-vitest-excerpt-9/input.log)
- [Full output](cases/09-vitest-excerpt-9/output.log)

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

### `10-vitest-excerpt-10`

- [Full input](cases/10-vitest-excerpt-10/input.log)
- [Full output](cases/10-vitest-excerpt-10/output.log)

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

