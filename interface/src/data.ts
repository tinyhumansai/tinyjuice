import type { AlgorithmSummary, CompressionRun, CompressionStatus } from "./types";

const KNOWN_STATUSES = new Set<CompressionStatus>([
  "compressed",
  "passthrough",
  "declined",
  "error",
]);

const STORAGE_KEY = "tinyjuice-interface-runs";

export async function loadSampleRuns(): Promise<CompressionRun[]> {
  const response = await fetch("/analytics/sample-runs.json", { cache: "no-store" });
  if (!response.ok) {
    throw new Error(`sample analytics unavailable: ${response.status}`);
  }
  return normalizeRuns(await response.json());
}

export function loadStoredRuns(): CompressionRun[] | null {
  const raw = window.localStorage.getItem(STORAGE_KEY);
  if (!raw) {
    return null;
  }
  try {
    return normalizeRuns(JSON.parse(raw));
  } catch {
    window.localStorage.removeItem(STORAGE_KEY);
    return null;
  }
}

export function storeRuns(runs: CompressionRun[]) {
  window.localStorage.setItem(STORAGE_KEY, JSON.stringify(runs));
}

export function clearStoredRuns() {
  window.localStorage.removeItem(STORAGE_KEY);
}

export function normalizeRuns(input: unknown): CompressionRun[] {
  const rows = Array.isArray(input) ? input : [input];
  return rows
    .map((row, index) => normalizeRun(row, index))
    .filter((row): row is CompressionRun => row !== null)
    .sort((a, b) => Date.parse(a.timestamp) - Date.parse(b.timestamp));
}

function normalizeRun(row: unknown, index: number): CompressionRun | null {
  if (!row || typeof row !== "object") {
    return null;
  }
  const value = row as Record<string, unknown>;
  const originalTokens = finiteNumber(value.originalTokens);
  const compressedTokens = finiteNumber(value.compressedTokens);
  const originalBytes = finiteNumber(value.originalBytes);
  const compressedBytes = finiteNumber(value.compressedBytes);
  if (
    originalTokens === null ||
    compressedTokens === null ||
    originalBytes === null ||
    compressedBytes === null
  ) {
    return null;
  }

  const status = typeof value.status === "string" && KNOWN_STATUSES.has(value.status as CompressionStatus)
    ? (value.status as CompressionStatus)
    : compressedTokens < originalTokens
      ? "compressed"
      : "passthrough";

  return {
    id: asString(value.id) ?? `run-${index + 1}`,
    timestamp: validTimestamp(value.timestamp) ?? new Date().toISOString(),
    algorithm: safeLabel(value.algorithm, "none"),
    contentKind: safeLabel(value.contentKind, "plain_text"),
    status,
    originalTokens,
    compressedTokens,
    originalBytes,
    compressedBytes,
    latencyMs: finiteNumber(value.latencyMs) ?? 0,
    lossy: Boolean(value.lossy),
    ccrStored: Boolean(value.ccrStored),
    source: asString(value.source),
    profile: asString(value.profile),
    note: asString(value.note),
  };
}

export function summarizeByAlgorithm(runs: CompressionRun[]): AlgorithmSummary[] {
  const map = new Map<string, CompressionRun[]>();
  for (const run of runs) {
    const current = map.get(run.algorithm) ?? [];
    current.push(run);
    map.set(run.algorithm, current);
  }

  return Array.from(map.entries())
    .map(([algorithm, rows]) => {
      const originalTokens = sum(rows, "originalTokens");
      const compressedTokens = sum(rows, "compressedTokens");
      return {
        algorithm,
        runs: rows.length,
        originalTokens,
        compressedTokens,
        savedTokens: Math.max(0, originalTokens - compressedTokens),
        originalBytes: sum(rows, "originalBytes"),
        compressedBytes: sum(rows, "compressedBytes"),
        medianLatencyMs: median(rows.map((row) => row.latencyMs)),
        ccrStored: rows.filter((row) => row.ccrStored).length,
        lossy: rows.filter((row) => row.lossy).length,
        errors: rows.filter((row) => row.status === "error").length,
      };
    })
    .sort((a, b) => b.savedTokens - a.savedTokens);
}

export function median(values: number[]): number {
  if (!values.length) {
    return 0;
  }
  const sorted = [...values].sort((a, b) => a - b);
  const mid = Math.floor(sorted.length / 2);
  return sorted.length % 2 === 0 ? (sorted[mid - 1] + sorted[mid]) / 2 : sorted[mid];
}

export function sum<T extends Record<K, number>, K extends keyof T>(rows: T[], key: K): number {
  return rows.reduce((total, row) => total + row[key], 0);
}

function finiteNumber(value: unknown): number | null {
  if (typeof value !== "number" || !Number.isFinite(value)) {
    return null;
  }
  return Math.max(0, value);
}

function asString(value: unknown): string | undefined {
  return typeof value === "string" && value.trim() ? value.trim() : undefined;
}

function safeLabel(value: unknown, fallback: string): string {
  const label = asString(value)?.toLowerCase().replace(/[^a-z0-9_:-]/g, "_");
  return label || fallback;
}

function validTimestamp(value: unknown): string | null {
  const timestamp = asString(value);
  if (!timestamp) {
    return null;
  }
  const parsed = Date.parse(timestamp);
  return Number.isNaN(parsed) ? null : new Date(parsed).toISOString();
}
