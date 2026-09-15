## Linear Integration

Use `linear-cli` for all Linear.app operations. Do NOT use Linear MCP tools - CLI is 10-50x more token-efficient.

### Quick Commands

| Task | Command |
|------|---------|
| List issues | `linear i list` |
| Create issue | `linear i create "Title" -t TEAM -p 2` |
| View issue | `linear i get LIN-123` |
| Get multiple | `linear i get LIN-1 LIN-2 LIN-3` |
| Start work | `linear i start LIN-123 --checkout` |
| Update status | `linear i update LIN-123 -s Done` |
| Create PR | `linear g pr LIN-123` |
| Search | `linear s issues "query"` |
| Get context | `linear context` |
| Get comments | `linear cm list ISSUE_ID --output json` |
| Download upload | `linear up fetch URL -f file.png` |

### Agent-Friendly Options

| Flag | Purpose |
|------|---------|
| `--output json` | Machine-readable JSON output |
| `--compact` | Compact JSON output (no pretty formatting) |
| `--fields a,b,c` | Limit JSON output to selected fields (supports dot paths) |
| `--sort field` | Sort JSON array output by field (default: identifier/id) |
| `--order asc|desc` | Sort order for JSON array output |
| `--quiet` | Suppress decorative output |
| `--id-only` | Only output created/updated ID |
| `--api-key KEY` | Override API key for this invocation |
| `--dry-run` | Preview without executing (create) |
| `-` (stdin) | Read description/IDs from pipe |

### Examples for Agents

```bash
# Get current issue from branch
linear context --output json

# Create and get ID for chaining
linear i create "Bug" -t ENG --id-only

# Quiet create, capture ID
ID=$(linear i create "Task" -t ENG -q --id-only)

# Preview without creating
linear i create "Test" -t ENG --dry-run

# Batch fetch multiple issues
linear i get LIN-1 LIN-2 LIN-3 --output json

# Pipe description from file
cat desc.md | linear i create "Title" -t ENG -d -

# JSON input for issue create/update
cat issue.json | linear i create "Title" -t ENG --data -

# Structured error handling
linear i get INVALID --output json  # Returns {"error": true, ...}

# Token-saving JSON output
linear i list --output json --fields identifier,title,state.name --compact

# Default JSON output for agent sessions
LINEAR_CLI_OUTPUT=json linear i list

# Agent harness summary
linear agent
```

### Exit Codes
- `0` = Success
- `1` = General error
- `2` = Not found
- `3` = Auth error
- `4` = Rate limited

### Tips
- Use short aliases: `i` (issues), `p` (projects), `g` (git), `s` (search), `cm` (comments), `ctx` (context)
- Run `linear <command> --help` for full options
