///
/// Does not compile due to the orphan rule.
/// The orphan rule states that you can implement a trait on a type as long as either the trait or the type is local to your crate.
///
/// impl PartialEq for u32 {
/// fn eq(&self, _other: &Self) -> bool {
///    // implementation goes here
///    true
/// }
///
/// hello world
pub fn hello() {
    println!("hello world");
}
