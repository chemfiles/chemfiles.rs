/// Get the julia triple & sha256 corresponding to the prebuilt chemfiles v0.10.4
/// for a given rust triple, if it exists
pub fn get_prebuilt_info(target: &str) -> Option<(&'static str, &'static str)> {
    match target {
        "aarch64-apple-darwin" => Some((
            "aarch64-apple-darwin",
            "fa31fd1c43fbf3954b00013f756c22f082d373f768aac853fad01142fedfce5d",
        )),
        "aarch64-unknown-linux-gnu" => Some((
            "aarch64-linux-gnu",
            "39c7d72a8a55783635783dee6777a9ada40318b95acdd2b8b072df23aec6c80f",
        )),
        "aarch64-unknown-linux-musl" => Some((
            "aarch64-linux-musl",
            "8a475ff74178781862a08f1fc219f64421c46477450d7f5a38b021e4736554bf",
        )),
        "armv7-unknown-linux-gnueabihf" => Some((
            "armv7l-linux-gnueabihf",
            "483c742b5b2121c976af0d1bb4d7faff79f4ca264213eab057c25969193735b0",
        )),
        "armv7-unknown-linux-musleabihf" => Some((
            "armv7l-linux-musleabihf",
            "eea196c4a27eb1078dd4bed62a1c80043b57bf86ceee4401e265279a418f9c71",
        )),
        "i686-unknown-linux-gnu" => Some((
            "i686-linux-gnu",
            "6b6c983dbc8603fd9dca4c865c4f5b78f694fcfc1857fae652d4efd1176f9422",
        )),
        "i686-unknown-linux-musl" => Some((
            "i686-linux-musl",
            "8ab6dc7e548d5db8988cd57afee28d45e6abce087630b705fa502587d0c94fe5",
        )),
        "i686-pc-windows-gnu" => Some((
            "i686-w64-mingw32",
            "4eda8baf539a83b7d58eda0ea5da520581040969c83dfe26135853dd6f21a1c1",
        )),
        "powerpc64le-unknown-linux-gnu" => Some((
            "powerpc64le-linux-gnu",
            "050732e22340a79c5acb724d04a191558cca6eddc320165210636c7fbe36a7be",
        )),
        "x86_64-apple-darwin" => Some((
            "x86_64-apple-darwin",
            "2c0be68e33ea1432477b28f41cdb2cceb40697aab222974c8682beacaccee84e",
        )),
        "x86_64-unknown-linux-gnu" => Some((
            "x86_64-linux-gnu",
            "2ac97eba5b79f8051cd86184e00379271c72e85d2bbcbcd50626b0ec602a1978",
        )),
        "x86_64-unknown-linux-musl" => Some((
            "x86_64-linux-musl",
            "afc7db8d2b4e5c55197d46cfb7453cb650e99162972290d7cd27b48d375b8512",
        )),
        "x86_64-pc-windows-msvc" => Some((
            "x86_64-pc-windows-msvc",
            "8ad4077c3f440566cb2a8455b646939662e679b323866339a91cec0a334c6168",
        )),
        "x86_64-pc-windows-gnu" => Some((
            "x86_64-w64-mingw32",
            "7a2c2922f2be57c7fbbe48e468dd55fff259aa65648d80cf05acb7ecbc26b011",
        )),
        _ => None,
    }
}
