use tokio::time::{Duration, sleep, timeout};
use tokio::sync::mpsc;



async fn download_file(name: &str, size_mb: u32, tx: mpsc::Sender<(String ,String)>) {
    // Sleep for 10 mb to download
    let mut downloaded = 0;
    while downloaded < size_mb {
        sleep(Duration::from_secs(1)).await;
        downloaded =  (downloaded + 10).min(size_mb);
        let _ = tx.send((name.to_string(), format!("{}/{} MB", downloaded, size_mb))).await;
    }
}



#[tokio::main]
async fn main() {  
    let (tx, mut rx) = mpsc::channel::<(String, String)>(32);
    
    let files: Vec<(&str, u32)> = vec![
        ("report.pdf", 20),
        ("video.mp4", 50),
        ("data.zip", 30),
    ];

    for (f, s) in files {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            download_file(f, s, tx_clone).await
        });
    }

    drop(tx);

    while let Some((f, s)) = rx.recv().await {
        println!("{}: {}", f, s);
    }

}