# TinyJuice Interface

Self-hostable analytics UI for TinyJuice compression runs. It follows the same
local-first dashboard shape as Headroom while staying decoupled from OpenHuman
runtime code and avoiding raw prompt/context storage.

## Run

```sh
npm install
npm run dev
```

Build a static bundle:

```sh
npm run build
npm run preview
```

Build the container:

```sh
docker build -t tinyjuice-interface .
docker run --rm -p 8080:80 tinyjuice-interface
```

## Analytics Format

The UI accepts a JSON array of run records. Keep this data to metadata and
token/byte counts only.

```json
[
  {
    "id": "run-001",
    "timestamp": "2026-07-04T15:05:00Z",
    "algorithm": "smartcrusher",
    "contentKind": "json",
    "status": "compressed",
    "originalTokens": 2840,
    "compressedTokens": 1180,
    "originalBytes": 11842,
    "compressedBytes": 4928,
    "latencyMs": 23,
    "lossy": true,
    "ccrStored": true,
    "source": "openhuman",
    "profile": "full"
  }
]
```

`public/analytics/sample-runs.json` is a demo fixture, not a benchmark result.
Loaded-run reductions in the UI are computed only from the active JSON dataset.
