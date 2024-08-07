use crab_rocket_utils::time::get_e8_time;
use obj_traits::{
    mapper::mapper_crud::MapperCRUD,
    request::{
        pagination_request_param::{Pagination, PaginationParam},
        request_param::RequestParam,
    },
    response::data::Data,
};

use crate::models::{
    supplier::{PatchSupplier, PostSupplier, Supplier},
    supplier_filter::SupplierFilter,
};
use crab_rocket_schema::schema::supplier_table::dsl;
use diesel::prelude::*;

pub struct SupplierMapper {}

impl MapperCRUD for SupplierMapper {
    type Item = Supplier;
    type PostItem = PostSupplier;
    type PatchItem = PatchSupplier;
    type Param = RequestParam<PaginationParam, SupplierFilter>;
    fn get_all(
        conn: &mut diesel::PgConnection,
        param: &Self::Param,
    ) -> Result<obj_traits::response::data::Data<Vec<Self::Item>>, diesel::result::Error> {
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
        let total_count = dsl::supplier_table.count().get_result::<i64>(conn)? as i32;
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
        // need to add macro QueryableByName to struct.
        // let custom: Vec<Supplier> =
        //     diesel::sql_query("SELECT * FROM supplier_table").load::<Supplier>(conn)?;

        // 分页查询
        let data = dsl::supplier_table
            .order(dsl::updated_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Supplier>(conn)?;
        let resp_body = Data::new(data, pagination);
        Ok(resp_body)
    }
    fn get_by_id(conn: &mut PgConnection, pid: i32) -> Result<Supplier, diesel::result::Error> {
        // 配合 use crate::schema::supplier_table::dsl::*;
        dsl::supplier_table.filter(dsl::supplier_id.eq(pid)).first(conn)
    }

    fn add_single(
        conn: &mut PgConnection,
        obj: &PostSupplier,
    ) -> Result<Supplier, diesel::result::Error> {
        match diesel::insert_into(dsl::supplier_table)
            .values(obj)
            .returning(Supplier::as_returning())
            .get_result(conn)
        {
            Ok(inserted_supplier) => Ok(inserted_supplier),
            Err(e) => Err(e),
        }
    }

    fn delete_by_id(conn: &mut PgConnection, pid: i32) -> Result<Supplier, diesel::result::Error> {
        diesel::delete(dsl::supplier_table.filter(dsl::supplier_id.eq(pid))).get_result(conn)
    }

    fn update_by_id(
        conn: &mut PgConnection,
        pid: i32,
        obj: &PatchSupplier,
    ) -> Result<Supplier, diesel::result::Error> {
        diesel::update(dsl::supplier_table.filter(dsl::supplier_id.eq(pid)))
            .set((
                dsl::name.eq(obj.name()),
                dsl::address.eq(obj.address()),
                dsl::phone_number.eq(obj.phone_number()),
                dsl::email.eq(obj.email()),
                dsl::created_at.eq(obj.created_at()),
                dsl::updated_at.eq(get_e8_time()),
            ))
            .get_result(conn)
    }
    fn filter(
        conn: &mut PgConnection,
        param: &RequestParam<PaginationParam, SupplierFilter>,
    ) -> Result<Data<Vec<Supplier>>, diesel::result::Error> {
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
        let total_count = dsl::supplier_table.count().get_result::<i64>(conn)? as i32;
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
        let mut query = dsl::supplier_table.into_boxed();
        let filter = &param.filter;
        if let Some(f) = filter {
            if let Some(supplier_id) = f.supplier_id {
                query = query.filter(dsl::supplier_id.eq(supplier_id));
            }
            if let Some(name) = &f.name {
                query = query.filter(dsl::name.like(format!("%{}%", name)));
            }
            if let Some(address) = &f.address {
                query = query.filter(dsl::address.like(format!("%{}%", address)));
            }
            if let Some(phone_number) = &f.phone_number {
                query = query.filter(dsl::phone_number.like(format!("%{}%", phone_number)));
            }
            if let Some(email) = &f.email {
                query = query.filter(dsl::email.like(format!("%{}%", email)));
            }
            if let Some(created_at_min) = f.created_at_min {
                query = query.filter(dsl::created_at.ge(created_at_min));
            }
            if let Some(created_at_max) = f.created_at_max {
                query = query.filter(dsl::created_at.le(created_at_max));
            }
            if let Some(updated_at_min) = f.updated_at_min {
                query = query.filter(dsl::updated_at.ge(updated_at_min));
            }
            if let Some(updated_at_max) = f.updated_at_max {
                query = query.filter(dsl::updated_at.le(updated_at_max));
            }
        }
        let data = query.load::<Supplier>(conn)?;
        let body = Data::new(data, pagination);
        Ok(body)
    }
}

mod test {

    #[test]
    fn test_fetch_all_supplier_table() {
        use crab_rocket_schema::establish_pg_connection;
        use obj_traits::{mapper::mapper_crud::MapperCRUD, request::request_param::RequestParam};

        use super::SupplierMapper;
        let param = RequestParam::default();
        match establish_pg_connection() {
            Ok(mut conn) => match SupplierMapper::get_all(&mut conn, &param) {
                Ok(data) => {
                    println!("{:#?}", data);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
