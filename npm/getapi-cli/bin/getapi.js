#!/usr/bin/env node

"use strict";

const { execFileSync } = require("child_process");
const os = require("os");
const path = require("path");

const PLATFORMS = {
  "darwin-arm64": "getapi-cli-darwin-arm64",
  "darwin-x64": "getapi-cli-darwin-x64",
  "linux-x64": "getapi-cli-linux-x64",
  "linux-arm64": "getapi-cli-linux-arm64",
  "win32-x64": "getapi-cli-win32-x64",
};

const platform = os.platform();
const arch = os.arch();
const key = `${platform}-${arch}`;
const pkg = PLATFORMS[key];

if (!pkg) {
  console.error(
    `getapi: unsupported platform ${platform}-${arch}. ` +
      `Supported: ${Object.keys(PLATFORMS).join(", ")}`
  );
  process.exit(1);
}

const exe = platform === "win32" ? "getapi.exe" : "getapi";

let binPath;
try {
  binPath = path.join(
    path.dirname(require.resolve(`${pkg}/package.json`)),
    "bin",
    exe
  );
} catch {
  console.error(
    `getapi: could not find the platform package "${pkg}". ` +
      `Try reinstalling with: npm install getapi-cli`
  );
  process.exit(1);
}

try {
  const result = execFileSync(binPath, process.argv.slice(2), {
    stdio: "inherit",
  });
} catch (e) {
  if (e.status !== null) {
    process.exit(e.status);
  }
  throw e;
}
