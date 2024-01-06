#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

type Message = String;

struct GroundStation;

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg)
    }
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

fn main() {
    /* Setup the satellite and ground station. */
    let base = GroundStation {};
    let mut sat_a = CubeSat {
        id: 0,
        mailbox: Mailbox { messages: vec![] },
    };
    println!("t0: {:?}", sat_a);

    /* Send the message from the base station. */
    base.send(&mut sat_a, Message::from("Hello There!"));
    println!("t1: {:?}", sat_a);

    /* Recieve the message as the satellite. */
    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);
    println!("msg: \"{}\"", msg.unwrap_or(String::from("No Message!")));

    /* Try and retrieve a message that doesn't exist. */
    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);
    println!("msg: \"{}\"", msg.unwrap_or(String::from("No Message!")));
}
