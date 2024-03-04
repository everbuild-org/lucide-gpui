fn main() {
    let url = "https://github.com/lucide-icons/lucide.git";
    // check if directory exists
    if std::path::Path::new("./lucide").exists() {
        return;
    }

    match git2::Repository::clone(url, "./lucide") {
        Ok(_) => {},
        Err(e) => panic!("failed to clone: {}", e),
    };
}
