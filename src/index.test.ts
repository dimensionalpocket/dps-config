import { describe, it, expect, afterEach } from "bun:test";
import { DpsConfig } from "./index";

describe("DpsConfig", () => {
  it("should have correct default values", () => {
    const config = new DpsConfig({});
    expect(config.getProjectName()).toBe("My Project");
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
    const config = new DpsConfig({});
    config.setProjectName("Custom Project");
    config.setDomain("example.com");
    config.setApiPath("v1");
    config.setDevelopmentMode(true);
    config.setAuthApiPort(3000);
    config.setAuthApiSessionSecret("s3cr3t");

    expect(config.getProjectName()).toBe("Custom Project");
    expect(config.getDomain()).toBe("example.com");
    expect(config.getApiPath()).toBe("v1");
    expect(config.getDevelopmentMode()).toBe(true);
    expect(config.getAuthApiPort()).toBe(3000);
    expect(config.getAuthApiSessionSecret()).toBe("s3cr3t");
  });

  it("should build auth API URL without port", () => {
    const config = new DpsConfig({});
    config.setAuthApiProtocol("https");
    config.setAuthApiSubdomain("auth");
    config.setDomain("dps.localhost");
    expect(config.getAuthApiUrl()).toBe("https://auth.dps.localhost/api");
  });

  it("should build auth API URL with port", () => {
    const config = new DpsConfig({});
    config.setAuthApiProtocol("http");
    config.setAuthApiPort(3000);
    config.setAuthApiSubdomain("auth");
    config.setDomain("dps.localhost");
    expect(config.getAuthApiUrl()).toBe("http://auth.dps.localhost:3000/api");
  });

  it("should follow README example", () => {
    const config = new DpsConfig({});
    config.setDomain("test.local");
    config.setAuthApiProtocol("http");
    config.setAuthApiPort(8080);
    expect(config.getAuthApiUrl()).toBe("http://auth.test.local:8080/api");
  });

  it("should load insecure cookie setting from env", () => {
    const c1 = new DpsConfig({});
    expect(c1.getAuthApiInsecureCookie()).toBe(false);
    c1.setAuthApiInsecureCookie(true);
    expect(c1.getAuthApiInsecureCookie()).toBe(true);

    const c2 = new DpsConfig({ DPS_AUTH_API_INSECURE_COOKIE: "Y" });
    expect(c2.getAuthApiInsecureCookie()).toBe(true);
  });

  it("should load SQLite pool sizes from env", () => {
    const config = new DpsConfig({
      DPS_AUTH_API_SQLITE_MAIN_POOL_SIZE: "8",
      DPS_AUTH_API_SQLITE_COLLECTION_POOL_SIZE: "4",
      DPS_AUTH_API_SQLITE_SESSION_POOL_SIZE: "2",
    });
    expect(config.getAuthApiSqliteMainPoolSize()).toBe(8);
    expect(config.getAuthApiSqliteCollectionPoolSize()).toBe(4);
    expect(config.getAuthApiSqliteSessionPoolSize()).toBe(2);
  });

  it("should load SQLite file paths from env", () => {
    const config = new DpsConfig({
      DPS_AUTH_API_SQLITE_MAIN_FILE_PATH: "data/test-main.db",
    });
    expect(config.getAuthApiSqliteMainFilePath()).toBe("data/test-main.db");
  });

  it("should return session secret bytes", () => {
    const config = new DpsConfig({});
    config.setAuthApiSessionSecret("my-secret-key");
    const bytes = config.getAuthApiSessionSecretBytes();
    expect(bytes).toBeDefined();
    expect(new TextDecoder().decode(bytes)).toBe("my-secret-key");
  });

  it("should load session TTL from env", () => {
    const config = new DpsConfig({ DPS_AUTH_API_SESSION_TTL_SECONDS: "1800" });
    expect(config.getAuthApiSessionTtlSeconds()).toBe(1800);
  });

  it("should load API path from env", () => {
    const config = new DpsConfig({ DPS_API_PATH: "api/v2" });
    expect(config.getApiPath()).toBe("api/v2");
  });

  it("should load project name from env", () => {
    const config = new DpsConfig({ DPS_PROJECT_NAME: "Env Project" });
    expect(config.getProjectName()).toBe("Env Project");
  });

  it("should support envPrefix", () => {
    const env = {
      VITE_DPS_DOMAIN: "vite.local",
      DPS_DOMAIN: "standard.local",
    };

    const config = new DpsConfig(env, "VITE_");
    expect(config.getDomain()).toBe("vite.local");

    const standardConfig = new DpsConfig(env);
    expect(standardConfig.getDomain()).toBe("standard.local");
  });

  it("should only store known environment variables", () => {
    const env = {
      DPS_DOMAIN: "example.com",
      SECRET_KEY: "super-secret",
    };
    const config = new DpsConfig(env);

    // @ts-ignore - accessing private field for testing
    expect(config.env["DPS_DOMAIN"]).toBe("example.com");
    // @ts-ignore - accessing private field for testing
    expect(config.env["SECRET_KEY"]).toBeUndefined();
  });
});
