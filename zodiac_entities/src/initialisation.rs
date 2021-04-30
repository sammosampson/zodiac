use shrev::*;
use legion::*;
use legion::systems::*;

use crate::*;

pub trait ApplicationBundleBuilder {
    fn description(&self) -> String;
    fn setup_build_systems(&self, builder: &mut Builder);
    fn setup_layout_systems(&self, builder: &mut Builder);
    fn setup_rendering_systems(&self, builder: &mut Builder);
    fn setup_cleanup_systems(&self, builder: &mut Builder);
    fn setup_final_functions(&self, builder: &mut Builder);
    fn setup_resources(&self, resources: &mut Resources, event_channel: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>;
}