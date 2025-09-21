
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * Environment variables [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env`. Like [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), this module cannot be imported into client-side code. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * _Unlike_ [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), the values exported from this module are statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * ```ts
 * import { API_KEY } from '$env/static/private';
 * ```
 * 
 * Note that all environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * 
 * ```
 * MY_FEATURE_FLAG=""
 * ```
 * 
 * You can override `.env` values from the command line like so:
 * 
 * ```sh
 * MY_FEATURE_FLAG="enabled" npm run dev
 * ```
 */
declare module '$env/static/private' {
	export const ELECTRON_RUN_AS_NODE: string;
	export const ANDROID_HOME: string;
	export const ANDROID_NDK_HOME: string;
	export const APPDIR: string;
	export const APPIMAGE: string;
	export const ARGV0: string;
	export const ATUIN_HISTORY: string;
	export const ATUIN_HISTORY_ID: string;
	export const ATUIN_SESSION: string;
	export const BUN_INSTALL: string;
	export const CHROME_DESKTOP: string;
	export const COLORTERM: string;
	export const CURSOR_TRACE_ID: string;
	export const DBUS_SESSION_BUS_ADDRESS: string;
	export const DEBUGINFOD_URLS: string;
	export const DEFAULTS_PATH: string;
	export const DESKTOP_SESSION: string;
	export const DISPLAY: string;
	export const FIGTERM_SESSION_ID: string;
	export const FIG_PID: string;
	export const FIG_SET_PARENT: string;
	export const FIG_SET_PARENT_CHECK: string;
	export const FIG_TERM: string;
	export const GDK_BACKEND: string;
	export const GDMSESSION: string;
	export const GNOME_DESKTOP_SESSION_ID: string;
	export const GNOME_KEYRING_CONTROL: string;
	export const GNOME_SHELL_SESSION_MODE: string;
	export const GNOME_TERMINAL_SCREEN: string;
	export const GNOME_TERMINAL_SERVICE: string;
	export const GPG_AGENT_INFO: string;
	export const GSETTINGS_SCHEMA_DIR: string;
	export const GSM_SKIP_SSH_AGENT_WORKAROUND: string;
	export const GTK_MODULES: string;
	export const HCFEAT: string;
	export const HOME: string;
	export const LANG: string;
	export const LC_FIG_SET_PARENT: string;
	export const LD_LIBRARY_PATH: string;
	export const LESS: string;
	export const LOGNAME: string;
	export const LSCOLORS: string;
	export const LS_COLORS: string;
	export const MANDATORY_PATH: string;
	export const MEMORY_PRESSURE_WATCH: string;
	export const MEMORY_PRESSURE_WRITE: string;
	export const NDK_HOME: string;
	export const NIX_STORE_DIR: string;
	export const OLDPWD: string;
	export const OPENAI_API_KEY: string;
	export const ORIGINAL_XDG_CURRENT_DESKTOP: string;
	export const OWD: string;
	export const PAGER: string;
	export const PATH: string;
	export const PERLLIB: string;
	export const PNPM_HOME: string;
	export const PWD: string;
	export const QT_ACCESSIBILITY: string;
	export const QT_IM_MODULE: string;
	export const QT_PLUGIN_PATH: string;
	export const SESSION_MANAGER: string;
	export const SHELL: string;
	export const SHLVL: string;
	export const SSH_AUTH_SOCK: string;
	export const SYSTEMD_EXEC_PID: string;
	export const TERM: string;
	export const TTY: string;
	export const USER: string;
	export const USERNAME: string;
	export const VSCODE_CODE_CACHE_PATH: string;
	export const VSCODE_CRASH_REPORTER_PROCESS_TYPE: string;
	export const VSCODE_CWD: string;
	export const VSCODE_ESM_ENTRYPOINT: string;
	export const VSCODE_HANDLES_UNCAUGHT_ERRORS: string;
	export const VSCODE_IPC_HOOK: string;
	export const VSCODE_NLS_CONFIG: string;
	export const VSCODE_PID: string;
	export const VSCODE_PROCESS_TITLE: string;
	export const VTE_VERSION: string;
	export const WINDOWPATH: string;
	export const XAUTHORITY: string;
	export const XDG_CONFIG_DIRS: string;
	export const XDG_CURRENT_DESKTOP: string;
	export const XDG_DATA_DIRS: string;
	export const XDG_MENU_PREFIX: string;
	export const XDG_RUNTIME_DIR: string;
	export const XDG_SESSION_CLASS: string;
	export const XDG_SESSION_DESKTOP: string;
	export const XDG_SESSION_TYPE: string;
	export const XMODIFIERS: string;
	export const ZSH: string;
	export const _: string;
	export const LOG_DST: string;
	export const LOG_LEVEL: string;
	export const VSCODE_L10N_BUNDLE_LOCATION: string;
	export const ELECTRON_NO_ASAR: string;
	export const NODE_ENV: string;
}

/**
 * Similar to [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private), except that it only includes environment variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Values are replaced statically at build time.
 * 
 * ```ts
 * import { PUBLIC_BASE_URL } from '$env/static/public';
 * ```
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to runtime environment variables, as defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * This module cannot be imported into client-side code.
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * console.log(env.DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 * 
 * > [!NOTE] In `dev`, `$env/dynamic` always includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 */
declare module '$env/dynamic/private' {
	export const env: {
		ELECTRON_RUN_AS_NODE: string;
		ANDROID_HOME: string;
		ANDROID_NDK_HOME: string;
		APPDIR: string;
		APPIMAGE: string;
		ARGV0: string;
		ATUIN_HISTORY: string;
		ATUIN_HISTORY_ID: string;
		ATUIN_SESSION: string;
		BUN_INSTALL: string;
		CHROME_DESKTOP: string;
		COLORTERM: string;
		CURSOR_TRACE_ID: string;
		DBUS_SESSION_BUS_ADDRESS: string;
		DEBUGINFOD_URLS: string;
		DEFAULTS_PATH: string;
		DESKTOP_SESSION: string;
		DISPLAY: string;
		FIGTERM_SESSION_ID: string;
		FIG_PID: string;
		FIG_SET_PARENT: string;
		FIG_SET_PARENT_CHECK: string;
		FIG_TERM: string;
		GDK_BACKEND: string;
		GDMSESSION: string;
		GNOME_DESKTOP_SESSION_ID: string;
		GNOME_KEYRING_CONTROL: string;
		GNOME_SHELL_SESSION_MODE: string;
		GNOME_TERMINAL_SCREEN: string;
		GNOME_TERMINAL_SERVICE: string;
		GPG_AGENT_INFO: string;
		GSETTINGS_SCHEMA_DIR: string;
		GSM_SKIP_SSH_AGENT_WORKAROUND: string;
		GTK_MODULES: string;
		HCFEAT: string;
		HOME: string;
		LANG: string;
		LC_FIG_SET_PARENT: string;
		LD_LIBRARY_PATH: string;
		LESS: string;
		LOGNAME: string;
		LSCOLORS: string;
		LS_COLORS: string;
		MANDATORY_PATH: string;
		MEMORY_PRESSURE_WATCH: string;
		MEMORY_PRESSURE_WRITE: string;
		NDK_HOME: string;
		NIX_STORE_DIR: string;
		OLDPWD: string;
		OPENAI_API_KEY: string;
		ORIGINAL_XDG_CURRENT_DESKTOP: string;
		OWD: string;
		PAGER: string;
		PATH: string;
		PERLLIB: string;
		PNPM_HOME: string;
		PWD: string;
		QT_ACCESSIBILITY: string;
		QT_IM_MODULE: string;
		QT_PLUGIN_PATH: string;
		SESSION_MANAGER: string;
		SHELL: string;
		SHLVL: string;
		SSH_AUTH_SOCK: string;
		SYSTEMD_EXEC_PID: string;
		TERM: string;
		TTY: string;
		USER: string;
		USERNAME: string;
		VSCODE_CODE_CACHE_PATH: string;
		VSCODE_CRASH_REPORTER_PROCESS_TYPE: string;
		VSCODE_CWD: string;
		VSCODE_ESM_ENTRYPOINT: string;
		VSCODE_HANDLES_UNCAUGHT_ERRORS: string;
		VSCODE_IPC_HOOK: string;
		VSCODE_NLS_CONFIG: string;
		VSCODE_PID: string;
		VSCODE_PROCESS_TITLE: string;
		VTE_VERSION: string;
		WINDOWPATH: string;
		XAUTHORITY: string;
		XDG_CONFIG_DIRS: string;
		XDG_CURRENT_DESKTOP: string;
		XDG_DATA_DIRS: string;
		XDG_MENU_PREFIX: string;
		XDG_RUNTIME_DIR: string;
		XDG_SESSION_CLASS: string;
		XDG_SESSION_DESKTOP: string;
		XDG_SESSION_TYPE: string;
		XMODIFIERS: string;
		ZSH: string;
		_: string;
		LOG_DST: string;
		LOG_LEVEL: string;
		VSCODE_L10N_BUNDLE_LOCATION: string;
		ELECTRON_NO_ASAR: string;
		NODE_ENV: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * Similar to [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), but only includes variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Note that public dynamic environment variables must all be sent from the server to the client, causing larger network requests — when possible, use `$env/static/public` instead.
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.PUBLIC_DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
