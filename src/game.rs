use std::collections::VecDeque;
use std::time::Duration;
use std::time::SystemTime;

extern crate rand;
use rand::rngs::ThreadRng;
use rand::Rng;

pub fn init() -> Game {
    Game::init()
}

pub struct Game {
    pub snek: SnekManager,
    pub food: FoodFactory,
    pub dir_mgr: DirectionManager,
    pub cfg: Settings,
    pub is_growing: bool,
    pub score: u16,
}

impl Game {
    pub fn init() -> Self {
        Self {
            snek: SnekManager::init(),
            food: FoodFactory::init(),
            dir_mgr: DirectionManager::init(),
            cfg: Settings::init(),
            is_growing: false,
            score: 0,
        }
    }

    pub fn restart(&mut self) {
        self.snek = SnekManager::init();
        self.food = FoodFactory::init();
        self.dir_mgr = DirectionManager::init();
        self.is_growing = false;
        self.score = 0;
    }

    pub fn peek(&self) -> Position {
        use Direction::*;

        let head = self.snek.get_head();
        let offset_pos: Position = match self.dir_mgr.current() {
            Left => (-1, 0).into(),
            Right => (1, 0).into(),
            Down => (0, 1).into(),
            Up => (0, -1).into(),
            Idle => (0, 0).into(),
        };

        Position(head.0 + offset_pos.0, head.1 + offset_pos.1)
    }

    pub fn is_position_outside(&self, pos: &Position) -> bool {
        let (columns, rows) = (self.cfg.columns as i16, self.cfg.rows as i16);

        !(0..columns).contains(&pos.0) || !(0..rows).contains(&pos.1)
    }

    pub fn is_position_onbody(&self, pos: &Position) -> bool {
        self.snek.body.range(1..).any(|x| x == pos)
    }

    pub fn is_position_onfood(&self, pos: &Position) -> bool {
        &self.food.position == pos
    }

    pub fn is_snek_updating(&self) -> bool {
        self.snek
            .last_update
            .elapsed()
            .expect("How do you travel back time?")
            >= self.cfg.speed
    }

    pub fn update_offset(&mut self) {
        self.snek.offset = self.snek.since_update().as_secs_f32() / self.cfg.speed.as_secs_f32()
    }
}

pub struct SnekManager {
    pub body: VecDeque<Position>,
    pub offset: f32,
    last_update: SystemTime,
}

impl SnekManager {
    fn init() -> Self {
        Self {
            body: [(2, 3), (3, 3)].map(|pos| pos.into()).into(),
            last_update: SystemTime::now(),
            offset: 0.0,
        }
    }

    fn since_update(&self) -> Duration {
        self.last_update
            .elapsed()
            .expect("How do you travel back time?")
    }

    pub fn get_head(&'_ self) -> &'_ Position {
        self.body.iter().last().unwrap()
    }

    pub fn update(&mut self, is_growing: bool, pos: Position) {
        self.body.push_back(pos);
        self.offset = 0.0;
        self.last_update = SystemTime::now();

        if !is_growing {
            self.body.pop_front();
        }
    }
}

pub struct FoodFactory {
    rng: ThreadRng,
    pub position: Position,
}

impl FoodFactory {
    fn init() -> Self {
        Self {
            rng: rand::thread_rng(),
            position: (10, 10).into(),
        }
    }

    fn find_unoccupied_cell(snek: &SnekManager, cfg: &Settings) -> Option<Position> {
        if snek.body.len() >= (cfg.columns * cfg.rows).into() {
            return None;
        }

        let offsets = [(0, -1), (-1, 0), (1, 0), (0, 1)];

        let find_offset = |pos: Position| {
            move |offset: &(i16, i16)| -> Option<Position> {
                let pos = Position(pos.0 + offset.0, pos.1 + offset.1);

                match pos {
                    Position(x, _) if !(0..cfg.columns as i16).contains(&x) => None,
                    Position(_, y) if !(0..cfg.rows as i16).contains(&y) => None,
                    p if !snek.body.contains(&p) => Some(p),
                    _ => None,
                }
            }
        };

        snek.body
            .iter()
            .find_map(|&pos| offsets.iter().find_map(find_offset(pos)))
    }

    pub fn relocate(&mut self, snek: &SnekManager, cfg: &Settings) {
        let mut food = Position(
            self.rng.gen_range(0..cfg.columns as i16),
            self.rng.gen_range(0..cfg.rows as i16),
        );

        if snek.body.contains(&food) {
            food = FoodFactory::find_unoccupied_cell(snek, cfg).unwrap_or(food);
        }

        self.position = food;
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
    Idle,
}

pub struct DirectionManager {
    current: Direction,
    queue: VecDeque<Direction>,
}

impl DirectionManager {
    fn init() -> Self {
        Self {
            current: Direction::Idle,
            queue: [].into(),
        }
    }

    pub fn current(&'_ self) -> &'_ Direction {
        &self.current
    }

    pub fn go(&mut self, dir: Direction) {
        if self.queue.len() == 2 {
            self.queue.pop_front();
        }

        self.queue.push_back(dir);
    }

    pub fn update(&mut self) {
        use Direction::*;
        let handle_direction = |v| match (self.current, v) {
            (Up, Down) | (Down, Up) | (Right, Left) | (Left, Right) => self.current,
            _ => v,
        };

        self.current = self
            .queue
            .pop_front()
            .map(handle_direction)
            .unwrap_or(self.current);
    }
}

pub struct Settings {
    pub columns: u16,
    pub rows: u16,
    speed: Duration,
}

impl Settings {
    fn init() -> Self {
        Self {
            columns: 25,
            rows: 25,
            speed: Duration::from_millis(150),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Position(pub i16, pub i16);

impl From<(i16, i16)> for Position {
    fn from(pos: (i16, i16)) -> Self {
        Self(pos.0, pos.1)
    }
}
