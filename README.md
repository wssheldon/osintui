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
 <br />
<div align="center", class="integrations">
  <p>
    <div class="card">
      <a href="https://search.censys.io/"><img src="assets/logos/censys_logo.png" alt="censys" /></a>
      <div class="container">
        <h4><b>Censys</b></h4>
      </div>
    </div>
  </p>
    <br />
  <p>
    <div class="card">
      <a href="https://www.virustotal.com/"><img src="assets/logos/virustotal_logo.png" alt="virustotal" /></a>
      <div class="container">
        <h4><b>Virustotal</b></h4>
      </div>
    </div>
  </p>
    <br />
  <p>
    <div class="card">
      <a href="https://www.shodan.io/"><img src="assets/logos/shodan_logo.png" alt="shodan" /></a>
      <div class="container">
        <h4><b>Shodan</b></h4>
      </div>
    </div>
  </p>
</div>
<br />

<style type="text/css" rel="stylesheet">
.card {
  box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2);
  transition: 0.3s;
  border-radius: 20px; /* 5px rounded corners */
  width: 100%;
  padding-top: 10px;
}

.card:hover {
  box-shadow: 0 8px 16px 0 rgba(0,0,0,0.2);
}

.container {
  padding: 2px 16px;
}

.integrations {
  display: flex;
  grid-auto-flow: column;
  grid-column-gap: 10px;
}

</style>

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
