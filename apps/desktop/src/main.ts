import { invoke } from "@tauri-apps/api/core";
import "./styles.css";

type AppState = { profileExists: boolean };
const app = document.querySelector<HTMLElement>("#app")!;

async function render() {
  const state = await invoke<AppState>("app_state").catch(() => ({ profileExists: false }));
  app.innerHTML = state.profileExists ? shell() : onboarding();
  document.querySelector<HTMLFormElement>("#identity-form")?.addEventListener("submit", createIdentity);
}
function onboarding() { return `<section class="onboarding"><p class="eyebrow">LOCAL-FIRST MESSAGING</p><h1>Your conversations belong on your device.</h1><p class="lede">Create a local identity to start. This build sets up the encrypted, peer-to-peer foundation; no messages are sent until the transport is configured.</p><form id="identity-form"><label>Username<input name="username" autocomplete="username" placeholder="ada_lovelace" required minlength="3" maxlength="32" pattern="[a-zA-Z0-9_]+" /></label><label>Display name<input name="displayName" autocomplete="name" placeholder="Ada Lovelace" required maxlength="64" /></label><button type="submit">Create local identity</button><p id="form-error" role="alert"></p></form><p class="fineprint">Your profile is saved in local SQLite. Username availability and device registration will arrive with cloud coordination.</p></section>`; }
function shell() { return `<section class="shell"><aside><span class="mark">H</span><nav><button class="active">Chats</button><button>Friends</button><button>Groups</button></nav><footer><span class="status"></span> Offline · local workspace</footer></aside><article><header><div><p class="eyebrow">WELCOME</p><h2>Your workspace is ready</h2></div><span class="badge">Foundation</span></header><div class="empty"><h3>Start with people you trust.</h3><p>Friend discovery, peer connections, and encrypted delivery are deliberately staged. This client never presents an unavailable feature as working.</p><button disabled>New conversation — coming next</button></div></article></section>`; }
async function createIdentity(event: SubmitEvent) { event.preventDefault(); const form = event.currentTarget as HTMLFormElement; const values = new FormData(form); const error = document.querySelector<HTMLElement>("#form-error")!; try { await invoke("create_profile", { username: values.get("username"), displayName: values.get("displayName") }); await render(); } catch (reason) { error.textContent = String(reason); } }
render();
