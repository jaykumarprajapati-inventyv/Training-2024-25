//Generic types:-Here,find_max() allow to find max elem. of any type of array.
fn main() {
    let ar1 = [1, 2, 4, 23, 3, 9];
    let size_i32 = find_max(&ar1);
    println!("Max of i32 size array is:{}", size_i32);

    let ar2 = [23.12, 4.12, 45.23, 3.23];
    let size_f64 = find_max(&ar2);
    println!("Max of f64 size array is:{}", size_f64);
}
/*PartialOrd:- It is one type of prelude trait,it allows to do "<, <=, >, and >= operation" just for numerical releated type.It doesn't work when we're passing string or any non-numer. ty*/

fn find_max<T: PartialOrd>(arr: &[T]) -> &T {
    let mut res = &arr[0];

    for elem in arr {
        if res < elem {
            res = elem
        }
    }
    res
}
