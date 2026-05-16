# -RPAM-reproducible-pallet-aliases-manager
Hierarchical Alias / Action Manager Concept

Main Idea

A graphical alias and workflow manager inspired by:

- Firefox bookmark folders
- command palettes
- launcher systems
- workflow automation tools

The goal is to replace chaotic flat shell aliases with a structured, searchable and user-friendly action system.

---

Problem

Traditional shell aliases become difficult to manage over time.

Examples:

- too many aliases
- unreadable abbreviations
- name conflicts
- hard to remember commands
- no grouping/categories
- difficult backup/export management

Typical aliases eventually turn into things like:

- "nsr"
- "gcm"
- "dcu"

which become hard to remember and maintain.

---

Proposed Solution

Instead of a flat alias list, use hierarchical grouped actions.

Example structure:

nix/
  rebuild
  update-flake
  backup-config

docker/
  up
  down
  logs

waydroid/
  start
  stop
  restart

Each entry contains:

- visible action name
- shell command/script
- optional description
- optional tags

---

UI Concept

The UI should behave similarly to bookmark folders or command palettes.

Example:

- left side → action name
- right side → command/script

Features:

- folders/categories
- search
- fuzzy search
- keyboard navigation
- launcher mode
- favorites/recent actions

---

Search-first Workflow

Instead of remembering aliases manually:

User types:

rebuild

Search results:

nix/rebuild-system
nix/rebuild-boot
nix/update-flake

Press Enter → execute.

---

Terminal Integration

When executing actions:

- optionally open an already running terminal
- show live command output
- allow cancelling execution
- keep logs/history

This is important because users should see what happens instead of background “magic”.

---

Config System

Suggested config location:

~/.config/aliases/

Possible formats:

- KDL
- TOML
- JSON

Example:

category "nix" {
  action "rebuild" {
    exec "sudo nixos-rebuild switch --flake /etc/nixos#Laptop"
  }
}

---

Backup System

Automatic backups before modifications.

Suggested features:

- timestamped backups
- configurable backup limit
- rollback support
- backup browser inside UI

Example:

config.16-05-2026_21-33-18.bak

---

NixOS Integration

The project is primarily designed for:

- NixOS
- Home Manager
- dotfiles workflows
- declarative setups

Potential ideas:

- generate shell aliases automatically
- export configs declaratively
- integrate with flakes/home-manager

---

Launcher Integration

Potential integration with:

- Walker
- Fuzzel
- Rofi
- Wofi
- custom launcher mode

Example:

Search: waydroid

Results:

waydroid/start
waydroid/stop
waydroid/restart

---

Long-term Vision

The project is not just an alias manager.

It is closer to:

- workflow launcher
- command knowledge base
- action palette
- personal automation hub

The focus is:

- organization
- discoverability
- reproducibility
- usability
- workflow ergonomics
- Linux power-user experience
