

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_layout.svelte.js')).default;
export const universal = {
  "ssr": false
};
export const universal_id = "src/routes/+layout.ts";
export const imports = ["_app/immutable/nodes/0.CBL-MVNK.js","_app/immutable/chunks/Bzak7iHL.js","_app/immutable/chunks/DKTCnNbX.js","_app/immutable/chunks/D6nPkcoS.js","_app/immutable/chunks/BDAMK1gS.js","_app/immutable/chunks/DycXGPVy.js","_app/immutable/chunks/CwdQBBZU.js","_app/immutable/chunks/B5_lSiN-.js"];
export const stylesheets = ["_app/immutable/assets/0.BfcZ9XOJ.css"];
export const fonts = [];
