pub trait Contract {
    fn create(&self) {
        println!("This create a contract")
    }

    fn execute(&self) {
        println!("This execute a contract")
    }

    fn commit(&self) {
        println!("This commit a contract")
    }
}
