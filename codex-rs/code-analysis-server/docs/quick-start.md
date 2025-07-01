# Quick Start Guide

This guide provides the exact commands to get started with the Code Analysis Server and MCP Inspector.

## Running the Code Analysis Server with MCP Inspector

### Step 1: Ensure the server binary is executable

```bash
chmod +x ~/project/codex/codex-rs/target/release/code-analysis-server
```

### Step 2: Run the MCP Inspector with the server

Use one of these commands:

```bash
# Option 1: Using the full path
npx @modelcontextprotocol/inspector --server-command="/home/arvkum/project/codex/codex-rs/target/release/code-analysis-server --project-dir /home/arvkum/project/codex"

# Option 2: Using relative path (if you're in the codex-rs directory)
cd ~/project/codex/codex-rs
npx @modelcontextprotocol/inspector --server-command="./target/release/code-analysis-server --project-dir .."

# Option 3: Start the server first, then connect to it
cd ~/project/codex/codex-rs
./target/release/code-analysis-server --project-dir .. &
SERVER_PID=$!
npx @modelcontextprotocol/inspector --server-pid=$SERVER_PID
```

### Step 3: Using the Inspector

Once connected, you can run commands like:

```
> tools/list
> tools/call analyze_code {"file_path": "core/src/code_analysis/tools.rs"}
```

## Troubleshooting

If you encounter the "Option '--env' argument is ambiguous" error:

1. Try using the `--` separator to ensure arguments are passed correctly:
   ```bash
   npx @modelcontextprotocol/inspector -- --server-command="./target/release/code-analysis-server --project-dir .."
   ```

2. Or try using the server PID method instead:
   ```bash
   # Start the server in one terminal
   ./target/release/code-analysis-server --project-dir ..
   
   # In another terminal, find the PID and connect
   PID=$(pgrep code-analysis-server)
   npx @modelcontextprotocol/inspector --server-pid=$PID
   ```

3. If all else fails, try creating a script file:
   ```bash
   echo 'tools/list' > commands.txt
   npx @modelcontextprotocol/inspector --server-command="./target/release/code-analysis-server --project-dir .." --script commands.txt
   ```