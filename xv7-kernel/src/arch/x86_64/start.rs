use super::*;
use crate::ansi::{CtrlSeq, EraseParam};
use crate::memory::FRAME_ALLOCATOR;
use boot::{BootArgs, KernelEntryFn, BOOT_ARGS_MAGIC};
use cpuid::CpuId;
use x86_64::structures::paging::{FrameAllocator, FrameDeallocator};

#[used]
static BSS_ZERO_CHECK: u64 = 0;
#[used]
static DATA_NONZERO_CHECK: u64 = 0xFFFF_FFFF_FFFF_FFFF;

#[doc(hidden)]
#[allow(unused)]
const KERNEL_ENTRY_SIGNATURE_TYPE_CHECK: KernelEntryFn = _start;

#[no_mangle]
extern "sysv64" fn _start(args: &BootArgs) -> ! {
    assert_eq!(BSS_ZERO_CHECK, 0);
    assert_eq!(DATA_NONZERO_CHECK, 0xFFFF_FFFF_FFFF_FFFF);
    assert_eq!(
        args.magic, BOOT_ARGS_MAGIC,
        "BootArgs magic number check failed"
    );

    print!(
        "{}{}{}",
        CtrlSeq::EraseDisplay(Some(EraseParam::Entire)),
        CtrlSeq::CursorPosition(None, None),
        CtrlSeq::SelectGraphicRendition(None),
    );

    dbg!(args);

    paging::disable_identity_mapping();

    paging::init_frame_allocator(args);

    // Test our frame allocator.
    {
        let frame = FRAME_ALLOCATOR.lock().allocate_frame();
        dbg!(frame);
        let frame = FRAME_ALLOCATOR.lock().allocate_frame();
        dbg!(frame);
        let frame = FRAME_ALLOCATOR.lock().allocate_frame();

        FRAME_ALLOCATOR.lock().deallocate_frame(frame.unwrap());
    }

    println!("{:x}", unsafe {
        x86_64::registers::model_specific::Msr::new(0x1b).read()
    });

    let cpuid = CpuId::new();

    println!(
        "Vendor: {}",
        cpuid
            .get_vendor_info()
            .as_ref()
            .map_or_else(|| "unknown", |vf| vf.as_string(),)
    );

    println!(
        "CPU Model: {}",
        cpuid.get_extended_function_info().as_ref().map_or_else(
            || "n/a",
            |extfuninfo| extfuninfo.processor_brand_string().unwrap_or("unreadable"),
        )
    );

    cpuid.get_feature_info().as_ref().map_or_else(
        || println!("Family: {}, Extended Family: {}, Model: {}, Extended Model: {}, Stepping: {}, Brand Index: {}", "n/a", "n/a", "n/a", "n/a", "n/a", "n/a"),
        |finfo| {
            println!(
                "Family: {}, Extended Family: {}, Model: {}, Extended Model: {}, Stepping: {}, Brand Index: {}",
                finfo.family_id(),
                finfo.extended_family_id(),
                finfo.model_id(),
                finfo.extended_model_id(),
                finfo.stepping_id(),
                finfo.brand_index(),
            );
        },
    );

    gdt::init();
    interrupt::init();

    unsafe {
        pic::disable_8259_pic();
    }

    apic::lapic::init();

    interrupt::enable();

    crate::video::fun_things(args);

    crate::kmain();
}
