pub struct Player<'a> {
    pub name:  &'a str ,
    pub location: u8
}

impl<'a> Player<'a> {
    pub fn say_hello(&self) {
    println!("hello {} with location of {}", self.name, self.location);
    }
}