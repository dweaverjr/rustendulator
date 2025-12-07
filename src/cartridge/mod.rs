mod mapper;

pub use mapper::Mapper;

pub struct Cartridge {
    mapper: Box<dyn Mapper>,
}