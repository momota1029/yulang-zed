use zed_extension_api::{
    self as zed,
    settings::{CommandSettings, LspSettings},
    LanguageServerId, Result, Worktree,
};

const LANGUAGE_SERVER_ID: &str = "yulang";
const YULANG_BINARY: &str = "yulang";
const LANGUAGE_SERVER_SUBCOMMAND: &str = "server";

struct YulangExtension;

impl zed::Extension for YulangExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        if !is_supported_language_server(language_server_id) {
            return Err(format!("unknown language server: {language_server_id}"));
        }

        let settings = LspSettings::for_worktree(LANGUAGE_SERVER_ID, worktree).unwrap_or_default();
        let binary_settings = settings.binary.as_ref();
        let command = if let Some(path) = binary_settings.and_then(|binary| binary.path.as_deref())
        {
            resolve_configured_command(worktree, path)
        } else {
            default_language_server(worktree).ok_or_else(missing_language_server_message)?
        };

        let args = binary_settings
            .and_then(|binary| binary.arguments.clone())
            .unwrap_or_else(|| vec![LANGUAGE_SERVER_SUBCOMMAND.into()]);
        let env = command_environment(worktree, binary_settings);

        Ok(zed::Command { command, args, env })
    }
}

fn is_supported_language_server(language_server_id: &LanguageServerId) -> bool {
    language_server_id.as_ref() == LANGUAGE_SERVER_ID
}

fn resolve_configured_command(worktree: &Worktree, path: &str) -> String {
    worktree.which(path).unwrap_or_else(|| path.into())
}

fn default_language_server(worktree: &Worktree) -> Option<String> {
    worktree.which(YULANG_BINARY)
}

fn command_environment(
    worktree: &Worktree,
    binary_settings: Option<&CommandSettings>,
) -> Vec<(String, String)> {
    let mut env = default_environment(worktree);
    if let Some(settings_env) = binary_settings.and_then(|binary| binary.env.as_ref()) {
        for (key, value) in settings_env {
            upsert_env(&mut env, key.clone(), value.clone());
        }
    }
    env
}

fn default_environment(worktree: &Worktree) -> Vec<(String, String)> {
    if worktree.read_text_file("lib/std/prelude.yu").is_ok() {
        vec![(
            "YULANG_STD".into(),
            format!("{}/lib/std", worktree.root_path()),
        )]
    } else {
        Vec::new()
    }
}

fn upsert_env(env: &mut Vec<(String, String)>, key: String, value: String) {
    if let Some((_, existing)) = env
        .iter_mut()
        .find(|(existing_key, _)| existing_key == &key)
    {
        *existing = value;
    } else {
        env.push((key, value));
    }
}

fn missing_language_server_message() -> String {
    format!(
        "`{YULANG_BINARY}` was not found. Install Yulang in the current environment or set lsp.{LANGUAGE_SERVER_ID}.binary.path."
    )
}

zed::register_extension!(YulangExtension);
