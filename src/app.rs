use rand::Rng;
use std::{cmp::max, collections::VecDeque};
use web_sys::KeyboardEvent;
use yew::prelude::*;
use yew_hooks::*;

#[function_component(App)]
pub fn app() -> Html {
    let game = use_mut_ref(|| Game::new(20));
    let direction = use_state(|| Direction::Up);
    let speed = use_state(|| 300);
    let tick = use_state(|| 0);
    let has_ticked = use_state(|| 0);

    {
        let direction = direction.clone();
        use_event_with_window("keydown", move |e: KeyboardEvent| {
            let code = e.key();
            let new_dir = match code.as_str() {
                "ArrowUp" => Direction::Up,
                "ArrowDown" => Direction::Down,
                "ArrowLeft" => Direction::Left,
                "ArrowRight" => Direction::Right,
                _ => Direction::Up,
            };
            direction.set(new_dir);
        });
    }

    {
        let tick = tick.clone();
        let speed = speed.clone();
        let to_apply = speed.clone();
        use_interval(
            move || {
                tick.set(*tick + 1);
            },
            *to_apply,
        );
    }

    let mut g = game.borrow_mut();
    let food = g.food.clone();
    if has_ticked != tick {
        if g.snake.slither(food, *direction) {
            g.spawn_food();
            speed.set(max(40, *speed - 20))
        }
        has_ticked.set(*tick);
        g.draw();
    }
    html! {
        <main>
            { g.board.iter().map(|row| html! {
                    <div class="row">
                        { row.iter().map(|cell| html! {
                            <div class="cell">
                                { cell.render() }
                            </div>
                        }).collect::<Html>()
                        }
                    </div>
                }).collect::<Html>()
            }
        </main>
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Game {
    board: Vec<Vec<Square>>,
    snake: Snake,
    food: Coord,
}

impl Game {
    fn new(size: i32) -> Self {
        let board = (1..=size)
            .map(|_| (1..=size).map(|_| Square::Void).collect::<Vec<Square>>())
            .collect();
        let mut game = Game {
            board,
            snake: Snake::new((size / 2) as usize, (size / 2) as usize),
            food: Coord(0, 0),
        };
        game.spawn_food();
        game.draw();
        game
    }

    fn draw(&mut self) {
        self.clear();

        self.board[self.food.0][self.food.1] = Square::Food;

        let mut snek = self.snake.0.iter_mut();
        let &mut head = snek.next().unwrap();
        self.board[head.0][head.1] = Square::Head;

        for Coord(x, y) in snek {
            self.board[*x][*y] = Square::Body;
        }
    }

    fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..self.board.len());
        let y = rng.gen_range(0..self.board.len());
        self.food = Coord(x, y)
    }

    fn clear(&mut self) {
        for row in self.board.iter_mut() {
            for cell in row.iter_mut() {
                *cell = Square::Void;
            }
        }
    }
}

impl Snake {
    fn new(x: usize, y: usize) -> Self {
        let mut snake = VecDeque::new();
        snake.push_back(Coord(y, x));
        snake.push_back(Coord(y + 1, x));
        snake.push_back(Coord(y + 2, x));
        Snake(snake)
    }

    fn slither(&mut self, food: Coord, direction: Direction) -> bool {
        let head = self.0.front().unwrap();
        let neck: &Coord = self.0.get(1).unwrap();
        let to_neck = head.direction_to(neck);
        let dir = if to_neck == direction {
            to_neck.opposite()
        } else {
            direction
        };
        let new_head = match dir {
            Direction::Left => Coord(head.0, head.1 - 1),
            Direction::Right => Coord(head.0, head.1 + 1),
            Direction::Up => Coord(head.0 - 1, head.1),
            Direction::Down => Coord(head.0 + 1, head.1),
        };
        self.0.push_front(new_head);
        if new_head != food {
            self.0.pop_back();
        }
        for c in self.0.iter().skip(1) {
            if c == &new_head {
                panic!("You died!");
            }
        }
        new_head == food
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Board(Vec<Vec<Square>>);

#[derive(PartialEq, Eq, Clone)]
struct Snake(VecDeque<Coord>);

#[derive(PartialEq, Eq, Clone)]
enum Square {
    Head,
    Body,
    Food,
    Void,
}

impl Square {
    fn render(&self) -> Html {
        match self {
            Square::Head => html! { <div class="head"></div> },
            Square::Body => html! { <div class="body"></div> },
            Square::Food => html! { <div class="food"></div> },
            Square::Void => html! { <div class="void"></div> },
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Up,
    Down,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Coord(usize, usize);

impl Coord {
    fn direction_to(&self, other: &Coord) -> Direction {
        if self.0 > other.0 {
            Direction::Up
        } else if self.0 < other.0 {
            Direction::Down
        } else if self.1 > other.1 {
            Direction::Left
        } else {
            Direction::Right
        }
    }
}
