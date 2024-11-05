use std::collections::HashMap;
use tokio::sync::mpsc;

#[derive(Default)]
pub struct SseHandler {
    account_session: HashMap<i32, HashMap<usize, mpsc::Sender<String>>>,
}

impl SseHandler {
    pub fn new_session(&mut self, account_id: i32) -> (usize, mpsc::Receiver<String>) {
        let (tx, rx) = mpsc::channel::<String>(32);
        let mp = self.account_session.entry(account_id).or_default();
        let l = mp.len();
        mp.insert(l, tx);
        (l, rx)
    }

    pub fn close_session(&mut self, account_id: i32, key: usize, _tx: mpsc::Receiver<String>) {
        if let Some(sessions) = self.account_session.get_mut(&account_id) {
            sessions.remove(&key);
        };
    }
}