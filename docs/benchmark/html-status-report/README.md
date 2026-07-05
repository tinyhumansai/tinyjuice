# HTML Status Report

## Fixture

`html_status_report` models a rendered admin status page with repeated table
markup, one critical row, and a script tag that should not be useful in the
model context.

## Benchmark Result

| Metric | Value |
| --- | ---: |
| Original bytes | 9,018 |
| Compacted bytes | 3,502 |
| Estimated token reduction | 61.2% |
| Average latency | 0.063 ms |
| Signal checks | 2/2 |
| Task checks | 1/1 |
| CCR recovery | yes |

## Full Artifacts

- [Full input](full-input.txt)
- [Full output](full-output.txt)

## Input Sample

```html
<!doctype html>
<html>
  <head>
    <title>Status</title>
    <script>window.secret='nope'</script>
  </head>
  <body>
    <main>
      <table>
        <tr><td>service-0</td><td>healthy</td><td>20</td></tr>
        ...
        <tr><td>service-91</td><td>critical backlog on ingestion queue</td><td>27</td></tr>
        ...
      </table>
    </main>
  </body>
</html>
```

## Output Sample

```text
Status
service-0 healthy 20
service-1 healthy 21
...
service-91 critical backlog on ingestion queue 27
...
service-159 healthy 23

[compacted tool output - this is a PARTIAL view; the full original is available
by calling tokenjuice_retrieve with the emitted token]
```

## Behind The Scenes

The HTML compressor strips markup and removes script/style content. It keeps
visible page text, which is what an agent usually needs when a browser or fetch
tool returns rendered HTML.

The output is smaller because repeated tags disappear. The important row remains
readable as text: `service-91` has a `critical backlog on ingestion queue`.

The exact HTML remains available in CCR.

## Human Review Notes

The report should look like readable page content, not source markup. The
critical table row must survive, while script content should disappear from the
inline view. If an agent needs the DOM or original attributes, it can retrieve
the full HTML through CCR.
