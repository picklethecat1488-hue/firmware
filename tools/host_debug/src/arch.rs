//! Processor architecture abstractions, single-sourced GDB register descriptors, and
//! Rinja template rendering for GDB `target.xml` feature discovery.

use rinja::Template;

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

/// Architecture-independent interface supplying register layout and XML descriptions to [`crate::rsp::RspCoreHandler`].
pub trait TargetArchProfile: Send + Sync {
    /// Underlying [`ProcessorArch`] variant.
    fn arch(&self) -> ProcessorArch;

    /// Rendered GDB `target.xml` document built dynamically from single-sourced register descriptors.
    fn target_xml(&self) -> &str;

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
}

/// Concrete register profile built from [`RegisterFeatureGroup`] descriptors and rendered via Rinja.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TargetRegisterLayout {
    arch: ProcessorArch,
    features: Vec<RegisterFeatureGroup>,
    flattened: Vec<RegisterDescriptor>,
    rendered_xml: String,
}

impl TargetRegisterLayout {
    /// Construct a [`TargetRegisterLayout`] and render its `target.xml` from `target.xml.jinja`.
    pub fn new(arch: ProcessorArch, features: Vec<RegisterFeatureGroup>) -> Self {
        let tpl = TargetXmlTemplate {
            architecture: arch.gdb_architecture_name(),
            features: &features,
        };
        let rendered_xml = tpl.render().expect("render target.xml.jinja");
        let flattened = features
            .iter()
            .flat_map(|f| f.registers.iter().cloned())
            .collect();
        Self {
            arch,
            features,
            flattened,
            rendered_xml,
        }
    }

    /// Feature groups comprising this register layout.
    pub fn features(&self) -> &[RegisterFeatureGroup] {
        &self.features
    }
}

impl TargetArchProfile for TargetRegisterLayout {
    fn arch(&self) -> ProcessorArch {
        self.arch
    }

    fn target_xml(&self) -> &str {
        &self.rendered_xml
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
