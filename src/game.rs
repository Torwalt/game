use anyhow::{bail, Ok, Result};
use winit::event::{Event, WindowEvent};

use crate::graphics;

use self::input::Input;

mod input;

// Temp struct.
pub struct ECS {
    pub game_state: GameState,
    pub renderer: graphics::State,
}

impl ECS {
    pub fn new(game_state: GameState, renderer: graphics::State) -> Self {
        Self {
            game_state,
            renderer,
        }
    }

    pub fn render(&mut self) -> Result<()> {
        self.renderer.update(&self.game_state)?;
        self.renderer.render()?;
        Ok(())
    }

    pub fn update_renderer(&mut self, renderer: graphics::State) {
        self.renderer = renderer
    }
}

pub struct GameState {
    entities: Vec<Entity>,
    map: TileMap,
    input: Input,

    // Worst state management ever, let me cook.
    invert_triangle: bool,
    render_quad: bool,
    exit: bool,
}

impl GameState {
    pub fn new() -> Result<Self> {
        let input = Input::new();
        let map = TileMap::default();

        Ok(Self {
            input,
            entities: Vec::new(),
            map,
            exit: false,
            invert_triangle: false,
            render_quad: false,
        })
    }

    pub fn update(&mut self) {
        if self
            .input
            .is_logical_key_pressed(winit::keyboard::NamedKey::Escape)
        {
            self.exit = true;
            return;
        }

        if self
            .input
            .is_physical_key_pressed(winit::keyboard::KeyCode::KeyI)
        {
            self.invert_triangle = !self.invert_triangle
        }

        if self
            .input
            .is_physical_key_pressed(winit::keyboard::KeyCode::KeyQ)
        {
            self.render_quad = !self.render_quad
        }
    }

    pub fn update_keys(&mut self) {
        self.input.update_keys()
    }

    pub fn inverted(&self) -> bool {
        self.invert_triangle
    }

    pub fn render_quad(&self) -> bool {
        self.render_quad
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

pub struct TileMap {
    tiles: Vec<TileType>,
    width: usize,
    height: usize,
}

impl TileMap {
    pub fn iter(&self) -> TileMapIter {
        TileMapIter {
            current_idx: 0,
            tile_map: self,
        }
    }

    pub fn default() -> Self {
        let width = 10;
        let height = 10;

        TileMap::new(width, height).unwrap()
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn new(width: usize, height: usize) -> Result<Self> {
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

pub struct TileMapIter<'a> {
    current_idx: usize,
    tile_map: &'a TileMap,
}

impl Iterator for TileMapIter<'_> {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        let tile = self.tile_map.tiles.get(self.current_idx)?;
        let x = self.current_idx % self.tile_map.width;
        let y = self.current_idx / self.tile_map.height;
        self.current_idx += 1;
        Some(Self::Item {
            position: (x as u32, y as u32),
            ty: tile.clone(),
        })
    }
}

pub struct Tile {
    pub position: (u32, u32),
    pub ty: TileType,
}

#[derive(Clone)]
pub enum TileType {
    Floor,
    Wall,
}

#[cfg(test)]
mod tests {
    use super::*; // Import the items from the outer module into the test module

    #[test]
    fn test_tile_map_iter() -> Result<()> {
        let positions: Vec<Tile> = TileMap::new(10, 10)?.iter().collect();
        assert_eq!(100, positions.len());
        Ok(())
    }
}
