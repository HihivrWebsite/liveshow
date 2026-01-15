use axum::{
    http::{Request, header::CONTENT_TYPE},
    middleware::Next,
    response::IntoResponse,
};
use std::time::Instant;
use tokio::time::timeout;
use std::time::Duration;

/// 性能监控中间件
pub async fn performance_monitor(
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let start = Instant::now();
    let uri = req.uri().clone();
    let method = req.method().clone();

    // 执行请求
    let response = next.run(req).await;

    let elapsed = start.elapsed();
    
    // 记录性能指标
    println!(
        "[PERFORMANCE] {} {} took {:?}ms",
        method,
        uri,
        elapsed.as_millis()
    );

    // 如果请求时间超过阈值，记录警告
    if elapsed.as_millis() > 5000 {
        println!(
            "[WARNING] Slow request detected: {} {} took {:?}ms",
            method,
            uri,
            elapsed.as_millis()
        );
    }

    Ok(response)
}

/// 安全中间件 - 输入验证和速率限制
pub async fn security_middleware(
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // 检查请求头
    let headers = req.headers();
    
    // 验证Content-Type（如果是POST请求）
    if req.method() == "POST" {
        if let Some(content_type) = headers.get(CONTENT_TYPE) {
            let content_type_str = content_type.to_str().unwrap_or("");
            if !content_type_str.starts_with("application/json") && 
               !content_type_str.starts_with("application/x-www-form-urlencoded") {
                return Err((
                    axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    "Unsupported media type".to_string()
                ));
            }
        }
    }

    // 验证URI长度（防止超长URI攻击）
    let uri_str = req.uri().to_string();
    if uri_str.len() > 2048 {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            "URI too long".to_string()
        ));
    }

    // 验证查询参数（防止超长参数攻击）
    if let Some(query) = req.uri().query() {
        if query.len() > 4096 {
            return Err((
                axum::http::StatusCode::BAD_REQUEST,
                "Query string too long".to_string()
            ));
        }
    }

    // 检查User-Agent（基本的安全检查）
    if let Some(user_agent) = headers.get("user-agent") {
        let ua_str = user_agent.to_str().unwrap_or("").to_lowercase();
        // 检查一些常见的恶意爬虫或工具
        if ua_str.contains("sqlmap") || 
           ua_str.contains("nmap") || 
           ua_str.contains("nikto") ||
           ua_str.contains("nessus") {
            println!("[SECURITY] Blocked suspicious User-Agent: {}", ua_str);
            return Err((
                axum::http::StatusCode::FORBIDDEN,
                "Access denied".to_string()
            ));
        }
    }

    // 执行请求
    let response = next.run(req).await;
    Ok(response)
}

/// 资源限制中间件
pub async fn resource_limit_middleware(
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // 设置请求超时
    let response = timeout(Duration::from_secs(30), next.run(req))
        .await
        .map_err(|_| {
            (
                axum::http::StatusCode::REQUEST_TIMEOUT,
                "Request timeout".to_string()
            )
        })?;

    Ok(response)
}

/// CORS安全配置
use tower_http::cors::{CorsLayer, AllowOrigin};
use axum::http::Method;

pub fn create_secure_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin([
            "http://localhost:2992".parse::<AllowOrigin>().unwrap(),
            "http://127.0.0.1:2992".parse::<AllowOrigin>().unwrap(),
            "https://localhost:2992".parse::<AllowOrigin>().unwrap(),
            "https://127.0.0.1:2992".parse::<AllowOrigin>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::OPTIONS])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
            axum::http::header::USER_AGENT,
        ])
        .max_age(Duration::from_secs(86400)) // 24小时缓存预检请求
}