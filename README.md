# linear-cli

[![Crates.io](https://img.shields.io/crates/v/linear-cli)](https://crates.io/crates/linear-cli)
[![CircleCI](https://dl.circleci.com/status-badge/img/gh/nesszer/linear-cli/tree/master.svg?style=shield)](https://app.circleci.com/pipelines/github/nesszer/linear-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

A fast, comprehensive command-line interface for [Linear](https://linear.app) built in Rust. Manage issues, projects, cycles, sprints, documents, and more -- entirely from your terminal.

## Installation

```bash
# Pre-built binary (fastest — no compilation)
cargo binstall linear-cli

# From crates.io (compiles from source)
cargo install linear-cli

# With OS keyring support (Keychain, Credential Manager, Secret Service)
cargo install linear-cli --features secure-storage

# From source
git clone https://github.com/nesszer/linear-cli.git
cd linear-cli && cargo build --release
```

Pre-built binaries for Linux (x86_64, aarch64), macOS (x86_64, aarch64), and Windows (x86_64) are available at [GitHub Releases](https://github.com/nesszer/linear-cli/releases). [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) downloads these automatically.

## Updating

```bash
# Recommended: let the CLI update itself
linear update

# Check without installing
linear update --check

# Manual fallback when you want the Cargo path directly
cargo install linear-cli --force

# Manual fallback for keyring-enabled builds
cargo install linear-cli --force --features secure-storage
```

`cargo update` updates a project's `Cargo.lock`. It does not upgrade an installed `linear-cli` binary.

## Quick Start

```bash
# 1. Set your API key (get one at https://linear.app/settings/api)
printf '%s\n' "$LINEAR_API_KEY" | linear config set-key

# Or use OAuth 2.0 (browser-based, auto-refreshing)
linear auth oauth

# 2. List your issues
linear i list --mine

# 3. Start working on an issue (assigns to you, sets In Progress, creates branch)
linear i start LIN-123 --checkout

# 4. When done, mark complete and create a PR
linear done
linear g pr LIN-123
```

## Commands

### Issues

Full issue lifecycle management with 16 subcommands.

```bash
linear issues list                           # List issues
linear i list -t ENG --mine                  # My issues on a team
linear i list --since 7d --group-by state    # Last 7 days, grouped by status
linear i list --label bug --count-only       # Count bugs
linear i list --view "My Sprint"             # Apply a saved custom view

linear i get LIN-123                         # Issue details
linear i get LIN-123 --history               # Activity timeline
linear i get LIN-123 --comments              # Inline comments
linear i get LIN-1 LIN-2 LIN-3              # Batch fetch

linear i create "Fix login" -t ENG -p 1      # Create urgent issue
linear i create "Fix login" -t ENG --project "Q2 Roadmap"
linear i update LIN-123 -s Done              # Update status
linear i update LIN-123 -l bug -l urgent     # Add labels
linear i update LIN-123 --due tomorrow       # Set due date
linear i update LIN-123 -e 3                 # Set estimate

linear i start LIN-123 --checkout            # Start + checkout branch
linear i stop LIN-123                        # Return to backlog
linear i close LIN-123                       # Mark as Done
linear i assign LIN-123 "Alice"              # Assign to user
linear i move LIN-123 "Q2 Project"           # Move to project
linear i transfer LIN-123 ENG                # Transfer to team
linear i comment LIN-123 -b "LGTM"           # Add comment
linear i archive LIN-123                     # Archive
linear i open LIN-123                        # Open in browser
linear i link LIN-123                        # Print URL
```

**Create flags:** `--team`, `--description`, `--data`, `--priority`, `--state`, `--assignee`, `--labels`, `--due`, `--estimate`, `--project`, `--template`, `--dry-run`

**List flags:** `--mine`, `--team`, `--state`, `--assignee`, `--project`, `--label`, `--since`, `--view`, `--group-by` (state/priority/assignee/project), `--count-only`, `--archived`

### Projects

Full project CRUD with label management and archiving.

```bash
linear projects list                         # List all projects
linear p get "Q1 Roadmap"                    # Project details
linear p create "New Feature" -t ENG         # Create project
linear p update PROJECT_ID --name "Renamed"  # Update project
linear p members "Q1 Roadmap"                # List members
linear p add-labels PROJECT_ID bug           # Add labels
linear p remove-labels PROJECT_ID bug        # Remove labels
linear p set-labels PROJECT_ID bug feat      # Replace all labels
linear p archive PROJECT_ID                  # Archive
linear p unarchive PROJECT_ID                # Unarchive
linear p open "Q1 Roadmap"                   # Open in browser
linear p delete PROJECT_ID                   # Delete
```

### Project Updates

Track project health with status updates (onTrack, atRisk, offTrack).

```bash
linear project-updates list PROJECT_ID       # List updates
linear pu get UPDATE_ID                      # Get update details
linear pu create PROJECT_ID -b "On track"    # Create update
linear pu update UPDATE_ID -b "Updated"      # Edit update
linear pu archive UPDATE_ID                  # Archive
linear pu unarchive UPDATE_ID                # Unarchive
```

### Teams

```bash
linear teams list                            # List all teams
linear t get ENG                             # Team details
linear t members ENG                         # List members
linear t create "Platform" -k PLT            # Create team
linear t update TEAM_ID --name "Infra"       # Update team
linear t delete TEAM_ID                      # Delete team
```

### Cycles

```bash
linear cycles list -t ENG                    # List cycles
linear c current -t ENG                      # Current cycle
linear c get CYCLE_ID                        # Cycle details with issues
linear c create -t ENG --start 2026-03-01 --end 2026-03-14
linear c update CYCLE_ID --name "Sprint 5"
linear c complete CYCLE_ID                   # Complete cycle
linear c delete CYCLE_ID
```

### Sprint Planning

Plan and manage cycle-based sprints with progress visualization, burndown charts, and velocity tracking.

```bash
linear sprint status -t ENG                  # Current sprint status
linear sp progress -t ENG                    # Progress bar visualization
linear sp plan -t ENG                        # Next sprint's planned issues
linear sp carry-over -t ENG --force          # Move incomplete to next cycle
linear sp burndown -t ENG                    # ASCII burndown chart
linear sp velocity -t ENG                    # Velocity across past 6 sprints
linear sp velocity -t ENG -n 10              # Velocity across past 10 sprints
```

### Documents, Labels, Comments

```bash
# Documents
linear documents list                        # List documents
linear d create "ADR-001" -c "Content..."    # Create document
linear d update DOC_ID -c "Updated"          # Update
linear d delete DOC_ID                       # Delete

# Labels
linear labels list                           # List labels
linear l create "priority:p0" -c "#FF0000"   # Create with color
linear l update LABEL_ID -n "Renamed"        # Rename
linear l delete LABEL_ID                     # Delete

# Comments
linear comments list ISSUE_ID                # List comments
linear cm create ISSUE_ID -b "Comment text"  # Add comment
linear cm update COMMENT_ID -b "Edited"      # Edit
linear cm delete COMMENT_ID                  # Delete
```

### Milestones, Roadmaps, Initiatives

```bash
# Milestones
linear milestones list -p "Q1 Roadmap"       # List project milestones
linear ms create "Beta" -p PROJECT_ID        # Create milestone
linear ms update MS_ID --name "GA"           # Update
linear ms delete MS_ID                       # Delete

# Roadmaps
linear roadmaps list                         # List roadmaps
linear rm get ROADMAP_ID                     # Roadmap details
linear rm create "2026 Plan"                 # Create
linear rm update RM_ID --name "H1 2026"      # Update
linear rm delete RM_ID                       # Delete

# Initiatives
linear initiatives list                      # List initiatives
linear init get INIT_ID                      # Initiative details
linear init create "Platform Migration"      # Create
linear init update INIT_ID --name "Renamed"  # Update
linear init delete INIT_ID                   # Delete
```

### Custom Views

```bash
linear views list                            # List saved views
linear v get VIEW_ID                         # View details
linear v create "My Bugs" -t ENG             # Create view
linear v update VIEW_ID --name "Open Bugs"   # Update
linear v delete VIEW_ID                      # Delete
linear i list --view "My Bugs"               # Apply view to issue list
```

### Relations

```bash
linear relations list LIN-123                # List relationships
linear rel add LIN-123 blocks LIN-456        # Add relation
linear rel remove LIN-123 blocks LIN-456     # Remove relation
linear rel parent LIN-456 LIN-123            # Set parent issue
linear rel unparent LIN-456                  # Remove parent
```

### Attachments

```bash
linear attachments list ISSUE_ID             # List attachments
linear att get ATTACHMENT_ID                 # Get details
linear att create ISSUE_ID -u URL -t "Doc"   # Create attachment
linear att link-url ISSUE_ID URL             # Link a URL
linear att update ATTACHMENT_ID -t "New"     # Update
linear att delete ATTACHMENT_ID              # Delete
```

### Templates

Local templates and Linear workspace (remote) templates.

```bash
# Local templates
linear templates list                        # List local templates
linear tpl create bug --team ENG --priority 2 --label bug
linear --dry-run --output json tpl create bug --team ENG
linear tpl show TEMPLATE_NAME                # Show details
linear tpl delete TEMPLATE_NAME --force      # Delete

# Linear workspace templates
linear tpl remote-list                       # List API templates
linear tpl remote-get TEMPLATE_ID            # Get template
linear tpl remote-create -n "Bug Report" --type issue
linear tpl remote-update TEMPLATE_ID         # Update
linear tpl remote-delete TEMPLATE_ID         # Delete
```

### Notifications

```bash
linear notifications list                    # Unread notifications
linear n count                               # Unread count
linear n read NOTIFICATION_ID                # Mark as read
linear n read-all                            # Mark all as read
linear n archive NOTIFICATION_ID             # Archive one
linear n archive-all                         # Archive all
```

### Statuses & Time Tracking

```bash
# Statuses
linear statuses list -t ENG                  # List workflow states
linear st update STATUS_ID --name "Review"   # Rename a status

# Time tracking
linear time list ISSUE_ID                    # List time entries
linear tm update ENTRY_ID --hours 2.5        # Update entry
```

### Favorites

```bash
linear favorites list                        # List favorites
linear fav add ISSUE_ID                      # Add to favorites
linear fav remove FAVORITE_ID                # Remove
```

### Users

```bash
linear users list                            # List workspace users
linear u me                                  # Current user
linear u get "alice@example.com"             # Look up a user
linear whoami                                # Alias for `users me`
```

### Webhooks

Full CRUD plus a local listener with HMAC-SHA256 signature verification.

```bash
linear webhooks list                         # List webhooks
linear wh get WEBHOOK_ID                     # Webhook details
linear wh create https://hook.example.com    # Create webhook
linear wh update WEBHOOK_ID --url NEW_URL    # Update
linear wh rotate-secret WEBHOOK_ID           # Rotate signing secret
linear wh delete WEBHOOK_ID                  # Delete
linear wh listen --port 8080                 # Start local listener
```

### Watch Mode

Poll for real-time changes to issues, projects, or teams.

```bash
linear watch issue LIN-123                   # Watch an issue
linear w project PROJECT_ID                  # Watch a project
linear w team ENG                            # Watch a team
```

### Triage

```bash
linear triage list -t ENG                    # Unassigned issues
linear tr claim LIN-123                      # Assign to self
linear tr snooze LIN-123                     # Snooze for later
```

### Bulk Operations

```bash
linear bulk update-state Done -i LIN-1,LIN-2   # Bulk status update
linear b assign "Alice" -i LIN-1,LIN-2         # Bulk assign
linear b label bug -i LIN-1,LIN-2              # Bulk add label
linear b unassign -i LIN-1,LIN-2               # Bulk unassign
```

### Git Integration

Works with both Git and Jujutsu (jj).

```bash
linear git checkout LIN-123                  # Create + checkout branch
linear g branch LIN-123                      # Show branch name
linear g create LIN-123                      # Create branch (no checkout)
linear g commits                             # Commits with Linear trailers (jj)
linear g pr LIN-123 --draft                  # Create GitHub PR
linear g review-url LIN-123                  # Linear review URL for the issue's PR
```

`review-url` reads the review URL from the issue's pull request notifications —
the one public place a pull request is paired with its review page — and falls
back to the pull requests linked to the issue's agent sessions. A pull request
that has produced neither (a brand-new PR with no CI result, comment, or review
activity yet) has nothing to resolve, and the command reports it instead of
guessing a URL.

Unresolved pull requests are part of the output, not a silent omission: `-o json`
returns `{"resolved": [...], "unresolved": [...]}`, and the plain-text form prints
the review URLs on stdout while naming any unresolved pull request on stderr. The
command fails only when it resolved nothing at all.

### Import / Export

Round-trip CSV and JSON import/export with field resolution for status, assignee, and labels.

```bash
# Import
linear import csv issues.csv -t ENG          # Import from CSV
linear import json issues.json -t ENG        # Import from JSON
linear import csv issues.csv -t ENG --dry-run  # Preview without creating

# Export
linear export csv -t ENG -f issues.csv       # Export issues to CSV
linear export json -t ENG -f issues.json     # Export issues to JSON
linear export markdown -t ENG                # Export to Markdown
linear export projects-csv -f projects.csv   # Export projects to CSV
```

### Search & Context

```bash
linear search issues "auth bug"              # Search issues
linear s projects "platform"                 # Search projects
linear context                               # Issue from current git branch
linear history LIN-123                       # Activity timeline
linear metrics -t ENG                        # Team velocity and stats
```

### Raw GraphQL

Direct API access for anything not covered by built-in commands.

```bash
linear api query '{ viewer { name email } }'
linear api mutate 'mutation { issueUpdate(id: "...", input: { ... }) { success } }'
```

### Other Commands

```bash
linear done                                  # Mark current branch issue as Done
linear interactive                           # TUI for browsing/managing issues
linear sync status                           # Compare local folders with Linear
linear sync push                             # Create Linear projects from folders
```

## Authentication

Two authentication methods are supported. Both can be used per-profile.

### API Key

```bash
# Set directly
printf '%s\n' "$LINEAR_API_KEY" | linear config set-key

# Or interactive login
linear auth login

# Store in OS keyring (requires --features secure-storage)
linear auth login --secure

# Or use environment variable (highest priority)
export LINEAR_API_KEY=lin_api_xxx
```

### OAuth 2.0

Browser-based Authorization Code + PKCE flow with automatic token refresh.

```bash
linear auth oauth          # Opens browser for authorization
linear auth oauth --secure # Store OAuth tokens in OS keyring (best on official release builds)
linear auth status         # Show auth type, token expiry
linear auth revoke         # Revoke OAuth tokens
linear auth logout         # Remove stored credentials
```

> `--secure` needs a build with `--features secure-storage` (enables OS backends: Keychain, Credential Manager, Secret Service). On macOS Keychain, unsigned local builds can still prompt repeatedly or fail readback — if that happens, use an official release, plain `linear auth oauth`, or `LINEAR_API_KEY`.

**Auth priority:** `LINEAR_API_KEY` env var > OS keyring > OAuth tokens > config file API key.

## Configuration

Config is stored at `~/.config/linear-cli/config.toml` (Linux/macOS) or `%APPDATA%\linear-cli\config.toml` (Windows).

```bash
linear config show                           # Show current config
linear config get default-team               # Get default team
linear config set default-team ENG           # Set default team
# also accepted: default_team, team

# Multiple workspaces
linear config workspace-add work             # Add workspace profile
linear config workspace-list                 # List profiles
linear config workspace-switch work          # Switch active profile
linear config workspace-current              # Show current
linear config workspace-remove work          # Remove profile

# Per-invocation profile override
linear --profile work i list
export LINEAR_CLI_PROFILE=work
```

### Setup & Diagnostics

```bash
linear setup                                 # Guided onboarding wizard
linear doctor                                # Check config + connectivity
linear doctor --fix                          # Auto-remediate issues
linear cache status                          # Cache stats
linear cache clear                           # Clear cache
```

## Shell Completions

### Static Completions

Generate tab completions for command names and flags.

```bash
# Bash
linear completions static bash > ~/.bash_completion.d/linear-cli

# Zsh
linear completions static zsh > ~/.zfunc/_linear-cli

# Fish
linear completions static fish > ~/.config/fish/completions/linear-cli.fish

# PowerShell
linear completions static powershell > linear-cli.ps1
```

### Dynamic Completions

Context-aware completions that query the Linear API for team names, project names, issue identifiers, statuses, and more.

```bash
linear completions dynamic bash              # Dynamic bash completions
linear completions dynamic zsh               # Dynamic zsh completions
linear completions dynamic fish              # Dynamic fish completions
linear completions dynamic powershell        # Dynamic PowerShell completions
```

Legacy alias: `linear config completions <shell>` also generates static completions.

## Agent & Automation Usage

`linear-cli` is designed to work well with AI agents and scripts. Every command supports machine-readable output.

### Output Flags

| Flag | Purpose |
|------|---------|
| `--output json` | JSON output (also `ndjson`) |
| `--compact` | Compact JSON (no pretty-printing) |
| `--fields a,b,c` | Limit JSON to specific fields (dot paths supported) |
| `--sort field` | Sort JSON arrays by field |
| `--order asc\|desc` | Sort direction |
| `--quiet` | Suppress decorative output |
| `--id-only` | Only output resource ID (for chaining) |
| `--format tpl` | Template output, e.g. `"{{identifier}} {{title}}"` |
| `--filter f=v` | Client-side filter (`=`, `!=`, `~=`; dot paths; case-insensitive) |
| `--fail-on-empty` | Non-zero exit when list is empty |
| `--dry-run` | Preview without making changes |
| `--yes` | Auto-confirm all prompts |
| `--no-pager` | Disable auto-paging |
| `--no-cache` | Bypass cache |

To allow an absolute `PAGER` path you explicitly trust, set `LINEAR_CLI_TRUST_PAGER=1`.

### Scripting Examples

```bash
# Get issue ID for chaining
ID=$(linear i create "Bug" -t ENG --id-only --quiet)

# JSON output for programmatic consumption
linear i list --output json --fields identifier,title,state.name --compact

# Pipe description from file
cat desc.md | linear i create "Title" -t ENG -d -

# JSON input for structured create/update
cat issue.json | linear i create "Title" -t ENG --data -

# Default JSON for entire session
export LINEAR_CLI_OUTPUT=json

# Batch get with structured output
linear i get LIN-1 LIN-2 LIN-3 --output json --compact
```

### Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error |
| `2` | Not found |
| `3` | Auth error |
| `4` | Rate limited |

### Pagination

```bash
linear i list --limit 25                     # Limit results
linear i list --all --page-size 100           # Fetch all pages
linear i list --after CURSOR                  # Cursor-based pagination
```

## Agent Skills

`linear-cli` includes Agent Skills for AI coding assistants (Claude Code, Cursor, Codex, etc.).

```bash
# Install all skills
npx skills add nesszer/linear-cli

# Install specific skill
npx skills add nesszer/linear-cli --skill linear-workflow
```

38 skills covering issues, git, planning, organization, operations, tracking, and advanced API usage. Skills are 10-50x more token-efficient than MCP tools. See [docs/skills.md](docs/skills.md) for details.

## Key Features

- **50+ commands** across 30+ command groups with short aliases
- **OAuth 2.0 + PKCE** authentication alongside API key auth
- **Dynamic shell completions** for bash, zsh, fish, and PowerShell
- **Import/Export** with round-trip CSV and JSON support
- **Sprint planning** with progress bars, burndown charts, velocity tracking, and carry-over between cycles
- **Webhook listener** with HMAC-SHA256 signature verification
- **Watch mode** for real-time polling on issues, projects, and teams
- **Custom views** that can be applied to issue and project lists
- **Bulk operations** for updating, assigning, and labeling multiple issues
- **Git and Jujutsu (jj)** support for branch management and PR creation
- **Interactive TUI** for browsing and managing issues
- **Template system** with both local and Linear workspace templates
- **Auto-paging** output through `less` on Unix terminals
- **Multiple workspaces** with named profiles and seamless switching
- **Reliable networking** with HTTP timeouts, jittered retries, and atomic cache writes

## Security

`linear-cli` is a local CLI, but it still handles sensitive credentials, exported Linear data, local webhook/OAuth listeners, and update flows that call local tooling. The short version:

- API-key auth can come from `LINEAR_API_KEY`, OS keyring storage, or the config file. OAuth token storage is a separate auth path.
- The OAuth callback server binds to `127.0.0.1` and validates `state` plus PKCE before token exchange.
- The webhook listener defaults to `127.0.0.1`, verifies HMAC-SHA256 signatures, and enforces header/body limits.
- Upload fetching is restricted to `https://uploads.linear.app`.
- The update flow checks GitHub Releases and runs explicit Cargo commands without a shell. Install attempts can come from `linear update` or from the interactive startup prompt path.

See [SECURITY.md](SECURITY.md) for reporting guidance and [docs/security-threat-model.md](docs/security-threat-model.md) for the detailed repository threat model.

## Documentation

- [Agent Skills](docs/skills.md) -- 38 skills for AI agents
- [AI Agent Integration](docs/ai-agents.md) -- Setup for Claude Code, Cursor, Codex
- [Security Policy](SECURITY.md) -- Reporting guidance and supported versions
- [Threat Model](docs/security-threat-model.md) -- Repo-grounded security overview and abuse paths
- [Usage Examples](docs/examples.md) -- Detailed command examples
- [Workflows](docs/workflows.md) -- Common workflow patterns
- [JSON Samples](docs/json/README.md) -- Example JSON output shapes
- [Shell Completions](docs/shell-completions.md) -- Tab completion setup

## Comparison with Other CLIs

| Feature | @linear/cli | linear-go | linear-cli |
|---------|-------------|-----------|------------|
| Last updated | 2021 | 2023 | 2026 |
| Commands | ~10 | ~10 | **50+** |
| Agent Skills | No | No | **38 skills** |
| OAuth 2.0 (PKCE) | No | No | Yes |
| Sprint planning | No | No | status, progress, burndown, velocity, carry-over |
| Import/Export | No | No | CSV, JSON, Markdown |
| Webhooks + listener | No | No | CRUD + HMAC-SHA256 listener |
| Custom views | No | No | Full CRUD + apply |
| Project updates | No | No | CRUD + health status |
| Templates (local + remote) | No | No | Full CRUD |
| Dynamic completions | No | No | bash/zsh/fish/pwsh |
| Issue workflow actions | No | No | assign, move, transfer, close, archive |
| Bulk operations | No | No | Yes |
| Watch mode | No | No | issue, project, team |
| Raw GraphQL API | No | No | query + mutate |
| Git + jj support | No | No | Yes |
| Interactive TUI | No | No | Yes |
| Multiple workspaces | No | No | Yes |
| JSON output | No | Yes | JSON, NDJSON, templates |

## Contributing

Contributions welcome! Please open an issue or submit a pull request.

## License

[MIT](LICENSE)
