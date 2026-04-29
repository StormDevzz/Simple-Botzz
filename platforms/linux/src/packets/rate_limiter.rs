use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Rate limiter для контроля частоты пакетов
pub struct RateLimiter {
    /// История отправленных пакетов
    history: VecDeque<Instant>,
    /// Максимальное количество пакетов в окне
    max_packets: usize,
    /// Размер временного окна
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_packets: usize, window: Duration) -> Self {
        Self {
            history: VecDeque::with_capacity(max_packets),
            max_packets,
            window,
        }
    }

    /// Проверяет, можно ли отправить пакет
    pub fn try_acquire(&mut self) -> bool {
        let now = Instant::now();
        
        // Удаляем старые записи за пределами окна
        while let Some(&front) = self.history.front() {
            if now.duration_since(front) > self.window {
                self.history.pop_front();
            } else {
                break;
            }
        }

        // Проверяем лимит
        if self.history.len() < self.max_packets {
            self.history.push_back(now);
            true
        } else {
            false
        }
    }

    /// Получает количество пакетов в текущем окне
    pub fn current_count(&self) -> usize {
        self.history.len()
    }

    /// Получает время до следующего разрешенного пакета
    pub fn time_until_next(&self) -> Option<Duration> {
        if self.history.len() < self.max_packets {
            Some(Duration::ZERO)
        } else if let Some(&front) = self.history.front() {
            let elapsed = Instant::now().duration_since(front);
            if elapsed >= self.window {
                Some(Duration::ZERO)
            } else {
                Some(self.window - elapsed)
            }
        } else {
            Some(Duration::ZERO)
        }
    }

    /// Сбрасывает лимитер
    pub fn reset(&mut self) {
        self.history.clear();
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(20, Duration::from_secs(1)) // 20 пакетов в секунду
    }
}
