import { w as writable } from "./index.js";
const groups = writable([]);
const friends = writable([]);
const messages = writable([]);
const participants = writable(/* @__PURE__ */ new Map());
const myPublicKey = writable("");
const toastMessage = writable("");
const showToast = writable(false);
const isErrorToast = writable(false);
export {
  messages as a,
  friends as f,
  groups as g,
  isErrorToast as i,
  myPublicKey as m,
  participants as p,
  showToast as s,
  toastMessage as t
};
