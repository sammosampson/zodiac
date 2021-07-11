use log::info;
use legion::*;
use zodiac::*;
use crate::borders::*;

#[system(par_for_each)]
#[filter(component::<Rebuild>())]
pub fn compose_full_border(
    radius: &BorderRadius,
    top: &BorderTop,
    left: &BorderLeft,
    bottom: &BorderBottom,
    right: &BorderRight,
    full_border: &mut FullBorder
) {
    info!("composing full border {:?}", full_border);
    full_border.set((top, left, bottom, right, radius));
}

