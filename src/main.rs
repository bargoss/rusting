use crate::factory_experiment::factory_experiment_mod::factory_experiment_main;
mod factory_experiment;


use std::io::{Error, Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt};
use memstream::MemStream;


// serializable interface
pub trait Serializable {
    fn serialize(&self, stream: &mut MemStream) -> Result<(), std::io::Error>;
    fn deserialize(stream: &mut MemStream) -> Result<Self, std::io::Error>
    where
        Self: Sized;
}
pub trait Command: Serializable {
    fn execute(&self);
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

struct PrintCommand{
    msg : String
}

impl Serializable for PrintCommand {
    fn serialize(&self, stream: &mut MemStream) -> Result<(), Error> {
        stream.write_u8(0)?;
        stream.write_u32::<byteorder::LittleEndian>(self.msg.len() as u32)?;
        stream.write_all(self.msg.as_bytes())?;
        Ok(())
    }

    fn deserialize(stream: &mut MemStream) -> Result<Self, Error> where Self: Sized {
        let len = stream.read_u32::<byteorder::LittleEndian>()?;
        let mut buf = vec![0; len as usize];
        stream.read_exact(&mut buf)?;
        let msg = String::from_utf8(buf).unwrap();
        Ok(PrintCommand{msg})
    }
}

impl Command for PrintCommand{
    fn execute(&self){
        println!("{}", self.msg);
    }
}


struct SleepCommand{
    milliseconds : i32
}

impl Serializable for SleepCommand {
    fn serialize(&self, stream: &mut MemStream) -> Result<(), Error> {
        stream.write_u8(1)?;
        stream.write_i32::<byteorder::LittleEndian>(self.milliseconds)?;
        Ok(())
    }

    fn deserialize(stream: &mut MemStream) -> Result<Self, Error> where Self: Sized {
        let milliseconds = stream.read_i32::<byteorder::LittleEndian>()?;
        Ok(SleepCommand{milliseconds})
    }
}



struct PrintColoredCommand{
    msg : String,
    colorR : i32,
    colorG : i32,
    colorB : i32
}

impl Serializable for PrintColoredCommand {
    fn serialize(&self, stream: &mut MemStream) -> Result<(), Error> {
        stream.write_u8(2)?;
        stream.write_u32::<byteorder::LittleEndian>(self.msg.len() as u32)?;
        stream.write_all(self.msg.as_bytes())?;
        stream.write_i32::<byteorder::LittleEndian>(self.colorR)?;
        stream.write_i32::<byteorder::LittleEndian>(self.colorG)?;
        stream.write_i32::<byteorder::LittleEndian>(self.colorB)?;
        Ok(())
    }

    fn deserialize(stream: &mut MemStream) -> Result<Self, Error> where Self: Sized {
        let len = stream.read_u32::<byteorder::LittleEndian>()?;
        let mut buf = vec![0; len as usize];
        stream.read_exact(&mut buf)?;
        let msg = String::from_utf8(buf).unwrap();
        let colorR = stream.read_i32::<byteorder::LittleEndian>()?;
        let colorG = stream.read_i32::<byteorder::LittleEndian>()?;
        let colorB = stream.read_i32::<byteorder::LittleEndian>()?;
        Ok(PrintColoredCommand{msg, colorR, colorG, colorB})
    }
}


fn main() {
    factory_experiment_main();


    return;
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
    //stream.write_all(&bytes).unwrap();
    // blocking seek

    // sleep the thread
    std::thread::sleep(std::time::Duration::from_millis(1000));
    println!("Hello, world!");
    std::thread::sleep(std::time::Duration::from_millis(1000));
    println!("Hello, world!");
    std::thread::sleep(std::time::Duration::from_millis(1000));
    println!("Hello, world!");
    std::thread::sleep(std::time::Duration::from_millis(1000));
    println!("Hello, world!");





    



}