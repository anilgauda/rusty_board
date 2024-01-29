use axum::http::HeaderMap;

pub async fn user_agent(header_map: HeaderMap) -> String{
    dbg!("{:?}",header_map.clone());
    
    let user_agent_option= header_map.get("user-agent").unwrap();
    let user_agent_value = user_agent_option.to_str().unwrap().to_string();

    user_agent_value
}