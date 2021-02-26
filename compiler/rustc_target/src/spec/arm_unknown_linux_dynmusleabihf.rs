use crate::spec::Target;
use crate::spec::crt_objects::new;

pub fn target() -> Target {
    let mut base = super::arm_unknown_linux_musleabihf::target();

    base.llvm_target = "armv6-unknown-linux-musleabihf".to_string();
    base.options.crt_static_default = false;
    base.options.pre_link_objects_fallback = new(&[]);
    base.options.post_link_objects_fallback = new(&[]);
    base.options.crt_objects_fallback = None;

    base
}
