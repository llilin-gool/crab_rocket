use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
use obj_traits::request::request_param::RequestParam;
use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};

use crate::controllers::order_controller::OrderController;
use crate::models::order::{PatchOrder, PostOrder};
use crate::models::order_filter::OrderFilter;

#[get("/order?<limit>&<offset>")]
pub fn get_orders(mut limit: Option<i32>, mut offset: Option<i32>) -> Json<serde_json::Value> {
    if limit.is_none() {
        limit = Some(10);
    };
    if offset.is_none() {
        offset = Some(0);
    };
    let params = RequestParam::new(PaginationParam::new(limit, offset), None);
    println!("{:?}", params);
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = OrderController::get_all(&params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}
#[post("/order/filter", data = "<param>")]
pub fn filter_orders(
    param: Option<Json<RequestParam<PaginationParam, OrderFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::new(PaginationParam::default(), None)));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = OrderController::filter(&param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/order/<id>")]
pub fn get_order_by_id(id: i32) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = OrderController::get_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/order", data = "<order>")]
pub fn insert_single_order(order: Json<PostOrder>) -> Json<serde_json::Value> {
    let mut obj: PostOrder = order.into_inner();

    let resp = OrderController::add_single(&mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/order/<id>")]
pub fn delete_order_by_id(id: i32) -> Json<serde_json::Value> {
    let resp = OrderController::delete_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/order/<id>", data = "<task>")]
pub fn update_order_by_id(id: i32, task: Json<PatchOrder>) -> Json<serde_json::Value> {
    let resp = OrderController::update_by_id(id, &task).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/order")]
pub fn options_order() -> Status {
    Status::Ok
}
