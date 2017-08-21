
use std::collections::{BTreeMap, HashMap};
use glium;

pub mod block;
pub mod entity;

use self::block::Block;
use ::game::GameState;
use ::game::renderer::{Drawable, GameRenderer};
use ::game::renderer::chunk::ChunkRenderer;

pub struct World {
    chunks: BTreeMap<(u32, u32), Chunk>,
    // entities: _
    seed: String
}

impl World {
    pub fn chunk_at(&mut self, x: u32, z: u32) -> &Chunk {
        if self.chunks.contains_key(&(x, z)) {
            return self.chunks.get(&(x, z)).unwrap();
        }
        self.load_chunk_at(x, z);
        self.chunks.get(&(x, z)).unwrap()
    }

    fn load_chunk_at(&mut self, x: u32, z: u32) {
        // Load chunk from file if it exists or generate it
        let chunk = self.generate_chunk_at(x, z);
        self.chunks.insert((x, z), chunk);
    }

    fn generate_chunk_at(&mut self, x: u32, z: u32) -> Chunk {
        let mut chunk = Chunk::new(x, z);
        for y in 0..3 {
            chunk.fill_layer(y, Block::Bedrock);
        }
        for y in 3..20 {
            chunk.fill_layer(y, Block::Stone);
        }
        for y in 20..22 {
            chunk.fill_layer(y, Block::Dirt);
        }
        chunk.fill_layer(22, Block::Grass);
        chunk
    }
}

pub struct Chunk {
    pub blocks: HashMap<(u8, u8, u8), Block>,
    pub coordinates: (u32, u32)
}

impl Chunk {
    pub fn new(x: u32, z: u32) -> Chunk {
        Chunk {
            blocks: HashMap::new(),
            coordinates: (x, z)
        }
    }

    pub fn coordinates(&self) -> (u32, u32) {
        self.coordinates
    }

    pub fn block_at(&self, x: u8, y: u8, z: u8) -> Block {
        let block = self.blocks.get(&(x, y, z));
        match block {
            Some(block) => block.clone(),
            None => Block::Air
        }
    }

    pub fn set_block_at(&mut self, x: u8, y: u8, z: u8, block: Block) {
        if x > 15 || z > 15 {
            panic!("Block indicies greater than 15 are not allowed except for height!");
        }
        match block {
            Block::Air => self.blocks.remove(&(x, y, z)),
            _ => self.blocks.insert((x, y, z), block)
        };
    }

    fn fill_layer(&mut self, y: u8, block: Block) {
        for x in 0..16 {
            for z in 0..16 {
                self.set_block_at(x, y, z, block.clone());
            }
        }
    }
}

impl Drawable for Chunk {
    fn draw(&self, frame: &mut glium::Frame, renderer: &mut GameRenderer, state: &mut GameState) {
        ChunkRenderer::draw(&self, frame, renderer, state);
    }
}
