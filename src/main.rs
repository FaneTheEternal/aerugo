mod wrapper;
mod control;
mod traits;
mod types;

fn main() {
    let mut wrapper = wrapper::main_wrapper::MainWrapper::new(None);
    wrapper.run();
}
