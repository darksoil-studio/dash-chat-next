import { a as ssr_context } from "./context.js";
function onDestroy(fn) {
  /** @type {SSRContext} */
  ssr_context.r.on_destroy(fn);
}
export {
  onDestroy as o
};
