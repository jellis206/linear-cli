---
name: linear-config
description: Configure linear-cli - auth (API key + OAuth), workspaces, diagnostics, setup wizard.
allowed-tools: Bash
---

# Configuration

```bash
# First-time setup wizard
linear setup

# Set API key
linear config set-key YOUR_API_KEY

# Show config
linear config show

# Auth commands
linear auth login                # Store API key
linear auth oauth                # OAuth 2.0 browser flow (PKCE)
linear auth oauth --client-id ID # Custom OAuth app
linear auth status               # Check auth status (shows type, expiry)
linear auth revoke               # Revoke OAuth tokens
linear auth logout               # Remove key

# Workspaces
linear config workspace-add work KEY
linear config workspace-list
linear config workspace-switch work
linear config workspace-current

# Profiles
linear --profile work i list     # Use profile

# Diagnostics
linear doctor                    # Check config and connectivity
linear doctor --fix              # Auto-fix common issues

# Shell completions (static)
linear config completions bash > ~/.bash_completion.d/linear-cli

# Shell completions (dynamic, context-aware)
linear completions dynamic bash >> ~/.bashrc
linear completions dynamic zsh >> ~/.zshrc
linear completions dynamic fish >> ~/.config/fish/completions/linear-cli.fish
```

## Environment Variables

| Variable | Purpose |
|----------|---------|
| `LINEAR_API_KEY` | API key override |
| `LINEAR_CLI_PROFILE` | Profile override |
| `LINEAR_CLI_OUTPUT` | Default output format |
| `LINEAR_CLI_YES` | Auto-confirm prompts |
| `LINEAR_CLI_NO_PAGER` | Disable pager |
