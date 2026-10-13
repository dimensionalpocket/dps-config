/**
 * Configuration management for the DPS ecosystem.
 *
 * This class provides a lightweight configuration container mirroring the Rust implementation.
 * It focuses on optional values, sensible defaults, and environment variable loading.
 */
export class DpsConfig {
  /**
   * List of supported environment variable keys (without prefix).
   */
  static readonly ENV_KEYS = [
    "DPS_PROJECT_NAME",
    "DPS_DOMAIN",
    "DPS_API_PATH",
    "DPS_DEVELOPMENT_MODE",
    "DPS_BACKEND_SUBDOMAIN",
    "DPS_BACKEND_PORT",
    "DPS_BACKEND_BIND_PORT",
    "PORT",
    "DPS_BACKEND_PROTOCOL",
    "DPS_BACKEND_INSECURE_COOKIE",
    "DPS_BACKEND_SQLITE_MAIN_FILE_PATH",
    "DPS_BACKEND_SQLITE_MAIN_POOL_SIZE",
    "DPS_BACKEND_SQLITE_COLLECTION_FILE_PATH",
    "DPS_BACKEND_SQLITE_COLLECTION_POOL_SIZE",
    "DPS_BACKEND_SQLITE_SESSION_FILE_PATH",
    "DPS_BACKEND_SQLITE_SESSION_POOL_SIZE",
    "DPS_BACKEND_SESSION_SECRET",
    "DPS_BACKEND_SESSION_TTL_SECONDS",
  ] as const;

  private env: Record<string, string | undefined>;
  private projectName?: string;
  private domain?: string;
  private apiPath?: string;
  private developmentMode?: boolean;

  private backendSubdomain?: string;
  private backendPort?: number;
  private backendBindPort?: number;
  private backendProtocol?: string;
  private backendInsecureCookie?: boolean;
  private backendSqliteMainFilePath?: string;
  private backendSqliteMainPoolSize?: number;
  private backendSqliteCollectionFilePath?: string;
  private backendSqliteCollectionPoolSize?: number;
  private backendSqliteSessionFilePath?: string;
  private backendSqliteSessionPoolSize?: number;
  private backendSessionSecret?: string;
  private backendSessionTtlSeconds?: number;

  /**
   * Create a new DpsConfig instance, loading values from environment variables when present.
   *
   * @param env Environment variables object (e.g. process.env or import.meta.env).
   * @param envPrefix Optional prefix for environment variables (e.g. "VITE_" for Vite support).
   */
  constructor(env: Record<string, string | undefined>, envPrefix: string = "") {
    this.env = {};
    for (const key of DpsConfig.ENV_KEYS) {
      const fullKey = envPrefix + key;
      if (env[fullKey] !== undefined) {
        this.env[fullKey] = env[fullKey];
      }
    }

    this.projectName = this.loadEnvString(envPrefix, "DPS_PROJECT_NAME");
    this.domain = this.loadEnvString(envPrefix, "DPS_DOMAIN");
    this.apiPath = this.loadEnvString(envPrefix, "DPS_API_PATH");
    this.developmentMode = this.loadEnvBool(envPrefix, "DPS_DEVELOPMENT_MODE");

    this.backendSubdomain = this.loadEnvString(envPrefix, "DPS_BACKEND_SUBDOMAIN");
    this.backendPort = this.loadEnvNumber(envPrefix, "DPS_BACKEND_PORT");
    this.backendBindPort =
      this.loadEnvNumber(envPrefix, "DPS_BACKEND_BIND_PORT") ??
      this.loadEnvNumber(envPrefix, "PORT");
    this.backendProtocol = this.loadEnvString(envPrefix, "DPS_BACKEND_PROTOCOL");
    this.backendInsecureCookie = this.loadEnvBool(envPrefix, "DPS_BACKEND_INSECURE_COOKIE");
    this.backendSqliteMainFilePath = this.loadEnvString(envPrefix, "DPS_BACKEND_SQLITE_MAIN_FILE_PATH");
    this.backendSqliteMainPoolSize = this.loadEnvNumber(envPrefix, "DPS_BACKEND_SQLITE_MAIN_POOL_SIZE");
    this.backendSqliteCollectionFilePath = this.loadEnvString(envPrefix, "DPS_BACKEND_SQLITE_COLLECTION_FILE_PATH");
    this.backendSqliteCollectionPoolSize = this.loadEnvNumber(envPrefix, "DPS_BACKEND_SQLITE_COLLECTION_POOL_SIZE");
    this.backendSqliteSessionFilePath = this.loadEnvString(envPrefix, "DPS_BACKEND_SQLITE_SESSION_FILE_PATH");
    this.backendSqliteSessionPoolSize = this.loadEnvNumber(envPrefix, "DPS_BACKEND_SQLITE_SESSION_POOL_SIZE");
    this.backendSessionSecret = this.loadEnvString(envPrefix, "DPS_BACKEND_SESSION_SECRET");
    this.backendSessionTtlSeconds = this.loadEnvNumber(envPrefix, "DPS_BACKEND_SESSION_TTL_SECONDS");
  }

  // --------------------
  // Global getters/setters
  // --------------------

  getProjectName(): string {
    return this.projectName ?? "My Project";
  }

  setProjectName(value: string) {
    this.projectName = value;
  }

  getDomain(): string {
    return this.domain ?? "dps.localhost";
  }

  setDomain(value: string) {
    this.domain = value;
  }

  getApiPath(): string {
    return this.apiPath ?? "api";
  }

  setApiPath(value: string) {
    this.apiPath = value;
  }

  getDevelopmentMode(): boolean {
    return this.developmentMode ?? false;
  }

  setDevelopmentMode(value: boolean) {
    this.developmentMode = value;
  }

  // --------------------
  // DpsBackend getters/setters
  // --------------------

  getBackendSubdomain(): string {
    return this.backendSubdomain ?? "auth";
  }

  setBackendSubdomain(value: string) {
    this.backendSubdomain = value;
  }

  getBackendPort(): number | undefined {
    return this.backendPort;
  }

  setBackendPort(value: number | undefined) {
    this.backendPort = value;
  }

  /**
   * Returns the internal port a server should bind, resolving
   * `DPS_BACKEND_BIND_PORT`, then the generic `PORT` environment variable, then
   * the default `3000`. Distinct from `getBackendPort()` (public URL port).
   */
  getBackendBindPort(): number {
    return this.backendBindPort ?? 3000;
  }

  setBackendBindPort(value: number | undefined) {
    this.backendBindPort = value;
  }

  getBackendProtocol(): string {
    return this.backendProtocol ?? "https";
  }

  setBackendProtocol(value: string) {
    this.backendProtocol = value;
  }

  getBackendInsecureCookie(): boolean {
    return this.backendInsecureCookie ?? false;
  }

  setBackendInsecureCookie(value: boolean) {
    this.backendInsecureCookie = value;
  }

  getBackendSqliteMainFilePath(): string {
    return this.backendSqliteMainFilePath ?? "data/main-development.db";
  }

  setBackendSqliteMainFilePath(value: string) {
    this.backendSqliteMainFilePath = value;
  }

  getBackendSqliteMainPoolSize(): number {
    return this.backendSqliteMainPoolSize ?? 1;
  }

  setBackendSqliteMainPoolSize(value: number | undefined) {
    this.backendSqliteMainPoolSize = value;
  }

  getBackendSqliteCollectionFilePath(): string {
    return this.backendSqliteCollectionFilePath ?? "data/collection-development.db";
  }

  setBackendSqliteCollectionFilePath(value: string) {
    this.backendSqliteCollectionFilePath = value;
  }

  getBackendSqliteCollectionPoolSize(): number {
    return this.backendSqliteCollectionPoolSize ?? 1;
  }

  setBackendSqliteCollectionPoolSize(value: number | undefined) {
    this.backendSqliteCollectionPoolSize = value;
  }

  getBackendSqliteSessionFilePath(): string {
    return this.backendSqliteSessionFilePath ?? "data/session-development.db";
  }

  setBackendSqliteSessionFilePath(value: string) {
    this.backendSqliteSessionFilePath = value;
  }

  getBackendSqliteSessionPoolSize(): number {
    return this.backendSqliteSessionPoolSize ?? 1;
  }

  setBackendSqliteSessionPoolSize(value: number | undefined) {
    this.backendSqliteSessionPoolSize = value;
  }

  getBackendSessionSecret(): string | undefined {
    return this.backendSessionSecret;
  }

  setBackendSessionSecret(value: string | undefined) {
    this.backendSessionSecret = value;
  }

  /**
   * Returns the backend session secret as bytes (Uint8Array), if configured.
   */
  getBackendSessionSecretBytes(): Uint8Array | undefined {
    if (this.backendSessionSecret === undefined) {
      return undefined;
    }
    return new TextEncoder().encode(this.backendSessionSecret);
  }

  getBackendSessionTtlSeconds(): number {
    return this.backendSessionTtlSeconds ?? 1209600;
  }

  setBackendSessionTtlSeconds(value: number | undefined) {
    this.backendSessionTtlSeconds = value;
  }

  // --------------------
  // Computed getters
  // --------------------

  getBackendUrl(): string {
    const protocol = this.getBackendProtocol();
    const backendSub = this.getBackendSubdomain();
    const domain = this.getDomain();
    const apiPath = this.getApiPath();
    const port = this.getBackendPort();
    const portString = port ? `:${port}` : "";
    return `${protocol}://${backendSub}.${domain}${portString}/${apiPath}`;
  }

  // --------------------
  // Helper methods
  // --------------------

  private loadEnvString(prefix: string, key: string): string | undefined {
    const value = this.env[prefix + key];
    return (value && value.length > 0) ? value : undefined;
  }

  private loadEnvBool(prefix: string, key: string): boolean | undefined {
    const value = this.env[prefix + key];
    return value === "Y" ? true : (value === undefined ? undefined : false);
  }

  private loadEnvNumber(prefix: string, key: string): number | undefined {
    const value = this.env[prefix + key];
    if (value === undefined || value.length === 0) {
      return undefined;
    }
    const parsed = parseInt(value, 10);
    return isNaN(parsed) ? undefined : parsed;
  }
}
