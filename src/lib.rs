const MAX_FLOOR: u8 = 5;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Idle,
    MovingUp,
    MovingDown,
    DoorsOpen,
}

#[derive(Debug, PartialEq)]
pub enum ElevatorError {
    InvalidFloor(u8),
    DoorsAlreadyOpen,
    DoorsAlreadyClosed,
    CannotOpenWhileMoving,
    CannotMoveDoorsOpen,
    EmptyQueue,
}

pub struct ElevatorStatus {
    pub floor: u8,
    pub state: State,
    pub queue: Vec<u8>,
}

pub struct Elevator {
    floor: u8,
    state: State,
    queue: Vec<u8>,
}

fn is_valid_floor(floor: u8) -> bool {
    floor <= MAX_FLOOR
}

impl Elevator {
    pub fn new(floor: u8) -> Result<Self, ElevatorError> {
        if !is_valid_floor(floor) {
            return Err(ElevatorError::InvalidFloor(floor));
        }
        Ok(Elevator {
            floor,
            state: State::Idle,
            queue: Vec::new(),
        })
    }

    pub fn floor(&self) -> u8 {
        self.floor
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn queue(&self) -> &[u8] {
        &self.queue
    }

    pub fn status(&self) -> ElevatorStatus {
        ElevatorStatus {
            floor: self.floor,
            state: self.state,
            queue: self.queue.clone(),
        }
    }

    pub fn call(&mut self, floor: u8) -> Result<(), ElevatorError> {
        if !is_valid_floor(floor) {
            return Err(ElevatorError::InvalidFloor(floor));
        }
        // ignore call to current floor or already queued floor
        if floor == self.floor || self.queue.contains(&floor) {
            return Ok(());
        }
        self.queue.push(floor);
        // if idle, immediately pick a direction toward the new destination
        if self.state == State::Idle {
            self.state = if floor > self.floor {
                State::MovingUp
            } else {
                State::MovingDown
            };
        }
        Ok(())
    }

    pub fn open_doors(&mut self) -> Result<(), ElevatorError> {
        match self.state {
            State::DoorsOpen => Err(ElevatorError::DoorsAlreadyOpen),
            State::MovingUp | State::MovingDown => Err(ElevatorError::CannotOpenWhileMoving),
            State::Idle => {
                self.state = State::DoorsOpen;
                Ok(())
            }
        }
    }

    pub fn close_doors(&mut self) -> Result<(), ElevatorError> {
        if self.state != State::DoorsOpen {
            return Err(ElevatorError::DoorsAlreadyClosed);
        }
        if self.queue.is_empty() {
            self.state = State::Idle;
        } else {
            let next = self.queue[0];
            self.state = if next > self.floor {
                State::MovingUp
            } else {
                State::MovingDown
            };
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), ElevatorError> {
        if self.state == State::DoorsOpen {
            return Err(ElevatorError::CannotMoveDoorsOpen);
        }
        if self.queue.is_empty() {
            self.state = State::Idle;
            return Err(ElevatorError::EmptyQueue);
        }
        let destination = self.queue[0];
        if destination > self.floor {
            self.floor += 1;
        } else {
            self.floor -= 1;
        }
        // arrived at destination: remove it and open doors
        if self.floor == destination {
            self.queue.remove(0);
            self.state = State::DoorsOpen;
        }
        Ok(())
    }
}
