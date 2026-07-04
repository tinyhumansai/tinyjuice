import type { AlgorithmSummary, CompressionRun } from "./types";

type TrendPoint = {
  label: string;
  value: number;
};

export function TokenTrend({ runs }: { runs: CompressionRun[] }) {
  const points = toTrend(runs);
  const width = 720;
  const height = 220;
  const path = linePath(points, width, height);
  const area = path ? `${path} L ${width} ${height} L 0 ${height} Z` : "";

  return (
    <svg className="chart" viewBox={`0 0 ${width} ${height}`} role="img" aria-label="Token savings trend">
      <defs>
        <linearGradient id="trendFill" x1="0" x2="0" y1="0" y2="1">
          <stop offset="0%" stopColor="#2f8f83" stopOpacity="0.32" />
          <stop offset="100%" stopColor="#2f8f83" stopOpacity="0.02" />
        </linearGradient>
      </defs>
      <rect x="0" y="0" width={width} height={height} rx="8" className="chart-bg" />
      {[0, 1, 2, 3].map((tick) => (
        <line
          key={tick}
          x1="0"
          x2={width}
          y1={(height / 4) * tick + 18}
          y2={(height / 4) * tick + 18}
          className="grid-line"
        />
      ))}
      {area && <path d={area} fill="url(#trendFill)" />}
      {path && <path d={path} className="trend-line" />}
      {points.map((point, index) => {
        const [x, y] = pointCoords(point.value, index, points, width, height);
        return (
          <g key={`${point.label}-${index}`}>
            <circle cx={x} cy={y} r="4" className="trend-dot" />
            {index === points.length - 1 && (
              <text x={Math.max(8, x - 66)} y={Math.max(18, y - 10)} className="chart-label">
                {formatCompact(point.value)} saved
              </text>
            )}
          </g>
        );
      })}
    </svg>
  );
}

export function AlgorithmBars({ summaries }: { summaries: AlgorithmSummary[] }) {
  const maxSaved = Math.max(1, ...summaries.map((summary) => summary.savedTokens));
  return (
    <div className="bars" aria-label="Algorithm token savings">
      {summaries.map((summary) => {
        const width = Math.max(4, (summary.savedTokens / maxSaved) * 100);
        const reduction = summary.originalTokens
          ? (summary.savedTokens / summary.originalTokens) * 100
          : 0;
        return (
          <div className="bar-row" key={summary.algorithm}>
            <div className="bar-meta">
              <span className="bar-title">{labelize(summary.algorithm)}</span>
              <span>{summary.runs} runs</span>
            </div>
            <div className="bar-track">
              <span className="bar-fill" style={{ width: `${width}%` }} />
            </div>
            <div className="bar-values">
              <span>{formatCompact(summary.savedTokens)} tokens</span>
              <span>{reduction.toFixed(1)}% loaded-run reduction</span>
            </div>
          </div>
        );
      })}
    </div>
  );
}

function toTrend(runs: CompressionRun[]): TrendPoint[] {
  let total = 0;
  return runs.map((run) => {
    total += Math.max(0, run.originalTokens - run.compressedTokens);
    return {
      label: run.id,
      value: total,
    };
  });
}

function linePath(points: TrendPoint[], width: number, height: number): string {
  if (!points.length) {
    return "";
  }
  return points
    .map((point, index) => {
      const [x, y] = pointCoords(point.value, index, points, width, height);
      return `${index === 0 ? "M" : "L"} ${x.toFixed(2)} ${y.toFixed(2)}`;
    })
    .join(" ");
}

function pointCoords(
  value: number,
  index: number,
  points: TrendPoint[],
  width: number,
  height: number,
): [number, number] {
  const max = Math.max(1, ...points.map((point) => point.value));
  const x = points.length === 1 ? width / 2 : (index / (points.length - 1)) * width;
  const y = height - (value / max) * (height - 32) - 16;
  return [x, y];
}

function formatCompact(value: number): string {
  return Intl.NumberFormat(undefined, { notation: "compact", maximumFractionDigits: 1 }).format(value);
}

function labelize(value: string): string {
  return value.replace(/_/g, " ");
}
