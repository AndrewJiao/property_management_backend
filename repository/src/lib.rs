pub mod models;
pub mod schema;


#[cfg(test)]
mod test {
    use common::db_config::establish_connection_str;

    use crate::models::{NewPost, Post};
    use crate::schema::posts::dsl::posts;
    use crate::schema::posts::published;
    use common::error::AppResult;
    use diesel::associations::HasTable;
    use diesel::r2d2::{ConnectionManager, PooledConnection};
    use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper, TextExpressionMethods};

    #[test]
    fn get_all_true() -> AppResult<()> {
        let connect = &mut get_connection();
        let results = posts
            .filter(published.eq(true))
            .limit(5)
            .select(Post::as_select())
            .load(connect)
            .expect("Error loading posts");

        println!("Displaying {} posts", results.len());
        for post in results {
            println!("{}", post.title);
            println!("-----------\n");
            println!("{}", post.body);
        }
        Ok(())
    }

    fn get_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
        let connection = &mut establish_connection_str("postgres://postgres:postgres@localhost:5432/property_management?options=-c%20search_path%3Dbasic");
        connection.get().expect("")
    }
    #[test]
    fn add_row() {
        let title = String::from("title_test");
        let body = String::from("body_test");


        println!("\nOk! Let's write {title} \n", );
        let post = create_post(&title, &body);
        println!("\nSaved draft {title} with id {}", post.id);
    }


    pub fn create_post(title: &str, body: &str) -> Post {
        let connection = &mut get_connection();


        let new_post = NewPost { title, body };

        diesel::insert_into(posts::table())
            .values(&new_post)
            .returning(Post::as_returning())
            .get_result(connection)
            .expect("Error saving new post")
    }

    #[test]
    fn update_row() {
        let connection = &mut get_connection();

        let post = diesel::update(posts.find(5))
            .set(published.eq(true))
            .returning(Post::as_returning())
            .get_result(connection)
            .unwrap();
        println!("Published post {}", post.title);
    }

    #[test]
    fn get_post() {
        let connect = &mut get_connection();
        let post_id = 5;
        let post = posts
            .find(post_id)
            .select(Post::as_select())
            .first(connect)
            .optional(); // This allows for returning an Option<Post>, otherwise it will throw an error

        match post {
            Ok(Some(post)) => println!("Post with id: {} has a title: {}", post.id, post.title),
            Ok(None) => println!("Unable to find post {}", post_id),
            Err(_) => println!("An error occured while fetching post {}", post_id),
        }
    }
    #[test]
    fn delete_post() {
        use crate::schema::posts::title;

        let pattern = format!("%{}%", "test");

        let connection = &mut get_connection();
        let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
            .execute(connection)
            .expect("Error deleting posts");

        println!("Deleted {} posts", num_deleted);
    }
}



