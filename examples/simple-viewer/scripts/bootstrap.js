const baseUrl = (window.CESIUM_BASE_URL ?? "/Cesium/").replace(/\/?$/, "/");
window.CESIUM_BASE_URL = baseUrl;

const ionToken = window.CESIUM_ION_TOKEN ?? null;

async function ensureCesium() {
  if (!window.Cesium) {
    await new Promise((resolve, reject) => {
      const script = document.createElement("script");
      script.src = `${baseUrl}Cesium.js`;
      script.onload = resolve;
      script.onerror = reject;
      document.head.appendChild(script);
    });
  }

  if (ionToken && window.Cesium?.Ion) {
    window.Cesium.Ion.defaultAccessToken = ionToken;
  }
}

await ensureCesium();

const init = (await import("/simple-viewer.js")).default;
await init();
