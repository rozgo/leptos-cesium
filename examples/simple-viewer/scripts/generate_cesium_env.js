const fs = require("fs");
const path = require("path");

const ROOT = path.resolve(__dirname, "..");
const envPath = path.join(ROOT, ".env.local");
const publicPath = path.join(ROOT, "public", "cesium-env.js");

let token = null;
if (fs.existsSync(envPath)) {
  try {
    const content = fs.readFileSync(envPath, "utf8");
    const match = content.match(/^CESIUM_ION_TOKEN\s*=\s*(.*)$/m);
    if (match) {
      token = match[1].trim();
      if ((token.startsWith("\"") && token.endsWith("\"")) || (token.startsWith("'") && token.endsWith("'"))) {
        token = token.slice(1, -1);
      }
    }
  } catch (err) {
    console.error("[hook] Failed to read .env.local:", err);
  }
} else {
  console.warn("[hook] .env.local not found; proceeding without Cesium Ion token");
}

const output = `window.CESIUM_ION_TOKEN = ${token ? JSON.stringify(token) : "null"};\n`;

function writeIfChanged(targetPath) {
  try {
    const dir = path.dirname(targetPath);
    fs.mkdirSync(dir, { recursive: true });
    if (fs.existsSync(targetPath)) {
      const current = fs.readFileSync(targetPath, "utf8");
      if (current === output) {
        console.info("[hook]", targetPath, "already up to date");
        return;
      }
    }
    fs.writeFileSync(targetPath, output, "utf8");
    console.info("[hook] Wrote", targetPath, token ? "(token set)" : "(no token)");
  } catch (err) {
    console.error("[hook] Failed to write", targetPath, err);
    process.exitCode = 1;
  }
}

writeIfChanged(publicPath);

const staging = process.env.TRUNK_STAGING_DIR;
if (staging) {
  writeIfChanged(path.join(staging, "cesium-env.js"));
}

const dist = process.env.TRUNK_DIST_DIR;
if (dist) {
  writeIfChanged(path.join(dist, "cesium-env.js"));
}
