# Savings Accounting Spec

## Status

Design reference for reporting reducer and call-collapse effects. This is
accounting infrastructure, not a compression algorithm.

## Goal

Track effects in auditable terms:

- calls avoided;
- bytes or tokens omitted;
- validation retries avoided;
- elapsed time estimate;
- cost estimate when usage and pricing are available.

## Inputs

```text
SavingsInput {
  baseline_calls: Vec<BaselineCall>,
  actual_calls: Vec<ActualCall>,
  usage: Option<ModelUsage>,
  pricing: Option<ModelPricing>,
  elapsed: TimingData,
}

ModelUsage {
  input_tokens: u64,
  output_tokens: u64,
  cache_read_tokens: u64,
  cache_creation_tokens: u64,
}
```

## Accounting Classes

1. Counted:
   - actual tool calls;
   - collapsed baseline calls;
   - retries avoided by validation or fuzzy matching.
2. Measured:
   - model usage fields from the host;
   - reducer input/output byte counts;
   - elapsed wall time.
3. Estimated:
   - counterfactual time saved;
   - counterfactual cost saved when baseline usage is unavailable.

Reports must label each number by class.

## Output

```text
SavingsReport {
  calls_saved: i64,
  input_tokens_saved: Option<i64>,
  output_tokens_saved: Option<i64>,
  cache_read_tokens_saved: Option<i64>,
  cache_creation_tokens_saved: Option<i64>,
  cost_saved: Option<Money>,
  time_saved: Option<Duration>,
  confidence: Confidence,
  notes: Vec<String>,
}
```

## TinyJuice Fit

`src/savings.rs` should own the accounting model. Each compressor should return
enough `CompressionReport` metadata for aggregation:

```text
raw_bytes
compressed_bytes
omitted_items
baseline_call_equivalent
actual_call_count
```

## Safety Rules

- Do not present estimates as measured facts.
- Do not claim percentages until benchmark fixtures exist.
- Keep per-call accounting inspectable.
- Avoid raw prompt, source, or tool-output content in accounting records.

## Test Fixtures

- counted call savings;
- measured token usage pass-through;
- estimated time labels;
- separate input/output/cache pricing;
- no raw content in serialized accounting records.
