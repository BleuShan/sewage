#[sewage::test(runtime(tokio))]
pub async fn runtime_tokio() {}

#[sewage::test(runtime = async_std)]
pub async fn runtime_async_std() {}


fn main() {

}