error_chain! {

    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Nix(::nix::Error);
    }

    errors {}
}
