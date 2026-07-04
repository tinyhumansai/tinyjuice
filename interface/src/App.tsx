import {
  Activity,
  Box,
  CheckCircle2,
  Download,
  Filter,
  Gauge,
  HardDrive,
  RefreshCw,
  ShieldCheck,
  Upload,
  XCircle,
} from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";
import { AlgorithmBars, TokenTrend } from "./charts";
import {
  clearStoredRuns,
  loadSampleRuns,
  loadStoredRuns,
  normalizeRuns,
  storeRuns,
  summarizeByAlgorithm,
  sum,
  median,
} from "./data";
import type { AlgorithmSummary, CompressionRun } from "./types";

type Range = "all" | "hour" | "compressed" | "lossy";

const formatter = new Intl.NumberFormat();
const shortFormatter = new Intl.NumberFormat(undefined, {
  notation: "compact",
  maximumFractionDigits: 1,
});

export default function App() {
  const [runs, setRuns] = useState<CompressionRun[]>([]);
  const [dataLabel, setDataLabel] = useState("sample fixtures");
  const [range, setRange] = useState<Range>("all");
  const [algorithm, setAlgorithm] = useState("all");
  const [kind, setKind] = useState("all");
  const [loadError, setLoadError] = useState<string | null>(null);
  const fileInput = useRef<HTMLInputElement | null>(null);

  useEffect(() => {
    const stored = loadStoredRuns();
    if (stored?.length) {
      setRuns(stored);
      setDataLabel("local upload");
      return;
    }
    void refreshSample();
  }, []);

  async function refreshSample() {
    try {
      setLoadError(null);
      const sample = await loadSampleRuns();
      setRuns(sample);
      setDataLabel("sample fixtures");
      clearStoredRuns();
    } catch (error) {
      setLoadError(error instanceof Error ? error.message : "unable to load analytics");
    }
  }

  const filtered = useMemo(
    () => applyFilters(runs, range, algorithm, kind),
    [algorithm, kind, range, runs],
  );
  const summaries = useMemo(() => summarizeByAlgorithm(filtered), [filtered]);
  const totals = useMemo(() => summarizeTotals(filtered, summaries), [filtered, summaries]);
  const algorithms = useMemo(() => unique(runs.map((run) => run.algorithm)), [runs]);
  const kinds = useMemo(() => unique(runs.map((run) => run.contentKind)), [runs]);

  async function importFile(file: File) {
    try {
      setLoadError(null);
      const imported = normalizeRuns(JSON.parse(await file.text()));
      if (!imported.length) {
        throw new Error("analytics file contained no valid runs");
      }
      setRuns(imported);
      setDataLabel(file.name);
      storeRuns(imported);
    } catch (error) {
      setLoadError(error instanceof Error ? error.message : "unable to import analytics");
    } finally {
      if (fileInput.current) {
        fileInput.current.value = "";
      }
    }
  }

  function exportFiltered() {
    const blob = new Blob([JSON.stringify(filtered, null, 2)], {
      type: "application/json",
    });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = "tinyjuice-analytics.json";
    link.click();
    URL.revokeObjectURL(url);
  }

  return (
    <main className="shell">
      <header className="topbar">
        <div className="brand">
          <div className="brand-mark">TJ</div>
          <div>
            <h1>TinyJuice Interface</h1>
            <p>{dataLabel}</p>
          </div>
        </div>
        <div className="top-actions">
          <input
            ref={fileInput}
            type="file"
            accept="application/json,.json"
            hidden
            onChange={(event) => {
              const file = event.target.files?.[0];
              if (file) {
                void importFile(file);
              }
            }}
          />
          <button className="icon-button" type="button" onClick={() => fileInput.current?.click()} title="Import JSON">
            <Upload size={18} />
            <span>Import</span>
          </button>
          <button className="icon-button" type="button" onClick={exportFiltered} title="Export filtered runs">
            <Download size={18} />
            <span>Export</span>
          </button>
          <button className="icon-button" type="button" onClick={() => void refreshSample()} title="Reload sample fixtures">
            <RefreshCw size={18} />
            <span>Reset</span>
          </button>
        </div>
      </header>

      {loadError && (
        <div className="alert" role="alert">
          <XCircle size={18} />
          <span>{loadError}</span>
        </div>
      )}

      <section className="controls" aria-label="Filters">
        <div className="control-group">
          <Filter size={17} />
          <button className={range === "all" ? "selected" : ""} onClick={() => setRange("all")} type="button">
            All
          </button>
          <button className={range === "hour" ? "selected" : ""} onClick={() => setRange("hour")} type="button">
            Last hour
          </button>
          <button className={range === "compressed" ? "selected" : ""} onClick={() => setRange("compressed")} type="button">
            Compressed
          </button>
          <button className={range === "lossy" ? "selected" : ""} onClick={() => setRange("lossy")} type="button">
            CCR
          </button>
        </div>
        <label>
          Algorithm
          <select value={algorithm} onChange={(event) => setAlgorithm(event.target.value)}>
            <option value="all">All</option>
            {algorithms.map((value) => (
              <option value={value} key={value}>
                {value}
              </option>
            ))}
          </select>
        </label>
        <label>
          Content
          <select value={kind} onChange={(event) => setKind(event.target.value)}>
            <option value="all">All</option>
            {kinds.map((value) => (
              <option value={value} key={value}>
                {value}
              </option>
            ))}
          </select>
        </label>
      </section>

      <section className="metric-grid" aria-label="Summary">
        <Metric icon={<Activity />} label="Runs" value={formatter.format(totals.runs)} detail={`${totals.compressedRuns} compressed`} />
        <Metric icon={<Gauge />} label="Tokens Saved" value={shortFormatter.format(totals.savedTokens)} detail={`${totals.reduction.toFixed(1)}% loaded-run reduction`} />
        <Metric icon={<ShieldCheck />} label="CCR Stored" value={formatter.format(totals.ccrStored)} detail={`${totals.lossyRuns} lossy outputs`} />
        <Metric icon={<HardDrive />} label="Median Latency" value={`${totals.medianLatencyMs.toFixed(0)} ms`} detail={`${shortFormatter.format(totals.savedBytes)} bytes avoided`} />
      </section>

      <section className="dashboard-grid">
        <Panel title="Token Savings" aside={`${filtered.length} runs`}>
          <TokenTrend runs={filtered} />
        </Panel>
        <Panel title="Algorithms" aside={`${summaries.length} active`}>
          <AlgorithmBars summaries={summaries} />
        </Panel>
      </section>

      <section className="dashboard-grid lower">
        <Panel title="Strategy Detail" aside="bounded labels">
          <StrategyTable summaries={summaries} />
        </Panel>
        <Panel title="Run Stream" aside="latest first">
          <RunTable runs={[...filtered].reverse().slice(0, 12)} />
        </Panel>
      </section>
    </main>
  );
}

function Metric({
  icon,
  label,
  value,
  detail,
}: {
  icon: React.ReactNode;
  label: string;
  value: string;
  detail: string;
}) {
  return (
    <article className="metric-card">
      <div className="metric-icon">{icon}</div>
      <div>
        <span>{label}</span>
        <strong>{value}</strong>
        <small>{detail}</small>
      </div>
    </article>
  );
}

function Panel({
  title,
  aside,
  children,
}: {
  title: string;
  aside: string;
  children: React.ReactNode;
}) {
  return (
    <section className="panel">
      <header>
        <h2>{title}</h2>
        <span>{aside}</span>
      </header>
      {children}
    </section>
  );
}

function StrategyTable({ summaries }: { summaries: AlgorithmSummary[] }) {
  return (
    <div className="table-wrap">
      <table>
        <thead>
          <tr>
            <th>Algorithm</th>
            <th>Runs</th>
            <th>Tokens</th>
            <th>Latency</th>
            <th>CCR</th>
            <th>Status</th>
          </tr>
        </thead>
        <tbody>
          {summaries.map((summary) => (
            <tr key={summary.algorithm}>
              <td className="mono">{summary.algorithm}</td>
              <td>{summary.runs}</td>
              <td>{shortFormatter.format(summary.savedTokens)}</td>
              <td>{summary.medianLatencyMs.toFixed(0)} ms</td>
              <td>{summary.ccrStored}</td>
              <td>
                <StatusPill ok={summary.errors === 0} />
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function RunTable({ runs }: { runs: CompressionRun[] }) {
  return (
    <div className="run-list">
      {runs.map((run) => {
        const saved = Math.max(0, run.originalTokens - run.compressedTokens);
        return (
          <article className="run-row" key={run.id}>
            <div>
              <strong>{run.id}</strong>
              <span>{new Date(run.timestamp).toLocaleTimeString()}</span>
            </div>
            <div className="run-tags">
              <span>{run.algorithm}</span>
              <span>{run.contentKind}</span>
              {run.ccrStored && <span>ccr</span>}
            </div>
            <div className="run-numbers">
              <Box size={16} />
              <span>{shortFormatter.format(saved)} tokens</span>
            </div>
          </article>
        );
      })}
    </div>
  );
}

function StatusPill({ ok }: { ok: boolean }) {
  return (
    <span className={ok ? "status ok" : "status bad"}>
      {ok ? <CheckCircle2 size={14} /> : <XCircle size={14} />}
      {ok ? "ok" : "error"}
    </span>
  );
}

function summarizeTotals(runs: CompressionRun[], summaries: AlgorithmSummary[]) {
  const originalTokens = sum(runs, "originalTokens");
  const compressedTokens = sum(runs, "compressedTokens");
  const originalBytes = sum(runs, "originalBytes");
  const compressedBytes = sum(runs, "compressedBytes");
  const savedTokens = Math.max(0, originalTokens - compressedTokens);
  return {
    runs: runs.length,
    compressedRuns: runs.filter((run) => run.status === "compressed").length,
    lossyRuns: runs.filter((run) => run.lossy).length,
    ccrStored: runs.filter((run) => run.ccrStored).length,
    savedTokens,
    savedBytes: Math.max(0, originalBytes - compressedBytes),
    medianLatencyMs: median(summaries.map((summary) => summary.medianLatencyMs)),
    reduction: originalTokens ? (savedTokens / originalTokens) * 100 : 0,
  };
}

function applyFilters(runs: CompressionRun[], range: Range, algorithm: string, kind: string) {
  let rows = runs;
  if (range === "hour" && runs.length) {
    const latest = Math.max(...runs.map((run) => Date.parse(run.timestamp)));
    rows = rows.filter((run) => latest - Date.parse(run.timestamp) <= 60 * 60 * 1000);
  }
  if (range === "compressed") {
    rows = rows.filter((run) => run.status === "compressed");
  }
  if (range === "lossy") {
    rows = rows.filter((run) => run.ccrStored);
  }
  if (algorithm !== "all") {
    rows = rows.filter((run) => run.algorithm === algorithm);
  }
  if (kind !== "all") {
    rows = rows.filter((run) => run.contentKind === kind);
  }
  return rows;
}

function unique(values: string[]) {
  return Array.from(new Set(values)).sort((a, b) => a.localeCompare(b));
}
