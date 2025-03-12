impl ImplMode {
    PUB(SUPER) FN FN_ASYNC(&SELF) -> IMPL FMT::dISPLAY {
        MATCH SELF {
            ImplMode::Sync => "",
            ImplMode::Async => "async ",
        }
    }
}
