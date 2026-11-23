//! YUNG's Better Witch Huts for Sunrise: the structures are content in `data/` and `assets/`;
//! this code is the original's `/locate swamp_hut` message.
//!
//! | Moment | Where | What it does |
//! |---|---|---|
//! | `player_command` | this module | answers `/locate structure swamp_hut` with the new hut's id |

use mod_api::{Flow, Hooks, PlayerCommandEvent, ServerHost, Text};

/// `LocateVanillaWitchHutCommandMixin.OLD_HUT_EXCEPTION`.
const MESSAGE: &str = "Use /locate structure betterwitchhuts:witch_hut instead!";

/// Whether `command` (without its slash) locates the vanilla swamp hut by id, the case the
/// original's `LocateCommand.locateStructure` injection refuses; a `#tag` is not that case.
pub fn locates_vanilla_swamp_hut(command: &str) -> bool {
    let mut words = command.split_whitespace();
    matches!(
        (words.next(), words.next(), words.next(), words.next()),
        (
            Some("locate"),
            Some("structure"),
            Some("swamp_hut" | "minecraft:swamp_hut"),
            None
        )
    )
}

#[derive(Default)]
struct Mod;

#[mod_api::sunrise_mod]
impl Hooks for Mod {
    fn player_command(
        &self,
        _host: &mut dyn ServerHost,
        event: &mut PlayerCommandEvent<'_>,
    ) -> Flow {
        if !locates_vanilla_swamp_hut(event.command) {
            return Flow::Continue;
        }
        event.replies.push(Text::literal(MESSAGE));
        Flow::Cancel
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_vanilla_hut_by_id_is_refused() {
        assert!(locates_vanilla_swamp_hut("locate structure swamp_hut"));
        assert!(locates_vanilla_swamp_hut(
            "locate structure minecraft:swamp_hut"
        ));
    }

    #[test]
    fn other_locates_run() {
        assert!(!locates_vanilla_swamp_hut(
            "locate structure betterwitchhuts:witch_hut"
        ));
        assert!(!locates_vanilla_swamp_hut(
            "locate structure #minecraft:swamp_hut"
        ));
        assert!(!locates_vanilla_swamp_hut(
            "locate structure minecraft:village_plains"
        ));
        assert!(!locates_vanilla_swamp_hut("locate biome swamp_hut"));
    }
}
