pub mod tcp_ping;
pub mod http_ping;
pub mod download;
pub mod scorer;

pub use self::{
    tcp_ping::tcp_ping,
    http_ping::http_ping,
    download::download_speed,
    scorer::calculate_score,
};
