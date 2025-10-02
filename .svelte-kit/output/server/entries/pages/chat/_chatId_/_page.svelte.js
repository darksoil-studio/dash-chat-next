import { X as store_get, Z as ensure_array_like, U as attr_class, _ as attr, Y as unsubscribe_stores, V as stringify } from "../../../../chunks/index2.js";
import "@tauri-apps/api/core";
import { o as onDestroy } from "../../../../chunks/index-server.js";
import { p as page } from "../../../../chunks/stores.js";
import { p as participants, m as myPublicKey, a as messages } from "../../../../chunks/stores2.js";
import { e as escape_html } from "../../../../chunks/context.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    var $$store_subs;
    store_get($$store_subs ??= {}, "$page", page).params.chatId;
    let newMessage = "";
    let membersInterval;
    function getParticipant(publicKey) {
      return store_get($$store_subs ??= {}, "$participants", participants).get(publicKey) || { publicKey, name: publicKey, avatar: "" };
    }
    function isMyMessage(publicKey) {
      return publicKey === store_get($$store_subs ??= {}, "$myPublicKey", myPublicKey);
    }
    function formatTimestamp(timestamp) {
      if (timestamp === void 0) {
        return "???";
      }
      return new Date(timestamp).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
    }
    onDestroy(() => {
      clearInterval(membersInterval);
    });
    $$renderer2.push(`<div class="chat-view svelte-kjloo9"><header class="chat-header svelte-kjloo9"><div class="chat-header-left svelte-kjloo9"><button class="back-btn svelte-kjloo9">←</button> <div class="chat-info svelte-kjloo9"><h2 class="svelte-kjloo9">Group Chat</h2> <p class="svelte-kjloo9">${escape_html(Object.keys(store_get($$store_subs ??= {}, "$participants", participants)).length)} member${escape_html(Object.keys(store_get($$store_subs ??= {}, "$participants", participants)).length !== 1 ? "s" : "")}</p></div></div> <button class="btn btn-small btn-outline svelte-kjloo9">Add Member</button></header> <div class="messages-container svelte-kjloo9">`);
    if (store_get($$store_subs ??= {}, "$messages", messages).length === 0) {
      $$renderer2.push("<!--[-->");
      $$renderer2.push(`<div class="empty-chat svelte-kjloo9"><p>No messages yet. Start the conversation!</p></div>`);
    } else {
      $$renderer2.push("<!--[!-->");
      $$renderer2.push(`<!--[-->`);
      const each_array = ensure_array_like(store_get($$store_subs ??= {}, "$messages", messages));
      for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
        let message = each_array[$$index];
        const participant = getParticipant(message.author);
        const isMine = isMyMessage(message.author);
        $$renderer2.push(`<div${attr_class(`message ${stringify(isMine ? "message-mine" : "message-other")}`, "svelte-kjloo9")}>`);
        if (!isMine && participant) {
          $$renderer2.push("<!--[-->");
          $$renderer2.push(`<img${attr("src", participant.avatar)}${attr("alt", participant.name)} class="message-avatar svelte-kjloo9"/>`);
        } else {
          $$renderer2.push("<!--[!-->");
        }
        $$renderer2.push(`<!--]--> <div class="message-content svelte-kjloo9">`);
        if (!isMine && participant) {
          $$renderer2.push("<!--[-->");
          $$renderer2.push(`<div class="message-author svelte-kjloo9">${escape_html(participant.name)}</div>`);
        } else {
          $$renderer2.push("<!--[!-->");
        }
        $$renderer2.push(`<!--]--> <div class="message-bubble svelte-kjloo9">${escape_html(message.content)}</div> <div class="message-time svelte-kjloo9">${escape_html(formatTimestamp(message.timestamp))}</div></div></div>`);
      }
      $$renderer2.push(`<!--]-->`);
    }
    $$renderer2.push(`<!--]--></div> <div class="message-input-container svelte-kjloo9"><form class="svelte-kjloo9"><input${attr("value", newMessage)} placeholder="Type a message..." class="message-input svelte-kjloo9"/> <button type="submit" class="send-btn svelte-kjloo9"${attr("disabled", !newMessage.trim(), true)}>Send</button></form></div></div> `);
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
