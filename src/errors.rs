// Create the Error, ErrorKind, ResultExt, and Result types
error_chain!{
    foreign_links {
        Io(::io::Error);
        Hyper(::hyper::Error);
        SerdeJSON(::serde_json::Error);
    }
}
