use std::path::{PathBuf};
use std::collections::HashMap;

use tokio::fs;
use tokio::task::JoinHandle;

// create a tokio task that downloads a file over http
pub async fn spawn_downloader(url: String, filename: String) -> JoinHandle<()> {
    tokio::task::spawn(async move {
        let resp = reqwest::get(url).await.expect("Request for url failed");
        fs::write(filename, resp.bytes().await.unwrap()).await.unwrap();
    })
}

// take a  url and spawwn a task to download it
pub async fn download_file(url: &str, dir: &str) {
    
    let task: JoinHandle<()>;
    
    // extract the actual resource from the url
    let stem = url.split('/').last().unwrap();           

    let mut path = PathBuf::new(); 
    path.push(dir);
    
    if !path.is_dir() {
        fs::create_dir(&path).await.unwrap();
    }

    path.push(stem);

    // await the task creation not its conclusion
    println!("{}", path.as_path().display());
    task = spawn_downloader(String::from(url), path.as_path().display().to_string()).await;
    task.await.unwrap();
}

// take a list of urls and spawwn tasks to download each of them
pub async fn download_files(urls: Vec<String>, dir: &str) {
    let mut tasks: Vec<JoinHandle<()>> = Vec::new();
    for url in urls.into_iter()
    {
       // extract the actual resource from the url
       let stem = url.split('/').last().unwrap();       
       
       let mut path = PathBuf::new(); 
       path.push(dir);
       
       if !path.is_dir() {
         fs::create_dir(&path).await.unwrap();
       }

       path.push(stem);

       // await the task creation not its conclusion
       println!("{}", path.as_path().display());
       tasks.push(spawn_downloader(url, path.as_path().display().to_string()).await);
    }

    for task in tasks
    {
        task.await.unwrap();
    }
}

// take a list of urls and spawwn tasks to download each of them
pub async fn download_files_named(urls: HashMap<String, String>, dir: &str) {
    let mut tasks: Vec<JoinHandle<()>> = Vec::new();
    for (filename, url) in urls.into_iter()
    {
       // extract the actual resource from the url
       let stem = url.split('/').last().unwrap();       
       
       let mut path = PathBuf::new(); 
       path.push(dir);
       
       if !path.is_dir() {
        fs::create_dir(&path).await.unwrap();
       }

       path.push(stem);

       // await the task creation not its conclusion
       println!("{}/{}", dir, filename);
       tasks.push(spawn_downloader(url, format!("{}/{}", dir, filename)).await);
    }

    for task in tasks
    {
        task.await.unwrap();
    }
}