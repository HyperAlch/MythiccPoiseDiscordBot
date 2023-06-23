#[derive(PartialEq, Default, Clone)]
pub enum CustomId {
    PickGamesAdd,
    PickGamesRemove,
    #[default]
    Invalid,
}

impl CustomId {
    pub fn new(custom_id: &str) -> Self {
        match custom_id {
            "pick-games-add" => Self::PickGamesAdd,
            "pick-games-remove" => Self::PickGamesRemove,
            _ => CustomId::Invalid,
        }
    }
}

impl std::fmt::Display for CustomId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_output: String = match *self {
            CustomId::PickGamesAdd => "pick-games-add".into(),
            CustomId::PickGamesRemove => "pick-games-remove".into(),
            CustomId::Invalid => "__invalid__".into(),
        };

        write!(f, "{}", display_output)
    }
}
