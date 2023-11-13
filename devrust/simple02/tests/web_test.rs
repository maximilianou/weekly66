#[cfg(test)]
mod tests {

    #[test]
    fn internal() {
        assert_eq!(4, 2 + 2);
    }

    use warp::http::StatusCode;
    use warp::test::request;
//    use warp::filters;

    #[tokio::test]
    async fn try_list() {
        let api = filters::list();

        let response = request()
            .method("GET")
            .path("/hello")
            .reply(&api)
            .await;

        assert_eq!(response.status(), StatusCode::ACCEPTED);
    }    
}

