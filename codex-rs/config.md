# Config

Codex supports several mechanisms for setting config values:

- Config-specific command-line flags, such as `--model o3` (highest precedence).
- A generic `-c`/`--config` flag that takes a `key=value` pair, such as `--config model="o3"`.
  - The key can contain dots to set a value deeper than the root, e.g. `--config model_providers.openai.wire_api="chat"`.
  - Values can contain objects, such as `--config shell_environment_policy.include_only=["PATH", "HOME", "USER"]`.
  - For consistency with `config.toml`, values are in TOML format rather than JSON format, so use `{a = 1, b = 2}` rather than `{"a": 1, "b": 2}`.
  - If `value` cannot be parsed as a valid TOML value, it is treated as a string value. This means that both `-c model="o3"` and `-c model=o3` are equivalent.
- The `$CODEX_HOME/config.toml` configuration file where the `CODEX_HOME` environment value defaults to `~/.codex`. (Note `CODEX_HOME` will also be where logs and other Codex-related information are stored.)

Both the `--config` flag and the `config.toml` file support the following options:

## model

The model that Codex should use.

```toml
model = "o3"  # overrides the default of "codex-mini-latest"
```

## model_provider

Codex comes bundled with a number of "model providers" predefined. This config value is a string that indicates which provider to use. You can also define your own providers via `model_providers`.

For example, if you are running ollama with Mistral locally, then you would need to add the following to your config:

```toml
model = "mistral"
model_provider = "ollama"
```

because the following definition for `ollama` is included in Codex:

```toml
[model_providers.ollama]
name = "Ollama"
base_url = "http://localhost:11434/v1"
wire_api = "chat"
```

This option defaults to `"openai"` and the corresponding provider is defined as follows:

```toml
[model_providers.openai]
name = "OpenAI"
base_url = "https://api.openai.com/v1"
env_key = "OPENAI_API_KEY"
wire_api = "responses"
```

## model_providers

This option lets you override and amend the default set of model providers bundled with Codex. This value is a map where the key is the value to use with `model_provider` to select the correspodning provider.

For example, if you wanted to add a provider that uses the OpenAI 4o model via the chat completions API, then you

```toml
# Recall that in TOML, root keys must be listed before tables.
model = "gpt-4o"
model_provider = "openai-chat-completions"

[model_providers.openai-chat-completions]
# Name of the provider that will be displayed in the Codex UI.
name = "OpenAI using Chat Completions"
# The path `/chat/completions` will be amended to this URL to make the POST
# request for the chat completions.
base_url = "https://api.openai.com/v1"
# If `env_key` is set, identifies an environment variable that must be set when
# using Codex with this provider. The value of the environment variable must be
# non-empty and will be used in the `Bearer TOKEN` HTTP header for the POST request.
env_key = "OPENAI_API_KEY"
# valid values for wire_api are "chat" and "responses".
wire_api = "chat"
```

## approval_policy

Determines when the user should be prompted to approve whether Codex can execute a command:

```toml
# This is analogous to --suggest in the TypeScript Codex CLI
approval_policy = "unless-allow-listed"
```

```toml
# If the command fails when run in the sandbox, Codex asks for permission to
# retry the command outside the sandbox.
approval_policy = "on-failure"
```

```toml
# User is never prompted: if the command fails, Codex will automatically try
# something out. Note the `exec` subcommand always uses this mode.
approval_policy = "never"
```

## profiles

A _profile_ is a collection of configuration values that can be set together. Multiple profiles can be defined in `config.toml` and you can specify the one you
want to use at runtime via the `--profile` flag.

Here is an example of a `config.toml` that defines multiple profiles:

```toml
model = "o3"
approval_policy = "unless-allow-listed"
sandbox_permissions = ["disk-full-read-access"]
disable_response_storage = false

# Setting `profile` is equivalent to specifying `--profile o3` on the command
# line, though the `--profile` flag can still be used to override this value.
profile = "o3"

[model_providers.openai-chat-completions]
name = "OpenAI using Chat Completions"
base_url = "https://api.openai.com/v1"
env_key = "OPENAI_API_KEY"
wire_api = "chat"

[profiles.o3]
model = "o3"
model_provider = "openai"
approval_policy = "never"

[profiles.gpt3]
model = "gpt-3.5-turbo"
model_provider = "openai-chat-completions"

[profiles.zdr]
model = "o3"
model_provider = "openai"
approval_policy = "on-failure"
disable_response_storage = true
```

Users can specify config values at multiple levels. Order of precedence is as follows:

1. custom command-line argument, e.g., `--model o3`
2. as part of a profile, where the `--profile` is specified via a CLI (or in the config file itself)
3. as an entry in `config.toml`, e.g., `model = "o3"`
4. the default value that comes with Codex CLI (i.e., Codex CLI defaults to `codex-mini-latest`)

## model_reasoning_effort

If the model name starts with `"o"` (as in `"o3"` or `"o4-mini"`) or `"codex"`, reasoning is enabled by default when using the Responses API. As explained in the [OpenAI Platform documentation](https://platform.openai.com/docs/guides/reasoning?api-mode=responses#get-started-with-reasoning), this can be set to:

- `"low"`
- `"medium"` (default)
- `"high"`

To disable reasoning, set `model_reasoning_effort` to `"none"` in your config:

```toml
model_reasoning_effort = "none"  # disable reasoning
```

## model_reasoning_summary

If the model name starts with `"o"` (as in `"o3"` or `"o4-mini"`) or `"codex"`, reasoning is enabled by default when using the Responses API. As explained in the [OpenAI Platform documentation](https://platform.openai.com/docs/guides/reasoning?api-mode=responses#reasoning-summaries), this can be set to:

- `"auto"` (default)
- `"concise"`
- `"detailed"`

To disable reasoning summaries, set `model_reasoning_summary` to `"none"` in your config:

```toml
model_reasoning_summary = "none"  # disable reasoning summaries
```

## sandbox_permissions

List of permissions to grant to the sandbox that Codex uses to execute untrusted commands:

```toml
# This is comparable to --full-auto in the TypeScript Codex CLI, though
# specifying `disk-write-platform-global-temp-folder` adds /tmp as a writable
# folder in addition to $TMPDIR.
sandbox_permissions = [
    "disk-full-read-access",
    "disk-write-platform-user-temp-folder",
    "disk-write-platform-global-temp-folder",
    "disk-write-cwd",
]
```

To add additional writable folders, use `disk-write-folder`, which takes a parameter (this can be specified multiple times):

```toml
sandbox_permissions = [
    # ...
    "disk-write-folder=/Users/mbolin/.pyenv/shims",
]
```

## mcp_servers

Defines the list of MCP servers that Codex can consult for tool use. Currently, only servers that are launched by executing a program that communicate over stdio are supported. For servers that use the SSE transport, consider an adapter like [mcp-proxy](https://github.com/sparfenyuk/mcp-proxy).

**Note:** Codex may cache the list of tools and resources from an MCP server so that Codex can include this information in context at startup without spawning all the servers. This is designed to save resources by loading MCP servers lazily.

This config option is comparable to how Claude and Cursor define `mcpServers` in their respective JSON config files, though because Codex uses TOML for its config language, the format is slightly different. For example, the following config in JSON:

```json
{
  "mcpServers": {
    "server-name": {
      "command": "npx",
      "args": ["-y", "mcp-server"],
      "env": {
        "API_KEY": "value"
      }
    }
  }
}
```

Should be represented as follows in `~/.codex/config.toml`:

```toml
# IMPORTANT: the top-level key is `mcp_servers` rather than `mcpServers`.
[mcp_servers.server-name]
command = "npx"
args = ["-y", "mcp-server"]
env = { "API_KEY" = "value" }
```

## disable_response_storage

Currently, customers whose accounts are set to use Zero Data Retention (ZDR) must set `disable_response_storage` to `true` so that Codex uses an alternative to the Responses API that works with ZDR:

```toml
disable_response_storage = true
```

## shell_environment_policy

Codex spawns subprocesses (e.g. when executing a `local_shell` tool-call suggested by the assistant). By default it passes **only a minimal core subset** of your environment to those subprocesses to avoid leaking credentials. You can tune this behavior via the **`shell_environment_policy`** block in
`config.toml`:

```toml
[shell_environment_policy]
# inherit can be "core" (default), "all", or "none"
inherit = "core"
# set to true to *skip* the filter for `"*KEY*"` and `"*TOKEN*"`
ignore_default_excludes = false
# exclude patterns (case-insensitive globs)
exclude = ["AWS_*", "AZURE_*"]
# force-set / override values
set = { CI = "1" }
# if provided, *only* vars matching these patterns are kept
include_only = ["PATH", "HOME"]
```

| Field                     | Type                       | Default | Description                                                                                                                                     |
| ------------------------- | -------------------------- | ------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `inherit`                 | string                     | `core`  | Starting template for the environment:<br>`core` (`HOME`, `PATH`, `USER`, …), `all` (clone full parent env), or `none` (start empty).           |
| `ignore_default_excludes` | boolean                    | `false` | When `false`, Codex removes any var whose **name** contains `KEY`, `SECRET`, or `TOKEN` (case-insensitive) before other rules run.              |
| `exclude`                 | array&lt;string&gt;        | `[]`    | Case-insensitive glob patterns to drop after the default filter.<br>Examples: `"AWS_*"`, `"AZURE_*"`.                                           |
| `set`                     | table&lt;string,string&gt; | `{}`    | Explicit key/value overrides or additions – always win over inherited values.                                                                   |
| `include_only`            | array&lt;string&gt;        | `[]`    | If non-empty, a whitelist of patterns; only variables that match _one_ pattern survive the final step. (Generally used with `inherit = "all"`.) |

The patterns are **glob style**, not full regular expressions: `*` matches any
number of characters, `?` matches exactly one, and character classes like
`[A-Z]`/`[^0-9]` are supported. Matching is always **case-insensitive**. This
syntax is documented in code as `EnvironmentVariablePattern` (see
`core/src/config_types.rs`).

If you just need a clean slate with a few custom entries you can write:

```toml
[shell_environment_policy]
inherit = "none"
set = { PATH = "/usr/bin", MY_FLAG = "1" }
```

Currently, `CODEX_SANDBOX_NETWORK_DISABLED=1` is also added to the environment, assuming network is disabled. This is not configurable.

## notify

Specify a program that will be executed to get notified about events generated by Codex. Note that the program will receive the notification argument as a string of JSON, e.g.:

```json
{
  "type": "agent-turn-complete",
  "turn-id": "12345",
  "input-messages": ["Rename `foo` to `bar` and update the callsites."],
  "last-assistant-message": "Rename complete and verified `cargo build` succeeds."
}
```

The `"type"` property will always be set. Currently, `"agent-turn-complete"` is the only notification type that is supported.

As an example, here is a Python script that parses the JSON and decides whether to show a desktop push notification using [terminal-notifier](https://github.com/julienXX/terminal-notifier) on macOS:

```python
#!/usr/bin/env python3

import json
import subprocess
import sys


def main() -> int:
    if len(sys.argv) != 2:
        print("Usage: notify.py <NOTIFICATION_JSON>")
        return 1

    try:
        notification = json.loads(sys.argv[1])
    except json.JSONDecodeError:
        return 1

    match notification_type := notification.get("type"):
        case "agent-turn-complete":
            assistant_message = notification.get("last-assistant-message")
            if assistant_message:
                title = f"Codex: {assistant_message}"
            else:
                title = "Codex: Turn Complete!"
            input_messages = notification.get("input_messages", [])
            message = " ".join(input_messages)
            title += message
        case _:
            print(f"not sending a push notification for: {notification_type}")
            return 0

    subprocess.check_output(
        [
            "terminal-notifier",
            "-title",
            title,
            "-message",
            message,
            "-group",
            "codex",
            "-ignoreDnD",
            "-activate",
            "com.googlecode.iterm2",
        ]
    )

    return 0


if __name__ == "__main__":
    sys.exit(main())
```

To have Codex use this script for notifications, you would configure it via `notify` in `~/.codex/config.toml` using the appropriate path to `notify.py` on your computer:

```toml
notify = ["python3", "/Users/mbolin/.codex/notify.py"]
```

## history

By default, Codex CLI records messages sent to the model in `$CODEX_HOME/history.jsonl`. Note that on UNIX, the file permissions are set to `o600`, so it should only be readable and writable by the owner.

To disable this behavior, configure `[history]` as follows:

```toml
[history]
persistence = "none"  # "save-all" is the default value
```

## file_opener

Identifies the editor/URI scheme to use for hyperlinking citations in model output. If set, citations to files in the model output will be hyperlinked using the specified URI scheme so they can be ctrl/cmd-clicked from the terminal to open them.

For example, if the model output includes a reference such as `【F:/home/user/project/main.py†L42-L50】`, then this would be rewritten to link to the URI `vscode://file/home/user/project/main.py:42`.

Note this is **not** a general editor setting (like `$EDITOR`), as it only accepts a fixed set of values:

- `"vscode"` (default)
- `"vscode-insiders"`
- `"windsurf"`
- `"cursor"`
- `"none"` to explicitly disable this feature

Currently, `"vscode"` is the default, though Codex does not verify VS Code is installed. As such, `file_opener` may default to `"none"` or something else in the future.

## hide_agent_reasoning

Codex intermittently emits "reasoning" events that show the model’s internal "thinking" before it produces a final answer. Some users may find these events distracting, especially in CI logs or minimal terminal output.

Setting `hide_agent_reasoning` to `true` suppresses these events in **both** the TUI as well as the headless `exec` sub-command:

```toml
hide_agent_reasoning = true   # defaults to false
```

## project_doc_max_bytes

Maximum number of bytes to read from an `AGENTS.md` file to include in the instructions sent with the first turn of a session. Defaults to 32 KiB.

## tui

Options that are specific to the TUI.

```toml
[tui]
# This will make it so that Codex does not try to process mouse events, which
# means your Terminal's native drag-to-text to text selection and copy/paste
# should work. The tradeoff is that Codex will not receive any mouse events, so
# it will not be possible to use the mouse to scroll conversation history.
#
# Note that most terminals support holding down a modifier key when using the
# mouse to support text selection. For example, even if Codex mouse capture is
# enabled (i.e., this is set to `false`), you can still hold down alt while
# dragging the mouse to select text.
disable_mouse_capture = true  # defaults to `false`
```
# Azure DevOps Configuration

Codex CLI now supports integration with Azure DevOps. You can configure it in your `~/.codex/config.toml` file:

```toml
[azure_devops]
# Required: Azure DevOps organization URL
organization_url = "https://dev.azure.com/your-organization"

# Optional: Personal Access Token for authentication
# Not recommended to set directly in config file for security reasons
# pat = "your-pat-here"

# Optional: Environment variable name that contains the PAT
# This is the recommended way to provide authentication
pat_env_var = "AZURE_DEVOPS_PAT"

# Optional: Default project to use when not specified in commands
default_project = "YourProject"

# Optional: API version to use (defaults to "7.0")
# api_version = "7.0"
```

## Authentication

For security reasons, it's recommended to set your Personal Access Token (PAT) as an environment variable rather than directly in the config file:

```bash
export AZURE_DEVOPS_PAT="your-pat-here"
```

## Creating a Personal Access Token

1. Go to your Azure DevOps organization: `https://dev.azure.com/your-organization`
2. Click on your profile picture in the top right
3. Select "Personal access tokens"
4. Click "New Token"
5. Give it a name and select the following scopes:
   - Work Items: Read & Write
   - Code: Read & Write
   - Pull Request Threads: Read & Write
   - Wiki: Read & Write
   - Build: Read & Execute
6. Click "Create" and copy the token

## Available Tools

When Azure DevOps is configured, the following tools become available to the model:

- `azure_devops_query_work_items`: Search for work items using WIQL
- `azure_devops_get_work_item`: Get details of a specific work item
- `azure_devops_create_work_item`: Create a new work item
- `azure_devops_update_work_item`: Update an existing work item
- `azure_devops_query_pull_requests`: Search for pull requests
- `azure_devops_get_pull_request`: Get details of a specific pull request
- `azure_devops_comment_on_pull_request`: Add a comment to a pull request
- `azure_devops_get_wiki_page`: Get content of a wiki page
- `azure_devops_update_wiki_page`: Update content of a wiki page
- `azure_devops_run_pipeline`: Run a pipeline
- `azure_devops_get_pipeline_status`: Get status of a pipeline run# Azure DevOps Configuration

Codex CLI supports integration with Azure DevOps services, allowing you to interact with work items, pull requests, wikis, and pipelines.

## Configuration

Add the following to your `~/.codex/config.toml` file:

```toml
[azure_devops]
# Required: URL of your Azure DevOps organization
organization_url = "https://dev.azure.com/your-organization"

# Authentication: Use one of the following options
# Option 1: Environment variable containing your PAT (recommended)
pat_env_var = "AZURE_DEVOPS_PAT"
# Option 2: Directly specify the PAT (less secure)
# pat = "your-personal-access-token"

# Optional: Default project to use when not specified in commands
default_project = "YourProject"

# Optional: API version to use (defaults to "7.0")
# api_version = "7.0"
```

## Authentication

You need to create a Personal Access Token (PAT) in Azure DevOps with the following permissions:

- Work Items: Read & Write
- Code: Read & Write
- Pull Request Threads: Read & Write
- Wiki: Read & Write
- Build: Read & Execute

## Available Tools

When Azure DevOps is configured, the following tools become available to the AI:

- `azure_devops_query_work_items`: Search for work items using WIQL
- `azure_devops_get_work_item`: Get details of a specific work item
- `azure_devops_create_work_item`: Create a new work item
- `azure_devops_update_work_item`: Update an existing work item
- `azure_devops_query_pull_requests`: Search for pull requests
- `azure_devops_get_pull_request`: Get details of a specific pull request
- `azure_devops_comment_on_pull_request`: Add a comment to a pull request
- `azure_devops_get_wiki_page`: Get content of a wiki page
- `azure_devops_update_wiki_page`: Update content of a wiki page
- `azure_devops_run_pipeline`: Run a pipeline
- `azure_devops_get_pipeline_status`: Get status of a pipeline run

For more details on using these tools, see the [Azure DevOps Usage Guide](../AZURE_DEVOPS_USAGE.md).