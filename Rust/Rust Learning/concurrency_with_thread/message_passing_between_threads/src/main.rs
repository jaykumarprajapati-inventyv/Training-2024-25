mod sending_multiple_values_wait_receiver;
mod simple_prog_of_passing;
mod multiple_thread_using_clone;

fn main() {
    //simple_prog_of_passing::simply_passing_val();
    // sending_multiple_values_wait_receiver::sending_multiple_value_receiver_wait_for_it();
    multiple_thread_using_clone::multiple_thread_by_using_clone();
}
