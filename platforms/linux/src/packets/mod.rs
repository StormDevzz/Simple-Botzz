//! Модуль для управления пакетами бота
//! 
//! Предотвращает флаги на античите Grim за счет корректной обработки пакетов

pub mod packet_manager;
pub mod packet_filter;
pub mod rate_limiter;

pub use packet_manager::PacketManager;
pub use packet_filter::PacketFilter;
pub use rate_limiter::RateLimiter;
