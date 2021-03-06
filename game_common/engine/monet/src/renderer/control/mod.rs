pub use descartes::{N, P3, P2, V3, V4, M4, Iso3, Persp3, Into2d, Into3d, WithUniqueOrthogonal};
use kay::{World, External};
use glium::Frame;

use super::{Renderer, RendererID};

impl Renderer {
    /// Critical
    pub fn setup(&mut self, world: &mut World) {
        for renderable in &self.scene.renderables {
            renderable.init(self.id, world);
        }
    }

    /// Critical
    pub fn prepare_render(&mut self, world: &mut World) {
        let self_id = self.id;
        let current_frame = self.current_frame;
        for renderable in &self.scene.renderables {
            renderable.prepare_render(self_id, current_frame, world);
        }
    }

    /// Critical
    pub fn render(&mut self, world: &mut World) {
        let self_id = self.id;
        let current_frame = self.current_frame;
        for renderable in &self.scene.renderables {
            renderable.render(self_id, current_frame, world);
        }
        self.current_frame += 1;
    }

    /// Critical
    pub fn submit(
        &mut self,
        given_target: &External<Frame>,
        return_to: TargetProviderID,
        world: &mut World,
    ) {
        let mut target = given_target.steal();
        let inner = &mut *self.inner;
        let scene = &inner.scene;
        let context = &mut inner.render_context;
        context.submit(scene, &mut *target);

        return_to.submitted(target, world);
    }
}

pub trait TargetProvider {
    fn submitted(&mut self, target: &External<Frame>, world: &mut World);
}

mod kay_auto;
pub use self::kay_auto::*;
