use clap::{Args, Subcommand};
use homunculus_utils::error::UtilResult;

#[derive(Args)]
pub struct ModsArgs {
    #[command(subcommand)]
    pub command: ModsSubcommand,
}

/// Available operations for managing installed mods.
#[derive(Subcommand)]
pub enum ModsSubcommand {
    /// List installed mods.
    List,
    /// Install the MOD.
    Install {
        /// The name of the MOD package.
        /// Internally, it is used as the argument for `pnpm add <pkg>`.
        #[arg(required = true)]
        pkg: Vec<String>,
    },
    /// Uninstall the MOD.
    Uninstall {
        #[arg(required = true)]
        mod_names: Vec<String>,
    },
}

impl ModsArgs {
    pub fn execute(self) -> UtilResult {
        match self.command {
            ModsSubcommand::List => output_installation_mods(),
            ModsSubcommand::Install { pkg } => homunculus_utils::mods::install(&pkg),
            ModsSubcommand::Uninstall { mod_names } => {
                homunculus_utils::mods::uninstall(&mod_names)
            }
        }
    }
}

fn output_installation_mods() -> UtilResult {
    let mods = homunculus_utils::mods::list::list_installation_mods()?;
    if mods.is_empty() {
        return Ok(());
    }

    let mut table = comfy_table::Table::new();
    table
        .load_preset(comfy_table::presets::NOTHING)
        .set_header(["NAME", "VERSION", "DESCRIPTION"]);

    for m in &mods {
        table.add_row([&m.name, &m.version, m.description.as_deref().unwrap_or("")]);
    }

    println!("{table}");
    Ok(())
}
