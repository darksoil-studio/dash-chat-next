
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
	export const GJS_DEBUG_TOPICS: string;
	export const ATUIN_SESSION: string;
	export const TTY: string;
	export const USER: string;
	export const npm_config_user_agent: string;
	export const BUN_INSTALL: string;
	export const GIT_ASKPASS: string;
	export const XDG_SESSION_TYPE: string;
	export const npm_node_execpath: string;
	export const CLUTTER_DISABLE_MIPMAPPED_TEXT: string;
	export const LD_LIBRARY_PATH: string;
	export const SHLVL: string;
	export const APPDIR: string;
	export const CHROME_DESKTOP: string;
	export const HOME: string;
	export const LESS: string;
	export const OLDPWD: string;
	export const DESKTOP_SESSION: string;
	export const TERM_PROGRAM_VERSION: string;
	export const npm_package_json: string;
	export const GIO_LAUNCHED_DESKTOP_FILE: string;
	export const LSCOLORS: string;
	export const PERLLIB: string;
	export const ZSH: string;
	export const GNOME_SHELL_SESSION_MODE: string;
	export const GTK_MODULES: string;
	export const OPENAI_API_KEY: string;
	export const PAGER: string;
	export const VSCODE_GIT_ASKPASS_MAIN: string;
	export const MANAGERPID: string;
	export const VSCODE_GIT_ASKPASS_NODE: string;
	export const DBUS_SESSION_BUS_ADDRESS: string;
	export const FIG_SET_PARENT_CHECK: string;
	export const GSM_SKIP_SSH_AGENT_WORKAROUND: string;
	export const SYSTEMD_EXEC_PID: string;
	export const COLORTERM: string;
	export const FIG_PID: string;
	export const GIO_LAUNCHED_DESKTOP_FILE_PID: string;
	export const DEBUGINFOD_URLS: string;
	export const MANDATORY_PATH: string;
	export const ATUIN_HISTORY: string;
	export const LOGNAME: string;
	export const pnpm_config_verify_deps_before_run: string;
	export const JOURNAL_STREAM: string;
	export const OWD: string;
	export const _: string;
	export const ATUIN_HISTORY_ID: string;
	export const DEFAULTS_PATH: string;
	export const LC_FIG_SET_PARENT: string;
	export const MEMORY_PRESSURE_WATCH: string;
	export const USER_ZDOTDIR: string;
	export const XDG_SESSION_CLASS: string;
	export const npm_config_registry: string;
	export const TERM: string;
	export const USERNAME: string;
	export const GNOME_DESKTOP_SESSION_ID: string;
	export const HCFEAT: string;
	export const WINDOWPATH: string;
	export const npm_config_node_gyp: string;
	export const PATH: string;
	export const APPIMAGE: string;
	export const INVOCATION_ID: string;
	export const NODE: string;
	export const SESSION_MANAGER: string;
	export const npm_package_name: string;
	export const GDK_BACKEND: string;
	export const XDG_MENU_PREFIX: string;
	export const XDG_RUNTIME_DIR: string;
	export const npm_config_frozen_lockfile: string;
	export const DISPLAY: string;
	export const LANG: string;
	export const VSCODE_INJECTION: string;
	export const XDG_CURRENT_DESKTOP: string;
	export const CURSOR_TRACE_ID: string;
	export const LS_COLORS: string;
	export const TERM_PROGRAM: string;
	export const VSCODE_GIT_IPC_HANDLE: string;
	export const XAUTHORITY: string;
	export const XDG_SESSION_DESKTOP: string;
	export const XMODIFIERS: string;
	export const npm_lifecycle_script: string;
	export const GSETTINGS_SCHEMA_DIR: string;
	export const ORIGINAL_XDG_CURRENT_DESKTOP: string;
	export const SSH_AUTH_SOCK: string;
	export const NODE_PATH: string;
	export const SHELL: string;
	export const npm_package_version: string;
	export const npm_config_verify_deps_before_run: string;
	export const npm_lifecycle_event: string;
	export const GDMSESSION: string;
	export const NDK_HOME: string;
	export const QT_ACCESSIBILITY: string;
	export const GJS_DEBUG_OUTPUT: string;
	export const GPG_AGENT_INFO: string;
	export const QT_IM_MODULE: string;
	export const VSCODE_GIT_ASKPASS_EXTRA_ARGS: string;
	export const PWD: string;
	export const npm_execpath: string;
	export const ANDROID_HOME: string;
	export const FIG_SET_PARENT: string;
	export const XDG_CONFIG_DIRS: string;
	export const XDG_DATA_DIRS: string;
	export const ZDOTDIR: string;
	export const NIX_STORE_DIR: string;
	export const PNPM_SCRIPT_SRC_DIR: string;
	export const npm_config__jsr_registry: string;
	export const npm_command: string;
	export const QT_PLUGIN_PATH: string;
	export const MEMORY_PRESSURE_WRITE: string;
	export const PNPM_HOME: string;
	export const ANDROID_NDK_HOME: string;
	export const INIT_CWD: string;
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
		GJS_DEBUG_TOPICS: string;
		ATUIN_SESSION: string;
		TTY: string;
		USER: string;
		npm_config_user_agent: string;
		BUN_INSTALL: string;
		GIT_ASKPASS: string;
		XDG_SESSION_TYPE: string;
		npm_node_execpath: string;
		CLUTTER_DISABLE_MIPMAPPED_TEXT: string;
		LD_LIBRARY_PATH: string;
		SHLVL: string;
		APPDIR: string;
		CHROME_DESKTOP: string;
		HOME: string;
		LESS: string;
		OLDPWD: string;
		DESKTOP_SESSION: string;
		TERM_PROGRAM_VERSION: string;
		npm_package_json: string;
		GIO_LAUNCHED_DESKTOP_FILE: string;
		LSCOLORS: string;
		PERLLIB: string;
		ZSH: string;
		GNOME_SHELL_SESSION_MODE: string;
		GTK_MODULES: string;
		OPENAI_API_KEY: string;
		PAGER: string;
		VSCODE_GIT_ASKPASS_MAIN: string;
		MANAGERPID: string;
		VSCODE_GIT_ASKPASS_NODE: string;
		DBUS_SESSION_BUS_ADDRESS: string;
		FIG_SET_PARENT_CHECK: string;
		GSM_SKIP_SSH_AGENT_WORKAROUND: string;
		SYSTEMD_EXEC_PID: string;
		COLORTERM: string;
		FIG_PID: string;
		GIO_LAUNCHED_DESKTOP_FILE_PID: string;
		DEBUGINFOD_URLS: string;
		MANDATORY_PATH: string;
		ATUIN_HISTORY: string;
		LOGNAME: string;
		pnpm_config_verify_deps_before_run: string;
		JOURNAL_STREAM: string;
		OWD: string;
		_: string;
		ATUIN_HISTORY_ID: string;
		DEFAULTS_PATH: string;
		LC_FIG_SET_PARENT: string;
		MEMORY_PRESSURE_WATCH: string;
		USER_ZDOTDIR: string;
		XDG_SESSION_CLASS: string;
		npm_config_registry: string;
		TERM: string;
		USERNAME: string;
		GNOME_DESKTOP_SESSION_ID: string;
		HCFEAT: string;
		WINDOWPATH: string;
		npm_config_node_gyp: string;
		PATH: string;
		APPIMAGE: string;
		INVOCATION_ID: string;
		NODE: string;
		SESSION_MANAGER: string;
		npm_package_name: string;
		GDK_BACKEND: string;
		XDG_MENU_PREFIX: string;
		XDG_RUNTIME_DIR: string;
		npm_config_frozen_lockfile: string;
		DISPLAY: string;
		LANG: string;
		VSCODE_INJECTION: string;
		XDG_CURRENT_DESKTOP: string;
		CURSOR_TRACE_ID: string;
		LS_COLORS: string;
		TERM_PROGRAM: string;
		VSCODE_GIT_IPC_HANDLE: string;
		XAUTHORITY: string;
		XDG_SESSION_DESKTOP: string;
		XMODIFIERS: string;
		npm_lifecycle_script: string;
		GSETTINGS_SCHEMA_DIR: string;
		ORIGINAL_XDG_CURRENT_DESKTOP: string;
		SSH_AUTH_SOCK: string;
		NODE_PATH: string;
		SHELL: string;
		npm_package_version: string;
		npm_config_verify_deps_before_run: string;
		npm_lifecycle_event: string;
		GDMSESSION: string;
		NDK_HOME: string;
		QT_ACCESSIBILITY: string;
		GJS_DEBUG_OUTPUT: string;
		GPG_AGENT_INFO: string;
		QT_IM_MODULE: string;
		VSCODE_GIT_ASKPASS_EXTRA_ARGS: string;
		PWD: string;
		npm_execpath: string;
		ANDROID_HOME: string;
		FIG_SET_PARENT: string;
		XDG_CONFIG_DIRS: string;
		XDG_DATA_DIRS: string;
		ZDOTDIR: string;
		NIX_STORE_DIR: string;
		PNPM_SCRIPT_SRC_DIR: string;
		npm_config__jsr_registry: string;
		npm_command: string;
		QT_PLUGIN_PATH: string;
		MEMORY_PRESSURE_WRITE: string;
		PNPM_HOME: string;
		ANDROID_NDK_HOME: string;
		INIT_CWD: string;
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
