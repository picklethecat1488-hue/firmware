//! Processor architecture abstractions, single-sourced GDB register and memory-map descriptors,
//! and Rinja template rendering for GDB `target.xml` and `memory-map` XML feature discovery.

use rinja::Template;

/// Non-instruction System Control Space / `EXC_RETURN` address threshold on ARM Cortex-M (`0xF000_0000`).
pub const ARM_NON_CODE_ADDRESS_THRESHOLD: u64 = 0xF000_0000;

/// Processor instruction-set architecture variants supported by [`crate::server::DebugBackend`] implementations.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ProcessorArch {
    /// ARMv6-M (`Cortex-M0` / `Cortex-M0+`, e.g. RP2040).
    Armv6m,
    /// ARMv7-M / ARMv7E-M (`Cortex-M3` / `Cortex-M4` / `Cortex-M7`).
    Armv7m,
    /// ARMv8-M (`Cortex-M23` / `Cortex-M33`, e.g. NXP MCX N947).
    Armv8m,
}

impl ProcessorArch {
    /// Canonical `<architecture>` tag string expected by GDB `target.xml`.
    pub const fn gdb_architecture_name(&self) -> &'static str {
        match self {
            Self::Armv6m | Self::Armv7m | Self::Armv8m => "arm",
        }
    }

    /// Normalize an instruction pointer value (clearing ARM Thumb bit `0`).
    pub const fn normalize_pc(&self, raw_pc: u32) -> u32 {
        match self {
            Self::Armv6m | Self::Armv7m | Self::Armv8m => raw_pc & !1,
        }
    }

    /// Return `true` when `addr` lies in a reserved architectural non-instruction region (such as Cortex-M `EXC_RETURN`).
    pub const fn is_non_code_address(&self, addr: u64) -> bool {
        match self {
            Self::Armv6m | Self::Armv7m | Self::Armv8m => addr >= ARM_NON_CODE_ADDRESS_THRESHOLD,
        }
    }
}

/// GDB `<memory-map>` region classification (`rom`, `ram`, or `flash`).
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MemoryRegionKind {
    /// Read-only code/XIP region where GDB must use hardware breakpoints (`Z1`).
    Rom,
    /// Read/write SRAM or peripheral MMIO region.
    Ram,
    /// Programmable flash region with explicit sector geometry.
    Flash,
}

impl MemoryRegionKind {
    /// Canonical GDB `memory-map` DTD `type` attribute string.
    pub const fn as_xml_type(&self) -> &'static str {
        match self {
            Self::Rom => "rom",
            Self::Ram => "ram",
            Self::Flash => "flash",
        }
    }
}

/// Single architectural address-space region rendered into GDB `qXfer:memory-map:read::` XML.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct MemoryRegionDescriptor {
    /// GDB memory type (`rom`, `ram`, `flash`).
    pub kind: MemoryRegionKind,
    /// Inclusive start address of the region.
    pub start: u64,
    /// Length of the region in bytes.
    pub length: u64,
}

impl MemoryRegionDescriptor {
    /// Format `start` as a lowercase `0x`-prefixed hexadecimal string for GDB XML.
    pub fn start_hex(&self) -> String {
        format!("0x{:08x}", self.start)
    }

    /// Format `length` as a lowercase `0x`-prefixed hexadecimal string for GDB XML.
    pub fn length_hex(&self) -> String {
        format!("0x{:08x}", self.length)
    }
}

/// Single architectural register descriptor mapping GDB XML `regnum` to backend `hw_reg_id`.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct RegisterDescriptor {
    /// Register name emitted in GDB `target.xml` (e.g. `"r0"`, `"pc"`, `"xpsr"`).
    pub name: String,
    /// Bit width of the register (e.g. `32`).
    pub bitsize: u32,
    /// Canonical GDB XML `regnum` attribute (single source of truth for XML and `p`/`P` packets).
    pub gdb_regnum: u32,
    /// Optional legacy sequential GDB register index accepted by `p`/`P` before `target.xml` loads.
    pub gdb_alias_regnum: Option<u32>,
    /// Underlying debug-probe register selector ID (e.g. ARM DCRSR ID).
    pub hw_reg_id: u16,
    /// GDB XML `type` attribute (`"uint32"`, `"data_ptr"`, `"code_ptr"`, `"ieee_single"`, `"int"`).
    pub reg_type: &'static str,
    /// Official `gdb-target.dtd` `group` attribute (`"general"`, `"system"`, `"float"`).
    pub reg_group: &'static str,
    /// Whether this register represents the architectural Program Counter (`PC`).
    pub is_pc: bool,
}

impl RegisterDescriptor {
    /// Width of this register in bytes.
    pub const fn byte_width(&self) -> usize {
        (self.bitsize as usize).div_ceil(8)
    }
}

/// Named `<feature>` block in GDB `target.xml` grouping an ordered slice of [`RegisterDescriptor`]s.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct RegisterFeatureGroup {
    /// GDB XML feature namespace (e.g. `"org.gnu.gdb.arm.m-profile"`).
    pub name: &'static str,
    /// Ordered architectural registers belonging to this feature group.
    pub registers: Vec<RegisterDescriptor>,
}

#[derive(Template)]
#[template(path = "target.xml.jinja", escape = "none")]
struct TargetXmlTemplate<'a> {
    architecture: &'a str,
    features: &'a [RegisterFeatureGroup],
}

#[derive(Template)]
#[template(path = "memory_map.xml.jinja", escape = "none")]
struct MemoryMapXmlTemplate<'a> {
    regions: &'a [MemoryRegionDescriptor],
}

/// Parse expedited `T05` stop-reply GDB `regnum`s from a rendered GDB `target.xml` DOM tree.
///
/// Loads `target_xml` into a [`roxmltree::Document`] DOM and selects `<reg>` elements carrying
/// `group="general"` in document order; if the XML omits `group` attributes, falls back to `<reg>`
/// elements nested inside the primary (first) `<feature>` element.
pub fn parse_expedited_regnums_from_target_xml(target_xml: &str) -> Vec<u32> {
    let parse_opts = roxmltree::ParsingOptions {
        allow_dtd: true,
        ..Default::default()
    };
    let Ok(doc) = roxmltree::Document::parse_with_options(target_xml, parse_opts) else {
        return Vec::new();
    };

    let mut general_group_regnums = Vec::new();
    let mut first_feature_regnums = Vec::new();

    for (feature_idx, feature_node) in doc
        .descendants()
        .filter(|node| node.is_element() && node.has_tag_name("feature"))
        .enumerate()
    {
        for reg_node in feature_node
            .descendants()
            .filter(|node| node.is_element() && node.has_tag_name("reg"))
        {
            let Some(regnum) = reg_node
                .attribute("regnum")
                .and_then(|raw| raw.trim().parse::<u32>().ok())
            else {
                continue;
            };
            if reg_node.attribute("group") == Some("general") {
                general_group_regnums.push(regnum);
            }
            if feature_idx == 0 {
                first_feature_regnums.push(regnum);
            }
        }
    }

    if !general_group_regnums.is_empty() {
        general_group_regnums
    } else {
        first_feature_regnums
    }
}

/// Architecture-independent interface supplying register layout and XML descriptions to [`crate::rsp::RspCoreHandler`].
pub trait TargetArchProfile: Send + Sync {
    /// Underlying [`ProcessorArch`] variant.
    fn arch(&self) -> ProcessorArch;

    /// Rendered GDB `target.xml` document built dynamically from single-sourced register descriptors.
    fn target_xml(&self) -> &str;

    /// Rendered GDB `memory-map` XML document built dynamically from processor memory regions (or `""` if unspecified).
    fn memory_map_xml(&self) -> &str;

    /// Ordered slice of registers serialized in `g` / `G` general-register packets.
    fn general_registers(&self) -> &[RegisterDescriptor];

    /// Look up a [`RegisterDescriptor`] by its GDB `p` / `P` register number.
    fn lookup_gdb_register(&self, gdb_regnum: u32) -> Option<&RegisterDescriptor>;

    /// Return the [`RegisterDescriptor`] representing the Program Counter (`PC`), if defined.
    fn pc_register(&self) -> Option<&RegisterDescriptor>;

    /// Normalize an instruction address or `PC` register value for this architecture.
    fn normalize_pc(&self, raw_pc: u32) -> u32 {
        self.arch().normalize_pc(raw_pc)
    }

    /// Return `true` if `addr` lies in a non-executable architectural trap/control region.
    fn is_non_code_address(&self, addr: u64) -> bool {
        self.arch().is_non_code_address(addr)
    }
}

/// Concrete register and memory-map profile built from descriptors and rendered via Rinja templates.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TargetRegisterLayout {
    arch: ProcessorArch,
    features: Vec<RegisterFeatureGroup>,
    memory_regions: Vec<MemoryRegionDescriptor>,
    flattened: Vec<RegisterDescriptor>,
    rendered_xml: String,
    rendered_memory_map_xml: String,
}

impl TargetRegisterLayout {
    /// Construct a [`TargetRegisterLayout`] and render `target.xml` + `memory-map` XML via Rinja.
    pub fn new(
        arch: ProcessorArch,
        features: Vec<RegisterFeatureGroup>,
        memory_regions: Vec<MemoryRegionDescriptor>,
    ) -> Self {
        let tpl = TargetXmlTemplate {
            architecture: arch.gdb_architecture_name(),
            features: &features,
        };
        let rendered_xml = tpl.render().expect("render target.xml.jinja");
        let rendered_memory_map_xml = if memory_regions.is_empty() {
            String::new()
        } else {
            let mem_tpl = MemoryMapXmlTemplate {
                regions: &memory_regions,
            };
            mem_tpl.render().expect("render memory_map.xml.jinja")
        };
        let flattened = features
            .iter()
            .flat_map(|f| f.registers.iter().cloned())
            .collect();
        Self {
            arch,
            features,
            memory_regions,
            flattened,
            rendered_xml,
            rendered_memory_map_xml,
        }
    }

    /// Feature groups comprising this register layout.
    pub fn features(&self) -> &[RegisterFeatureGroup] {
        &self.features
    }

    /// Memory-map region descriptors comprising this processor layout.
    pub fn memory_regions(&self) -> &[MemoryRegionDescriptor] {
        &self.memory_regions
    }
}

impl TargetArchProfile for TargetRegisterLayout {
    fn arch(&self) -> ProcessorArch {
        self.arch
    }

    fn target_xml(&self) -> &str {
        &self.rendered_xml
    }

    fn memory_map_xml(&self) -> &str {
        &self.rendered_memory_map_xml
    }

    fn general_registers(&self) -> &[RegisterDescriptor] {
        &self.flattened
    }

    fn lookup_gdb_register(&self, gdb_regnum: u32) -> Option<&RegisterDescriptor> {
        self.flattened
            .iter()
            .find(|r| r.gdb_regnum == gdb_regnum || r.gdb_alias_regnum == Some(gdb_regnum))
    }

    fn pc_register(&self) -> Option<&RegisterDescriptor> {
        self.flattened.iter().find(|r| r.is_pc)
    }
}
