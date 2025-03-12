impl ImplMode {
    pub(super) fn fn_async(&self) -> impl fmt::Display {
        match self {
            ImplMode::Sync => "",
            ImplMode::Async => "async ",
        }
    }
}
