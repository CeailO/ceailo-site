use std::str::EncodeUtf16;

pub async fn test() -> EncodeUtf16<'static> {
    "test".encode_utf16()
}
