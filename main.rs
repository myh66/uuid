use uuid::Uuid;

fn main() {
    // 生成一个随机版本的UUID (v4)
    let my_uuid = Uuid::new_v4();
    println!("随机生成的UUID是：{}", my_uuid);
}
