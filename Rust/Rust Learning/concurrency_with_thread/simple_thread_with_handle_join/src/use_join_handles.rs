use std::thread;
use std::time::Duration;

/*Need of join hanldes method:- By default, main thread mujab j spawn thread run thay chhe, pn jo aakhu spawn thread run karavu hoi tyare aa use thay chhe. */
pub fn join_handles() {
    let handles = thread::spawn(|| {
        for i in 1..=10 {
            println!("{i} time print Jay from spawn thread!!");
            thread::sleep(Duration::from_millis(1000));
        }
    });
    /*  handles.join().unwrap();:- If we're using handle before main thread it'll print all spawn thread first then it'll print main thread's logic. */

    //main thread
    for i in 1..=5 {
        println!("{i} time print Prajapati from main thread!!");
        thread::sleep(Duration::from_millis(1000));
    }
    handles.join().unwrap(); //Here, join() return Result<T,V> that why we're handling by unwrap().
}
