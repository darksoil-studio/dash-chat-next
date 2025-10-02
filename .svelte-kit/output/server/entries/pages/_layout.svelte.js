import { U as attr_class, V as stringify, W as slot, X as store_get, Y as unsubscribe_stores } from "../../chunks/index2.js";
import { p as page } from "../../chunks/stores.js";
import "@sveltejs/kit/internal";
import "../../chunks/exports.js";
import "../../chunks/utils.js";
import { e as escape_html } from "../../chunks/context.js";
import "../../chunks/state.svelte.js";
import "@tauri-apps/api/core";
import { s as showToast, i as isErrorToast, t as toastMessage } from "../../chunks/stores2.js";
function _layout($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    var $$store_subs;
    function isActive(path) {
      return store_get($$store_subs ??= {}, "$page", page).url.pathname === path;
    }
    $$renderer2.push(`<main class="app svelte-12qhfyh"><nav class="nav-bar svelte-12qhfyh"><div class="nav-content svelte-12qhfyh"><h1 class="nav-title svelte-12qhfyh">p2pandashchat!</h1> <div class="nav-links svelte-12qhfyh"><button${attr_class(`nav-link ${stringify(isActive("/groups") ? "active" : "")}`, "svelte-12qhfyh")}>Groups</button> <button${attr_class(`nav-link ${stringify(isActive("/friends") ? "active" : "")}`, "svelte-12qhfyh")}>Friends</button> <button class="nav-link btn-outline svelte-12qhfyh">Copy Friend Code</button></div></div></nav> <div class="main-content svelte-12qhfyh"><!---->`);
    slot($$renderer2, $$props, "default", {});
    $$renderer2.push(`<!----></div></main> `);
    if (store_get($$store_subs ??= {}, "$showToast", showToast)) {
      $$renderer2.push("<!--[-->");
      $$renderer2.push(`<div${attr_class(`toast ${stringify(store_get($$store_subs ??= {}, "$isErrorToast", isErrorToast) ? "error" : "")} ${stringify(store_get($$store_subs ??= {}, "$showToast", showToast) ? "toast-show" : "")}`, "svelte-12qhfyh")}><div class="toast-content svelte-12qhfyh"><div class="toast-icon svelte-12qhfyh">${escape_html(store_get($$store_subs ??= {}, "$isErrorToast", isErrorToast) ? "✕" : "✓")}</div> <div class="toast-message svelte-12qhfyh">${escape_html(store_get($$store_subs ??= {}, "$toastMessage", toastMessage))}</div></div></div>`);
    } else {
      $$renderer2.push("<!--[!-->");
    }
    $$renderer2.push(`<!--]-->`);
    if ($$store_subs) unsubscribe_stores($$store_subs);
  });
}
export {
  _layout as default
};
