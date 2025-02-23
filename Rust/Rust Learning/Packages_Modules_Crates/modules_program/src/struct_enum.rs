/* Imp note:- (a)By default Struct and its fields are "private", so for access outside we need to make these "public".
(b)By default, items inside a module (like restaurant::Item) cannot access anything outside the module unless explicitly made public (pub). So if we want to use enum which is outside of module we need to make enum as public.
(c)super:-It allows you to access functions, structs, or other items from the parent module and outside of module thing also.
*/

#[derive(Debug)]

//If we write "pub" ahead of enum then its variants and it also become public.
pub enum ServingStyle {
    DineIn,
    TakeAway,
}

//If we want to access this module outside then we've to make it "public".
pub mod restaurant {

    use super::ServingStyle; //Here, we're importing whole enum by using "super".
    pub struct Item {
        pub item_name: String,
        pub serve_way: ServingStyle,
    }

    pub fn prepare_dish() -> Item {
        Item {
            item_name: String::from("Pizza"),
            serve_way: ServingStyle::DineIn,
        }
    }
}
