use super::*;
use warp::http::StatusCode;
use warp::test::request;

#[tokio::test]
async fn test_status() {
	let routes = todos_list();
	
	let resp = request()
		.method("GET")
		.path("/status")
		.reply(&routes)
		.await;
		
	assert_eq!(resp.status(), StatusCode::OK);
	assert_eq!(resp.body(), r##"{"result":true,"code":0,"description":"Everything is OK!"}"##);
}

#[tokio::test]
async fn test_get() {
	let routes = todos_list();
	
	let resp = request()
		.method("GET")
		.path("/data/v1/test")
		.reply(&routes)
		.await;
		
	assert_eq!(resp.status(), StatusCode::OK);
	assert_eq!(resp.body(), r##"{"result":true,"code":0,"description":"You requested get method with name: test"}"##);
}

#[tokio::test]
async fn test_post() {
	let routes = todos_list();
	let data = InputData{data: "test data".to_string()};
	
	let resp = request()
		.method("POST")
		.path("/data/v1/test")
		.json(&data)
		.reply(&routes)
		.await;
		
	assert_eq!(resp.status(), StatusCode::OK);
	assert_eq!(resp.body(), r##"{"result":true,"code":0,"description":"You requested post method with name: test, data is test data"}"##);
}

#[tokio::test]
async fn test_put() {
	let routes = todos_list();
	let data = InputData{data: "test data".to_string()};
	
	let resp = request()
		.method("PUT")
		.path("/data/v1/test")
		.json(&data)
		.reply(&routes)
		.await;
		
	assert_eq!(resp.status(), StatusCode::OK);
	assert_eq!(resp.body(), r##"{"result":true,"code":0,"description":"You requested put method with name: test, data is test data"}"##);
}

#[tokio::test]
async fn test_delete() {
	let routes = todos_list();
	let data = InputData{data: "test data".to_string()};
	
	let resp = request()
		.method("DELETE")
		.path("/data/v1/test")
		.json(&data)
		.reply(&routes)
		.await;
		
	assert_eq!(resp.status(), StatusCode::OK);
	assert_eq!(resp.body(), r##"{"result":true,"code":0,"description":"You requested delete method with name: test, data is test data"}"##);
}