use serde_json::json;
use zino::{ActixCluster, Application, Request, Response, Result};

pub(crate) async fn index(req: Request) -> Result {
    let res = Response::default().provide_context(&req);
    let stats = json!({
        "method": "GET",
        "path": "/stats",
        "app_state_data": ActixCluster::state_data(),
        "app_sysinfo": ActixCluster::sysinfo(),
    });
    let data = json!({
        "title": "Stats",
        "output": serde_json::to_string_pretty(&stats).unwrap_or_default(),
    });
    Ok(res.render("output.html", data).into())
}