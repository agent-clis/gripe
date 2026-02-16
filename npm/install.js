#!/usr/bin/env node
"use strict";

const https = require("https");
const fs = require("fs");
const path = require("path");
const { execSync } = require("child_process");

const REPO = "agent-clis/gripe";
const BIN_DIR = path.join(__dirname, "bin");

function getPlatform() {
  const platform = process.platform;
  const arch = process.arch;

  const osMap = { darwin: "darwin", linux: "linux", win32: "windows" };
  const archMap = { x64: "amd64", arm64: "arm64" };

  const os = osMap[platform];
  const cpu = archMap[arch];

  if (!os || !cpu) {
    console.error(`Unsupported platform: ${platform}-${arch}`);
    process.exit(1);
  }

  return { os, arch: cpu };
}

function download(url) {
  return new Promise((resolve, reject) => {
    https.get(url, (res) => {
      if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
        return download(res.headers.location).then(resolve, reject);
      }
      if (res.statusCode !== 200) {
        return reject(new Error(`Download failed: HTTP ${res.statusCode}`));
      }
      const chunks = [];
      res.on("data", (chunk) => chunks.push(chunk));
      res.on("end", () => resolve(Buffer.concat(chunks)));
      res.on("error", reject);
    }).on("error", reject);
  });
}

async function main() {
  const { os, arch } = getPlatform();
  const artifact = `gripe-${os}-${arch}`;
  const isWindows = os === "windows";
  const ext = isWindows ? "zip" : "tar.gz";
  const url = `https://github.com/${REPO}/releases/latest/download/${artifact}.${ext}`;

  console.log(`Downloading gripe for ${os}-${arch}...`);
  const data = await download(url);

  fs.mkdirSync(BIN_DIR, { recursive: true });

  const archive = path.join(BIN_DIR, `${artifact}.${ext}`);
  fs.writeFileSync(archive, data);

  if (isWindows) {
    execSync(`powershell -Command "Expand-Archive -Path '${archive}' -DestinationPath '${BIN_DIR}' -Force"`, { stdio: "inherit" });
    const src = path.join(BIN_DIR, `${artifact}.exe`);
    const dest = path.join(BIN_DIR, "gripe.exe");
    if (fs.existsSync(src) && src !== dest) fs.renameSync(src, dest);
  } else {
    execSync(`tar xzf "${archive}" -C "${BIN_DIR}"`, { stdio: "inherit" });
    const src = path.join(BIN_DIR, artifact);
    const dest = path.join(BIN_DIR, "gripe");
    if (fs.existsSync(src) && src !== dest) fs.renameSync(src, dest);
    fs.chmodSync(dest, 0o755);
  }

  fs.unlinkSync(archive);
  console.log("gripe installed successfully.");
}

main().catch((err) => {
  console.error("Failed to install gripe:", err.message);
  process.exit(1);
});
