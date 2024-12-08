// use std::collections::HashMap;
// use tokio::io::{AsyncBufReadExt, BufReader};
//
// struct LineMatcher<'a> {
//     handler: HashMap<&'a str, Box<dyn FnOnce(String)>>,
// }
//
// trait Handler<I, O> {
//     fn handle(&mut self, data: I) -> O;
// }
//
// struct FunctionFactory<I, O, H>
// where
//     H: Handler<I, O>,
// {
//     handler: H,
// }
// impl<I, O, H> FunctionFactory<I, O, H> {
//     fn from(handler: H) -> FunctionFactory<String, O, H> {
//         FunctionFactory {
//             handler
//         }
//     }
//     fn build(&self) -> Box<dyn FnOnce(String)> {
//         todo!()
//     }
// }
//
//
//
// impl LineMatcher<'_> {
//     fn new() -> LineMatcher<'_> {
//         Self {
//             handler: HashMap::new(),
//         }
//     }
//     fn add(&mut self, line: &'_ str, f: FunctionFactory) -> &'_ mut Self {
//         self.handler.insert(line, f.build());
//         self
//     }
//
//     fn run(&self, s: String) {
//         if let Some(f) = self.handler.get(&s) {
//             f(s);
//         }
//     }
// }
//
// async fn command_line_interface() {
//     let handler = LineMatcher::new()
//         .add("device", FunctionFactory::new());
//
//     loop {
//         let mut lines = BufReader::new(tokio::io::stdin()).lines();
//         match lines.next_line().await {
//             Ok(Some(v)) => {
//                 handler.run(v);
//             }
//             Err(err) => {}
//             Ok(None) => (),
//         }
//     }
// }
