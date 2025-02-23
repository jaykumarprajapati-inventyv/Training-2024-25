mod read_to_elem;
mod iterator_on_vector;
mod to_store_diff_type_value_by_enum;

fn main() {
    let mut v1=Vec::new();

    v1.push(10);
    v1.push(20);
    v1.push(30);
    println!("Vector 1 elem. are:{:?}",v1);

    //Initialize by using vector macro.
    let mut v2=vec!['J','a','y']; 
    v2.push('P');
    println!("Vector 2 using macro:{:?}",v2);

    //Read to elem. of vectors
    read_to_elem::reading_elem();

    //Iteration
    iterator_on_vector::iterate_on_vector();

    //Store diff type's val
    to_store_diff_type_value_by_enum::diff_types_val();
}
