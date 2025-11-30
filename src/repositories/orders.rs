use sqlx::{Error, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::{
    models::orders::{Order, OrderItem},
    payloads::orders::PaginationPayload,
};

pub async fn insert(
    tx: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    total_price: i32,
) -> Result<Uuid, Error> {
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
            INSERT INTO orders (id, created_at, user_id, total_price)
            VALUES ($1, NOW(), $2, $3)
        "#,
        id,
        user_id,
        total_price
    )
    .execute(&mut **tx)
    .await
    .map(|_| id)
}

pub async fn insert_item(
    tx: &mut Transaction<'_, Postgres>,
    order_id: &Uuid,
    book_id: &Uuid,
    price: i32,
    amount: i32,
) -> Result<(), Error> {
    let item_id = Uuid::new_v4();
    sqlx::query!(
        r#"
            INSERT INTO order_items (id, order_id, book_id, price, amount)
            VALUES ($1, $2, $3, $4, $5)
        "#,
        item_id,
        order_id,
        book_id,
        price,
        amount
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn fetch_all(
    pool: &PgPool,
    user_id: &Uuid,
    p: PaginationPayload,
) -> Result<Vec<Order>, Error> {
    sqlx::query_as!(
        Order,
        r#"
            SELECT 
                    o.id,
                    o.created_at,
                    o.total_price,
                    ARRAY_AGG(
                        ROW(
                            i.id, i.price, i.amount,
                            b.id, b.title, b.author, b.publication_date
                        )
                    ) AS "items!: Vec<OrderItem>"
            FROM orders o
            JOIN order_items i ON o.id = i.order_id
            JOIN books b ON b.id = i.book_id
            WHERE o.user_id = $1
            GROUP BY o.id
            ORDER BY o.created_at DESC
            OFFSET $2 LIMIT $3
        "#,
        user_id,
        p.offset(),
        p.limit(),
    )
    .fetch_all(pool)
    .await
}
