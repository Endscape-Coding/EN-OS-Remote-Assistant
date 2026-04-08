//!
//! Работа с прокси  
//! Поддерживает SOCKS5/HTTP прокси без авторизации
//! 
use std::env;
use std::time::Duration;
use std::process::Command;
use tokio::fs;
use reqwest::{Client, Proxy};

/// Чтение прокси из файла и запись в массив
async fn proxyfile(path: &str) -> Vec<String> {
    let mut proxies = Vec::new();

    match fs::read_to_string(path).await {
        Ok(proxylist) => {
            for line in proxylist.lines() {
                let clear = line.trim();
                if !clear.is_empty() && !clear.starts_with('#'){
                    proxies.push(clear.to_string());
                }
            }
            log::info!("Loaded {} proxies", proxies.len() )
        }
        Err(e) => {
            log::warn!("Error load proxies! Error: {}", e)
        }

    }
    proxies
}

/// Алгоритм для отбора прокси. Подбирает первый рабочий
pub async fn proxy_work() -> Client {
    let builder = Client::builder()
    .timeout(Duration::from_secs(120))
    .connect_timeout(Duration::from_secs(30));

    let mut lists: Vec<(String, &'static str)> = Vec::new();

    match env::var("SOCKS5_PROXY_PATH") {
        Ok(path) => {
            let socks_list = proxyfile(&path).await;
            for proxy in socks_list {
                lists.push((proxy, "SOCKS"));
            }
        }
        Err(_) => log::info!("SOCKS5 proxy dont use")
    }

    match env::var("HTTP_PROXY_PATH") {
        Ok(path) => {
            let http_list = proxyfile(&path).await;
            for proxy in http_list {
                lists.push((proxy, "HTTP"));
            }
        }
        Err(_) => log::info!("HTTP proxy dont use")
    }

    for (proxy_url, p_type) in lists {
        log::info!("Check proxy from file: {}", proxy_url);
        
        if check_proxy(&proxy_url, p_type) { 
            log::info!("Working proxy found: {} ({})", proxy_url, p_type);
            
            let builder = match p_type {
                "SOCKS" => {
                    let fixed = proxy_formatter(&proxy_url);
                    builder.proxy(Proxy::all(&fixed).expect("Invalid SOCKS proxy"))
                }
                "HTTP" => {
                    let fixed = proxy_formatter(&proxy_url);
                    builder.proxy(Proxy::http(&fixed).expect("Invalid HTTP proxy"))
                }
                _ => builder,
            };

            return builder.build().expect("Failed to build http client");
        }
    }

    log::info!("Work without proxies..");
    builder.build().expect("Failed to build HTTP client")
}

/// Проверяет прокси сервер на подключение к telegram API
pub fn check_proxy(proxy: &str, proxy_type: &str) -> bool {
    let (proxy_url, auth) = {
        if let Some(pos) = proxy.find("://") {
            let scheme = &proxy[..pos + 3];
            let rest = &proxy[pos + 3..];
            let parts: Vec<&str> = rest.split(':').collect();
            
            if parts.len() == 4 {
                let url = format!("{}{}:{}", scheme, parts[0], parts[1]);
                let auth = Some(format!("{}:{}", parts[2], parts[3]));
                (url, auth)
            } else {
                (proxy.to_string(), None)
            }
        } else {
            (proxy.to_string(), None)
        }
    };

    let mut curl = Command::new("curl");
    curl.arg("-s")
        .arg("-o")
        .arg("/dev/null")
        .arg("-w")
        .arg("%{http_code}")
        .arg("-x")
        .arg(&proxy_url)
        .arg("--connect-timeout")
        .arg("10");

    if let Some(auth) = auth {
        curl.arg("-U").arg(&auth);
    }

    curl.arg("https://api.telegram.org");

    match curl.output() {
        Ok(out) => {
            let code = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if out.status.success() && (code == "200" || code == "401" || code == "404" || code == "302") {
                log::info!("Proxy {} ({}) working! HTTP {}", proxy, proxy_type, code);
                true
            } else {
                log::warn!("Proxy {} ({}) failed. HTTP code: {}", proxy, proxy_type, code);
                false
            }
        }
        Err(e) => {
            log::error!("Failed to run curl: {}", e);
            false
        }
    }
}

fn proxy_formatter(proxy: &str) -> String {
    if let Some(pos) = proxy.find("://") {
        let scheme = &proxy[..pos + 3];
        let rest = &proxy[pos + 3..];
        let parts: Vec<&str> = rest.split(':').collect();
        
        if parts.len() == 4 {
            return format!("{}{}:{}@{}:{}", scheme, parts[2], parts[3], parts[0], parts[1]);
        }
    }
    proxy.to_string()
}