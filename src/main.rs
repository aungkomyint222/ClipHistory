use std::{sync::Arc, time::Duration};
use sled;
use arboard::Clipboard;
use iced::{
    subscription, widget::{button, column, container, row, scrollable, text},
    Application, Command, Element, Length, Settings, Subscription, Theme,
};
use tokio::sync::broadcast;

// Message type for communication between the app and the clipboard monitor
#[derive(Debug, Clone)]
enum Message {
    NewClipboardEntry(String),
    ClearEntries,
    CopyEntry(usize), // Copy a specific entry
}

// Main application state
struct ClipboardApp {
    db: Arc<sled::Db>,              // Shared database reference
    entries: Vec<String>,           // List of clipboard entries to display (max 10)
    sender: broadcast::Sender<Message>, // Channel to send messages to the UI
    clipboard: Clipboard,           // Clipboard instance for copying
}

impl Application for ClipboardApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        // Open the Sled database
        let db = Arc::new(sled::open("clipboard_db").expect("Failed to open database"));

        // Create clipboard instance
        let clipboard = Clipboard::new().expect("Failed to create clipboard");

        // Load existing entries from the database
        let mut entries = Vec::new();
        for entry in db.iter().values() {
            if let Ok(value) = entry {
                if let Ok(text) = String::from_utf8(value.to_vec()) {
                    if !entries.contains(&text) { // Only add unique entries
                        entries.push(text);
                    }
                }
            }
        }
        entries.reverse(); // Newest first
        while entries.len() > 10 {
            entries.pop(); // Trim to 10
        }

        // Create a broadcast channel
        let (sender, _) = broadcast::channel(100);

        // Spawn the clipboard monitoring task
        let db_clone = Arc::clone(&db);
        let sender_clone = sender.clone();
        tokio::spawn(async move {
            monitor_clipboard(db_clone, sender_clone).await;
        });

        (
            ClipboardApp {
                db,
                entries,
                sender,
                clipboard,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Clipboard Monitor")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NewClipboardEntry(text) => {
                // Only add if not already in entries (no dupes)
                if !self.entries.contains(&text) {
                    self.entries.insert(0, text.clone()); // Add new entry at the top
                    if self.entries.len() > 10 {
                        self.entries.pop(); // Keep only the last 10
                    }
                    // Update DB (remove oldest if needed, then insert)
                    if self.db.len() >= 10 {
                        if let Some(Ok((oldest_key, _))) = self.db.iter().next() {
                            let _ = self.db.remove(oldest_key);
                        }
                    }
                    let _ = self.db.insert(self.db.len().to_be_bytes(), text.as_bytes());
                    let _ = self.db.flush();
                }
            }
            Message::ClearEntries => {
                self.entries.clear();
                let _ = self.db.clear(); // Clear the database too
            }
            Message::CopyEntry(index) => {
                if let Some(text) = self.entries.get(index) {
                    if let Err(e) = self.clipboard.set_text(text.clone()) {
                        eprintln!("Failed to copy text: {:?}", e);
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            button("Clear Entries").on_press(Message::ClearEntries),
            scrollable(
                column(
                    self.entries
                        .iter()
                        .enumerate() // Get index along with entry
                        .map(|(index, entry)| {
                            // Truncate to 100 chars for display
                            let display_text = if entry.len() > 100 {
                                format!("{}...", &entry[..100])
                            } else {
                                entry.to_string()
                            };
                            container(
                                row![
                                    text(display_text).size(16),
                                    button("Copy")
                                        .on_press(Message::CopyEntry(index))
                                        .padding(5)
                                ]
                                .spacing(10)
                                .align_items(iced::Alignment::Center)
                            )
                            .padding(10)
                            .style(iced::theme::Container::Box)
                            .width(Length::Fill)
                            .into()
                        })
                        .collect::<Vec<_>>()
                )
                .spacing(10)
            )
            .height(Length::Fill),
        ]
        .padding(20)
        .spacing(20);

        content.into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        let receiver = self.sender.subscribe();
        subscription::unfold(
            "clipboard_subscription",
            receiver,
            |mut receiver| async move {
                match receiver.recv().await {
                    Ok(message) => (message, receiver),
                    Err(_) => {
                        tokio::time::sleep(Duration::from_millis(100)).await;
                        (Message::NewClipboardEntry("Error receiving message".to_string()), receiver)
                    }
                }
            },
        )
    }
}

// Async function to monitor the clipboard
async fn monitor_clipboard(db: Arc<sled::Db>, sender: broadcast::Sender<Message>) {
    let mut clipboard = Clipboard::new().expect("Failed to create clipboard");
    let mut last_text = String::new();

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;

        if let Ok(current_text) = clipboard.get_text() {
            if !current_text.is_empty() && current_text != last_text {
                last_text = current_text.clone();

                // Check if it’s already in the DB
                let is_duplicate = db.iter()
                    .values()
                    .any(|value| {
                        value.as_ref().map(|v| String::from_utf8_lossy(v) == current_text).unwrap_or(false)
                    });

                // Only send if it’s not a duplicate
                if !is_duplicate {
                    if let Err(e) = sender.send(Message::NewClipboardEntry(current_text)) {
                        eprintln!("Failed to send message: {:?}", e);
                    }
                }
            }
        }
    }
}

fn main() -> iced::Result {
    ClipboardApp::run(Settings::default())
}