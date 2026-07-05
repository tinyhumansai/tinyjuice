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
  constructor(public status: number, public url: string, message: string) { … 4 line(s) … ⟦tj:deeff2d96d5f65cbe2c8703822a4674a⟧ }
}

export async function fetchJson<T>(url: string, opts: RequestOptions = { method: "GET" }): Promise<T> { … 63 line(s) … ⟦tj:2c1d11b9e98fc15768ffbeb235d218f7⟧ }

export async function postJson<T>(url: string, opts: RequestOptions = { method: "POST" }): Promise<T> { … 63 line(s) … ⟦tj:2c1d11b9e98fc15768ffbeb235d218f7⟧ }

export async function putJson<T>(url: string, opts: RequestOptions = { method: "PUT" }): Promise<T> { … 63 line(s) … ⟦tj:2c1d11b9e98fc15768ffbeb235d218f7⟧ }

export async function deleteResource<T>(url: string, opts: RequestOptions = { method: "DELETE" }): Promise<T> { … 63 line(s) … ⟦tj:2c1d11b9e98fc15768ffbeb235d218f7⟧ }

export class ApiClient extends EventEmitter {
  private cache = new Map<string, CacheEntry>();

  constructor(private baseUrl: string, private defaultTimeoutMs = 15000) {
    super();
  }

  async listUsers(page = 1, perPage = 50): Promise<unknown[]> { … 17 line(s) … ⟦tj:35255fc603849d59e327d29d40f1f21c⟧ }

  async listProjects(page = 1, perPage = 50): Promise<unknown[]> { … 17 line(s) … ⟦tj:04f9ea44c4e90c85afbf1c155854442a⟧ }

  async listDeployments(page = 1, perPage = 50): Promise<unknown[]> { … 17 line(s) … ⟦tj:90fa275881668dac865288a62ee82a77⟧ }

  async listInvoices(page = 1, perPage = 50): Promise<unknown[]> { … 17 line(s) … ⟦tj:c7444c047ca61f0c7ff01577cb76aeec⟧ }

  async listWebhooks(page = 1, perPage = 50): Promise<unknown[]> { … 17 line(s) … ⟦tj:3f90e34e5c9eeae801da562f0787b06e⟧ }

}
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (17114 bytes) is available by calling tinyjuice_retrieve with token "f69eac7f33def4b2779de4e545ed50eb" (marker ⟦tj:f69eac7f33def4b2779de4e545ed50eb⟧)]