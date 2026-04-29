use super::packet_manager::PacketType;

/// Фильтр пакетов для предотвращения подозрительной активности
pub struct PacketFilter {
    /// Блокировать подозрительные паттерны движения
    block_suspicious_movement: bool,
    /// Блокировать быстрые атаки (клики)
    block_fast_clicks: bool,
    /// Блокировать спам чата
    block_chat_spam: bool,
    /// Минимальный интервал между атаками (мс)
    min_attack_interval: u64,
    /// Минимальный интервал между сообщениями чата (мс)
    min_chat_interval: u64,
}

impl PacketFilter {
    pub fn new() -> Self {
        Self {
            block_suspicious_movement: true,
            block_fast_clicks: true,
            block_chat_spam: true,
            min_attack_interval: 100, // 100мс между атаками
            min_chat_interval: 1000, // 1 секунда между сообщениями
        }
    }

    /// Проверяет, нужно ли заблокировать пакет
    pub fn should_block(&self, packet_type: PacketType) -> bool {
        match packet_type {
            PacketType::PlayerAction if self.block_fast_clicks => true,
            PacketType::ChatMessage if self.block_chat_spam => true,
            _ => false,
        }
    }

    /// Включает/выключает блокировку подозрительного движения
    pub fn set_block_suspicious_movement(&mut self, enabled: bool) {
        self.block_suspicious_movement = enabled;
    }

    /// Включает/выключает блокировку быстрых атак
    pub fn set_block_fast_clicks(&mut self, enabled: bool) {
        self.block_fast_clicks = enabled;
    }

    /// Включает/выключает блокировку спама чата
    pub fn set_block_chat_spam(&mut self, enabled: bool) {
        self.block_chat_spam = enabled;
    }

    /// Устанавливает минимальный интервал между атаками
    pub fn set_attack_interval(&mut self, interval_ms: u64) {
        self.min_attack_interval = interval_ms;
    }

    /// Устанавливает минимальный интервал между сообщениями чата
    pub fn set_chat_interval(&mut self, interval_ms: u64) {
        self.min_chat_interval = interval_ms;
    }
}

impl Default for PacketFilter {
    fn default() -> Self {
        Self::new()
    }
}
