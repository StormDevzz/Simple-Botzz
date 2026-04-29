use crate::bridge::message::{BridgeMessage, MessageType};
use std::sync::{Arc, Mutex};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use anyhow::Result;

/// IPC мост для общения между Rust и Node.js
pub struct IPCBridge {
    port: u16,
    running: Arc<Mutex<bool>>,
    message_handlers: Arc<Mutex<Vec<Box<dyn Fn(BridgeMessage) -> BridgeMessage + Send>>>>,
}

impl IPCBridge {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            running: Arc::new(Mutex::new(false)),
            message_handlers: Arc::new(Mutex::new(vec![])),
        }
    }

    /// Запускает TCP сервер для приема сообщений от Node.js
    pub fn start_server(&self) -> Result<()> {
        let running = self.running.clone();
        let handlers = self.message_handlers.clone();
        
        *running.lock().unwrap() = true;
        
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))?;
        
        thread::spawn(move || {
            for stream in listener.incoming() {
                if !*running.lock().unwrap() {
                    break;
                }
                
                match stream {
                    Ok(stream) => {
                        let handlers = handlers.clone();
                        thread::spawn(move || {
                            Self::handle_client(stream, handlers);
                        });
                    }
                    Err(e) => {
                        eprintln!("Connection error: {}", e);
                    }
                }
            }
        });
        
        Ok(())
    }

    /// Останавливает сервер
    pub fn stop_server(&self) {
        *self.running.lock().unwrap() = false;
    }

    /// Добавляет обработчик сообщений
    pub fn add_handler<F>(&self, handler: F)
    where
        F: Fn(BridgeMessage) -> BridgeMessage + Send + 'static,
    {
        self.message_handlers.lock().unwrap().push(Box::new(handler));
    }

    /// Отправляет сообщение в Node.js процесс
    pub fn send_to_nodejs(&self, message: &BridgeMessage) -> Result<()> {
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", self.port))?;
        let json = message.to_json()?;
        writeln!(stream, "{}", json)?;
        Ok(())
    }

    /// Обрабатывает клиента
    fn handle_client(stream: TcpStream, handlers: Arc<Mutex<Vec<Box<dyn Fn(BridgeMessage) -> BridgeMessage + Send>>>>) {
        let reader = BufReader::new(&stream);
        let mut writer = stream.try_clone().expect("Failed to clone stream");
        
        for line in reader.lines() {
            match line {
                Ok(json) => {
                    if let Ok(message) = BridgeMessage::from_json(&json) {
                        let handlers = handlers.lock().unwrap();
                        let mut response = message.clone();
                        
                        for handler in handlers.iter() {
                            response = handler(message.clone());
                        }
                        
                        let response_json = response.to_json().unwrap_or_else(|_| "{}".to_string());
                        let _ = writeln!(writer, "{}", response_json);
                    }
                }
                Err(e) => {
                    eprintln!("Error reading line: {}", e);
                    break;
                }
            }
        }
    }

    /// Отправляет пинг
    pub fn ping(&self) -> Result<()> {
        let message = BridgeMessage::new(MessageType::Ping, None, serde_json::json!({}));
        self.send_to_nodejs(&message)
    }

    /// Запускает бота через Node.js
    pub fn start_bot(&self, bot_id: &str, config: serde_json::Value) -> Result<()> {
        let message = BridgeMessage::new(MessageType::StartBot, Some(bot_id.to_string()), config);
        self.send_to_nodejs(&message)
    }

    /// Останавливает бота через Node.js
    pub fn stop_bot(&self, bot_id: &str) -> Result<()> {
        let message = BridgeMessage::new(MessageType::StopBot, Some(bot_id.to_string()), serde_json::json!({}));
        self.send_to_nodejs(&message)
    }
}
