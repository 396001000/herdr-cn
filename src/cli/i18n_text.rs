//! CLI 帮助文本的本地化收口点。
//!
//! 上游 `refactor: share cli and api command contracts (#5212)` 把命令帮助挪进了
//! `const LEAVES: &[Leaf]` 之类的常量表；`rust_i18n::t!()` 要求字面量键、也不能出现在
//! 常量初始化里，所以这里改成在**渲染处**按「英文原文 → locale 键」反查翻译：
//!
//! - 上游的常量表与英文字面量一个字都不改，以后同步上游不会在这些表里撞冲突；
//! - 表里没有的串（上游新增或改词）自动回退英文，既不编译失败也不会翻错；
//! - 默认 locale 是 en，而 en.yml 里 `cli.*` 的值就是这些英文原文，因此非中文用户
//!   与单元/契约测试看到的输出与改动前逐字相同。
//!
//! 本文件由 locales/en.yml 的 `cli.*` 词条生成（旧版 spec.rs 用到、现已成孤儿的键），
//! 词条有增改时重新生成即可。

use rust_i18n::t;

/// 把上游 CLI 英文字面量映射到汉化词条；未收录的原样返回。
pub(crate) fn localize(text: &str) -> String {
    match text {
        "Attach directly to a terminal stream" => t!("cli.terminal_attach_about").to_string(),
        "Attach directly to an agent terminal" => t!("cli.agent_attach_about").to_string(),
        "Attach to a session" => t!("cli.session_attach_about").to_string(),
        "Attach to or observe raw terminal streams" => t!("cli.terminal_about").to_string(),
        "Clear the outer terminal title" => t!("cli.terminal_clear_about").to_string(),
        "Close a pane" => t!("cli.pane_close_about").to_string(),
        "Close a plugin pane" => t!("cli.plugin_pane_close_about").to_string(),
        "Close a tab" => t!("cli.tab_close_about").to_string(),
        "Close a workspace" => t!("cli.workspace_close_about").to_string(),
        "Control a terminal stream" => t!("cli.terminal_control_about").to_string(),
        "Create a tab" => t!("cli.tab_create_about").to_string(),
        "Create a workspace" => t!("cli.workspace_create_about").to_string(),
        "Create and open a Git worktree" => t!("cli.worktree_create_about").to_string(),
        "Delete a stopped session" => t!("cli.session_delete_about").to_string(),
        "Disable a plugin" => t!("cli.plugin_disable_about").to_string(),
        "Enable a plugin" => t!("cli.plugin_enable_about").to_string(),
        "Existing pane at an interactive shell prompt" => t!("cli.agent_start_pane_help").to_string(),
        "Explain agent detection state" => t!("cli.agent_explain_about").to_string(),
        "Fail after this many milliseconds" => t!("cli.agent_wait_timeout_help").to_string(),
        "Fetch and reload agent detection manifests" => t!("cli.server_update_agent_manifests_about").to_string(),
        "Find a pane neighbor" => t!("cli.pane_neighbor_about").to_string(),
        "Focus a neighboring pane" => t!("cli.pane_focus_about").to_string(),
        "Focus a plugin pane" => t!("cli.plugin_pane_focus_about").to_string(),
        "Focus a tab" => t!("cli.tab_focus_about").to_string(),
        "Focus a workspace" => t!("cli.workspace_focus_about").to_string(),
        "Focus an agent" => t!("cli.agent_focus_about").to_string(),
        "Inspect plugin command logs" => t!("cli.plugin_log_about").to_string(),
        "Install a plugin from GitHub" => t!("cli.plugin_install_about").to_string(),
        "Install an integration" => t!("cli.integration_install_about").to_string(),
        "Install and run workflow plugins" => t!("cli.plugin_about").to_string(),
        "Invoke a plugin action" => t!("cli.plugin_action_invoke_about").to_string(),
        "Keep ANSI escape sequences while matching" => t!("cli.pane_match_raw_help").to_string(),
        "Link a local plugin" => t!("cli.plugin_link_about").to_string(),
        "List agents" => t!("cli.agent_list_about").to_string(),
        "List installed plugins" => t!("cli.plugin_list_about").to_string(),
        "List or invoke plugin actions" => t!("cli.plugin_action_about").to_string(),
        "List panes" => t!("cli.pane_list_about").to_string(),
        "List plugin actions" => t!("cli.plugin_action_list_about").to_string(),
        "List plugin command logs" => t!("cli.plugin_log_list_about").to_string(),
        "List sessions" => t!("cli.session_list_about").to_string(),
        "List tabs" => t!("cli.tab_list_about").to_string(),
        "List workspaces" => t!("cli.workspace_list_about").to_string(),
        "List worktree workspaces" => t!("cli.worktree_list_about").to_string(),
        "Manage local configuration" => t!("cli.config_about").to_string(),
        "Manage plugin-owned panes" => t!("cli.plugin_pane_about").to_string(),
        "Manage the outer terminal title" => t!("cli.terminal_title_about").to_string(),
        "Match a Rust regular expression" => t!("cli.pane_match_regex_help").to_string(),
        "Match a literal substring" => t!("cli.pane_match_literal_help").to_string(),
        "Move a pane" => t!("cli.pane_move_about").to_string(),
        "Observe a terminal stream" => t!("cli.terminal_observe_about").to_string(),
        "Open a plugin pane" => t!("cli.plugin_pane_open_about").to_string(),
        "Open an existing Git worktree" => t!("cli.worktree_open_about").to_string(),
        "Print a plugin config directory" => t!("cli.plugin_config_dir_about").to_string(),
        "Print or write the bundled API schema" => t!("cli.api_schema_about").to_string(),
        "Print the configured update channel" => t!("cli.channel_show_about").to_string(),
        "Print the live session snapshot" => t!("cli.api_snapshot_about").to_string(),
        "Read agent terminal output" => t!("cli.agent_read_about").to_string(),
        "Read pane terminal output" => t!("cli.pane_read_about").to_string(),
        "Release pane agent lifecycle authority" => t!("cli.pane_release_agent_about").to_string(),
        "Reload local agent detection manifest overrides" => t!("cli.server_reload_agent_manifests_about").to_string(),
        "Remove a worktree checkout" => t!("cli.worktree_remove_about").to_string(),
        "Rename a pane" => t!("cli.pane_rename_about").to_string(),
        "Rename a tab" => t!("cli.tab_rename_about").to_string(),
        "Rename a workspace" => t!("cli.workspace_rename_about").to_string(),
        "Rename an agent" => t!("cli.agent_rename_about").to_string(),
        "Report display-only pane metadata" => t!("cli.pane_report_metadata_about").to_string(),
        "Report display-only workspace metadata" => t!("cli.workspace_report_metadata_about").to_string(),
        "Report pane agent lifecycle state" => t!("cli.pane_report_agent_about").to_string(),
        "Report pane agent session identity" => t!("cli.pane_report_agent_session_about").to_string(),
        "Resize a pane split" => t!("cli.pane_resize_about").to_string(),
        "Restrict the searched snapshot to N lines" => t!("cli.pane_match_lines_help").to_string(),
        "Run a command in a pane" => t!("cli.pane_run_about").to_string(),
        "Send key presses to a pane" => t!("cli.pane_send_keys_about").to_string(),
        "Send key presses to an agent" => t!("cli.agent_send_keys_about").to_string(),
        "Send literal text to a pane" => t!("cli.pane_send_text_about").to_string(),
        "Set an environment variable for the launched process" => t!("cli.env_help").to_string(),
        "Set pane input routing" => t!("cli.pane_input_about").to_string(),
        "Set the outer terminal title" => t!("cli.terminal_set_about").to_string(),
        "Shell to generate completions for" => t!("cli.completion_shell_help").to_string(),
        "Show a notification" => t!("cli.notification_show_about").to_string(),
        "Show a pane" => t!("cli.pane_get_about").to_string(),
        "Show a tab" => t!("cli.tab_get_about").to_string(),
        "Show a workspace" => t!("cli.workspace_get_about").to_string(),
        "Show active agent detection manifests" => t!("cli.server_agent_manifests_about").to_string(),
        "Show an agent" => t!("cli.agent_get_about").to_string(),
        "Show integration status" => t!("cli.integration_status_about").to_string(),
        "Show local client status" => t!("cli.status_client_about").to_string(),
        "Show pane edge information" => t!("cli.pane_edges_about").to_string(),
        "Show pane layout information" => t!("cli.pane_layout_about").to_string(),
        "Show pane process information" => t!("cli.pane_process_info_about").to_string(),
        "Show running server status" => t!("cli.status_server_about").to_string(),
        "Show the current pane" => t!("cli.pane_current_about").to_string(),
        "Split a pane" => t!("cli.pane_split_about").to_string(),
        "Start a supported interactive agent in an existing pane" => t!("cli.agent_start_interactive_about").to_string(),
        "Stop a session" => t!("cli.session_stop_about").to_string(),
        "Submit a prompt to an agent" => t!("cli.agent_prompt_about").to_string(),
        "Supported agent kind and canonical executable" => t!("cli.agent_start_kind_help").to_string(),
        "Swap panes" => t!("cli.pane_swap_about").to_string(),
        "Toggle or set pane zoom" => t!("cli.pane_zoom_about").to_string(),
        "Try live handoff after installing" => t!("cli.update_handoff_help").to_string(),
        "Uninstall a plugin" => t!("cli.plugin_uninstall_about").to_string(),
        "Uninstall an integration" => t!("cli.integration_uninstall_about").to_string(),
        "Unlink a local plugin" => t!("cli.plugin_unlink_about").to_string(),
        "Validate config.toml and print diagnostics" => t!("cli.config_check_about").to_string(),
        "Wait for matching pane output" => t!("cli.wait_output_about").to_string(),
        "Wait for the first matching state observed after submission" => t!("cli.agent_prompt_wait_help").to_string(),
        "Wait until an agent reaches one of the requested states" => t!("cli.agent_wait_states_about").to_string(),
        "Work with terminal sessions" => t!("cli.terminal_session_about").to_string(),
        _ => text.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::localize;
    use clap::Command;

    /// 中文确实生效、英文与未收录串都原样回退——整套收口机制的护栏。
    /// nextest 每用例独立进程，set locale 不会污染别的用例。
    #[test]
    fn cli_help_translates_under_zh_and_falls_back_otherwise() {
        crate::i18n::apply_locale("en");
        assert_eq!(localize("List agents"), "List agents");
        crate::i18n::apply_locale("zh");
        assert_eq!(localize("List agents"), "列出智能体");
        assert_eq!(localize("List panes"), "列出窗格");
        assert_eq!(
            localize("Some brand new upstream wording"),
            "Some brand new upstream wording"
        );
    }

    fn walk(command: &Command, gaps: &mut Vec<String>) {
        if let Some(about) = command.get_about() {
            let text = about.to_string();
            // 有译文、却还是英文 → 这个构建点绕过了 localize，接线漏了
            if text.is_ascii() && localize(&text) != text {
                gaps.push(format!("{}: {}", command.get_name(), text));
            }
        }
        for sub in command.get_subcommands() {
            walk(sub, gaps);
        }
    }

    /// 遍历整棵命令树：凡是有译文可用却没被翻译的描述，一律算接线缺口。
    #[test]
    fn every_translatable_cli_description_actually_goes_through_localize() {
        crate::i18n::apply_locale("zh");
        let mut root = super::super::spec::command();
        root.build_self();
        let mut gaps = Vec::new();
        walk(&root, &mut gaps);
        assert!(
            gaps.is_empty(),
            "以下描述有译文却没走 localize（构建点绕过了收口）：\n{}",
            gaps.join("\n")
        );
    }
}
