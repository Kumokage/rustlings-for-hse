use std::{sync::mpsc, thread, time::Duration};

struct Queue {
    halflife1: Vec<u32>,
    halflife2: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            halflife1: vec![1, 2, 3, 4, 5],
            halflife2: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    // TODO: We want to send `tx` to both threads. But currently, it is moved
    // into the first thread. How could you solve this problem?
    let Queue {halflife1, halflife2} = q;
    let tx0 = tx.clone();
    thread::spawn(move || {
        for val in halflife1 {
            println!("Sending {val:?}");
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    thread::spawn(move || {
        for val in halflife2 {
            println!("Sending {val:?}");
            tx0.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();

        send_tx(queue, tx);

        let mut received = Vec::with_capacity(10);
        for value in rx {
            received.push(value);
        }

        received.sort();
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
