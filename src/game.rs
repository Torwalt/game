use anyhow::{bail, Ok, Result};
use winit::event::{Event, WindowEvent};

use crate::graphics;

use self::input::Input;

mod input;

pub struct GameState {
    exit: bool,

    entities: Vec<Entity>,
    map: TileMap,
    renderer: graphics::State,
    input: Input,
}

impl GameState {
    pub fn new(renderer: graphics::State) -> Result<Self> {
        let input = Input::new();
        let map = TileMap::default();

        Ok(Self {
            renderer,
            input,
            exit: false,
            entities: Vec::new(),
            map,
        })
    }

    pub fn update_renderer(&mut self, renderer: graphics::State) {
        self.renderer = renderer
    }

    pub fn update(&mut self) {
        if self
            .input
            .is_logical_key_pressed(winit::keyboard::NamedKey::Escape)
        {
            self.exit = true;
        }
    }

    pub fn render(&mut self) -> Result<()> {
        self.renderer.render()?;
        Ok(())
    }

    pub fn input(&mut self, event: &WindowEvent) {
        self.input.process_event(event);
    }

    // NOTE: Right now no user events. When there are such, I can make `Event` generic on my user
    // event.
    pub fn handle(&self, _event: Event<()>) {}

    pub fn exit(&self) -> bool {
        self.exit
    }
}

type Entity = usize;

struct TileMap {
    tiles: Vec<TileType>,
    width: usize,
    height: usize,
}

impl TileMap {
    fn default() -> Self {
        let width = 100;
        let height = 100;

        TileMap::new(width, height).unwrap()
    }

    fn new(width: usize, height: usize) -> Result<Self> {
        if width == 0 || height == 0 {
            bail!("width and height must be larger than 0")
        }

        let mut tiles = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    tiles.push(TileType::Wall);
                } else {
                    tiles.push(TileType::Floor);
                }
            }
        }

        Ok(TileMap {
            tiles,
            width,
            height,
        })
    }
}

#[derive(Clone)]
enum TileType {
    Floor,
    Wall,
}
