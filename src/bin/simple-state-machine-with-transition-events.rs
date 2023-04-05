use std::collections::HashMap;
#[derive(Eq,PartialEq,Hash,Copy,Clone)]
enum State {
    Start,
    Running,
    Paused,
    Stopped,
}
#[derive(Eq,PartialEq,Hash)]
enum Event {
    Start,
    Stop,
    Pause,
    Resume,
}

struct StateMachine {
    state: State,
    transitions: HashMap<(State, Event), State>,
}

impl StateMachine {
    fn new() -> Self {
        let mut transitions = HashMap::new();
        transitions.insert((State::Start, Event::Start), State::Running);
        transitions.insert((State::Running, Event::Stop), State::Stopped);
        transitions.insert((State::Running, Event::Pause), State::Paused);
        transitions.insert((State::Paused, Event::Resume), State::Running);
        transitions.insert((State::Paused, Event::Stop), State::Stopped);

        StateMachine {
            state: State::Start,
            transitions,
        }
    }

    fn transition(&mut self, event: Event) -> Result<(), &'static str> {
        let current_state = &self.state;
        let next_state = match self.transitions.get(&(*current_state, event)) {
            Some(next_state) => &*next_state,
            None => {
                return Err("Invalid transition");
            }
        };
        self.state = *next_state;
        Ok(())
    }
}

fn main() {
    let mut state_machine = StateMachine::new();

    match state_machine.transition(Event::Start) {
        Ok(_) => println!("Started"),
        Err(e) => println!("Error: {}", e),
    }
    match state_machine.transition(Event::Pause) {
        Ok(_) => println!("Paused"),
        Err(e) => println!("Error: {}", e),
    }
    match state_machine.transition(Event::Resume) {
        Ok(_) => println!("Resumed"),
        Err(e) => println!("Error: {}", e),
    }
    match state_machine.transition(Event::Stop) {
        Ok(_) => println!("Stopped"),
        Err(e) => println!("Error: {}", e),
    }
}
