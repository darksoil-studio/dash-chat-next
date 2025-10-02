import { X as store_get, Z as ensure_array_like, Y as unsubscribe_stores } from "../../../chunks/index2.js";
import "@tauri-apps/api/core";
import { f as friends } from "../../../chunks/stores2.js";
import { e as escape_html } from "../../../chunks/context.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    var $$store_subs;
    $$renderer2.push(`<div class="friends-view svelte-1ggmuka"><header class="friends-header svelte-1ggmuka"><h1 class="svelte-1ggmuka">Friends</h1> <div class="friend-actions svelte-1ggmuka"><button class="btn btn-primary svelte-1ggmuka">Add Friend</button></div></header> <div class="friends-list svelte-1ggmuka">`);
    if (store_get($$store_subs ??= {}, "$friends", friends).length === 0) {
      $$renderer2.push("<!--[-->");
      $$renderer2.push(`<div class="empty-state svelte-1ggmuka"><p>No friends yet. Add a friend using their FriendCode!</p></div>`);
    } else {
      $$renderer2.push("<!--[!-->");
      $$renderer2.push(`<!--[-->`);
      const each_array = ensure_array_like(store_get($$store_subs ??= {}, "$friends", friends));
      for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
        let friend = each_array[$$index];
        $$renderer2.push(`<div class="friend-card svelte-1ggmuka"><div class="friend-info svelte-1ggmuka"><h3 class="friend-key svelte-1ggmuka">${escape_html(friend.publicKey)}</h3> <p class="friend-added svelte-1ggmuka">Added ${escape_html(new Date(friend.addedAt).toLocaleDateString())}</p></div> <button class="btn btn-small btn-outline svelte-1ggmuka">Remove</button></div>`);
      }
      $$renderer2.push(`<!--]-->`);
    }
    $$renderer2.push(`<!--]--></div></div> `);
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
