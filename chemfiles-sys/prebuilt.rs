/// Get the julia triple & sha256 corresponding to the prebuilt libchemfiles
/// for a given rust triple, if it exists
pub fn get_prebuilt_info(target: &str) -> Option<(&'static str, &'static str)> {
    match target {
        "aarch64-apple-darwin" => Some(("aarch64-apple-darwin", "de8fc3c7534cc9077f91e276524ca53e482a69de966df21965508ba71ee3d21a")),
        "aarch64-unknown-linux-gnu" => Some(("aarch64-linux-gnu", "8fd043575a198745cfc6993714dc754aa4ce0fae20743f326137b292cb7292fa")),
        "aarch64-unknown-linux-musl" => Some(("aarch64-linux-musl", "e8d40230bfc8e37910e6766b27f2751598dc2c79d27d93d9b9377625add91e94")),
        "armv7-unknown-linux-gnueabihf" => Some(("armv7l-linux-gnueabihf", "98ced72ad195129c8045733fd63fe3573910c5fcf2fe0565d9dd445c0b4ee538")),
        "armv7-unknown-linux-musleabihf" => Some(("armv7l-linux-musleabihf", "2306657577e0c33a8f562ea72579dd4785feea6df9fcfd76e4f104f3804d4a3c")),
        "i686-unknown-linux-gnu" => Some(("i686-linux-gnu", "297ab506dbdac6dd6febedf544c276475988cff0273b455c5cd9cba7e23805f2")),
        "i686-unknown-linux-musl" => Some(("i686-linux-musl", "ef29187fed702472178186acfb0e6fddedbc1cd5511cf50b0e976509c0582cad")),
        "i686-pc-windows-gnu" => Some(("i686-w64-mingw32", "a206dc9ce00b998c8996a7fcf4d3de3ac6d487af950dd9d53fb9009c12b7e860")),
        "powerpc64le-unknown-linux-gnu" => Some(("powerpc64le-linux-gnu", "7b87fc0c303652ad28101cd0d75ebd03a44cd303918da855c2f40cebac313208")),
        "x86_64-apple-darwin" => Some(("x86_64-apple-darwin", "6490efd0fef41c4014a48406fe89d30c464bb9857194bc6cf6012fa807acfa71")),
        "x86_64-unknown-linux-gnu" => Some(("x86_64-linux-gnu", "16cf6f38f817e555d0393db2726ab80b3b6ff1b2d8f884478f938eda9a452371")),
        "x86_64-unknown-linux-musl" => Some(("x86_64-linux-musl", "8d0ebba7b42cb7bcc123a0ffd118a73046eea19f47f298d790f9dbd88937b4f1")),
        "x86_64-unknown-freebsd" => Some(("x86_64-unknown-freebsd", "4d2ecca4109198f37c416bf3f5d85cf6d21bd5d3fb1d08c2b0559405d951bfcc")),
        "x86_64-pc-windows-gnu" => Some(("x86_64-w64-mingw32", "f622c55ff6b20b0c3e2433f2f28dff8b81e863785a1c295f25066b0953eb08fe")),
        _ => None,
    }
}
