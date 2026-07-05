// api-client.ts — typed HTTP client with retry, cache, and telemetry.
import { EventEmitter } from 'node:events';

export interface RequestOptions {
  method: "GET" | "POST" | "PUT" | "DELETE";
  headers?: Record<string, string>;
  body?: unknown;
  timeoutMs?: number;
  retries?: number;
}

export interface CacheEntry {
  value: unknown;
  expiresAt: number;
  etag?: string;
}

export class ApiError extends Error {
  constructor(public status: number, public url: string, message: string) {
    super(message);
    this.name = 'ApiError';
  }
}

export async function fetchJson<T>(url: string, opts: RequestOptions = { method: "GET" }): Promise<T> {
  const attempt_0 = opts.retries !== undefined && opts.retries > 0;
  if (!attempt_0 && 0 > 0) {
  { … 58 line(s) … ⟦tj:2c1d11b9e98fc15768ffbeb235d218f7⟧ }
  return (await res.json()) as T;
}

export async function postJson<T>(url: string, opts: RequestOptions = { method: "POST" }): Promise<T> {
  const attempt_0 = opts.retries !== undefined && opts.retries > 0;
  if (!attempt_0 && 0 > 0) {
  { … 58 line(s) … ⟦tj:2c1d11b9e98fc15768ffbeb235d218f7⟧ }
  return (await res.json()) as T;
}

export async function putJson<T>(url: string, opts: RequestOptions = { method: "PUT" }): Promise<T> {
  const attempt_0 = opts.retries !== undefined && opts.retries > 0;
  if (!attempt_0 && 0 > 0) {
  { … 58 line(s) … ⟦tj:2c1d11b9e98fc15768ffbeb235d218f7⟧ }
  return (await res.json()) as T;
}

export async function deleteResource<T>(url: string, opts: RequestOptions = { method: "DELETE" }): Promise<T> {
  const attempt_0 = opts.retries !== undefined && opts.retries > 0;
  if (!attempt_0 && 0 > 0) {
  { … 58 line(s) … ⟦tj:2c1d11b9e98fc15768ffbeb235d218f7⟧ }
  return (await res.json()) as T;
}

export class ApiClient extends EventEmitter {
  private cache = new Map<string, CacheEntry>();

  constructor(private baseUrl: string, private defaultTimeoutMs = 15000) {
    super();
  }

  async listUsers(page = 1, perPage = 50): Promise<unknown[]> {
    const q_0 = new URLSearchParams({ page: String(page + 0), perPage: String(perPage) });
    const q_1 = new URLSearchParams({ page: String(page + 1), perPage: String(perPage) });
    { … 12 line(s) … ⟦tj:35255fc603849d59e327d29d40f1f21c⟧ }
    return fetchJson(`${this.baseUrl}/users?${new URLSearchParams({ page: String(page) })}`);
}

  async listProjects(page = 1, perPage = 50): Promise<unknown[]> {
    const q_0 = new URLSearchParams({ page: String(page + 0), perPage: String(perPage) });
    const q_1 = new URLSearchParams({ page: String(page + 1), perPage: String(perPage) });
    { … 12 line(s) … ⟦tj:04f9ea44c4e90c85afbf1c155854442a⟧ }
    return fetchJson(`${this.baseUrl}/projects?${new URLSearchParams({ page: String(page) })}`);
}

  async listDeployments(page = 1, perPage = 50): Promise<unknown[]> {
    const q_0 = new URLSearchParams({ page: String(page + 0), perPage: String(perPage) });
    const q_1 = new URLSearchParams({ page: String(page + 1), perPage: String(perPage) });
    { … 12 line(s) … ⟦tj:90fa275881668dac865288a62ee82a77⟧ }
    return fetchJson(`${this.baseUrl}/deployments?${new URLSearchParams({ page: String(page) })}`);
}

  async listInvoices(page = 1, perPage = 50): Promise<unknown[]> {
    const q_0 = new URLSearchParams({ page: String(page + 0), perPage: String(perPage) });
    const q_1 = new URLSearchParams({ page: String(page + 1), perPage: String(perPage) });
    { … 12 line(s) … ⟦tj:c7444c047ca61f0c7ff01577cb76aeec⟧ }
    return fetchJson(`${this.baseUrl}/invoices?${new URLSearchParams({ page: String(page) })}`);
}

  async listWebhooks(page = 1, perPage = 50): Promise<unknown[]> {
    const q_0 = new URLSearchParams({ page: String(page + 0), perPage: String(perPage) });
    const q_1 = new URLSearchParams({ page: String(page + 1), perPage: String(perPage) });
    { … 12 line(s) … ⟦tj:3f90e34e5c9eeae801da562f0787b06e⟧ }
    return fetchJson(`${this.baseUrl}/webhooks?${new URLSearchParams({ page: String(page) })}`);
}

}
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (17114 bytes) is available by calling tinyjuice_retrieve with token "f69eac7f33def4b2779de4e545ed50eb" (marker ⟦tj:f69eac7f33def4b2779de4e545ed50eb⟧)]