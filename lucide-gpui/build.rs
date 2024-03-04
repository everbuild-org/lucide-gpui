fn main() {
    const url = "github.com/lucide-icons/lucide.git";
    let repo = match git2::Repository::clone(url, "./lucide") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
}