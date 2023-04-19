/// Get the julia triple & sha256 corresponding to the prebuilt chemfiles v0.10.3
/// for a given rust triple, if it exists
pub fn get_prebuilt_info(target: &str) -> Option<(&'static str, &'static str)> {
    match target {
        "aarch64-apple-darwin" => Some((
            "aarch64-apple-darwin",
            "d363921687c3d9a292a680567f3244cd1159f2c17a4b897361d7703a954b3d31",
        )),
        "aarch64-unknown-linux-gnu" => Some((
            "aarch64-linux-gnu",
            "503b5a384cbc627d6dfa0e3c38923fd4b881619aa2c58d8d37257fb00d593b73",
        )),
        "aarch64-unknown-linux-musl" => Some((
            "aarch64-linux-musl",
            "26da29259916f1b57c567a7e4ad72fa113065d69b2c131a0de92ed77bdc1c943",
        )),
        "armv7-unknown-linux-gnueabihf" => Some((
            "armv7l-linux-gnueabihf",
            "14e950a8e10c2e1e3408c9fbfee0da463a978f3c139fc9ddb6f79323c7eab48d",
        )),
        "armv7-unknown-linux-musleabihf" => Some((
            "armv7l-linux-musleabihf",
            "444d30b2c82c1ba9f9be2c0ba5092b3d86bfbb02f995935490ab12bea0767423",
        )),
        "i686-unknown-linux-gnu" => Some((
            "i686-linux-gnu",
            "d4d4833269b7589e8035b5718236c40477969c3c281b20cdb3a9f09eeb291396",
        )),
        "i686-unknown-linux-musl" => Some((
            "i686-linux-musl",
            "b8914c51fe06fd1848ceaf0b5666e05ff4b1dd3edb05dea26b9bbc99d51426e3",
        )),
        "i686-pc-windows-gnu" => Some((
            "i686-w64-mingw32",
            "f4cae7e8fcca7d32e539f9c080dfd950cf5b835ffa3d75aae7dd16f597829573",
        )),
        "powerpc64le-unknown-linux-gnu" => Some((
            "powerpc64le-linux-gnu",
            "af7adf574ddc953593dda2e89b39d71c6674e44cd8a3fc729edc1f48b9f01542",
        )),
        "x86_64-apple-darwin" => Some((
            "x86_64-apple-darwin",
            "0783cb15705576f4b5af6bed583540a256a83a906d160dc6f157b7d5f594579b",
        )),
        "x86_64-unknown-linux-gnu" => Some((
            "x86_64-linux-gnu",
            "0a2a839885effae7df0c279ad024b6fa7cb73666c2c2fe9b6675cebc614b088e",
        )),
        "x86_64-unknown-linux-musl" => Some((
            "x86_64-linux-musl",
            "866edb9fb201e05b24c04d115ba9cb27e5381f94bf5637c7409e43b4513ba4a4",
        )),
        "x86_64-unknown-freebsd" => Some((
            "x86_64-unknown-freebsd",
            "51f659041fb65f43038541dabb2dfa47712bb0ec08ea57065954e42334d9f797",
        )),
        "x86_64-pc-windows-gnu" => Some((
            "x86_64-w64-mingw32",
            "e183dfbef91e5ce608be7ae8c3ca902982ab0f5b7276d4049958cbc75c0155c6",
        )),
        _ => None,
    }
}
