//By using lifetime, our returned reference become more alive.
pub fn solution2_by_lifetime<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a.len() > b.len() {
        return &a; //If we're not write "return" before of it,it'll be return "Jay" not "Prajapati",bec. it'll be consider just as one statement,not an expression.
    }
    &b
}
