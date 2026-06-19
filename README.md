# Yulang for Zed

Yulang language support for Zed.

This extension provides syntax highlighting through the `tree-sitter-yulang`
grammar and starts the Yulang language server from the active Zed worktree
environment.

## Language Server

The language server ships inside the main `yulang` binary. Install a release
binary first:

```sh
curl -fsSL https://yulang.momota.pw/install.sh | sh -s -- --version v0.1.0-alpha.1
```

The extension searches the active worktree environment, then invokes
`yulang server`. It does not bundle a language server binary yet.

You can override the command from Zed settings:

```json
{
  "lsp": {
    "yulang": {
      "binary": {
        "path": "/home/me/.yulang/bin/yulang",
        "arguments": ["server"]
      }
    }
  }
}
```

## Semantic Highlighting

Zed keeps tree-sitter highlighting and language-server semantic highlighting
as separate settings. Enable semantic tokens for Yulang in `settings.json`:

```json
{
  "languages": {
    "Yulang": {
      "semantic_tokens": "full"
    }
  }
}
```

Use `"combined"` instead of `"full"` to layer semantic tokens on top of the
tree-sitter highlights.
