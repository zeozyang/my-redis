///# 任务
/// - 一个 Tokio 任务是一个异步的绿色线程，它们通过 tokio::spawn 进行创建，该函数会返回一个 JoinHandle 类型的句柄，调用者可以使用该句柄跟创建的任务进行交互。
/// - spawn 函数的参数是一个 async 语句块，该语句块甚至可以返回一个值，然后调用者可以通过 JoinHandle 句柄获取该值:
/// - 任务是调度器管理的执行单元。spawn生成的任务会首先提交给调度器，然后由它负责调度执行。需要注意的是，执行任务的线程未必是创建任务的线程，任务完全有可能运行在另一个不同的线程上，而且任务在生成后，它还可能会在线程间被移动。
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        10086
    });

    let out = handle.await.unwrap();
    // 以上代码会打印出GOT 10086。实际上，上面代码中.await 会返回一个 Result ，
    // 若 spawn 创建的任务正常运行结束，则返回一个 Ok(T)的值，否则会返回一个错误 Err：
    // 例如任务内部发生了 panic 或任务因为运行时关闭被强制取消时。
    println!("GOT {}", out);
}