include!(concat!(env!("OUT_DIR"), "/embedded.rs"));

pub fn find_file(path: &str) -> Option<&'static [u8]> {
    FILES.iter().find(|(name, _)| *name == path).map(|(_, bytes)| *bytes)
}

pub fn get_embedded_text(path: &str) -> &'static str {
    let bytes = find_file(path).unwrap_or_else(|| panic!("the binary embeds {path}"));
    std::str::from_utf8(bytes).unwrap_or_else(|_| panic!("{path} is UTF-8 text"))
}

pub fn list_files(prefix: &str) -> Vec<(&'static str, &'static [u8])> {
    FILES.iter().filter(|(name, _)| name.starts_with(prefix)).copied().collect()
}
