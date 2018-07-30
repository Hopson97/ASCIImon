pub mod state_explore;

use game::UpdateResult;
use graphics::renderer::Renderer;

pub trait GameState {
    fn update(&mut self, input_args: &[&str]) -> Option<UpdateResult>;
    fn draw(&mut self, renderer: &mut Renderer);
}
