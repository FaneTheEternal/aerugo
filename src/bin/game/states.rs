#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MainState {
    MainMenu,
    InGame,
    OnLoad,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum OverlayState {
    Hidden,
    Menu,
    Settings,
    Save,
    Load,
}
