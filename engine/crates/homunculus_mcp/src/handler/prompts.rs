//! MCP prompt definitions.

use rmcp::model::{GetPromptResult, Prompt, PromptArgument, PromptMessage, PromptMessageRole};

/// Returns the list of prompts exposed by this MCP server.
pub(super) fn prompt_definitions() -> Vec<Prompt> {
    vec![
        Prompt::new(
            "developer-assistant",
            Some("Generate appropriate character reactions for development events"),
            Some(vec![
                PromptArgument::new("event")
                    .with_description(
                        "Development event: build-success, build-failure, test-pass, \
                     test-fail, git-push, git-commit, deploy",
                    )
                    .with_required(true),
            ]),
        ),
        Prompt::new(
            "character-interaction",
            Some("Have a natural interaction with the desktop character"),
            Some(vec![
                PromptArgument::new("message")
                    .with_description("What to say or do")
                    .with_required(true),
                PromptArgument::new("mood")
                    .with_description("Desired mood: happy, playful, serious, encouraging")
                    .with_required(false),
            ]),
        ),
        Prompt::new(
            "mod-command-helper",
            Some("Discover and execute MOD commands"),
            Some(vec![
                PromptArgument::new("mod_name")
                    .with_description("MOD name to explore")
                    .with_required(true),
            ]),
        ),
    ]
}

/// Extracts a string value from the JSON arguments map.
fn get_string_arg(args: &serde_json::Map<String, serde_json::Value>, key: &str) -> Option<String> {
    args.get(key).and_then(|v| v.as_str()).map(String::from)
}

/// Resolves a prompt by name, substituting the given arguments.
pub(super) fn get_prompt(
    name: &str,
    args: &serde_json::Map<String, serde_json::Value>,
) -> Result<GetPromptResult, rmcp::ErrorData> {
    let text = match name {
        "developer-assistant" => {
            let event = get_string_arg(args, "event").unwrap_or_default();
            format!(
                "A development event occurred: \"{event}\". \
                 Use the play_reaction tool to make the desktop character react appropriately. \
                 Choose the best reaction from: happy, sad, confused, error, success, thinking, \
                 surprised, neutral. \
                 For success events use \"success\" or \"happy\". \
                 For failures use \"error\" or \"sad\". \
                 For uncertain outcomes use \"thinking\" or \"confused\"."
            )
        }
        "character-interaction" => {
            let message = get_string_arg(args, "message").unwrap_or_default();
            let mood_part = get_string_arg(args, "mood")
                .map(|m| format!(" Mood: {m}."))
                .unwrap_or_default();
            format!(
                "Interact with the desktop character. Message: \"{message}\".{mood_part} \
                 First use get_character_snapshot to check the current state, \
                 then use play_reaction for the appropriate expression, \
                 and optionally speak_message if the character should say something aloud."
            )
        }
        "mod-command-helper" => {
            let mod_name = get_string_arg(args, "mod_name").unwrap_or_default();
            format!(
                "Help me use the \"{mod_name}\" MOD. \
                 First, read the homunculus://mods resource to find available commands for \
                 this MOD. \
                 Then explain what each command does and how to use it with the execute_command \
                 tool. \
                 Show example execute_command calls with proper arguments."
            )
        }
        _ => {
            return Err(rmcp::ErrorData::invalid_params(
                format!("Unknown prompt: {name}"),
                None,
            ));
        }
    };

    Ok(GetPromptResult::new(vec![PromptMessage::new_text(
        PromptMessageRole::User,
        text,
    )]))
}
