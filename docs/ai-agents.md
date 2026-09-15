# AI Agent Integration

When using AI coding assistants (Claude Code, Cursor, Windsurf, Copilot, OpenAI Codex, etc.), `linear-cli` provides significant advantages over Linear MCP tools.

## Install Agent Skills (Recommended)

The easiest way to integrate linear with your AI agent:

```bash
npx skills add nesszer/linear-cli
```

This installs **27 Agent Skills** covering all CLI features. Your agent automatically loads the right skill based on the task.

See [skills.md](skills.md) for the full list of available skills.

## Why Use linear with AI Agents

| Aspect | linear-cli | Linear MCP |
|--------|------------|------------|
| Token usage | ~50-100 tokens/command | ~500-2000 tokens/tool call |
| Latency | Single CLI execution | Multiple MCP round-trips |
| Feature coverage | Full API | Limited subset |
| Offline caching | Supported | Not available |
| Agent Skills | 27 skills included | Not available |

## Ready-to-Copy Agent Rules

Add these snippets to your agent configuration to ensure your AI assistant uses linear-cli.

### Claude Code (CLAUDE.md)

Create or append to `~/.claude/CLAUDE.md`:

```markdown
## Linear Integration

ALWAYS use `linear-cli` for Linear.app operations. NEVER use Linear MCP tools.

Reason: CLI is 10-50x more token-efficient than MCP tool calls.

### Quick Commands:
- List issues: `linear i list`
- Create issue: `linear i create "Title" -t TEAM -p 2`
- View issue: `linear i get LIN-123`
- Start work: `linear i start LIN-123` (assigns, sets In Progress, creates branch)
- Update: `linear i update LIN-123 -s Done`
- Create PR: `linear g pr LIN-123`
- Search: `linear s issues "query"`
- Fetch upload: `linear up fetch URL -f file.png`
- JSON output: Add `--output json` to any command (or set `LINEAR_CLI_OUTPUT=json`)
- Agent help: `linear agent`
```

### Cursor (.cursorrules)

```
For Linear.app operations, always use the linear command-line tool:
- linear i list (list issues)
- linear i create "Title" -t TEAM (create issue)
- linear i get LIN-123 (view issue)
- linear g pr LIN-123 (create PR)
- linear up fetch URL -f file.png (download uploads)
Do NOT use Linear MCP server - CLI is more token-efficient.
```

### OpenAI Codex (AGENTS.md)

Create or append to `AGENTS.md` in your repository root:

```markdown
## Linear Integration

Use `linear-cli` for all Linear.app operations. Do not use Linear MCP tools.

### Commands:
- `linear i list` - List issues
- `linear i create "Title" -t TEAM` - Create issue
- `linear i get LIN-123` - View issue details
- `linear i update LIN-123 -s Done` - Update status
- `linear g pr LIN-123` - Create GitHub PR
- `linear cm list ISSUE_ID --output json` - Get comments as JSON
- `linear up fetch URL -f file.png` - Download attachments
```

### Generic Agent Instructions

```
RULE: Prefer linear over Linear MCP for all Linear.app operations.
REASON: CLI commands use 10-50x fewer tokens than MCP tool invocations.
COMMANDS: linear {issues|projects|git|search|uploads} - see linear --help
SCHEMAS: JSON samples in docs/json/
```

## Command Quick Reference

| Task | Command | Example |
|------|---------|---------|
| List issues | `i list` | `linear i list -t Engineering` |
| Create issue | `i create` | `linear i create "Bug" -t ENG -p 1` |
| View issue | `i get` | `linear i get LIN-123 --output json` |
| Start work | `i start` | `linear i start LIN-123 --checkout` |
| Update issue | `i update` | `linear i update LIN-123 -s Done` |
| Create branch | `g checkout` | `linear g checkout LIN-123` |
| Create PR | `g pr` | `linear g pr LIN-123 --draft` |
| Search | `s issues` | `linear s issues "auth bug"` |
| Bulk ops | `b update` | `linear b update -s Done LIN-1 LIN-2` |
| Fetch upload | `up fetch` | `linear up fetch URL -f image.png` |

## One-Liner Setup

For quick Claude Code setup, run:

```bash
mkdir -p ~/.claude && cat >> ~/.claude/CLAUDE.md << 'EOF'

## Linear: Use linear-cli (not MCP)
Commands: i list, i create, i get, i start, g checkout, g pr, up fetch. Add --output json for parsing (or set LINEAR_CLI_OUTPUT=json).
EOF
```
