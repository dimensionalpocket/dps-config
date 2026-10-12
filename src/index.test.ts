import { describe, it, expect, afterEach } from "bun:test";
import { DpsConfig } from "./index";

describe("DpsConfig", () => {
  it("should have correct default values", () => {
    const config = new DpsConfig({});
    expect(config.getProjectName()).toBe("My Project");
    expect(config.getDomain()).toBe("dps.localhost");
    expect(config.getApiPath()).toBe("api");
    expect(config.getDevelopmentMode()).toBe(false);
    expect(config.getBackendSubdomain()).toBe("auth");
    expect(config.getBackendProtocol()).toBe("https");
    expect(config.getBackendInsecureCookie()).toBe(false);
    expect(config.getBackendSqliteMainFilePath()).toBe("data/main-development.db");
    expect(config.getBackendSqliteMainPoolSize()).toBe(1);
    expect(config.getBackendSqliteCollectionFilePath()).toBe("data/collection-development.db");
    expect(config.getBackendSqliteCollectionPoolSize()).toBe(1);
    expect(config.getBackendSqliteSessionFilePath()).toBe("data/session-development.db");
    expect(config.getBackendSqliteSessionPoolSize()).toBe(1);
    expect(config.getBackendPort()).toBeUndefined();
    expect(config.getBackendSessionSecret()).toBeUndefined();
    expect(config.getBackendSessionSecretBytes()).toBeUndefined();
  });

  it("should work with setters", () => {
    const config = new DpsConfig({});
    config.setProjectName("Custom Project");
    config.setDomain("example.com");
    config.setApiPath("v1");
    config.setDevelopmentMode(true);
    config.setBackendPort(3000);
    config.setBackendSessionSecret("s3cr3t");

    expect(config.getProjectName()).toBe("Custom Project");
    expect(config.getDomain()).toBe("example.com");
    expect(config.getApiPath()).toBe("v1");
    expect(config.getDevelopmentMode()).toBe(true);
    expect(config.getBackendPort()).toBe(3000);
    expect(config.getBackendSessionSecret()).toBe("s3cr3t");
  });

  it("should build backend URL without port", () => {
    const config = new DpsConfig({});
    config.setBackendProtocol("https");
    config.setBackendSubdomain("auth");
    config.setDomain("dps.localhost");
    expect(config.getBackendUrl()).toBe("https://auth.dps.localhost/api");
  });

  it("should build backend URL with port", () => {
    const config = new DpsConfig({});
    config.setBackendProtocol("http");
    config.setBackendPort(3000);
    config.setBackendSubdomain("auth");
    config.setDomain("dps.localhost");
    expect(config.getBackendUrl()).toBe("http://auth.dps.localhost:3000/api");
  });

  it("should follow README example", () => {
    const config = new DpsConfig({});
    config.setDomain("test.local");
    config.setBackendProtocol("http");
    config.setBackendPort(8080);
    expect(config.getBackendUrl()).toBe("http://auth.test.local:8080/api");
  });

  it("should load insecure cookie setting from env", () => {
    const c1 = new DpsConfig({});
    expect(c1.getBackendInsecureCookie()).toBe(false);
    c1.setBackendInsecureCookie(true);
    expect(c1.getBackendInsecureCookie()).toBe(true);

    const c2 = new DpsConfig({ DPS_BACKEND_INSECURE_COOKIE: "Y" });
    expect(c2.getBackendInsecureCookie()).toBe(true);
  });

  it("should load SQLite pool sizes from env", () => {
    const config = new DpsConfig({
      DPS_BACKEND_SQLITE_MAIN_POOL_SIZE: "8",
      DPS_BACKEND_SQLITE_COLLECTION_POOL_SIZE: "4",
      DPS_BACKEND_SQLITE_SESSION_POOL_SIZE: "2",
    });
    expect(config.getBackendSqliteMainPoolSize()).toBe(8);
    expect(config.getBackendSqliteCollectionPoolSize()).toBe(4);
    expect(config.getBackendSqliteSessionPoolSize()).toBe(2);
  });

  it("should load SQLite file paths from env", () => {
    const config = new DpsConfig({
      DPS_BACKEND_SQLITE_MAIN_FILE_PATH: "data/test-main.db",
    });
    expect(config.getBackendSqliteMainFilePath()).toBe("data/test-main.db");
  });

  it("should return session secret bytes", () => {
    const config = new DpsConfig({});
    config.setBackendSessionSecret("my-secret-key");
    const bytes = config.getBackendSessionSecretBytes();
    expect(bytes).toBeDefined();
    expect(new TextDecoder().decode(bytes)).toBe("my-secret-key");
  });

  it("should load session TTL from env", () => {
    const config = new DpsConfig({ DPS_BACKEND_SESSION_TTL_SECONDS: "1800" });
    expect(config.getBackendSessionTtlSeconds()).toBe(1800);
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
