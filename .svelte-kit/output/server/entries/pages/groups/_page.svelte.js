import { X as store_get, Z as ensure_array_like, Y as unsubscribe_stores } from "../../../chunks/index2.js";
import "@tauri-apps/api/core";
import { o as onDestroy } from "../../../chunks/index-server.js";
import "@sveltejs/kit/internal";
import "../../../chunks/exports.js";
import "../../../chunks/utils.js";
import { e as escape_html } from "../../../chunks/context.js";
import "../../../chunks/state.svelte.js";
import { g as groups } from "../../../chunks/stores2.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    var $$store_subs;
    let groupsInterval;
    onDestroy(() => {
      clearInterval(groupsInterval);
    });
    $$renderer2.push(`<div class="groups-view svelte-1sgss7h"><header class="groups-header svelte-1sgss7h"><h1 class="svelte-1sgss7h">Groups</h1> <div class="group-actions svelte-1sgss7h"><button class="btn btn-primary svelte-1sgss7h">Create Group</button> <button class="btn btn-secondary svelte-1sgss7h">Join Group</button></div></header> <div class="groups-list svelte-1sgss7h">`);
    if (store_get($$store_subs ??= {}, "$groups", groups).length === 0) {
      $$renderer2.push("<!--[-->");
      $$renderer2.push(`<div class="empty-state svelte-1sgss7h"><p>No groups yet. Create one or join an existing group!</p></div>`);
    } else {
      $$renderer2.push("<!--[!-->");
      $$renderer2.push(`<!--[-->`);
      const each_array = ensure_array_like(store_get($$store_subs ??= {}, "$groups", groups));
      for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
        let group = each_array[$$index];
        $$renderer2.push(`<div class="group-card svelte-1sgss7h" role="button" tabindex="0"><div class="group-info svelte-1sgss7h"><h3 class="group-name svelte-1sgss7h">${escape_html(group.name)}</h3> <p class="group-members svelte-1sgss7h">${escape_html(group.memberCount)} member${escape_html(group.memberCount !== 1 ? "s" : "")}</p></div> <button class="btn btn-small btn-outline svelte-1sgss7h">Copy Join Code</button></div>`);
      }
      $$renderer2.push(`<!--]-->`);
    }
    $$renderer2.push(`<!--]--></div></div> `);
    {
      $$renderer2.push("<!--[!-->");
    }
    $$renderer2.push(`<!--]--> `);
    {
      $$renderer2.push("<!--[!-->");
    }
    $$renderer2.push(`<!--]-->`);
    if ($$store_subs) unsubscribe_stores($$store_subs);
  });
}
export {
  _page as default
};
