/*
 * In this module, we'll run a background service in a thread
 * that will receive a URL over a channel, and append that URL
 * into a local file called playlists.txt.
 *
 * When an empty string is sent over the channel, the service
 * will stop.
 * */

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;
use std::thread;

pub fn run() -> (mpsc::Sender<String>, thread::JoinHandle<()>) {
    let (tx, rx) = mpsc::channel::<String>();
    let handle = thread::spawn(move || {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("playlists.txt")
            .unwrap();
        loop {
            let url = rx.recv().unwrap();
            if url.is_empty() {
                break;
            }
            writeln!(file, "{}", url).unwrap();
        }
    });
    (tx, handle)
}
