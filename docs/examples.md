# Usage Examples

## Common Tasks

```bash
linear common
linear tasks
linear agent
```

## Projects

```bash
linear p list                              # List all projects
linear p list --archived                   # Include archived
linear p get PROJECT_ID                    # View project details
linear p create "Q1 Roadmap" -t Engineering
linear p update PROJECT_ID --name "New Name"
linear p update PROJECT_ID --name "New Name" --dry-run
linear p delete PROJECT_ID --force
linear p add-labels PROJECT_ID LABEL_ID
```

## Issues

```bash
linear i list                              # List issues
linear i list -t Engineering -s "In Progress"
linear i list --output json                # Output as JSON
linear i get LIN-123                       # View issue details
linear i get LIN-123 --output json         # JSON output
linear i create "Bug fix" -t Eng -p 1      # Priority: 1=urgent, 4=low
cat issue.json | linear i create "Bug fix" -t Eng --data -
linear i update LIN-123 -s Done
linear i update LIN-123 -s Done --dry-run
linear i delete LIN-123 --force
linear i start LIN-123                     # Start working: assigns to you, sets In Progress, creates branch
linear i stop LIN-123                      # Stop working: unassigns, resets status
```

## Labels

```bash
linear l list                              # List project labels
linear l list --type issue                 # List issue labels
linear l create "Feature" --color "#10B981"
linear l create "Bug" --type issue --color "#EF4444"
linear l delete LABEL_ID --force
```

## Git Integration

```bash
linear g checkout LIN-123                  # Create/checkout branch for issue
linear g branch LIN-123                    # Show branch name for issue
linear g create LIN-123                    # Create branch without checkout
linear g checkout LIN-123 -b custom-branch # Use custom branch name
linear g pr LIN-123                        # Create PR linked to issue
linear g pr LIN-123 --draft                # Create draft PR
linear g pr LIN-123 --base main            # Specify base branch
```

## jj (Jujutsu) Integration

```bash
linear j checkout LIN-123                  # Create bookmark for issue
linear j bookmark LIN-123                  # Show bookmark name for issue
linear j create LIN-123                    # Create bookmark without checkout
linear j pr LIN-123                        # Create PR using jj git push
```

## Sync Local Folders

```bash
linear sy status                           # Compare local folders with Linear
linear sy push -t Engineering              # Create Linear projects for local folders
linear sy push -t Engineering --dry-run    # Preview without creating
```

## Search

```bash
linear s issues "authentication bug"
linear s projects "backend" --limit 10
```

## Uploads

Download attachments and images from Linear issues/comments:

```bash
# Download to file
linear up fetch "https://uploads.linear.app/..." -f image.png

# Output to stdout (for piping to other tools)
linear up fetch "https://uploads.linear.app/..." | base64

# Useful for AI agents that need to view images
linear uploads fetch URL -f /tmp/screenshot.png
```

## Other Commands

```bash
# Teams
linear t list
linear t get TEAM_ID

# Users
linear u list
linear u get me

# Cycles
linear c list -t Engineering
linear c current -t Engineering

# Comments
linear cm list ISSUE_ID
linear cm list ISSUE_ID --output json      # JSON output for LLMs
linear cm create ISSUE_ID -b "This is a comment"

# Documents
linear d list
linear d get DOC_ID
linear d create "Doc Title" -p PROJECT_ID
linear d update DOC_ID --title "New title" --dry-run
linear d list --output json

# Templates
linear tpl list
linear tpl list --output json
linear tpl show bug --output json

# Statuses
linear st list -t Engineering
linear st get "In Progress" -t Engineering

# Config
printf '%s\n' "$LINEAR_API_KEY" | linear config set-key
linear config show
```

## Interactive Mode

```bash
linear ui                                  # Launch interactive TUI
linear ui --team ENG                       # Launch with preselected team
linear ui issues                           # Browse issues interactively
linear ui projects                         # Browse projects interactively
linear interactive --team Engineering      # Filter by team
```

## Multiple Workspaces

```bash
linear ws list                             # List configured workspaces
linear ws add personal                     # Add a new workspace
linear ws switch personal                  # Switch active workspace
linear ws current                          # Show current workspace
linear ws remove personal                  # Remove a workspace
```

## Bulk Operations

```bash
linear b update -s Done LIN-1 LIN-2 LIN-3  # Update multiple issues
linear b assign --user me LIN-1 LIN-2      # Assign multiple issues
linear b label --add bug LIN-1 LIN-2       # Add label to multiple issues
linear b move --project "Q1" LIN-1 LIN-2   # Move issues to project
linear b delete --force LIN-1 LIN-2 LIN-3  # Delete multiple issues
```

## JSON Output

```bash
# Use --output json with any list or get command
linear i list --output json
linear p list --output json | jq '.[] | .name'
linear i get LIN-123 --output json
linear t list --output json
linear cm list ISSUE_ID --output json    # Comments as JSON (great for LLMs)

# Token-saving JSON output options
linear i list --output json --fields identifier,title,state.name --compact
LINEAR_CLI_OUTPUT=json linear i list --sort identifier --order desc

# Color control for logs/CI
linear i list --no-color

# Table width control
linear i list --width 80
linear i list --no-truncate
```
