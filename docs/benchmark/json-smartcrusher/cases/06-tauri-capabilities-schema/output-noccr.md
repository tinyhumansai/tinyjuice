{
  "default": {
    "identifier": "default",
    "description": "Capability for the main and overlay windows (desktop only)",
    "local": true,
    "windows": [
      "main",
      "overlay"
    ],
    "permissions": [
      "core:default",
      "core:window:default",
      "core:window:allow-hide",
      "core:window:allow-show",
      "core:window:allow-set-focus",
      "core:window:allow-unminimize",
      "core:window:allow-start-dragging",
      "core:window:allow-set-always-on-top",
      "core:event:default",
      "deep-link:default",
      "notification:default",
      "notification:allow-is-permission-granted",
      "notification:allow-request-permission",
      "notification:allow-notify",
      "opener:default",
      "opener:allow-reveal-item-in-dir",
      {
        "identifier": "opener:allow-open-url",
        "allow": [
          {
            "url": "obsidian://open*"
          },
          {
            "url": "https://ollama.com/*"
          }
        ]
      },
      "updater:default",
      "allow-core-process",
      "allow-workspace-files",
      "allow-artifact-download",
      "allow-artifact-save",
      "allow-app-update",
      "allow-loopback-oauth"
    ],
    "platforms": [
      "linux",
      "macOS",
      "windows"
    ]
  },
  "webview-accounts-recipes": {
    "identifier": "webview-accounts-recipes",
    "description": "Permissions for embedded webview-account child webviews. The injected per-provider recipe runtime calls webview_recipe_event from inside untrusted third-party origins (e.g. web.whatsapp.com, meet.google.com), so this capability is intentionally scoped to ONLY that one command, and only on acct_* webview labels. remote.urls lists the third-party origins whose recipes are allowed to invoke the command.",
    "remote": {
      "urls": [
        "https://web.whatsapp.com/*",
        "https://web.telegram.org/*",
        "https://www.linkedin.com/*",
        "https://mail.google.com/*",
        "https://app.slack.com/*",
        "https://discord.com/*",
        "https://meet.google.com/*",
        "https://accounts.google.com/*",
        "https://www.browserscan.net/*"
      ]
    },
    "local": true,
    "webviews": [
      "acct_*"
    ],
    "permissions": [
      "allow-webview-recipe"
    ],
    "platforms": [
      "linux",
      "macOS",
      "windows"
    ]
  }
}
