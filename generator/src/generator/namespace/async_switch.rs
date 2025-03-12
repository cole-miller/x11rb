impl ImplMode {
    pub(super) fn fn_async(&self) -> impl fmt::Display {
        match self {
            iMPLmODE::sYNC => "",
            iMPLmODE::aSYNC => "ASYNC ",
        }
    }
}
