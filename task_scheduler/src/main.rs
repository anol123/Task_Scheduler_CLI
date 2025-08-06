enum TaskType{
    Work,
    Study,
    Leisure
}
struct Task<T>{
    id:u32,
    title: String,
    task_type: TaskType,
    due_date:Option<NaiveDate>,
    metadata:T,
    on_complete:Option<Box<dyn Fn()>>
}
fn main() {
    println!("Hello, world!");
}
