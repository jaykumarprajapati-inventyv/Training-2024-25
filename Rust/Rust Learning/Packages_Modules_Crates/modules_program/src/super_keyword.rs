pub mod parent {

    pub mod child {
        pub fn msg() {
            println!(
                "I'm parent module's code so if you want to access me then use \"super\" keyword."
            );
        }
    }

    pub fn call_child() {
        use super::parent::child; //When you want to import & access anything which is in child.
        child::msg();

        //or
        // super::parent::child::msg(); //Use this when you want to directly access a function or item with the full path, without importing.
    }
}
