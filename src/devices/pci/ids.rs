use alloc::format;

#[link_section = ".rodata"]
static DEVICE_TYPE: [(u8, &str, [(u8, &str); 1]); 5] = [
    (0x1, "Mass Storage Controller", [(0x0, "SCSI Bus Controller")]),
    (0x2, "Network Controller", [(0x0, "Ethernet Controller")]),
    (0x3, "Display Controller", [(0x0, "VGA Compatible Controller")]),
    (0x6, "Bridge", [(0x0, "Host Bridge")]),
    (0xC, "Serial Bus Controller", [(0x3, "USB Controller")])
];
///usr/share/hwdata/pci.ids
#[link_section = ".rodata"]
static PCI_IDS: &str = r#"
1af4    Red Hat, Inc.
    1000    Virtio network device
    1001    Virtio block device
    1002    Virtio memory balloon
    1003    Virtio console
    1004    Virtio SCSI
    1005    Virtio RNG
    1009    Virtio filesystem
    1041    Virtio network device
    1042    Virtio block device
    1043    Virtio console
    1044    Virtio RNG
    1045    Virtio memory balloon
    1048    Virtio SCSI
    1049    Virtio filesystem
    1050    Virtio GPU
    1052    Virtio input
    1053    Virtio socket
    105a    Virtio file system
    1110    Inter-VM shared memory
    1100    QEMU Virtual Machine
1b36    Red Hat, Inc.
    0001    QEMU PCI-PCI bridge
    0002    QEMU PCI 16550A Adapter
    0003    QEMU PCI Dual-port 16550A Adapter
    0004    QEMU PCI Quad-port 16550A Adapter
    0005    QEMU PCI Test Device
    0006    PCI Rocker Ethernet switch device
    0007    PCI SD Card Host Controller Interface
    0008    QEMU PCIe Host bridge
    0009    QEMU PCI Expander bridge
    000a    PCI-PCI bridge (multiseat)
    000b    QEMU PCIe Expander bridge
    000c    QEMU PCIe Root port
    000d    QEMU XHCI Host Controller
    0010    QEMU NVM Express Controller
    0100    QXL paravirtual graphic card
"#;

pub fn find(vendor: u16, device: u16) -> (&'static str, &'static str) {
    let mut vendor_name = "";
    let mut device_name = "";
    for line in PCI_IDS.lines() {
        if line.starts_with(&format!("{:04x}", vendor)) {
            vendor_name = line.split("    ").last().unwrap();
            device_name = "";
            continue;
        }
        if line.starts_with(&format!("    {:04x}", device)) {
            device_name = line.split("    ").last().unwrap();
            break;
        }
    }
    if vendor_name.is_empty() || device_name.is_empty() {
        vendor_name = "unknown";
        device_name = "unknown"
    }
    (vendor_name, device_name)
}

pub fn dev_type(base: u8, sub: u8) -> &'static str {
    let mut b_name = "";
    let mut s_name = "";
    for (base_id, base_name, sub_v) in DEVICE_TYPE {
        if base == base_id {
            b_name = base_name
        } else { continue; }
        for (s_i, s_n) in sub_v {
            if s_i == sub {
                s_name = s_n
            } else { break; }
        }
    }
    if !s_name.is_empty() {
        return s_name;
    } else if !b_name.is_empty() {
        return "";
    }
    return b_name;
}
