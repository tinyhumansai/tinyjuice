export type CompressionStatus = "compressed" | "passthrough" | "declined" | "error";

export type CompressionRun = {
  id: string;
  timestamp: string;
  algorithm: string;
  contentKind: string;
  status: CompressionStatus;
  originalTokens: number;
  compressedTokens: number;
  originalBytes: number;
  compressedBytes: number;
  latencyMs: number;
  lossy: boolean;
  ccrStored: boolean;
  source?: string;
  profile?: string;
  note?: string;
};

export type AlgorithmSummary = {
  algorithm: string;
  runs: number;
  originalTokens: number;
  compressedTokens: number;
  savedTokens: number;
  originalBytes: number;
  compressedBytes: number;
  medianLatencyMs: number;
  ccrStored: number;
  lossy: number;
  errors: number;
};
