<div align="center">
  <h1>osintui</h1>
  <p> Open Source Intelligence Terminal User Interface </p>
  <!-- Badges -->
  <p>
    <a href="https://github.com/wssheldon/osintui/graphs/contributors">
      <img src="https://img.shields.io/github/contributors/wssheldon/osintui" alt="contributors" />
    </a>
    <a href="">
      <img src="https://img.shields.io/github/last-commit/wssheldon/osintui" alt="last update" />
    </a>
    <a href="https://github.com/wssheldon/osintui/stargazers">
      <img src="https://img.shields.io/github/stars/wssheldon/osintui" alt="stars" />
    </a>
    <a href="https://github.com/wssheldon/osintui/issues/">
      <img src="https://img.shields.io/github/issues/wssheldon/osintui" alt="open issues" />
    </a>
    <a href="https://github.com/wssheldon/osintui/blob/master/LICENSE">
      <img src="https://img.shields.io/github/license/wssheldon/osintui.svg" alt="license" />
    </a>
  </p>
  <h4>
    <a href="https://github.com/wssheldon/osintui/issues/">Report Bug</a>
    <span> · </span>
    <a href="https://github.com/wssheldon/osintui/issues/">Request Feature</a>
  </h4>
</div>
<br />
<div align="center">
  <img src="assets/demo.gif" alt="screenshot" />
</div>

----

## Installation

First, install [Rust](https://www.rust-lang.org/tools/install) (using the recommended rustup installation method) and then

```
cargo install osintui
```

## Integrations

<div align="center">
    <img src="example.svg" width="400" height="400" alt="css-in-readme">
</div>

## Configuration

osintui expects a TOML configuration file stored at `~/.osintui/config/config.toml` that sets the necessary API tokens for each service. The configuration file will be created for you on first run if one was not found.

```toml
[keys]
virustotal = "api_key"
shodan = "api_key"
censys_id = "api_id"
censys_secret = "api_key"
```

## Hotkeys

| Key         | Description |
| ----------- | ----------- |
| h           | Home        |
| /           | Input       |
| q           | Back        |
| c           | Censys      |
| s           | Shodan      |
| v           | Virustotal  |
| →           | Move Right  |
| ←           | Move Left   |
| ↑           | Move Up     |
| ↓           | Move Down   |

## Credits

⭐ **[spotify-tui](https://github.com/Rigellute/spotify-tui)**

The software architecture is almost entirely modeled after spotify-tui. The codebase was invaluable in learning how to cleanly manage complex TUI state and implement generic handling of TUI components.

⭐ **[wtfis](https://github.com/pirxthepilot/wtfis)**

I needed a good first project to learn rust and wtfis was the primary source of inspiration for osintui.
