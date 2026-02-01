# Changes in v0.1.0

## Major Changes

### Switched to Official Language Server

The extension now uses the official `@actions/languageserver` package instead of the third-party `gh-actions-language-server` wrapper. This provides:

- **Full expression support** from `@actions/expressions` library
- **Direct access** to GitHub's official implementation
- **Better maintenance** - updates directly from GitHub
- **Enhanced features** - all capabilities of the official language server

### New Expression Features

This release adds comprehensive GitHub Actions expression support:

#### 1. **Expression Validation**
- Syntax error detection in `${{ }}` blocks
- Type checking (e.g., prevents comparing incompatible types)
- Invalid function call detection
- Unknown context property warnings
- Operator precedence validation

#### 2. **Context Autocompletion**
All GitHub Actions contexts are now supported with intelligent completions:
- `github.*` - GitHub context (event, ref, repository, actor, etc.)
- `env.*` - Environment variables
- `secrets.*` - Repository and organization secrets
- `runner.*` - Runner information (os, arch, temp, etc.)
- `job.*` - Current job information
- `steps.*` - Previous step outputs
- `matrix.*` - Matrix parameters
- `needs.*` - Dependent job outputs
- `inputs.*` - Workflow dispatch or reusable workflow inputs

#### 3. **Function Autocompletion**
All built-in GitHub Actions functions with documentation:
- **String**: `contains()`, `startsWith()`, `endsWith()`, `format()`
- **Type conversion**: `toJSON()`, `fromJSON()`
- **Array**: `join()`
- **Operators**: `==`, `!=`, `<`, `<=`, `>`, `>=`, `&&`, `||`, `!`

#### 4. **Hover Documentation**
- Context property descriptions
- Function signatures and usage examples
- Inline documentation for workflow syntax

#### 5. **Code Actions** (Experimental)
- Quick fix to add missing required inputs
- Warning for implicit block scalar chomping

#### 6. **Inlay Hints**
- Type information display
- Parameter hints for functions

### Environment Variable Support

New support for GitHub token configuration via environment variables:

```bash
# Option 1: GITHUB_TOKEN
export GITHUB_TOKEN="ghp_your_token_here"

# Option 2: GH_TOKEN (GitHub CLI default)
export GH_TOKEN=$(gh auth token)
```

The extension automatically detects these environment variables with the following priority:
1. User-configured `sessionToken` in Zed settings (highest priority)
2. `GITHUB_TOKEN` environment variable
3. `GH_TOKEN` environment variable
4. No token (language server still works with reduced functionality)

### Experimental Features

Users can now opt-in to experimental features via Zed settings:

```jsonc
{
  "lsp": {
    "gh-actions-language-server": {
      "initialization_options": {
        "experimentalFeatures": {
          "missingInputsQuickfix": true,
          "blockScalarChompingWarning": true
        }
      }
    }
  }
}
```

## Technical Implementation

### Architecture Changes

1. **Direct Official Package Usage**
   - Uses `@actions/languageserver` official binary (`bin/actions-languageserver`)
   - No custom wrapper needed - the official package includes everything
   - Installed via npm to `node_modules/@actions/languageserver`

2. **Updated Rust Extension** (`src/gh-actions-server.rs`)
   - Changed package from `gh-actions-language-server` to `@actions/languageserver`
   - Points to official binary in `node_modules/@actions/languageserver/bin/actions-languageserver`
   - Added environment variable detection for GitHub tokens (`GITHUB_TOKEN` and `GH_TOKEN`)
   - Enhanced initialization options with experimental features support

3. **Version Bump**
   - Updated to v0.1.0 to reflect significant feature additions
   - Updated description to highlight expression capabilities

### Files Modified

- `src/gh-actions-server.rs` - Core extension logic to use official package
- `extension.toml` - Metadata and version bump
- `README.md` - Comprehensive documentation of new features

### Files Added

- `test-workflow.yml` - Example workflow demonstrating all expression features
- `CHANGES.md` - This changelog

## Migration Notes

### Breaking Changes

- The NPM package name changes from `gh-actions-language-server` to `@actions/languageserver`
- Existing users will automatically download the new package on extension update
- No configuration changes required

### Backward Compatibility

- All existing Zed configuration remains compatible
- File associations unchanged
- Language detection unchanged
- The `sessionToken` initialization option works exactly the same

## Benefits

1. **Official Support** - Using GitHub's official language server implementation
2. **Feature Complete** - All expression features from `@actions/expressions`
3. **Future Proof** - Will receive updates as GitHub Actions evolves
4. **Better Validation** - More accurate error detection and type checking
5. **Enhanced DX** - Better autocompletion and documentation

## Testing

The implementation has been validated with:
- ✅ Rust code compilation (cargo check & cargo build)
- ✅ Extension metadata validation
- ✅ Wrapper script creation and permissions
- ✅ Example workflow file with all expression features

## Next Steps

Users should:
1. Update the extension when available
2. Optionally configure `GITHUB_TOKEN` environment variable for enhanced features
3. Review the updated README for new configuration options
4. Try the expression autocompletion features in workflow files

## Known Limitations

1. **Node.js Required** - The language server requires Node.js to be installed
2. **GitHub Token Optional** - Some features require a GitHub PAT (but server works without it)
3. **Experimental Features** - Some features are opt-in and may change

## Credits

- Language Server: [actions/languageservices](https://github.com/actions/languageservices) by GitHub
- Expression Library: [@actions/expressions](https://www.npmjs.com/package/@actions/expressions)
- Original Extension: [zed-github-actions](https://github.com/neoncitylights/zed-github-actions)
