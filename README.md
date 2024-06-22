# nu_plugin_mime

A simple plugin for working with mime types without performing disk access.

This plugin requires Nushell 0.94 or later.

## Install

Clone this repository, enter the directory and run `cargo build --release`. Afterwards move the binary `target/release/nu_plugin_mime` to a location of your choosing and run the following commands from Nushell:

```nu
plugin add /path/to/nu_plugin_mime
plugin use nu_plugin_mime
mime --help
```

Mime commands should now be ready to use.

## Examples

### mime guess

```nu
# Guess the MIME type from the path and return a string.
  > "video.mkv" | mime guess
  video/x-matroska

# Guess the MIME types from the paths and return a table.
  > ["video.mkv" "audio.mp3"] | mime guess
  ╭───┬───────────┬──────────────────╮
  │ # │   name    │       type       │
  ├───┼───────────┼──────────────────┤
  │ 0 │ video.mkv │ video/x-matroska │
  │ 1 │ audio.mp3 │ audio/mpeg       │
  ╰───┴───────────┴──────────────────╯

# Guess the MIME types from the extensions and return a table.
  > ["mkv" "mp3"] | mime guess -e
  ╭───┬──────┬──────────────────╮
  │ # │ name │       type       │
  ├───┼──────┼──────────────────┤
  │ 0 │ mkv  │ video/x-matroska │
  │ 1 │ mp3  │ audio/mpeg       │
  ╰───┴──────┴──────────────────╯

  # Add a MIME type column to a table.
  > let input = glob * | wrap filename; $input | merge ($input | get filename | mime guess | select type)
  ╭───┬──────────┬──────╮
  │ # │ filename │ type │
  ├───┼──────────┼──────┤
  │ 0 │ ...      │ ...  │
  ╰───┴──────────┴──────╯
```

### mime list

```nu
# Get known extensions for the "video/x-matroska" mime type
  > mime list "video/x-matroska"
  ╭───┬──────╮
  │ 0 │ mk3d │
  │ 1 │ mks  │
  │ 2 │ mkv  │
  ╰───┴──────╯

  Get all known video extensions
  > mime list "video/*"

  # Get all known extensions
  > mime list "*/whatever"

  # Unrecognized MIME types return an empty list
  > mime list "nonexistent"
  ╭────────────╮
  │ empty list │
  ╰────────────╯
```
