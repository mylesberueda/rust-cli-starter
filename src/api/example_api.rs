pub(crate) struct ExampleApi;

impl ExampleApi {
    pub(crate) fn foo() -> crate::Result<()> {
        println!("run from the api");
        Ok(())
    }
}
