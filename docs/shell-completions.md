# Shell Completions

Enable tab completions for your shell.

## Bash

```bash
# Create completions directory if needed
mkdir -p ~/.bash_completion.d

# Generate and install completions
linear completions bash > ~/.bash_completion.d/linear-cli

# Add to ~/.bashrc if not already present
echo 'source ~/.bash_completion.d/linear-cli' >> ~/.bashrc
source ~/.bashrc
```

## Zsh

```bash
# Create completions directory if needed
mkdir -p ~/.zsh/completions

# Generate completions
linear completions zsh > ~/.zsh/completions/_linear-cli

# Add to ~/.zshrc if not already present
echo 'fpath=(~/.zsh/completions $fpath)' >> ~/.zshrc
echo 'autoload -Uz compinit && compinit' >> ~/.zshrc
source ~/.zshrc
```

## Fish

```bash
# Generate and install completions
linear completions fish > ~/.config/fish/completions/linear-cli.fish
```

## PowerShell

```powershell
# Generate completions
linear completions powershell > $HOME\linear-cli.ps1

# Add to your PowerShell profile
Add-Content $PROFILE '. $HOME\linear-cli.ps1'
```
