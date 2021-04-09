mod configuration {
    struct conf_struct {
        exclude_patterns: std::vec<std::string::String>,
        source: std::string::String,
        destination: std::string::String,
        help: bool,
    }
}
