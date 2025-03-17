pub fn check_palindrome(s1: &mut String) -> bool {
    if s1.len() > 0 {
        let v1: Vec<char> = s1.chars().collect(); //Vec 1

        let mut v2: Vec<_> = Vec::new(); //Vec2
        for i in (0..=v1.len() - 1).rev() {
            v2.push(v1[i]); //vec1's all ele. push into vec2
        }

        let v1_string: String = v1.into_iter().map(|e| e.to_string()).collect(); //Convert v1 into string
        let v2_string: String = v2.into_iter().map(|e| e.to_string()).collect(); //Convert v2 into string

        println!("v1_string={v1_string:?}");
        println!("v2_string={v2_string:?}");

        v1_string == v2_string //Both equal then it'll return true
    } else {
        false
    }
}

/*
  if s1.is_empty(){
   return false;
  }
  *s1 == s1.chars().rev().collect()::<String>();
*/
