# zed-github-actions
[![Zed Extension][zed-extension-badge]][zed-extension-url]
[![License][license-badge]][license-url]

[zed-extension-badge]: https://img.shields.io/badge/Zed%20Extension-%230951CF?style=flat-square&logo=zedindustries&logoColor=white&labelColor=black
[zed-extension-url]: https://zed.dev/extensions/github-actions
[license-badge]: https://img.shields.io/badge/License-Apache%202.0-blue?style=flat-square&labelColor=black&color=blue
[license-url]: #license

GitHub Actions LSP support for Zed with full GitHub expression validation and autocompletion. As this repository uses code based on some official Zed extensions (e.g [Svelte](https://github.com/zed-extensions/svelte) and [Astro](https://github.com/zed-extensions/astro)), this repository is under the same license as those.

## Features

This extension provides comprehensive GitHub Actions workflow support including:

- **Expression Validation** - Syntax checking and type validation for `${{ }}` expressions
- **Context Autocompletion** - Smart completions for `github.*`, `env.*`, `secrets.*`, `runner.*`, `job.*`, `steps.*`, `matrix.*`, `needs.*`, and `inputs.*` contexts
- **Function Autocompletion** - Built-in functions like `contains()`, `startsWith()`, `endsWith()`, `format()`, `toJSON()`, `fromJSON()`, and `join()`
- **Hover Documentation** - Inline documentation for contexts, functions, and workflow syntax
- **Diagnostics** - Real-time error detection and warnings
- **Code Actions** - Quick fixes for common issues (experimental)
- **Inlay Hints** - Type information and parameter hints

### Technology Stack

- Tree-sitter: [zed-industries/tree-sitter-yaml](https://github.com/zed-industries/tree-sitter-yaml)
- Language Server: [@actions/languageserver](https://www.npmjs.com/package/@actions/languageserver) - Official GitHub Actions language server with full [`@actions/expressions`](https://www.npmjs.com/package/@actions/expressions) support

The extension uses the official `@actions/languageserver` package directly, including its bundled `actions-languageserver` binary, ensuring you get the latest features and bug fixes from GitHub.

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.

## Configuration

### Filetype settings
This extension by default does not have any file associations built-in, as Zed doesn't support glob patterns at the extension-level to recognize a language within a specific directory. Instead, you can edit your Zed settings file (`settings.json`) with:

```jsonc
{
	// ...
	"file_types": {
		"GitHub Actions": [
			".github/workflows/*.yml",
			".github/workflows/*.yaml"
		]
	}
}
```

This extension avoids conflicting with the built-in YAML support for Zed by following how other Zed extensions for specific YAML files resolve this issue, including the [Ansible](https://github.com/kartikvashistha/zed-ansible) extension and [Docker Compose](https://github.com/eth0net/zed-docker-compose) extension.

### GitHub Token Configuration

The language server supports an optional GitHub Personal Access Token (PAT) to provide enhanced features like repository-specific completions and validation. You can configure this in three ways:

#### Option 1: Environment Variable (Recommended)

Set the `GITHUB_TOKEN` or `GH_TOKEN` environment variable before starting Zed:

```bash
export GITHUB_TOKEN="ghp_your_token_here"
# or if using GitHub CLI
export GH_TOKEN=$(gh auth token)
```

The extension will automatically detect and use these environment variables.

#### Option 2: Zed Settings

Configure the token directly in your Zed settings (`settings.json`):

```jsonc
{
	// ...
	"lsp": {
		"gh-actions-language-server": {
			"initialization_options": {
				"sessionToken": "ghp_your_token_here"
			}
		}
	}
}
```

#### Option 3: No Token

The language server works without a token, but some features (like repository-specific completions) won't be available.

#### Token Requirements

- [Classic PATs](https://github.com/settings/tokens/new) need `repo` and `workflow` scopes
- [Fine-grained PATs](https://github.com/settings/personal-access-tokens/new) need:
  - Access to "Public repositories" OR "All repositories"/"Only select repositories"
  - Repository permission: `Workflows` (read)

### LSP Settings
You can configure additional LSP settings in Zed:

```jsonc
{
	// ...
	"lsp": {
		"gh-actions-language-server": {
			"initialization_options": {
				"sessionToken": "",  // GitHub PAT (or use GITHUB_TOKEN env var)
				"experimentalFeatures": {
					// Enable all experimental features
					"all": false,
					// Or enable specific features:
					"missingInputsQuickfix": true,        // Code action to add missing inputs
					"blockScalarChompingWarning": true    // Warn about implicit chomping
				},
				"logLevel": "info"  // Options: "off", "error", "warn", "info", "debug"
			}
		}
	}
}
```

#### Available Settings

- **sessionToken** - GitHub PAT for enhanced features (see GitHub Token Configuration above)
- **experimentalFeatures** - Opt-in experimental features:
  - `all` - Enable all experimental features
  - `missingInputsQuickfix` - Code action to add missing required inputs for actions
  - `blockScalarChompingWarning` - Warn when block scalars use implicit clip chomping
- **logLevel** - Control logging verbosity: `"off"`, `"error"`, `"warn"`, `"info"`, or `"debug"`

Individual feature flags take precedence over `all`. For example, `{ all: true, missingInputsQuickfix: false }` enables all experimental features except `missingInputsQuickfix`.

## Expression Features

The extension provides comprehensive support for GitHub Actions expressions:

### Context Autocompletion

Type `${{ ` followed by any context name to get completions:

- **github** - GitHub context (event, ref, repository, actor, etc.)
- **env** - Environment variables defined in workflow
- **secrets** - Repository and organization secrets
- **runner** - Information about the runner executing the job
- **job** - Information about the current job
- **steps** - Outputs from previous steps
- **matrix** - Matrix parameters
- **needs** - Outputs from dependent jobs
- **inputs** - Workflow dispatch or reusable workflow inputs

### Function Autocompletion

All GitHub Actions functions are available with documentation:

- **String functions**: `contains()`, `startsWith()`, `endsWith()`, `format()`
- **Type conversion**: `toJSON()`, `fromJSON()`
- **Array functions**: `join()`
- **Comparison operators**: `==`, `!=`, `<`, `<=`, `>`, `>=`
- **Logical operators**: `&&`, `||`, `!`

### Expression Validation

The language server validates:

- Syntax errors in expressions
- Type mismatches (e.g., comparing string to number)
- Invalid function calls
- Unknown context properties
- Operator precedence issues

### Examples

```yaml
name: Example Workflow

on: push

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        if: ${{ github.event_name == 'push' }}  # ✅ Validated expression
        
      - name: Check branch
        if: ${{ startsWith(github.ref, 'refs/heads/') }}  # ✅ Function completion
        
      - name: Use secrets
        env:
          TOKEN: ${{ secrets.GITHUB_TOKEN }}  # ✅ Context completion
```

## Troubleshooting

### Language Server Not Starting

1. Check that Node.js is installed: `node --version`
2. Check Zed's language server logs for errors
3. Try restarting Zed

### Missing Completions

1. Verify file is detected as GitHub Actions workflow (check filetype)
2. Ensure the file is in `.github/workflows/` directory
3. Check if expressions are properly formatted with `${{ }}`

### Enable Debug Logging

Add to your Zed settings:

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

Then check Zed's LSP logs for detailed information.

## License
Licensed under Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>).

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.
