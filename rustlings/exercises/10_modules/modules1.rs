// TODO: Fix the compiler error about calling a private function.
use sausage_factory::*;

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    pub fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
