use std::fmt::Debug;

pub fn invalid_arg_handler<T: Debug>(obj: T) -> String {
    format!(
        "Expected an address in the form of <host>:<port> or <host>, got {:?}",
        obj
    )
}
