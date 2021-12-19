use backend::utils::mq::*;
use tokio::signal;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::broadcast;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    backend::log_init();
    let (notify_shutdown, _): (broadcast::Sender<()>, _) = broadcast::channel(1);
    let mut shutdown_recv = signal(SignalKind::terminate()).unwrap();

    let _ = tokio::spawn(trending_executor(notify_shutdown.subscribe()));
    let _ = tokio::spawn(relation_executor(notify_shutdown.subscribe()));
    let _ = tokio::spawn(search_executor(notify_shutdown.subscribe()));
    let _ = tokio::spawn(email_executor(notify_shutdown.subscribe()));
    // futures::future::join_all(handles).await;
    // futures::future::join_all(scheduler).await;
    tokio::select! {
        _ = signal::ctrl_c() => {
            log::warn!("\n[TASK-EXEC] Gracefully shutdown. Wait for 5 seconds...\n");
            std::thread::sleep(Duration::from_millis(5000));
            drop(notify_shutdown);
            std::thread::sleep(Duration::from_millis(1000));
            log::warn!("\n[TASK-EXEC] Shutdown finished\n");
        },
        _ = shutdown_recv.recv() => {
            log::warn!("\n[TASK-EXEC] Gracefully shutdown. Wait for 5 seconds...\n");
            std::thread::sleep(Duration::from_millis(5000));
            drop(notify_shutdown);
            std::thread::sleep(Duration::from_millis(1000));
            log::warn!("\n[TASK-EXEC] Shutdown finished\n");
        },
    }
}

async fn trending_executor(mut shutdown: broadcast::Receiver<()>) {
    tokio::select! {
        output = generate_trending() => {
            log::error!("[TASK-EXEC] Trending executor result: {:?}", output);
        },
        _ = shutdown.recv() => {
            log::warn!("[TASK-EXEC] Trending executor is shutdown.");
        },
    }
}

async fn relation_executor(mut shutdown: broadcast::Receiver<()>) {
    tokio::select! {
        output = pulsar_typesense() => {
            log::error!("[TASK-EXEC] Relation executor result: {:?}", output);
        },
        _ = shutdown.recv() => {
            log::warn!("[TASK-EXEC] Relation executor is shutdown.");
        },
    }
}

async fn search_executor(mut shutdown: broadcast::Receiver<()>) {
    tokio::select! {
        output = pulsar_email() => {
            log::error!("[TASK-EXEC] Typesense executor result: {:?}", output);
        },
        _ = shutdown.recv() => {
            log::warn!("[TASK-EXEC] Typesense executor is shutdown.");
        },
    }
}

async fn email_executor(mut shutdown: broadcast::Receiver<()>) {
    tokio::select! {
        output = pulsar_relation() => {
            log::error!("[TASK-EXEC] Email executor result: {:?}", output);
        },
        _ = shutdown.recv() => {
            log::warn!("[TASK-EXEC] Email executor is shutdown.");
        },
    }
}
