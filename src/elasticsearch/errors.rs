#[derive(Debug, Eq, PartialEq)]
pub enum EsError {
    HostUnreachable,
    IndexNotFound,
    Timeout,
}
