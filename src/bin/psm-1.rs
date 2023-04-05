use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
enum State {
    Initial,
    Running,
    Paused,
    Stopped
}

struct StateMachine {
    state: State,
    conn: Connection,
}

impl StateMachine {
    fn new() -> Result<Self> {
        // Connect to an in-memory SQLite database
        let conn = Connection::open_in_memory()?;
        // Create a table to store the state
        conn.execute(
            "CREATE TABLE state (
                      id INTEGER PRIMARY KEY,
                      value TEXT NOT NULL
                      )",
            params![],
        )?;

        // Insert an initial state
        let initial_state = State::Initial;
        let initial_state_str = initial_state.to_string();
        conn.execute(
            "INSERT INTO state (id, value) VALUES (1, ?1)",
            params![initial_state_str],
        )?;
        let mut stmt = conn.prepare("SELECT * FROM state")?;
        let state_str: String = stmt.query_row(params![], |row| Ok(row.get(1)?))?;
        let state = State::from_str(&state_str).expect("Failed to parse state from db");
        Ok(Self { state, conn })
    }

    fn transition(&mut self, new_state: State) -> Result<()> {
        // Check if the transition is valid
        match (self.state.clone(), new_state.clone()) {
            (State::Initial, State::Running) => {},
            (State::Running, State::Paused) => {},
            (State::Paused, State::Running) => {},
            (State::Running, State::Stopped) => {},
            (State::Paused, State::Stopped) => {},
            _ => {
                return Err(rusqlite::Error::new(
                    rusqlite::ErrorKind::InvalidInput,
                    "Invalid state transition",
                ));
            }
        }

        self.state = new_state;
        let new_state_str = new_state.to_string();
        self.conn.execute(
            "UPDATE state SET value = ?1 WHERE id = 1",
            params![new_state_str],
        )
    }

    fn current_state(&self) -> &State {
        &self.state
    }
}

fn main() -> Result<()> {
    let mut state_machine = StateMachine::new()?;
    println!("Current state: {:?}", state_machine.current_state());
    state_machine.transition(State::Running)?;
    println!("Current state: {:?}", state_machine.current_state());
    state_machine.transition(State::Paused)?;
    println!("Current state: {:?}", state_machine.current_state());
}
