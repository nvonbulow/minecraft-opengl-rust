
#[derive(Copy, Clone)]
pub enum Block {
    Air = 0,
    Stone = 1,
    Grass = 2,
    Dirt = 3,
    Bedrock = 7,
}

impl Block {
    pub fn info(&self) -> BlockInfo {
        match *self {
            Block::Air => BlockInfo::with_name_nonsolid("air"),
            Block::Stone => BlockInfo::with_name("stone"),
            Block::Grass => BlockInfo::with_name("grass"),
            Block::Dirt => BlockInfo::with_name("dirt"),

            Block::Bedrock => BlockInfo::with_name("bedrock")
        }
    }

    pub fn is_solid(&self) -> bool{
        self.info().is_solid
    }
}

pub struct BlockInfo {
    namespace: &'static str,
    name: &'static str,
    is_solid: bool
}

impl BlockInfo {
    fn with_name(name: &'static str) -> BlockInfo {
        BlockInfo {
            namespace: &"default",
            name: name,
            is_solid: true
        }
    }

    fn with_name_nonsolid(name: &'static str) -> BlockInfo {
        let mut blockinfo = BlockInfo::with_name(name);
        blockinfo.is_solid = false;
        blockinfo
    }
}