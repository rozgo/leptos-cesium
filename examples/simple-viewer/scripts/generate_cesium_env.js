const fs = require("fs");
const path = require("path");

const ROOT = path.resolve(__dirname, "..");
const envPath = path.join(ROOT, ".env.local");
const outputPath = path.join(ROOT, "public", "cesium-env.js");

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

let shouldWrite = true;
if (fs.existsSync(outputPath)) {
  try {
    const current = fs.readFileSync(outputPath, "utf8");
    if (current === output) {
      shouldWrite = false;
      console.info("[hook] cesium-env.js already up to date");
    }
  } catch (err) {
    console.error("[hook] Failed to read existing cesium-env.js:", err);
  }
}

if (shouldWrite) {
  try {
    fs.writeFileSync(outputPath, output, "utf8");
    console.info("[hook] Wrote", outputPath, token ? "(token set)" : "(no token)");
  } catch (err) {
    console.error("[hook] Failed to write", outputPath, err);
    process.exitCode = 1;
  }
}
