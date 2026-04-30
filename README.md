# asfy-wall 🖼️🦀

A blazing fast, no-nonsense wallpaper manager. It handles the caching, sorting, and cycling of your image directories, delegating the heavy lifting to the [`awww`](https://github.com/greenjoe/Awww) command.

Because changing your wallpaper should be instant, not a memory hog.

## Features

- **Fast by design:** Written in Rust with a smart caching system, so it doesn't have to re-evaluate your massive image folder on every run.
- **Logical hierarchy:** CLI arguments always override your config file. As it should be.
- **Path expansion:** Native shell path support.

## Usage

Run the binary directly. asfy-wall will evaluate your config (or args) and call the underlying engine.

```bash
asfywall [OPTIONS] [-- <EXTERNAL_ARGS>]
```


### Core Options

- `-i, --images-dir <PATH>`: Override the base directory where your wallpapers live.
- `-o, --order-by <METHOD>`: Override the sorting method. If this differs from your base config, it automatically triggers a reorder.
- `-r, --reverse [true|false]`: Flip the image order.
- `--reorder`: Ignore the cache and force a complete reorder of your directory based on the current criteria.
- `--set-index <N>`: Cut to the chase and jump directly to index `N` in your cache.
- `--dry-run`: Paranoid mode. Preview exactly what it will do and what args it will pass to `awww` without actually executing anything.

### External Arguments (`awww`)

Anything you append at the end (after `--` if you're using complex options) gets passed straight to the external `awww` tool.

```bash
asfywall --images-dir ~/Wallpapers --reorder -- --transition-type wipe
```

## Configuration

You don't need to create the config file from scratch; asfy-wall automatically generates a default one the first time it runs. 

It lives in `$XDG_CONFIG_HOME/asfy/asfy-wall/config.toml` (or your OS equivalent config directory).

Here is what the structure looks like:

```toml
# The base directory where your wallpapers live
images_dir = "/path/to/your/wallpapers"

# Sorting method: "None" (Random), "Name", "CreatedAt", or "ModifiedAt"
order_by = "None"

# Flip the image order?
reverse = false

# Default arguments passed directly to `awww`
external_args = [
    "--transition-type",
    "wipe",
    "--transition-step",
    "10",
]
```

## Building

Nothing crazy here. Just make sure you have Rust and Cargo installed.

```bash
cd ~/this/repo/asfy-wall
cargo install --path .
```

