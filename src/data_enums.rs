#[derive(PartialEq, Default, Clone, Copy)]
pub enum CustomId {
    PickGamesAdd,
    PickGamesRemove,
    PickGamesAddExecute,
    PickGamesRemoveExecute,
    GuildApply,
    #[default]
    Invalid,
}

impl CustomId {
    pub fn new(custom_id: &str) -> Self {
        match custom_id {
            "pick-games-add" => Self::PickGamesAdd,
            "pick-games-remove" => Self::PickGamesRemove,
            "pick-games-add-execute" => Self::PickGamesAddExecute,
            "pick-games-remove-execute" => Self::PickGamesRemoveExecute,
            "guild-apply" => Self::GuildApply,
            _ => Self::Invalid,
        }
    }
}

impl std::fmt::Display for CustomId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_output: String = match *self {
            Self::PickGamesAdd => "pick-games-add".into(),
            Self::PickGamesRemove => "pick-games-remove".into(),
            Self::PickGamesAddExecute => "pick-games-add-execute".into(),
            Self::PickGamesRemoveExecute => "pick-games-remove-execute".into(),
            Self::GuildApply => "guild-apply".into(),
            Self::Invalid => "__invalid__".into(),
        };

        write!(f, "{}", display_output)
    }
}
