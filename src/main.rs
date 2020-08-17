use warp::Filter;
use serde::{Serialize, Deserialize};

#[cfg(test)] mod test;

#[derive(Serialize, Deserialize)]
struct OutputData {
	result: bool,
	code:   u32,
	description: String,
}

#[derive(Serialize, Deserialize)]
struct InputData {
	data: String,
}

pub fn todos_list() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	//GET /status
	let status_callback = warp::get()
		.and(warp::path("status"))
		.and(warp::get())
		.map(|| {
			let output_data = OutputData{result: true, code: 0, description: "Everything is OK!".to_string()};
			warp::reply::json(&output_data)
		});

	//GET /data/v1/:name
    let get_callback = warp::get()
        .and(warp::path("data"))
		.and(warp::path("v1"))
        .and(warp::path::param::<String>())
        .map(|name| {
			let output_data = OutputData {result: true, code: 0, description: format!("You requested get method with name: {}", name)};
            warp::reply::json(&output_data)
        });		
		
	//POST /data/v1/:name  {"data":"test data"}
    let post_callback = warp::post()
        .and(warp::path("data"))
		.and(warp::path("v1"))
        .and(warp::path::param::<String>())
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|name, data: InputData| {
			let output_data = OutputData {result: true, code: 0, description: format!("You requested post method with name: {}, data is {}", name, data.data)};
            warp::reply::json(&output_data)
        });
		
	//PUT /data/v1/:name  {"data":"test data"}
    let put_callback = warp::put()
        .and(warp::path("data"))
		.and(warp::path("v1"))
        .and(warp::path::param::<String>())
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|name, data: InputData| {
			let output_data = OutputData {result: true, code: 0, description: format!("You requested put method with name: {}, data is {}", name, data.data)};
            warp::reply::json(&output_data)
        });		
		
	//DELETE /data/v1/:name  {"data":"test data"}
    let delete_callback = warp::delete()
        .and(warp::path("data"))
		.and(warp::path("v1"))
        .and(warp::path::param::<String>())
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|name, data: InputData| {
			let output_data = OutputData {result: true, code: 0, description: format!("You requested delete method with name: {}, data is {}", name, data.data)};
            warp::reply::json(&output_data)
        });			
		
	status_callback.or(get_callback).or(post_callback).or(put_callback).or(delete_callback)	
}

#[tokio::main]
async fn main() {		
	let routes = todos_list();
	
    warp::serve(routes)
		.run(([0, 0, 0, 0], 8000))
		.await;
}
