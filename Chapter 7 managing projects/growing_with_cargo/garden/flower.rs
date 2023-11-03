//This is where the code for the flower module is defined
#[allow(non_snake_case)]
pub struct Flower {//Here the petal code is defined. The mod keyword is not needed.
    FlowerType: String,
    PetalColor: (u32,u32,u32), //Represents rgb values
    PetalsLeft: u32
}

impl Flower {
    pub fn new(ft:String, pl: u32, pc:(u32,u32,u32)) -> Self {
        Flower { FlowerType:ft, PetalColor: pc, PetalsLeft: pl }
    }

    pub fn display_flower(&self) {
        println!("The flower is a {}. The RBG values of the flowers petals are {:?}. The flower has {} petals left.", self.FlowerType, self.PetalColor, self.PetalsLeft);
    }

    fn pick_petal(&mut self) { self.PetalsLeft -= 1; } //Cannot be accessed when used because its private

    pub fn she_loves_me(&mut self) {
        if self.PetalsLeft == 0 { println!("No more petals to pick."); }
        else { 
            self.pick_petal(); 
            let result = if self.PetalsLeft % 2 == 0 { "" } else { " not" };
            println!("You pick a petal. She loves me{}.", result);
        }
    }
}