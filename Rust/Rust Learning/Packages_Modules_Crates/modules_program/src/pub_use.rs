/*Re export way:- It usefull to re-exports the function inside the same module by using pub use. We can also use of it in main.rs .*/
pub mod student {
    const MARK: u8 = 90;
    pub fn student_rollno() {
        let roll_num: i32 = 5054;
        println!("Roll num {} got {} mark out of 100.", roll_num, MARK);
    }
}

pub use student::student_rollno; //Here, we're exportung not calling thats why we didn't use () at the end.

//If main func. is inside the same file then we can acces this code as below:
/*
 fn main(){
   student_rollno(); // Get Shorter path due to re-exporting
  }
*/
