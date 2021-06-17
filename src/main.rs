mod wrapper;
mod control;
mod shorts;
mod types;
mod widgets;

fn main() {
    let mut wrapper = wrapper::main_wrapper::MainWrapper::new(Some(144));
    wrapper.run();
}
