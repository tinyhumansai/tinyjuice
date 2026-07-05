# JSON SmartCrusher

## Fixture

`json_service_inventory` models a large API list: 260 service records with the
same object shape, one critical status row, and one latency outlier.

## Benchmark Result

| Metric | Value |
| --- | ---: |
| Original bytes | 52,506 |
| Compacted bytes | 2,670 |
| Estimated token reduction | 94.9% |
| Average latency | 0.397 ms |
| Signal checks | 3/3 |
| Task checks | 2/2 |
| CCR recovery | yes |

## Full Artifacts

- [Full input](full-input.txt)
- [Full output](full-output.txt)

## Input Sample

```json
[
  {
    "id": 0,
    "service": "svc-000",
    "owner": "team-0",
    "region": "us-east-1",
    "status": "ok",
    "latency_ms": 20,
    "replicas": 2,
    "updated_at": "2026-07-01T12:00:00Z"
  },
  {
    "id": 137,
    "service": "svc-137",
    "owner": "team-2",
    "region": "ap-south-1",
    "status": "critical",
    "latency_ms": 9250,
    "replicas": 3,
    "updated_at": "2026-07-03T12:17:00Z"
  }
]
```

## Output Sample

```text
[json table: 260 rows x 8 cols - blank=absent key - exact original via retrieve footer]
id | latency_ms | owner | region | replicas | service | status | updated_at
0 | 20 | team-0 | us-east-1 | 2 | svc-000 | ok | 2026-07-01T12:00:00Z
1 | 21 | team-1 | eu-west-1 | 3 | svc-001 | ok | 2026-07-02T12:01:00Z
[... 117 row(s) omitted ...]
137 | 9250 | team-2 | ap-south-1 | 3 | svc-137 | critical | 2026-07-03T12:17:00Z
[... 112 row(s) omitted ...]
250 | 45 | team-7 | eu-west-1 | 4 | svc-250 | ok | 2026-07-01T12:10:00Z

[compacted tool output - this is a PARTIAL view; the full original is available
by calling tokenjuice_retrieve with the emitted token]
```

## Behind The Scenes

TinyJuice detects that the payload is a JSON array of repeated objects. The
SmartCrusher path renders the repeated object keys once as table columns instead
of paying for the same key names on every row.

For large arrays, the output is intentionally row-dropped. It keeps a head
window, a tail window, and rows that carry signal. In this fixture the
`critical` service row and the `9250` latency outlier survive because anomaly
rows are more useful to an agent than the homogeneous `ok` rows around them.

The compacted table is a partial view, so TinyJuice offloads the full JSON into
CCR and appends a retrieval footer. The benchmark byte-compares that retrieved
original against the input.

## Human Review Notes

The output should make the anomaly obvious without forcing a reader through all
260 rows. Reviewers should verify that the table columns are understandable, the
critical row is retained, the outlier latency is visible, and omission markers
make it clear that this is not the complete array.
