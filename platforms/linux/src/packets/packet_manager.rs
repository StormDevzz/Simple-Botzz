use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Типы пакетов Minecraft
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PacketType {
    /// Пакет движения игрока
    PlayerPosition,
    /// Пакет поворота головы
    PlayerLook,
    /// Пакет движения и поворота
    PlayerPositionLook,
    /// Пакет атаки
    PlayerAction,
    /// Пакет использования предмета
    UseItem,
    /// Пакет чата
    ChatMessage,
    /// Пакет взаимодействия с блоком
    BlockInteract,
    /// Пакет клика в инвентаре
    WindowClick,
    /// Прочие пакеты
    Other,
}

/// Статистика пакетов
#[derive(Debug, Clone)]
pub struct PacketStats {
    pub total_sent: u64,
    pub total_received: u64,
    pub sent_per_type: HashMap<PacketType, u64>,
    pub received_per_type: HashMap<PacketType, u64>,
}

/// Менеджер пакетов для предотвращения флагов на античите
pub struct PacketManager {
    stats: PacketStats,
    last_packet_time: Instant,
    packet_history: Vec<(PacketType, Instant)>,
    max_history_size: usize,
    rate_limit_enabled: bool,
    min_packet_interval: Duration,
}

impl PacketManager {
    pub fn new() -> Self {
        Self {
            stats: PacketStats {
                total_sent: 0,
                total_received: 0,
                sent_per_type: HashMap::new(),
                received_per_type: HashMap::new(),
            },
            last_packet_time: Instant::now(),
            packet_history: Vec::new(),
            max_history_size: 1000,
            rate_limit_enabled: true,
            min_packet_interval: Duration::from_millis(50), // Минимум 50мс между пакетами
        }
    }

    /// Проверяет, можно ли отправить пакет (rate limiting)
    pub fn can_send_packet(&mut self, packet_type: PacketType) -> bool {
        if !self.rate_limit_enabled {
            return true;
        }

        let now = Instant::now();
        let elapsed = now.duration_since(self.last_packet_time);

        if elapsed < self.min_packet_interval {
            return false;
        }

        // Проверяем на спам однотипных пакетов
        let recent_same_type = self.packet_history.iter()
            .rev()
            .take(10)
            .filter(|(pt, _)| *pt == packet_type)
            .count();

        if recent_same_type > 5 {
            return false;
        }

        true
    }

    /// Регистрирует отправку пакета
    pub fn record_sent(&mut self, packet_type: PacketType) {
        self.stats.total_sent += 1;
        *self.stats.sent_per_type.entry(packet_type).or_insert(0) += 1;
        self.last_packet_time = Instant::now();
        
        self.packet_history.push((packet_type, Instant::now()));
        if self.packet_history.len() > self.max_history_size {
            self.packet_history.remove(0);
        }
    }

    /// Регистрирует получение пакета
    pub fn record_received(&mut self, packet_type: PacketType) {
        self.stats.total_received += 1;
        *self.stats.received_per_type.entry(packet_type).or_insert(0) += 1;
    }

    /// Получает статистику пакетов
    pub fn get_stats(&self) -> &PacketStats {
        &self.stats
    }

    /// Сбрасывает статистику
    pub fn reset_stats(&mut self) {
        self.stats = PacketStats {
            total_sent: 0,
            total_received: 0,
            sent_per_type: HashMap::new(),
            received_per_type: HashMap::new(),
        };
        self.packet_history.clear();
    }

    /// Включает/выключает rate limiting
    pub fn set_rate_limit(&mut self, enabled: bool) {
        self.rate_limit_enabled = enabled;
    }

    /// Устанавливает минимальный интервал между пакетами
    pub fn set_min_interval(&mut self, duration: Duration) {
        self.min_packet_interval = duration;
    }

    /// Получает текущий интервал между последними пакетами
    pub fn get_last_interval(&self) -> Duration {
        if self.packet_history.len() < 2 {
            return Duration::ZERO;
        }
        let (_, last) = self.packet_history.last().unwrap();
        let (_, prev) = self.packet_history.get(self.packet_history.len() - 2).unwrap();
        last.duration_since(*prev)
    }
}

impl Default for PacketManager {
    fn default() -> Self {
        Self::new()
    }
}
