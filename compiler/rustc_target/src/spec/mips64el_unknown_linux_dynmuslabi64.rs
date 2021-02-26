use crate::spec::Target;

pub fn target() -> Target {
    let mut base = super::mips64el_unknown_linux_muslabi64::target();

    base.llvm_target = "mips64el-unknown-linux-musl".to_string();
    base.options.crt_static_default = false;

    base
}
