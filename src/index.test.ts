import { describe, it, expect, afterEach } from "bun:test";
import { DpsConfig } from "./index";

describe("DpsConfig", () => {
  afterEach(() => {
    // Clean up environment variables after each test
    const keysToRemove = Object.keys(process.env).filter(key => key.startsWith("DPS_") || key.startsWith("VITE_DPS_"));
    for (const key of keysToRemove) {
      delete process.env[key];
    }
  });

  it("should have correct default values", () => {
    const config = new DpsConfig();
    expect(config.getDomain()).toBe("dps.localhost");
    expect(config.getApiPath()).toBe("api");
    expect(config.getDevelopmentMode()).toBe(false);
    expect(config.getAuthApiSubdomain()).toBe("auth");
    expect(config.getAuthApiProtocol()).toBe("https");
    expect(config.getAuthApiInsecureCookie()).toBe(false);
    expect(config.getAuthApiSqliteMainFilePath()).toBe("data/main-development.db");
    expect(config.getAuthApiSqliteMainPoolSize()).toBe(1);
    expect(config.getAuthApiSqliteCollectionFilePath()).toBe("data/collection-development.db");
    expect(config.getAuthApiSqliteCollectionPoolSize()).toBe(1);
    expect(config.getAuthApiSqliteSessionFilePath()).toBe("data/session-development.db");
    expect(config.getAuthApiSqliteSessionPoolSize()).toBe(1);
    expect(config.getAuthApiPort()).toBeUndefined();
    expect(config.getAuthApiSessionSecret()).toBeUndefined();
    expect(config.getAuthApiSessionSecretBytes()).toBeUndefined();
  });

  it("should work with setters", () => {
    const config = new DpsConfig();
    config.setDomain("example.com");
    config.setApiPath("v1");
    config.setDevelopmentMode(true);
    config.setAuthApiPort(3000);
    config.setAuthApiSessionSecret("s3cr3t");

    expect(config.getDomain()).toBe("example.com");
    expect(config.getApiPath()).toBe("v1");
    expect(config.getDevelopmentMode()).toBe(true);
    expect(config.getAuthApiPort()).toBe(3000);
    expect(config.getAuthApiSessionSecret()).toBe("s3cr3t");
  });

  it("should build auth API URL without port", () => {
    const config = new DpsConfig();
    config.setAuthApiProtocol("https");
    config.setAuthApiSubdomain("auth");
    config.setDomain("dps.localhost");
    expect(config.getAuthApiUrl()).toBe("https://auth.dps.localhost/api");
  });

  it("should build auth API URL with port", () => {
    const config = new DpsConfig();
    config.setAuthApiProtocol("http");
    config.setAuthApiPort(3000);
    config.setAuthApiSubdomain("auth");
    config.setDomain("dps.localhost");
    expect(config.getAuthApiUrl()).toBe("http://auth.dps.localhost:3000/api");
  });

  it("should follow README example", () => {
    const config = new DpsConfig();
    config.setDomain("test.local");
    config.setAuthApiProtocol("http");
    config.setAuthApiPort(8080);
    expect(config.getAuthApiUrl()).toBe("http://auth.test.local:8080/api");
  });

  it("should load insecure cookie setting from env", () => {
    const c1 = new DpsConfig();
    expect(c1.getAuthApiInsecureCookie()).toBe(false);
    c1.setAuthApiInsecureCookie(true);
    expect(c1.getAuthApiInsecureCookie()).toBe(true);

    process.env.DPS_AUTH_API_INSECURE_COOKIE = "Y";
    const c2 = new DpsConfig();
    expect(c2.getAuthApiInsecureCookie()).toBe(true);
  });

  it("should load SQLite pool sizes from env", () => {
    process.env.DPS_AUTH_API_SQLITE_MAIN_POOL_SIZE = "8";
    process.env.DPS_AUTH_API_SQLITE_COLLECTION_POOL_SIZE = "4";
    process.env.DPS_AUTH_API_SQLITE_SESSION_POOL_SIZE = "2";

    const config = new DpsConfig();
    expect(config.getAuthApiSqliteMainPoolSize()).toBe(8);
    expect(config.getAuthApiSqliteCollectionPoolSize()).toBe(4);
    expect(config.getAuthApiSqliteSessionPoolSize()).toBe(2);
  });

  it("should load SQLite file paths from env", () => {
    process.env.DPS_AUTH_API_SQLITE_MAIN_FILE_PATH = "data/test-main.db";
    const config = new DpsConfig();
    expect(config.getAuthApiSqliteMainFilePath()).toBe("data/test-main.db");
  });

  it("should return session secret bytes", () => {
    const config = new DpsConfig();
    config.setAuthApiSessionSecret("my-secret-key");
    const bytes = config.getAuthApiSessionSecretBytes();
    expect(bytes).toBeDefined();
    expect(new TextDecoder().decode(bytes)).toBe("my-secret-key");
  });

  it("should load session TTL from env", () => {
    process.env.DPS_AUTH_API_SESSION_TTL_SECONDS = "1800";
    const config = new DpsConfig();
    expect(config.getAuthApiSessionTtlSeconds()).toBe(1800);
  });

  it("should load API path from env", () => {
    process.env.DPS_API_PATH = "api/v2";
    const config = new DpsConfig();
    expect(config.getApiPath()).toBe("api/v2");
  });

  it("should support envPrefix", () => {
    process.env.VITE_DPS_DOMAIN = "vite.local";
    process.env.DPS_DOMAIN = "standard.local";

    const config = new DpsConfig("VITE_");
    expect(config.getDomain()).toBe("vite.local");

    const standardConfig = new DpsConfig();
    expect(standardConfig.getDomain()).toBe("standard.local");
  });
});
