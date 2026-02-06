/**
 * Configuration management for the DPS ecosystem.
 *
 * This class provides a lightweight configuration container mirroring the Rust implementation.
 * It focuses on optional values, sensible defaults, and environment variable loading.
 */
export class DpsConfig {
  private domain?: string;
  private apiPath?: string;
  private developmentMode?: boolean;

  private authApiSubdomain?: string;
  private authApiPort?: number;
  private authApiProtocol?: string;
  private authApiInsecureCookie?: boolean;
  private authApiSqliteMainFilePath?: string;
  private authApiSqliteMainPoolSize?: number;
  private authApiSqliteCollectionFilePath?: string;
  private authApiSqliteCollectionPoolSize?: number;
  private authApiSqliteSessionFilePath?: string;
  private authApiSqliteSessionPoolSize?: number;
  private authApiSessionSecret?: string;
  private authApiSessionTtlSeconds?: number;

  /**
   * Create a new DpsConfig instance, loading values from environment variables when present.
   *
   * @param envPrefix Optional prefix for environment variables (e.g. "VITE_" for Vite support).
   */
  constructor(envPrefix: string = "") {
    this.domain = this.loadEnvString(envPrefix, "DPS_DOMAIN");
    this.apiPath = this.loadEnvString(envPrefix, "DPS_API_PATH");
    this.developmentMode = this.loadEnvBool(envPrefix, "DPS_DEVELOPMENT_MODE");

    this.authApiSubdomain = this.loadEnvString(envPrefix, "DPS_AUTH_API_SUBDOMAIN");
    this.authApiPort = this.loadEnvNumber(envPrefix, "DPS_AUTH_API_PORT");
    this.authApiProtocol = this.loadEnvString(envPrefix, "DPS_AUTH_API_PROTOCOL");
    this.authApiInsecureCookie = this.loadEnvBool(envPrefix, "DPS_AUTH_API_INSECURE_COOKIE");
    this.authApiSqliteMainFilePath = this.loadEnvString(envPrefix, "DPS_AUTH_API_SQLITE_MAIN_FILE_PATH");
    this.authApiSqliteMainPoolSize = this.loadEnvNumber(envPrefix, "DPS_AUTH_API_SQLITE_MAIN_POOL_SIZE");
    this.authApiSqliteCollectionFilePath = this.loadEnvString(envPrefix, "DPS_AUTH_API_SQLITE_COLLECTION_FILE_PATH");
    this.authApiSqliteCollectionPoolSize = this.loadEnvNumber(envPrefix, "DPS_AUTH_API_SQLITE_COLLECTION_POOL_SIZE");
    this.authApiSqliteSessionFilePath = this.loadEnvString(envPrefix, "DPS_AUTH_API_SQLITE_SESSION_FILE_PATH");
    this.authApiSqliteSessionPoolSize = this.loadEnvNumber(envPrefix, "DPS_AUTH_API_SQLITE_SESSION_POOL_SIZE");
    this.authApiSessionSecret = this.loadEnvString(envPrefix, "DPS_AUTH_API_SESSION_SECRET");
    this.authApiSessionTtlSeconds = this.loadEnvNumber(envPrefix, "DPS_AUTH_API_SESSION_TTL_SECONDS");
  }

  // --------------------
  // Global getters/setters
  // --------------------

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
  // DpsAuthApi getters/setters
  // --------------------

  getAuthApiSubdomain(): string {
    return this.authApiSubdomain ?? "auth";
  }

  setAuthApiSubdomain(value: string) {
    this.authApiSubdomain = value;
  }

  getAuthApiPort(): number | undefined {
    return this.authApiPort;
  }

  setAuthApiPort(value: number | undefined) {
    this.authApiPort = value;
  }

  getAuthApiProtocol(): string {
    return this.authApiProtocol ?? "https";
  }

  setAuthApiProtocol(value: string) {
    this.authApiProtocol = value;
  }

  getAuthApiInsecureCookie(): boolean {
    return this.authApiInsecureCookie ?? false;
  }

  setAuthApiInsecureCookie(value: boolean) {
    this.authApiInsecureCookie = value;
  }

  getAuthApiSqliteMainFilePath(): string {
    return this.authApiSqliteMainFilePath ?? "data/main-development.db";
  }

  setAuthApiSqliteMainFilePath(value: string) {
    this.authApiSqliteMainFilePath = value;
  }

  getAuthApiSqliteMainPoolSize(): number {
    return this.authApiSqliteMainPoolSize ?? 1;
  }

  setAuthApiSqliteMainPoolSize(value: number | undefined) {
    this.authApiSqliteMainPoolSize = value;
  }

  getAuthApiSqliteCollectionFilePath(): string {
    return this.authApiSqliteCollectionFilePath ?? "data/collection-development.db";
  }

  setAuthApiSqliteCollectionFilePath(value: string) {
    this.authApiSqliteCollectionFilePath = value;
  }

  getAuthApiSqliteCollectionPoolSize(): number {
    return this.authApiSqliteCollectionPoolSize ?? 1;
  }

  setAuthApiSqliteCollectionPoolSize(value: number | undefined) {
    this.authApiSqliteCollectionPoolSize = value;
  }

  getAuthApiSqliteSessionFilePath(): string {
    return this.authApiSqliteSessionFilePath ?? "data/session-development.db";
  }

  setAuthApiSqliteSessionFilePath(value: string) {
    this.authApiSqliteSessionFilePath = value;
  }

  getAuthApiSqliteSessionPoolSize(): number {
    return this.authApiSqliteSessionPoolSize ?? 1;
  }

  setAuthApiSqliteSessionPoolSize(value: number | undefined) {
    this.authApiSqliteSessionPoolSize = value;
  }

  getAuthApiSessionSecret(): string | undefined {
    return this.authApiSessionSecret;
  }

  setAuthApiSessionSecret(value: string | undefined) {
    this.authApiSessionSecret = value;
  }

  /**
   * Returns the auth API session secret as bytes (Uint8Array), if configured.
   */
  getAuthApiSessionSecretBytes(): Uint8Array | undefined {
    if (this.authApiSessionSecret === undefined) {
      return undefined;
    }
    return new TextEncoder().encode(this.authApiSessionSecret);
  }

  getAuthApiSessionTtlSeconds(): number {
    return this.authApiSessionTtlSeconds ?? 1209600;
  }

  setAuthApiSessionTtlSeconds(value: number | undefined) {
    this.authApiSessionTtlSeconds = value;
  }

  // --------------------
  // Computed getters
  // --------------------

  getAuthApiUrl(): string {
    const protocol = this.getAuthApiProtocol();
    const authSub = this.getAuthApiSubdomain();
    const domain = this.getDomain();
    const apiPath = this.getApiPath();
    const port = this.getAuthApiPort();
    const portString = port ? `:${port}` : "";
    return `${protocol}://${authSub}.${domain}${portString}/${apiPath}`;
  }

  // --------------------
  // Helper methods
  // --------------------

  private loadEnvString(prefix: string, key: string): string | undefined {
    const value = process.env[prefix + key];
    return (value && value.length > 0) ? value : undefined;
  }

  private loadEnvBool(prefix: string, key: string): boolean | undefined {
    const value = process.env[prefix + key];
    return value === "Y" ? true : (value === undefined ? undefined : false);
  }

  private loadEnvNumber(prefix: string, key: string): number | undefined {
    const value = process.env[prefix + key];
    if (value === undefined || value.length === 0) {
      return undefined;
    }
    const parsed = parseInt(value, 10);
    return isNaN(parsed) ? undefined : parsed;
  }
}
