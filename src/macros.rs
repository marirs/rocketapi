/// Returns a custom json response
///
/// ## Example Usage
///
/// - Specify a error core to respond with
/// ```ignore
/// json_response!("msg" => "some message", "error_code" => 200, {k: v, ...})
/// ```
/// - With default error core as 200
/// ```ignore
/// json_response!("msg" => "some message", {k: v, ...})
/// ```
/// - With default error core as 200, and just respond with key, value pairs
/// ```ignore
/// json_response!({k: v, ...})
/// ```
#[macro_export]
macro_rules! json_response {
    ( $map:expr ) => {
        rocket::response::status::Custom(rocket::http::Status::from_code(200).unwrap(), json!($map))
    };
    ( $status_code:expr, $msg:expr $(, $key:expr => $val:expr )* ) => {
        return rocket::response::status::Custom(rocket::http::Status::from_code($status_code).unwrap(), json!({
                "message": $msg,
                "error_code": $status_code,
                $( $key: $val, )*
            }));
    };
    ( $msg:expr $(, $key:expr => $val:expr )* ) => {
        rocket::response::status::Custom(rocket::http::Status::from_code(200).unwrap(), json!({
                "message": $msg,
                $( $key: $val, )*
            }))
    };
    ( $($key:expr => $val:expr ),* ) => {
        rocket::response::status::Custom(rocket::http::Status::from_code(200).unwrap(), json!({
                $( $key: $val, )*
            }))
    };
}
