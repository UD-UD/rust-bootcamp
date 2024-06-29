use blog_shared::Post;

fn main() {
    let post = Post::new("Hello, world!".to_string(), "This is my first post.".to_string());
    println!("{:?}", post);
}
