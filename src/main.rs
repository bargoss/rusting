use std::io::{Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt};
use memstream::MemStream;


// serializable interface
pub trait Serializable {
    fn serialize(&self, stream: &mut MemStream) -> Result<(), std::io::Error>;
    fn deserialize(stream: &mut MemStream) -> Result<Self, std::io::Error>
    where
        Self: Sized;
}



struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}
impl Serializable for Vector3 {
    fn serialize(&self, stream: &mut MemStream) -> Result<(), std::io::Error> {
        stream.write_i32::<byteorder::LittleEndian>(self.x)?;
        stream.write_i32::<byteorder::LittleEndian>(self.y)?;
        stream.write_i32::<byteorder::LittleEndian>(self.z)?;
        Ok(())
    }

    fn deserialize(stream: &mut MemStream) -> Result<Self, std::io::Error> {
        let x = stream.read_i32::<byteorder::LittleEndian>()?;
        let y = stream.read_i32::<byteorder::LittleEndian>()?;
        let z = stream.read_i32::<byteorder::LittleEndian>()?;
        Ok(Vector3 { x, y, z })
    }
}


struct Position{
    point: Vector3
}
impl Serializable for Position{
    fn serialize(&self, stream: &mut MemStream) -> Result<(), std::io::Error> {
        self.point.serialize(stream)?;
        Ok(())
    }

    fn deserialize(stream: &mut MemStream) -> Result<Self, std::io::Error> {
        let point = Vector3::deserialize(stream)?;
        Ok(Position{point})
    }
}

struct Velocity{
    point: Vector3
}

impl Serializable for Velocity{
    fn serialize(&self, stream: &mut MemStream) -> Result<(), std::io::Error> {
        self.point.serialize(stream)?;
        Ok(())
    }

    fn deserialize(stream: &mut MemStream) -> Result<Self, std::io::Error> {
        let point = Vector3::deserialize(stream)?;
        Ok(Velocity{point})
    }
}


struct Health{
    current: i32,
    maxHealth: i32
}

impl Serializable for Health{
    fn serialize(&self, stream: &mut MemStream) -> Result<(), std::io::Error> {
        stream.write_i32::<byteorder::LittleEndian>(self.current)?;
        stream.write_i32::<byteorder::LittleEndian>(self.maxHealth)?;
        Ok(())
    }

    fn deserialize(stream: &mut MemStream) -> Result<Self, std::io::Error> {
        let current = stream.read_i32::<byteorder::LittleEndian>()?;
        let maxHealth = stream.read_i32::<byteorder::LittleEndian>()?;
        Ok(Health{current, maxHealth})
    }
}


struct Factory{

}

impl Factory{
    // read first byte and go to switch case
    fn from_stream(stream: &mut MemStream) -> Result<Box<dyn Serializable>, std::io::Error>{
        let type_id = stream.read_u8()?;
        match type_id{
            0 => {
                let position = Position::deserialize(stream)?;
                Ok(Box::new(position))
            },
            1 => {
                let velocity = Velocity::deserialize(stream)?;
                Ok(Box::new(velocity))
            },
            2 => {
                let health = Health::deserialize(stream)?;
                Ok(Box::new(health))
            },
            _ => {
                panic!("Unknown type id");
            }
        }
    }

}

fn main() {
    // create random byte array
    let mut stream = MemStream::new();
    let position = Position { point: Vector3 { x: 1, y: 2, z: 3 } };
    position.serialize(&mut stream).unwrap();

    let velocity = Velocity { point: Vector3 { x: 4, y: 5, z: 6 } };
    velocity.serialize(&mut stream).unwrap();



    let mut bytes = Vec::new();
    let byteCount = stream.read_to_end(&mut bytes).unwrap();
    println!("byteCount: {}", byteCount);

    // print this
    for byte in bytes {
        print!("{}, ", byte);
    }

    // read byte array
    let mut stream = MemStream::new();
    stream.write_all(&bytes).unwrap();
    // blocking seek
    stream.seek(std::io::SeekFrom::Start(0)).unwrap();

    



}