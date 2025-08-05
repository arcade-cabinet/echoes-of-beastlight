use bevy::prelude::*;
use bevy_egui::egui;
use std::collections::VecDeque;

#[derive(Resource, Default)]
pub struct ConsoleState {
    pub logs: VecDeque<LogEntry>,
    pub command_history: Vec<String>,
    pub current_command: String,
    pub scroll_to_bottom: bool,
    pub filter: LogFilter,
}

#[derive(Clone)]
pub struct LogEntry {
    pub timestamp: std::time::Instant,
    pub level: ConsoleLevel,
    pub message: String,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConsoleLevel {
    Info,
    Warning,
    Error,
    Success,
    Debug,
}

#[derive(Default)]
pub struct LogFilter {
    pub show_info: bool,
    pub show_warnings: bool,
    pub show_errors: bool,
    pub show_success: bool,
    pub show_debug: bool,
    pub search_term: String,
}

impl Default for LogFilter {
    fn default() -> Self {
        Self {
            show_info: true,
            show_warnings: true,
            show_errors: true,
            show_success: true,
            show_debug: false,
            search_term: String::new(),
        }
    }
}

impl ConsoleState {
    pub fn log(&mut self, level: ConsoleLevel, message: impl Into<String>) {
        self.logs.push_back(LogEntry {
            timestamp: std::time::Instant::now(),
            level,
            message: message.into(),
            source: None,
        });
        
        // Keep only last 1000 entries
        while self.logs.len() > 1000 {
            self.logs.pop_front();
        }
        
        self.scroll_to_bottom = true;
    }
    
    pub fn log_with_source(&mut self, level: ConsoleLevel, message: impl Into<String>, source: impl Into<String>) {
        self.logs.push_back(LogEntry {
            timestamp: std::time::Instant::now(),
            level,
            message: message.into(),
            source: Some(source.into()),
        });
        
        while self.logs.len() > 1000 {
            self.logs.pop_front();
        }
        
        self.scroll_to_bottom = true;
    }
    
    pub fn clear(&mut self) {
        self.logs.clear();
    }
    
    pub fn execute_command(&mut self, command: &str) {
        if !command.is_empty() {
            self.command_history.push(command.to_string());
            self.log(ConsoleLevel::Info, format!("> {}", command));
            
            // Parse and execute command
            match self.parse_command(command) {
                Ok(result) => self.log(ConsoleLevel::Success, result),
                Err(error) => self.log(ConsoleLevel::Error, error),
            }
        }
        
        self.current_command.clear();
    }
    
    fn parse_command(&self, command: &str) -> Result<String, String> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Err("Empty command".into());
        }
        
        match parts[0] {
            "help" => Ok(self.show_help()),
            "clear" => Ok("Console cleared".into()),
            "generate" => {
                if parts.len() > 1 {
                    Ok(format!("Starting generation for: {}", parts[1..].join(" ")))
                } else {
                    Err("Usage: generate <asset_type> [options]".into())
                }
            }
            "list" => Ok("Available assets:\n- characters\n- tilesets\n- ui_elements\n- audio".into()),
            _ => Err(format!("Unknown command: {}", parts[0])),
        }
    }
    
    fn show_help(&self) -> String {
        "Available commands:\n\
         help - Show this help message\n\
         clear - Clear console\n\
         generate <type> - Generate asset of specified type\n\
         list - List available asset types\n\
         ".into()
    }
}

pub fn show_console(ui: &mut egui::Ui, console_state: &mut ConsoleState) {
    ui.heading("📜 Console");
    ui.separator();
    
    // Filter controls
    ui.horizontal(|ui| {
        ui.label("Filter:");
        ui.checkbox(&mut console_state.filter.show_info, "ℹ️ Info");
        ui.checkbox(&mut console_state.filter.show_warnings, "⚠️ Warnings");
        ui.checkbox(&mut console_state.filter.show_errors, "❌ Errors");
        ui.checkbox(&mut console_state.filter.show_success, "✅ Success");
        ui.checkbox(&mut console_state.filter.show_debug, "🐛 Debug");
        
        ui.separator();
        
        ui.label("Search:");
        ui.text_edit_singleline(&mut console_state.filter.search_term);
        
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("Clear").clicked() {
                console_state.clear();
            }
        });
    });
    
    ui.separator();
    
    // Log display
    let text_height = ui.text_style_height(&egui::TextStyle::Monospace);
    let num_rows = ((ui.available_height() - 50.0) / text_height).floor() as usize;
    
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .max_height(ui.available_height() - 50.0)
        .show(ui, |ui| {
            for entry in &console_state.logs {
                if !should_show_entry(entry, &console_state.filter) {
                    continue;
                }
                
                ui.horizontal(|ui| {
                    // Timestamp
                    let elapsed = entry.timestamp.elapsed();
                    ui.weak(format!("[{:>6.1}s]", elapsed.as_secs_f32()));
                    
                    // Level icon and color
                    let (icon, color) = match entry.level {
                        ConsoleLevel::Info => ("ℹ️", egui::Color32::LIGHT_BLUE),
                        ConsoleLevel::Warning => ("⚠️", egui::Color32::YELLOW),
                        ConsoleLevel::Error => ("❌", egui::Color32::RED),
                        ConsoleLevel::Success => ("✅", egui::Color32::GREEN),
                        ConsoleLevel::Debug => ("🐛", egui::Color32::GRAY),
                    };
                    
                    ui.colored_label(color, icon);
                    
                    // Source if available
                    if let Some(source) = &entry.source {
                        ui.weak(format!("[{}]", source));
                    }
                    
                    // Message
                    ui.label(&entry.message);
                });
            }
            
            // Auto-scroll to bottom
            if console_state.scroll_to_bottom {
                ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                console_state.scroll_to_bottom = false;
            }
        });
    
    ui.separator();
    
    // Command input
    ui.horizontal(|ui| {
        ui.label(">");
        
        let response = ui.text_edit_singleline(&mut console_state.current_command);
        
        // Handle Enter key
        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            console_state.execute_command(&console_state.current_command.clone());
            response.request_focus();
        }
        
        // Command history with up/down arrows
        if response.has_focus() {
            if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                if let Some(last_command) = console_state.command_history.last() {
                    console_state.current_command = last_command.clone();
                }
            }
        }
    });
}

fn should_show_entry(entry: &LogEntry, filter: &LogFilter) -> bool {
    // Check level filter
    let level_match = match entry.level {
        ConsoleLevel::Info => filter.show_info,
        ConsoleLevel::Warning => filter.show_warnings,
        ConsoleLevel::Error => filter.show_errors,
        ConsoleLevel::Success => filter.show_success,
        ConsoleLevel::Debug => filter.show_debug,
    };
    
    if !level_match {
        return false;
    }
    
    // Check search filter
    if !filter.search_term.is_empty() {
        let search_lower = filter.search_term.to_lowercase();
        let message_lower = entry.message.to_lowercase();
        let source_match = entry.source.as_ref()
            .map(|s| s.to_lowercase().contains(&search_lower))
            .unwrap_or(false);
        
        return message_lower.contains(&search_lower) || source_match;
    }
    
    true
}