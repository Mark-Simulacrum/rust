use crate::spec::{base, Cc, LinkerFlavor, Lld, StackProbeType, Target};

pub(crate) fn target() -> Target {
    let mut base = base::freebsd::opts();
    base.cpu = "pentium4".into();
    base.max_atomic_width = Some(64);
    base.add_pre_link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-m32", "-Wl,-znotext"]);
    base.stack_probes = StackProbeType::Inline;

    Target {
        llvm_target: "i686-unknown-freebsd".into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("32-bit FreeBSD".into()),
            tier: Some(2),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-\
            i128:128-f64:32:64-f80:32-n8:16:32-S128"
            .into(),
        arch: "x86".into(),
        options: base,
    }
}
