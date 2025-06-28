//! Default implementation for Config

use std::collections::HashMap;
use std::path::PathBuf;

use crate::config::Config;
use crate::config_types::{History, ReasoningEffort, ReasoningSummary, Tui, UriBasedFileOpener};
use crate::flags::OPENAI_DEFAULT_MODEL;
use crate::model_provider_info::built_in_model_providers;
use crate::protocol::{AskForApproval, SandboxPolicy};

impl Default for Config {
    fn default() -> Self {
        let model_providers = built_in_model_providers();
        let model_provider_id = "openai".to_string();
        let model_provider = model_providers.get(&model_provider_id).cloned().unwrap_or_default();
        
        Self {
            model: OPENAI_DEFAULT_MODEL.to_string(),
            model_provider_id,
            model_provider,
            approval_policy: AskForApproval::default(),
            sandbox_policy: SandboxPolicy::new_read_only_policy(),
            shell_environment_policy: Default::default(),
            disable_response_storage: false,
            instructions: None,
            notify: None,
            cwd: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            mcp_servers: HashMap::new(),
            model_providers,
            project_doc_max_bytes: crate::config::PROJECT_DOC_MAX_BYTES,
            codex_home: dirs::home_dir().map_or_else(
                || PathBuf::from("./.codex"),
                |mut p| {
                    p.push(".codex");
                    p
                },
            ),
            history: History::default(),
            file_opener: UriBasedFileOpener::VsCode,
            tui: Tui::default(),
            codex_linux_sandbox_exe: None,
            hide_agent_reasoning: false,
            model_reasoning_effort: ReasoningEffort::default(),
            model_reasoning_summary: ReasoningSummary::default(),
            azure_devops: None,
            kusto: None,
            recovery_services: None,
            model_context_window: None,
            model_max_output_tokens: None,
        }
    }
}