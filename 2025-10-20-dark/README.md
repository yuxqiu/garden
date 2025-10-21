# Force Dark Mode CSS

This project enables dark mode on websites that support `prefers-color-scheme: dark` while Firefox's **Resist Fingerprinting** is enabled. It works by intercepting stylesheet requests and replacing every occurrence of `prefers-color-scheme: dark` with `prefers-color-scheme: light`, and vice versa. It also modifies inline styles on the page.

This tricks the website into applying dark mode styles even when fingerprinting protection is active.

Tested and working on GitHub, Reddit, and many other websites (not working on Google). Since the extension performs multiple string replacements, it may slightly delay page loading. For a smoother experience with fingerprinting protection, see [Just a side note](#just-a-side-note).

## Use the plugin

* Clone this repo.
* Open [about:debugging#/runtime/this-firefox](about:debugging#/runtime/this-firefox) and click **Load Temporary Add-on**.

## Just a side note

If you want to keep Resist Fingerprinting enabled and still use dark mode, check out this [Bugzilla ticket](https://bugzilla.mozilla.org/show_bug.cgi?id=1732114). It explains how to switch to FFP and override it with `+AllTargets,-CSSPrefersColorScheme`.