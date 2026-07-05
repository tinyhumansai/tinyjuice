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
  { … 58 line(s) … }
  return (await res.json()) as T;
}

export async function postJson<T>(url: string, opts: RequestOptions = { method: "POST" }): Promise<T> {
  const attempt_0 = opts.retries !== undefined && opts.retries > 0;
  if (!attempt_0 && 0 > 0) {
  { … 58 line(s) … }
  return (await res.json()) as T;
}

export async function putJson<T>(url: string, opts: RequestOptions = { method: "PUT" }): Promise<T> {
  const attempt_0 = opts.retries !== undefined && opts.retries > 0;
  if (!attempt_0 && 0 > 0) {
  { … 58 line(s) … }
  return (await res.json()) as T;
}

export async function deleteResource<T>(url: string, opts: RequestOptions = { method: "DELETE" }): Promise<T> {
  const attempt_0 = opts.retries !== undefined && opts.retries > 0;
  if (!attempt_0 && 0 > 0) {
  { … 58 line(s) … }
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
    { … 12 line(s) … }
    return fetchJson(`${this.baseUrl}/users?${new URLSearchParams({ page: String(page) })}`);
}

  async listProjects(page = 1, perPage = 50): Promise<unknown[]> {
    const q_0 = new URLSearchParams({ page: String(page + 0), perPage: String(perPage) });
    const q_1 = new URLSearchParams({ page: String(page + 1), perPage: String(perPage) });
    { … 12 line(s) … }
    return fetchJson(`${this.baseUrl}/projects?${new URLSearchParams({ page: String(page) })}`);
}

  async listDeployments(page = 1, perPage = 50): Promise<unknown[]> {
    const q_0 = new URLSearchParams({ page: String(page + 0), perPage: String(perPage) });
    const q_1 = new URLSearchParams({ page: String(page + 1), perPage: String(perPage) });
    { … 12 line(s) … }
    return fetchJson(`${this.baseUrl}/deployments?${new URLSearchParams({ page: String(page) })}`);
}

  async listInvoices(page = 1, perPage = 50): Promise<unknown[]> {
    const q_0 = new URLSearchParams({ page: String(page + 0), perPage: String(perPage) });
    const q_1 = new URLSearchParams({ page: String(page + 1), perPage: String(perPage) });
    { … 12 line(s) … }
    return fetchJson(`${this.baseUrl}/invoices?${new URLSearchParams({ page: String(page) })}`);
}

  async listWebhooks(page = 1, perPage = 50): Promise<unknown[]> {
    const q_0 = new URLSearchParams({ page: String(page + 0), perPage: String(perPage) });
    const q_1 = new URLSearchParams({ page: String(page + 1), perPage: String(perPage) });
    { … 12 line(s) … }
    return fetchJson(`${this.baseUrl}/webhooks?${new URLSearchParams({ page: String(page) })}`);
}

}