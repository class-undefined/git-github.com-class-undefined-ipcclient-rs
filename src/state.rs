pub enum IpcStatus {
    OK = 200,
    CACHED = 304,
    BadRequest = 400,
    UNAUTHORIZED = 401,
    FORBIDDEN = 403,
    NotFound = 404,
    InternalServerError = 500,
    BadGateway = 502,
}

impl From<i64> for IpcStatus {
    fn from(value: i64) -> Self {
        match value {
            200 => IpcStatus::OK,
            304 => IpcStatus::CACHED,
            400 => IpcStatus::BadRequest,
            401 => IpcStatus::UNAUTHORIZED,
            403 => IpcStatus::FORBIDDEN,
            404 => IpcStatus::NotFound,
            500 => IpcStatus::InternalServerError,
            502 => IpcStatus::BadGateway,
            _ => panic!("Unknown status code: {}", value),
        }
    }
}
