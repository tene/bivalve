# Bivalve

Bivalve is an attempt to integrate large parts of the command-line experience.  I'm trying to merge tmux/screen, the shell, and ssh, and optionally also the terminal.

# TODO
- Traditional interactive blocking shell, run commands, print output
- Interactive line editor, pinned to the end of text
    - Snapshot prompt before printing output
    - Live widgets in prompt?
    - How to print concurrent output from multiple commands?
        - Interleaved, or chronological?
        - If any output from command is still visible on screen, inject lines there
        - If not, then add a new continuation header at bottom of screen before output
        - If output line is short enough, put small continuation header on right side?
- Global aliases, functions, and variables
- Run commands over ssh
    - UX for swapping between contexts?
    - Connection status in prompt
- Readline input handling
- TUI, commands as cards
- Persistent daemon, reconnect or share sessions
- Store commands and output in sled db
- Search commands and output
- Hierarchal tabs
    - Variables local to tab groups?
- Transparently access files across multiple hosts
- Agent/buffer for task handling on remote nodes
- Run interactive TUI apps
    - How to detect these??

- wry webview, commands as cards
    - Live filesystem watch/reload on themes

- Generic library for similar interaction/repl/task-management

# Scope Creep
- Editor?
- VFS?
- File browser?
- Cron?