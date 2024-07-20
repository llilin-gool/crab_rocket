use crate::models::task::{PostTask, PatchTask, Task};
use crate::models::task_filter::TaskFilter;
use crab_rocket_schema::schema::task_table::dsl; //配合下面的 `tasks.filter()`
use crab_rocket_schema::schema::task_table::{self};
use crab_rocket_utils::time::get_e8_time;
use diesel::prelude::*;
use obj_traits::mapper::mapper_crud::MapperCRUD;
use obj_traits::request::pagination_request_param::{Pagination, PaginationParam};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;

pub struct TaskMapper {}

impl MapperCRUD for TaskMapper {
    type Item = Task;
    type PostItem = PostTask;
    type PatchItem = PatchTask;
    type Param = RequestParam<PaginationParam, TaskFilter>;
    fn get_all(
        conn: &mut PgConnection,
        param: &RequestParam<PaginationParam, TaskFilter>,
    ) -> Result<Data<Vec<Task>>, diesel::result::Error> {
        // 当前页码（page）
        // 每页条目数（per_page）
        //
        // 总页数（total_pages）
        //
        // 公式
        //
        // 当前页的 offset: (page - 1) * per_page
        //
        // 下一页的 offset: page * per_page
        //
        // 上一页的 offset: (page - 2) * per_page （如果 page > 1）
        //
        // limit 始终为 per_page
        // 计算分页相关
        let page = (param.pagination.offset.unwrap() / param.pagination.limit.unwrap()) + 1;
        let per_page = param.pagination.limit.unwrap();
        // 获取总记录数
        let total_count = dsl::task_table.count().get_result::<i64>(conn)? as i32;
        // 计算总页数
        let total_pages = (total_count + per_page - 1) / per_page;

        let previous_page_offset = (page - 2) * per_page;
        let next_page_offset = page * per_page;
        let pagination = Pagination::new(
            page,
            per_page,
            total_pages,
            total_count,
            Some(format!("?limit={}&offset={}", per_page, next_page_offset)),
            Some(format!("?limit={}&offset={}", per_page, previous_page_offset)),
        );

        // 分页查询
        let data = dsl::task_table
            .order(dsl::updated_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Task>(conn)?;
        let body = Data::new(data, pagination);
        println!("Getting tasks successfully.");
        Ok(body)
    }

    fn get_by_id(conn: &mut PgConnection, pid: i32) -> Result<Task, diesel::result::Error> {
        dsl::task_table.filter(task_table::task_id.eq(pid)).first(conn)
    }

    fn add_single(conn: &mut PgConnection, obj: &PostTask) -> Result<Task, diesel::result::Error> {
        match diesel::insert_into(task_table::table)
            .values(obj)
            .returning(Task::as_returning())
            .get_result(conn)
        {
            Ok(inserted_task) => Ok(inserted_task),
            Err(e) => {
                println!("{e:?}");
                Err(e)
            }
        }
    }

    fn delete_by_id(conn: &mut PgConnection, pid: i32) -> Result<Task, diesel::result::Error> {
        diesel::delete(dsl::task_table.filter(task_table::task_id.eq(pid))).get_result(conn)
    }

    fn update_by_id(
        conn: &mut PgConnection,
        pid: i32,
        obj: &PatchTask,
    ) -> Result<Task, diesel::result::Error> {
        diesel::update(dsl::task_table.filter(dsl::task_id.eq(pid)))
            .set((
                task_table::title.eq(obj.title()),
                task_table::content.eq(obj.content()),
                task_table::updated_at.eq(Some(get_e8_time())), //Update time
                task_table::user_id.eq(obj.user_id()),
            ))
            .get_result(conn)
    }
    fn filter(
        conn: &mut PgConnection,
        param: &RequestParam<PaginationParam, TaskFilter>,
    ) -> Result<Data<Vec<Task>>, diesel::result::Error> {
        // 当前页码（page）
        // 每页条目数（per_page）
        //
        // 总页数（total_pages）
        //
        // 公式
        //
        // 当前页的 offset: (page - 1) * per_page
        //
        // 下一页的 offset: page * per_page
        //
        // 上一页的 offset: (page - 2) * per_page （如果 page > 1）
        //
        // limit 始终为 per_page
        // 计算分页相关
        let page = (param.pagination.offset.unwrap() / param.pagination.limit.unwrap()) + 1;
        let per_page = param.pagination.limit.unwrap();
        // 获取总记录数
        let total_count = dsl::task_table.count().get_result::<i64>(conn)? as i32;
        // 计算总页数
        let total_pages = (total_count + per_page - 1) / per_page;

        let previous_page_offset = (page - 2) * per_page;
        let next_page_offset = page * per_page;
        let pagination = Pagination::new(
            page,
            per_page,
            total_pages,
            total_count,
            Some(format!("?limit={}&offset={}", per_page, next_page_offset)),
            Some(format!("?limit={}&offset={}", per_page, previous_page_offset)),
        );

        let mut query = dsl::task_table.into_boxed();

        // 分页查询
        query = query
            .order(dsl::created_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64);

        let filter = &param.filter;
        if let Some(f) = filter {
            if let Some(id) = &f.id {
                query = query.filter(dsl::task_id.eq(id));
            }
            if let Some(title) = &f.title {
                query = query.filter(dsl::title.like(format!("%{}%", title)));
            }
            if let Some(content) = &f.content {
                query = query.filter(dsl::content.like(format!("%{}%", content)));
            }
            if let Some(created_at_min) = &f.created_at_min {
                query = query.filter(dsl::created_at.ge(created_at_min));
            }
            if let Some(created_at_max) = &f.created_at_max {
                query = query.filter(dsl::created_at.le(created_at_max));
            }
            if let Some(updated_at_min) = &f.updated_at_min {
                query = query.filter(dsl::updated_at.ge(updated_at_min));
            }
            if let Some(updated_at_max) = &f.updated_at_max {
                query = query.filter(dsl::updated_at.le(updated_at_max));
            }
            if let Some(user_id) = &f.user_id {
                query = query.filter(dsl::user_id.eq(user_id));
            }
        }
        let data = query.load::<Task>(conn)?;
        let body = Data::new(data, pagination);
        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::TaskMapper;
    use crate::models::task::{PostTask, PatchTask};
    use crab_rocket_schema::establish_pg_connection;
    use obj_traits::mapper::mapper_crud::MapperCRUD;
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;

    #[test]
    fn test_insert_task() {
        match establish_pg_connection() {
            Ok(mut conn) => {
                let task = PostTask::new(
                    "title".to_string().into(),
                    "new content".to_string().into(),
                    Some(chrono::Local::now().naive_utc()),
                    Some(chrono::Local::now().naive_utc()),
                    Some(4),
                );
                let _ = super::TaskMapper::add_single(&mut conn, &task);
            }
            Err(_) => println!("establish_pg_connection error"),
        }
    }

    #[test]
    fn test_fetch_all_tasks() {
        match establish_pg_connection() {
            Ok(mut conn) => {
                let param = RequestParam::new(PaginationParam::demo(), None);
                let all_tasks = TaskMapper::get_all(&mut conn, &param).unwrap();
                let json_string = serde_json::to_string_pretty(&all_tasks).unwrap();
                println!("{json_string:?}");
            }
            Err(_) => println!("establish_pg_connection error"),
        }
    }

    #[test]
    fn test_fetch_task_by_id() {
        match establish_pg_connection() {
            Ok(mut conn) => {
                let t_id = 3;
                match super::TaskMapper::get_by_id(&mut conn, t_id) {
                    Ok(task) => println!("{task:?}"),
                    Err(_) => println!("err"),
                }
            }
            Err(_) => println!("establish_pg_connection error"),
        }
    }

    #[test]
    fn test_update_task_by_id() {
        match establish_pg_connection() {
            Ok(mut conn) => {
                let t_id = 1;
                let patch_task: PatchTask = PatchTask::new(
                    "title for put 1".to_string(),
                    "new content for put".to_string().into(),
                    Some(4),
                );
                match super::TaskMapper::update_by_id(&mut conn, t_id, &patch_task) {
                    Ok(res) => println!("{res:?}"),
                    Err(_) => println!("Err"),
                }
            }
            Err(_) => println!("establish_pg_connection error"),
        }
    }

    #[test]
    fn test_delete_task_by_id() {
        match establish_pg_connection() {
            Ok(mut conn) => {
                let t_id = 2;
                match super::TaskMapper::delete_by_id(&mut conn, t_id) {
                    Ok(res) => println!("{res:?}"),
                    Err(_) => println!("Err"),
                }
            }
            Err(_) => println!("establish_pg_connection error"),
        }
    }
}
