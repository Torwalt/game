use anyhow::{bail, Ok, Result};
use rand::prelude::*;
use winit::event::{Event, WindowEvent};

use crate::graphics::mesh_builder::Instance;
use crate::graphics::{self, mesh_builder};

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
    map: TileMap,
    input: Input,
    exit: bool,
    monsters: Vec<Monster>,
}

impl GameState {
    pub fn new() -> Result<Self> {
        let input = Input::new();
        let map = TileMap::default();

        Ok(Self {
            input,
            map,
            exit: false,
            monsters: vec![Monster::default()],
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

        self.monsters.iter_mut().for_each(|m| m.update_pos());

        if self
            .input
            .is_physical_key_pressed(winit::keyboard::KeyCode::Digit1)
        {
            self.monsters.push(Monster::default())
        }
    }

    pub fn update_keys(&mut self) {
        self.input.update_keys()
    }

    pub fn input(&mut self, event: &WindowEvent) {
        self.input.process_event(event);
    }

    pub fn instances(&self) -> Vec<mesh_builder::Instance> {
        let mut instances = self.map.to_instances();
        instances.extend(self.monsters.iter().map(|m| m.to_instance()));
        instances.sort_by(|a, b| {
            b.z_order
                .partial_cmp(&a.z_order)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        instances
    }

    // NOTE: Right now no user events. When there are such, I can make `Event` generic on my user
    // event.
    pub fn handle(&self, _event: Event<()>) {}

    pub fn exit(&self) -> bool {
        self.exit
    }
}

pub struct TileMap {
    tiles: Vec<TileType>,
    width: usize,
    height: usize,
}

impl TileMap {
    pub fn default() -> Self {
        let width = 20;
        let height = 20;

        TileMap::new(width, height).unwrap()
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

    pub fn to_instances(&self) -> Vec<Instance> {
        self.iter()
            .map(|tile| Instance {
                position: [tile.position.0 as f32, tile.position.1 as f32],
                z_order: match tile.ty {
                    TileType::Floor => 0.9,
                    TileType::Wall => 0.8,
                },
                texture_index: tile.ty as u32,
                entity_type: EntityType::Tile as u32,
                animation_frame: 0,
                frame_pos_offset: [1.0, 1.0],
            })
            .collect()
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn iter(&self) -> TileMapIter {
        TileMapIter {
            current_idx: 0,
            tile_map: self,
        }
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

#[derive(Clone)]
pub enum EntityType {
    Tile,
    Monster,
}

// This is not ECS style, I will adjust this after exploration phase.
pub struct Monster {
    pub texture_id: String,
    pub monster_state: MonsterState,
    pub health: u8,
    pub damage: u8,
    pub position: [f32; 2],
}

impl Monster {
    pub fn default() -> Self {
        Monster {
            texture_id: "".to_string(),
            monster_state: MonsterState::Idling,
            health: 100,
            damage: 10,
            position: [10.0, 10.0],
        }
    }

    pub fn to_instance(&self) -> Instance {
        Instance {
            position: self.position,
            texture_index: 0,
            z_order: 0.8,
            entity_type: EntityType::Monster as u32,
            animation_frame: 0,
            frame_pos_offset: [0.1, 0.2],
        }
    }

    pub fn update_pos(&mut self) {
        let mut rng = rand::thread_rng();
        let dx = rng.gen_range(-0.05..=0.05);
        let dy = rng.gen_range(-0.05..=0.05);
        self.position = [self.position[0] + dx, self.position[1] + dy];
    }
}

pub enum MonsterState {
    Idling,
    Moving,
    Attacking,
    Dead,
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
