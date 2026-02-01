# Installation and Testing Guide

## Quick Start

This extension has been updated to use the official `@actions/languageserver` package with full GitHub expressions support.

### For Zed Users

1. **Install the Extension**
   
   The extension will be available through Zed's extension marketplace. Once installed, it will automatically:
   - Download and install `@actions/languageserver` from npm
   - Configure the language server for `.github/workflows/*.yml` files
   - Enable all expression features

2. **Configure File Types** (Required)
   
   Add to your Zed `settings.json`:
   ```jsonc
   {
     "file_types": {
       "GitHub Actions": [
         ".github/workflows/*.yml",
         ".github/workflows/*.yaml"
       ]
     }
   }
   ```

3. **Optional: Add GitHub Token**
   
   For enhanced features (repository-specific completions):
   
   **Option A: Environment Variable (Recommended)**
   ```bash
   export GITHUB_TOKEN="ghp_your_token_here"
   # Or use GitHub CLI
   export GH_TOKEN=$(gh auth token)
   ```
   
   **Option B: Zed Settings**
   ```jsonc
   {
     "lsp": {
       "gh-actions-language-server": {
         "initialization_options": {
           "sessionToken": "ghp_your_token_here"
         }
       }
     }
   }
   ```

### For Extension Developers

#### Development Setup

1. **Clone and Build**
   ```bash
   cd /path/to/zed-github-actions
   cargo build --target wasm32-wasip2 --release
   ```

2. **Install in Zed**
   
   The built extension will be at:
   ```
   target/wasm32-wasip2/release/zed_github_actions.wasm
   ```
   
   To test in Zed:
   - Open Zed
   - Go to Extensions panel
   - Click "Install Dev Extension"
   - Select this directory

3. **Verify Installation**
   
   Open a workflow file (`.github/workflows/*.yml`) and check:
   - [ ] Language server starts without errors
   - [ ] Syntax highlighting works
   - [ ] Expression autocompletion appears when typing `${{ `
   - [ ] Context completions work (e.g., `github.`, `env.`, `secrets.`)
   - [ ] Function completions work (e.g., `contains(`, `startsWith(`)
   - [ ] Hover documentation appears
   - [ ] Errors are shown for invalid expressions

#### Testing Expression Features

Use the included `test-workflow.yml` file to verify all features:

```bash
# Open the test file in Zed
zed test-workflow.yml
```

The test file includes 13 examples demonstrating:
- Context autocompletion (github, env, secrets, runner, etc.)
- Function autocompletion (contains, startsWith, format, etc.)
- Expression validation
- Type checking
- Operator precedence

### Troubleshooting

#### Extension Won't Load

1. Check Node.js is installed: `node --version` (requires v18+)
2. Check Zed's extension logs for errors
3. Try reloading Zed or reinstalling the extension

#### Language Server Crashes

Check stderr output for:
- Missing `@actions/languageserver` package
- Path resolution issues
- Node.js version incompatibility

Enable debug logging:
```jsonc
{
  "lsp": {
    "gh-actions-language-server": {
      "initialization_options": {
        "logLevel": "debug"
      }
    }
  }
}
```

#### No Completions Appearing

1. Verify file is in `.github/workflows/` directory
2. Check file type is set to "GitHub Actions"
3. Ensure expression is inside `${{ }}` blocks
4. Try restarting the language server

#### Environment Variables Not Working

The extension reads `GITHUB_TOKEN` and `GH_TOKEN` when Zed starts. If you set them after Zed is running:
1. Close Zed completely
2. Set the environment variable
3. Start Zed from the terminal where the variable is set

Example:
```bash
export GITHUB_TOKEN="ghp_..."
zed
```

## What's New in v0.1.0

### Major Changes

- ✅ Switched to official `@actions/languageserver` package
- ✅ Full GitHub expressions validation and autocompletion
- ✅ Environment variable support for GitHub token
- ✅ Experimental features support
- ✅ Better error messages and diagnostics

### Features

- **Expression Validation** - Real-time syntax and type checking
- **Context Autocompletion** - All GitHub contexts with documentation
- **Function Autocompletion** - Built-in functions with signatures
- **Hover Documentation** - Inline help for everything
- **Code Actions** - Quick fixes (experimental)
- **Inlay Hints** - Type information display

### Migration from Previous Version

No breaking changes! If you were using the extension before:
- All settings remain the same
- File associations are unchanged
- The extension will automatically download the new package

## Support

- **Issues**: [GitHub Issues](https://github.com/neoncitylights/zed-github-actions/issues)
- **Documentation**: [README.md](README.md)
- **Changelog**: [CHANGES.md](CHANGES.md)
- **GitHub Actions Docs**: [expressions](https://docs.github.com/actions/learn-github-actions/expressions)

## License

Apache License 2.0 - See [LICENSE](LICENSE) for details.
