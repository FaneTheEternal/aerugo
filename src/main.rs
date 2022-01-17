mod wrapper;
mod control;
mod shorts;
mod types;
mod widgets;
mod fabula;

fn main() {
    // let mut wrapper = wrapper::main_wrapper::MainWrapper::new(Some(255));
    // wrapper.run();
    let aerugo = wrapper::aerugo_wrapper::Aerugo::new();
    aerugo.run().unwrap();
}
