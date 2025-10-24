/* dmi.rs
 *
 * Copyright 2025 Michail Krasnov <mskrasnov07@ya.ru>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

//! DMI table parser. Uses `smbios-lib` to data provider
//!
//! ## Usage
//!
//! ```no-test
//! use ferrix::dmi::DMITable;
//! let dmi = DMITable::new().unwrap();
//! dbg!(dmi);
//! ```

use crate::traits::ToJson;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
pub use smbioslib::SMBiosData;

/// A structure containing data from the DMI table
///
/// ## Usage
///
/// ```no-test
/// use ferrix::dmi::DMITable;
/// let dmi = DMITable::new().unwrap();
/// dbg!(dmi);
/// ```
#[derive(Debug, Serialize)]
pub struct DMITable {
    /// Information about BIOS (Type 0)
    pub bios: Bios,

    /// Information about system (Type 1)
    pub system: System,

    /// Information about baseboard (or module) - Type 2
    pub baseboard: Baseboard,

    /// Information about system enclosure or chassis - Type 3
    pub chassis: Chassis,

    /// Information about processor - Type 4
    pub processor: Processor,

    // /// Information about memory controller (Type 5)
    // pub memory_controller: MemoryController,
    //
    // /// Information about memory module (Type 6)
    // pub memory_module: MemoryModule,
    /// Information about CPU cache (Type 7)
    pub caches: Caches,

    /// Information about port connectors (Type 8)
    pub ports: PortConnectors,

    /// Information about physical memory array (Type 16)
    pub mem_array: MemoryArray,

    /// Information about installed memory devices (Type 17)
    pub mem_devices: MemoryDevices,
}

/// Each SMBIOS structure has a handle or instance value associated
/// with it. Some structs will reference other structures by using
/// this value.
#[derive(Debug, Serialize, Clone)]
pub struct Handle(pub u16);

impl From<smbioslib::Handle> for Handle {
    fn from(value: smbioslib::Handle) -> Self {
        Self(value.0)
    }
}

/* TODO:
 * - Type 5, 6, 10, 14m 15, 22, 23, 25, 27, 28, 29, 30, 31, 33, 34,
 *        35, 36, 37, 38, 39, 41, ... - в последнюю очередь
 * - Type 7i, 8i, 9i, 11, 12, 13, 16, 17i, 18i, 19, 20i, 21, 24, 26,
 *        32, 40, 41i, 133, 200, 248, 127 - в первую очередь
 */

impl DMITable {
    /// Get information from DMI table
    ///
    /// > **NOTE:** This data DOES NOT NEED to be updated periodically!
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Ok(Self {
            bios: Bios::new_from_table(&table)?,
            system: System::new_from_table(&table)?,
            baseboard: Baseboard::new_from_table(&table)?,
            chassis: Chassis::new_from_table(&table)?,
            processor: Processor::new_from_table(&table)?,
            caches: Caches::new_from_table(&table)?,
            ports: PortConnectors::new_from_table(&table)?,
            mem_array: MemoryArray::new_from_table(&table)?,
            mem_devices: MemoryDevices::new_from_table(&table)?,
        })
    }

    /// Performs serialization of structure data in JSON.
    ///
    /// The returned value will be a SINGLE LINE of JSON data
    /// intended for reading by third-party software or for
    /// transmission over the network.
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }

    /// Performs serialization in "pretty" JSON
    ///
    /// JSON will contain unnecessary newline transitions and spaces
    /// to visually separate the blocks. It is well suited for human
    /// reading and analysis.
    pub fn to_json_pretty(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self)?)
    }

    /// Performs data serialization in XML format
    pub fn to_xml(&self) -> Result<String> {
        let xml = DMITableXml::from(self);
        xml.to_xml()
    }
}

impl ToJson for DMITable {}

/****************************** NOTE *********************************
 * Дичайший костыль для того, чтобы структура XML была корректной    *
 * Если не обернуть данные в поле "hardware" (тег <hardware> в XML), *
 * то итоговый XML просто не распарсится.                            *
 *********************************************************************/
#[derive(Serialize, Clone)]
pub struct DMITableXml<'a> {
    pub hardware: &'a DMITable,
}

impl<'a> DMITableXml<'a> {
    pub fn to_xml(&self) -> Result<String> {
        Ok(xml_serde::to_string(&self)?)
    }
}

impl<'a> From<&'a DMITable> for DMITableXml<'a> {
    fn from(value: &'a DMITable) -> Self {
        Self { hardware: value }
    }
}

macro_rules! impl_from_struct {
    ($s:ident, $p:path, {
        $(
            $field_name:ident : $field_type:ty
        ),* $(,)?
    }) => {
        impl From<$p> for $s {
            fn from(value: $p) -> Self {
                Self {
                    $(
                        $field_name: value.$field_name,
                    )*
                }
            }
        }

        impl ToJson for $s {}
    };
}

/// BIOS ROM Size
#[derive(Debug, Serialize, Clone)]
pub enum RomSize {
    /// Size of this ROM in bytes
    Kilobytes(u16),

    /// Extended size of the physical device(s) containing the
    /// BIOS (in MB)
    Megabytes(u16),

    /// Extended size of the physical device(s) containing the
    /// BIOS (in GB)
    Gigabytes(u16),

    /// Extended size of the physical device(s) containing the
    /// BIOS in raw form.
    ///
    /// The standard currently only defines MB and GB as given
    /// in the high nibble (bits 15-14).
    Undefined(u16),

    SeeExtendedRomSize,
}

impl From<smbioslib::RomSize> for RomSize {
    fn from(value: smbioslib::RomSize) -> Self {
        match value {
            smbioslib::RomSize::Kilobytes(s) => Self::Kilobytes(s),
            smbioslib::RomSize::Megabytes(s) => Self::Megabytes(s),
            smbioslib::RomSize::Gigabytes(s) => Self::Gigabytes(s),
            smbioslib::RomSize::Undefined(s) => Self::Undefined(s),
            smbioslib::RomSize::SeeExtendedRomSize => Self::SeeExtendedRomSize,
        }
    }
}

/// Information about BIOS/UEFI
#[derive(Debug, Serialize, Clone)]
pub struct Bios {
    /// BIOS vendor's name
    pub vendor: Option<String>,

    /// BIOS version
    pub version: Option<String>,

    /// BIOS starting address segment
    pub starting_address_segment: Option<u16>,

    /// BIOS release date
    pub release_date: Option<String>,

    /// BIOS ROM size
    pub rom_size: Option<RomSize>,

    /// BIOS characteristics
    pub characteristics: Option<BiosCharacteristics>,

    /// BIOS vendor reserved characteristics
    pub bios_vendor_reserved_characteristics: Option<u16>,

    /// System vendor reserved characteristics
    pub system_vendor_reserved_characteristics: Option<u16>,

    /// Characteristics extension byte 0
    pub characteristics_extension0: Option<BiosCharacteristicsExtension0>,

    /// Characteristics extension byte 1
    pub characteristics_extension1: Option<BiosCharacteristicsExtension1>,

    /// System BIOS major release
    pub system_bios_major_release: Option<u8>,

    /// System BIOS minor release
    pub system_bios_minor_release: Option<u8>,

    /// Embedded controller firmware major release
    pub e_c_firmware_major_release: Option<u8>,

    /// Embedded controller firmware minor release
    pub e_c_firmware_minor_release: Option<u8>,

    /// Extended BIOS ROM size
    pub extended_rom_size: Option<RomSize>,
}

impl Bios {
    /// Creates a new instance of `Self`
    ///
    /// It is usually not required, since an instance of this
    /// structure will be created using the method
    /// [`Self::new_from_table(table: &SMBiosData)`] in the constructor
    /// [`DMITable::new()`].
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Self::new_from_table(&table)
    }

    pub fn new_from_table(table: &SMBiosData) -> Result<Self> {
        let t = table
            .find_map(|f: smbioslib::SMBiosInformation| Some(f))
            .ok_or(anyhow!("Failed to get information about BIOS (type 0)!"))?;

        Ok(Self {
            vendor: t.vendor().ok(),
            version: t.version().ok(),
            starting_address_segment: t.starting_address_segment(),
            release_date: t.release_date().ok(),
            rom_size: match t.rom_size() {
                Some(s) => Some(RomSize::from(s)),
                None => None,
            },
            characteristics: match t.characteristics() {
                Some(c) => Some(BiosCharacteristics::from(c)),
                None => None,
            },
            bios_vendor_reserved_characteristics: t.bios_vendor_reserved_characteristics(),
            system_vendor_reserved_characteristics: t.system_vendor_reserved_characteristics(),
            characteristics_extension0: match t.characteristics_extension0() {
                Some(ce0) => Some(BiosCharacteristicsExtension0::from(ce0)),
                None => None,
            },
            characteristics_extension1: match t.characteristics_extension1() {
                Some(ce1) => Some(BiosCharacteristicsExtension1::from(ce1)),
                None => None,
            },
            system_bios_major_release: t.system_bios_major_release(),
            system_bios_minor_release: t.system_bios_minor_release(),
            e_c_firmware_major_release: t.e_c_firmware_major_release(),
            e_c_firmware_minor_release: t.e_c_firmware_minor_release(),
            extended_rom_size: match t.extended_rom_size() {
                Some(s) => Some(RomSize::from(s)),
                None => None,
            },
        })
    }
}

impl ToJson for Bios {}

/// BIOS characteristics
#[derive(Debug, Serialize, Clone)]
pub struct BiosCharacteristics {
    /// Unknown
    pub unknown: bool,

    /// BIOS Characteristics aren't supported
    pub bios_characteristics_not_supported: bool,

    /// ISA is supported
    pub isa_supported: bool,

    /// MCA is supported
    pub mca_supported: bool,

    /// EISA is supported
    pub eisa_supported: bool,

    /// PCI is supported
    pub pci_supported: bool,

    /// PCMCIA is supported
    pub pcmcia_supported: bool,

    /// Plug-n-play is supported
    pub plug_and_play_supported: bool,

    /// APM is supported
    pub apm_supported: bool,

    /// BIOS is upgradeable (Flash)
    pub bios_upgradeable: bool,

    /// BIOS shadowing is allowed
    pub bios_shadowing_allowed: bool,

    /// VL-VESA is supported
    pub vlvesa_supported: bool,

    /// ESCD support is available
    pub escd_support_available: bool,

    /// Boot from CD is supported
    pub boot_from_cdsupported: bool,

    /// Selectable boot is supported
    pub selectable_boot_supported: bool,

    /// BIOS ROM is socketed (e.g. PLCC or SOP socket)
    pub bios_rom_socketed: bool,

    /// Boot from PCMCIA is supported
    pub boot_from_pcmcia_supported: bool,

    /// EDD specification is supported
    pub edd_specification_supported: bool,

    /// Japanese floppy for NEC 9800 1.2 MB (3.5", 1K bytes/sector,
    /// 360 RPM) is supported
    pub floppy_nec_japanese_supported: bool,

    /// Japanese floppy for Toshiba 1.2 MB (3.5", 360 RPM) is
    /// supported
    pub floppy_toshiba_japanese_supported: bool,

    /// 5.25" / 360 KB floppy services are supported
    pub floppy_525_360_supported: bool,

    /// 5.25" / 1.2 MB floppy services are supported
    pub floppy_525_12_supported: bool,

    /// 3.5" / 720 KB floppy services are supported
    pub floppy_35_720_supported: bool,

    /// 3.5" 2.88 MB floppy services are supported
    pub floppy_35_288_supported: bool,

    /// PrintScreen service is supported
    pub print_screen_service_supported: bool,

    /// 8042 keyboard services are supported
    pub keyboard_8042services_supported: bool,

    /// Serial services are supported
    pub serial_services_supported: bool,

    /// Printer services are supported
    pub printer_services_supported: bool,

    /// CGA/Mono Video Services are supported
    pub cga_mono_video_services_supported: bool,

    /// NEC PC-98 supported
    pub nec_pc_98supported: bool,
}

impl From<smbioslib::BiosCharacteristics> for BiosCharacteristics {
    fn from(value: smbioslib::BiosCharacteristics) -> Self {
        Self {
            unknown: value.unknown(),
            bios_characteristics_not_supported: value.bios_characteristics_not_supported(),
            isa_supported: value.isa_supported(),
            mca_supported: value.mca_supported(),
            eisa_supported: value.eisa_supported(),
            pci_supported: value.pci_supported(),
            pcmcia_supported: value.pcmcia_supported(),
            plug_and_play_supported: value.plug_and_play_supported(),
            apm_supported: value.apm_supported(),
            bios_upgradeable: value.bios_upgradeable(),
            bios_shadowing_allowed: value.bios_shadowing_allowed(),
            vlvesa_supported: value.vlvesa_supported(),
            escd_support_available: value.escd_support_available(),
            boot_from_cdsupported: value.boot_from_cdsupported(),
            selectable_boot_supported: value.selectable_boot_supported(),
            bios_rom_socketed: value.bios_rom_socketed(),
            boot_from_pcmcia_supported: value.boot_from_pcmcia_supported(),
            edd_specification_supported: value.edd_specification_supported(),
            floppy_nec_japanese_supported: value.floppy_nec_japanese_supported(),
            floppy_toshiba_japanese_supported: value.floppy_toshiba_japanese_supported(),
            floppy_525_360_supported: value.floppy_525_360_supported(),
            floppy_525_12_supported: value.floppy_525_12_supported(),
            floppy_35_720_supported: value.floppy_35_720_supported(),
            floppy_35_288_supported: value.floppy_35_288_supported(),
            print_screen_service_supported: value.print_screen_service_supported(),
            keyboard_8042services_supported: value.keyboard_8042services_supported(),
            serial_services_supported: value.serial_services_supported(),
            printer_services_supported: value.printer_services_supported(),
            cga_mono_video_services_supported: value.cga_mono_video_services_supported(),
            nec_pc_98supported: value.nec_pc_98supported(),
        }
    }
}
impl ToJson for BiosCharacteristics {}

/// Characteristics extension byte 0
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BiosCharacteristicsExtension0 {
    /// ACPI is supported
    pub acpi_is_supported: bool,

    /// USB Legacy is supported
    pub usb_legacy_is_supported: bool,

    /// AGP is supported
    pub agp_is_supported: bool,

    /// I2O boot is supported
    pub i2oboot_is_supported: bool,

    /// LS-120 SuperDisk boot is supported
    pub ls120super_disk_boot_is_supported: bool,

    /// ATAPI ZIP drive boot is supported
    pub atapi_zip_drive_boot_is_supported: bool,

    /// 1394 boot is supported
    pub boot_1394is_supported: bool,

    /// Smart battery is supported
    pub smart_battery_is_supported: bool,
}

impl From<smbioslib::BiosCharacteristicsExtension0> for BiosCharacteristicsExtension0 {
    fn from(value: smbioslib::BiosCharacteristicsExtension0) -> Self {
        Self {
            acpi_is_supported: value.acpi_is_supported(),
            usb_legacy_is_supported: value.usb_legacy_is_supported(),
            agp_is_supported: value.agp_is_supported(),
            i2oboot_is_supported: value.i2oboot_is_supported(),
            ls120super_disk_boot_is_supported: value.ls120super_disk_boot_is_supported(),
            atapi_zip_drive_boot_is_supported: value.atapi_zip_drive_boot_is_supported(),
            boot_1394is_supported: value.boot_1394is_supported(),
            smart_battery_is_supported: value.smart_battery_is_supported(),
        }
    }
}
impl ToJson for BiosCharacteristicsExtension0 {}

/// Characteristics extension byte 0
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BiosCharacteristicsExtension1 {
    /// BIOS Boot Specification is supported
    pub bios_boot_specification_is_supported: bool,

    /// Function key-initiated network service boot is supported.
    /// When function key-uninitiated network service boot is not
    /// supported, a network adapter option ROM may choose to offer
    /// this functionality on its own, thus offering this capability
    /// to legacy systems. When the function is supported, the
    /// network adapter option ROM shall not offer this capability
    pub fkey_initiated_network_boot_is_supported: bool,

    /// Enable targeted content distribution. The manufacturer has
    /// ensured that the SMBIOS data is useful in identifying the
    /// computer for targeted delivery of model-specific software
    /// and firmware content through third-party content
    /// distribution services
    pub targeted_content_distribution_is_supported: bool,

    /// UEFI Specification is supported
    pub uefi_specification_is_supported: bool,

    /// SMBIOS table describes a virtual machine
    pub smbios_table_describes_avirtual_machine: bool,

    /// Manufacturing mode is supported. (Manufacturing mode is a
    /// special boot mode, not normally available to end users, that
    /// modifies BIOS features and settings for use while the
    /// computer is being manufactured and tested)
    pub manufacturing_mode_is_supported: bool,

    /// Manufacturing mode is enabled
    pub manufacturing_mode_is_enabled: bool,
}

impl From<smbioslib::BiosCharacteristicsExtension1> for BiosCharacteristicsExtension1 {
    fn from(value: smbioslib::BiosCharacteristicsExtension1) -> Self {
        Self {
            bios_boot_specification_is_supported: value.bios_boot_specification_is_supported(),
            fkey_initiated_network_boot_is_supported: value
                .fkey_initiated_network_boot_is_supported(),
            targeted_content_distribution_is_supported: value
                .targeted_content_distribution_is_supported(),
            uefi_specification_is_supported: value.uefi_specification_is_supported(),
            smbios_table_describes_avirtual_machine: value
                .smbios_table_describes_avirtual_machine(),
            manufacturing_mode_is_supported: value.manufacturing_mode_is_supported(),
            manufacturing_mode_is_enabled: value.manufacturing_mode_is_enabled(),
        }
    }
}
impl ToJson for BiosCharacteristicsExtension1 {}

/// System UUID Data
#[derive(Debug, Serialize, Clone)]
pub enum SystemUuidData {
    IdNotPresentButSettable,
    IdNotPresent,
    Uuid(SystemUuid),
}

impl From<smbioslib::SystemUuidData> for SystemUuidData {
    fn from(value: smbioslib::SystemUuidData) -> Self {
        match value {
            smbioslib::SystemUuidData::IdNotPresentButSettable => Self::IdNotPresentButSettable,
            smbioslib::SystemUuidData::IdNotPresent => Self::IdNotPresent,
            smbioslib::SystemUuidData::Uuid(u) => Self::Uuid(SystemUuid::from(u)),
        }
    }
}

/// System UUID
#[derive(Debug, Serialize, Clone)]
pub struct SystemUuid {
    /// Raw byte array for this UUID
    pub raw: [u8; 16],
}

impl_from_struct!(SystemUuid, smbioslib::SystemUuid, {
    raw: [u8; 16],
});

/// System wakeup data
#[derive(Debug, Serialize, Clone)]
pub struct SystemWakeUpTypeData {
    /// Raw value
    ///
    /// Is most usable when `value` is None
    pub raw: u8,

    pub value: SystemWakeUpType,
}

impl From<smbioslib::SystemWakeUpTypeData> for SystemWakeUpTypeData {
    fn from(value: smbioslib::SystemWakeUpTypeData) -> Self {
        Self {
            raw: value.raw,
            value: SystemWakeUpType::from(value.value),
        }
    }
}

/// System wakeup type
#[derive(Debug, Serialize, Clone)]
pub enum SystemWakeUpType {
    Other,
    Unknown,
    ApmTimer,
    ModernRing,
    LanRemote,
    PowerSwitch,
    PciPme,
    ACPowerRestored,
    None,
}

impl From<smbioslib::SystemWakeUpType> for SystemWakeUpType {
    fn from(value: smbioslib::SystemWakeUpType) -> Self {
        match value {
            smbioslib::SystemWakeUpType::Other => Self::Other,
            smbioslib::SystemWakeUpType::Unknown => Self::Unknown,
            smbioslib::SystemWakeUpType::ApmTimer => Self::ApmTimer,
            smbioslib::SystemWakeUpType::ModernRing => Self::ModernRing,
            smbioslib::SystemWakeUpType::LanRemote => Self::LanRemote,
            smbioslib::SystemWakeUpType::PowerSwitch => Self::PowerSwitch,
            smbioslib::SystemWakeUpType::PciPme => Self::PciPme,
            smbioslib::SystemWakeUpType::ACPowerRestored => Self::ACPowerRestored,
            smbioslib::SystemWakeUpType::None => Self::None,
        }
    }
}

/// Attributes of the overall system
#[derive(Debug, Serialize, Clone)]
pub struct System {
    /// System manufacturer
    pub manufacturer: Option<String>,

    /// System product name
    pub product_name: Option<String>,

    /// System version
    pub version: Option<String>,

    /// Serial number
    pub serial_number: Option<String>,

    /// System UUID
    pub uuid: Option<SystemUuidData>,

    /// Wake-up type
    ///
    /// Identifies the event that caused the system to power up
    pub wakeup_type: Option<SystemWakeUpTypeData>,

    /// SKU Number (particular computer information for sale.
    /// Also called a product ID or purchase order number.
    /// Typically for a given system board from a given OEM,
    /// there are tens of unique processor, memory, hard
    /// drive, and optical drive configurations).
    pub sku_number: Option<String>,

    /// Family to which a particular computer belongs
    pub family: Option<String>,
}

impl System {
    /// Creates a new instance of `Self`
    ///
    /// It is usually not required, since an instance of this
    /// structure will be created using the method
    /// `Self::new_from_table(table: &SMBiosData)` in the constructor
    /// [`DMITable::new()`].
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Self::new_from_table(&table)
    }

    pub fn new_from_table(table: &SMBiosData) -> Result<Self> {
        let t = table
            .find_map(|f: smbioslib::SMBiosSystemInformation| Some(f))
            .ok_or(anyhow!("Failed to get information about system (type 1)!"))?;

        Ok(Self {
            manufacturer: t.manufacturer().ok(),
            product_name: t.product_name().ok(),
            version: t.version().ok(),
            serial_number: t.serial_number().ok(),
            uuid: match t.uuid() {
                Some(u) => Some(SystemUuidData::from(u)),
                None => None,
            },
            wakeup_type: match t.wakeup_type() {
                Some(wt) => Some(SystemWakeUpTypeData::from(wt)),
                None => None,
            },
            sku_number: t.sku_number().ok(),
            family: t.family().ok(),
        })
    }
}

impl ToJson for System {}

/// Board type data
#[derive(Debug, Serialize, Clone)]
pub struct BoardTypeData {
    pub raw: u8,
    pub value: BoardType,
}

impl From<smbioslib::BoardTypeData> for BoardTypeData {
    fn from(value: smbioslib::BoardTypeData) -> Self {
        Self {
            raw: value.raw,
            value: BoardType::from(value.value),
        }
    }
}

/// Board type
#[derive(Debug, Serialize, Clone)]
pub enum BoardType {
    Unknown,
    Other,
    ServerBlade,
    ConnectivitySwitch,
    SystemManagementModule,
    ProcessorModule,
    IOModule,
    MemoryModule,
    Daughterboard,
    Motherboard,
    ProcessorMemoryModule,
    ProcessorIOModule,
    InterconnectBoard,
    None,
}

impl From<smbioslib::BoardType> for BoardType {
    fn from(value: smbioslib::BoardType) -> Self {
        match value {
            smbioslib::BoardType::Unknown => Self::Unknown,
            smbioslib::BoardType::Other => Self::Other,
            smbioslib::BoardType::ServerBlade => Self::ServerBlade,
            smbioslib::BoardType::ConnectivitySwitch => Self::ConnectivitySwitch,
            smbioslib::BoardType::SystemManagementModule => Self::SystemManagementModule,
            smbioslib::BoardType::ProcessorModule => Self::ProcessorModule,
            smbioslib::BoardType::IOModule => Self::IOModule,
            smbioslib::BoardType::MemoryModule => Self::MemoryModule,
            smbioslib::BoardType::Daughterboard => Self::Daughterboard,
            smbioslib::BoardType::Motherboard => Self::Motherboard,
            smbioslib::BoardType::ProcessorMemoryModule => Self::ProcessorMemoryModule,
            smbioslib::BoardType::ProcessorIOModule => Self::ProcessorIOModule,
            smbioslib::BoardType::InterconnectBoard => Self::InterconnectBoard,
            smbioslib::BoardType::None => Self::None,
        }
    }
}

/// Information about baseboard/module
#[derive(Debug, Serialize, Clone)]
pub struct Baseboard {
    /// Baseboard manufacturer
    pub manufacturer: Option<String>,

    /// Baseboard product
    pub product: Option<String>,

    /// Baseboard serial number
    pub serial_number: Option<String>,

    /// Asset tag
    pub asset_tag: Option<String>,

    /// Baseboard feature flags
    pub feature_flags: Option<BaseboardFeatures>,

    /// The board's location within the chassis
    pub location_in_chassis: Option<String>,

    /// Handle, or instance number, associated with the chassis in
    /// which this board resides.
    pub chassis_handle: Option<Handle>,

    /// Type of baseboard
    pub board_type: Option<BoardTypeData>,
}

impl Baseboard {
    /// Creates a new instance of `Self`
    ///
    /// It is usually not required, since an instance of this
    /// structure will be created using the method
    /// `Self::new_from_table(table: &SMBiosData)` in the constructor
    /// [`DMITable::new()`].
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Self::new_from_table(&table)
    }

    pub fn new_from_table(table: &SMBiosData) -> Result<Self> {
        let t = table
            .find_map(|f: smbioslib::SMBiosBaseboardInformation| Some(f))
            .ok_or(anyhow!(
                "Failed to get information about baseboard/module (type 2)!"
            ))?;
        Ok(Self {
            manufacturer: t.manufacturer().ok(),
            product: t.product().ok(),
            serial_number: t.serial_number().ok(),
            asset_tag: t.asset_tag().ok(),
            feature_flags: match t.feature_flags() {
                Some(ff) => Some(BaseboardFeatures::from(ff)),
                None => None,
            },
            location_in_chassis: t.location_in_chassis().ok(),
            chassis_handle: match t.chassis_handle() {
                Some(h) => Some(Handle::from(h)),
                None => None,
            },
            board_type: match t.board_type() {
                Some(bt) => Some(BoardTypeData::from(bt)),
                None => None,
            },
        })
    }
}

impl ToJson for Baseboard {}

/// Baseboard feature flags
///
/// Collection of flags that identify features of this board
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BaseboardFeatures {
    /// Set if the board is a hosting board (e.g. motherboard)
    pub hosting_board: bool,

    /// Set if the board requires at least one daughter board or
    /// auxiliary card to function properly
    pub requires_daughterboard: bool,

    /// Set if the board is removable; it is designed to be taken in
    /// and out of the chassis without impairing the function of
    /// the chassis
    pub is_removable: bool,

    /// Set if the board is replaceable; it is possible to replace
    /// (either as a field repair or as an upgrade) the board with a
    /// physically different board. The board is inherently removable
    pub is_replaceable: bool,

    /// Set if the board if hot swappable; it is possible to replace
    /// the board with a physically different but equivalent board
    /// while power is applied to the board. The board is
    /// inherently replaceable and removable.
    pub is_hot_swappable: bool,
}

impl From<smbioslib::BaseboardFeatures> for BaseboardFeatures {
    fn from(value: smbioslib::BaseboardFeatures) -> Self {
        Self {
            hosting_board: value.hosting_board(),
            requires_daughterboard: value.requires_daughterboard(),
            is_removable: value.is_removable(),
            is_replaceable: value.is_replaceable(),
            is_hot_swappable: value.is_hot_swappable(),
        }
    }
}
impl ToJson for BaseboardFeatures {}

/// Chassis type data
#[derive(Debug, Serialize, Clone)]
pub struct ChassisTypeData {
    pub raw: u8,
    pub value: ChassisType,
    pub lock_presence: ChassisLockPresence,
}

impl From<smbioslib::ChassisTypeData> for ChassisTypeData {
    fn from(value: smbioslib::ChassisTypeData) -> Self {
        Self {
            raw: value.raw,
            value: ChassisType::from(value.value),
            lock_presence: ChassisLockPresence::from(value.lock_presence),
        }
    }
}

/// Chassis type
#[derive(Debug, Serialize, Clone)]
pub enum ChassisType {
    Other,
    Unknown,
    Desktop,
    LowProfileDesktop,
    PizzaBox,
    MiniTower,
    Tower,
    Portable,
    Laptop,
    Notebook,
    HandHeld,
    DockingStation,
    AllInOne,
    SubNotebook,
    SpaceSaving,
    LunchBox,
    MainServerChassis,
    ExpansionChassis,
    SubChassis,
    BusExpansionChassis,
    PeripheralChassis,
    RaidChassis,
    RackMountChassis,
    SealedCasePC,
    MultiSystemChassis,
    CompactPci,
    AdvancedTca,
    Blade,
    BladeEnclosure,
    Tablet,
    Convertible,
    Detachable,
    IoTGateway,
    EmbeddedPC,
    MiniPC,
    StickPC,
    None,
}

impl From<smbioslib::ChassisType> for ChassisType {
    fn from(value: smbioslib::ChassisType) -> Self {
        match value {
            smbioslib::ChassisType::Other => Self::Other,
            smbioslib::ChassisType::Unknown => Self::Unknown,
            smbioslib::ChassisType::Desktop => Self::Desktop,
            smbioslib::ChassisType::LowProfileDesktop => Self::LowProfileDesktop,
            smbioslib::ChassisType::PizzaBox => Self::PizzaBox,
            smbioslib::ChassisType::MiniTower => Self::MiniTower,
            smbioslib::ChassisType::Tower => Self::Tower,
            smbioslib::ChassisType::Portable => Self::Portable,
            smbioslib::ChassisType::Laptop => Self::Laptop,
            smbioslib::ChassisType::Notebook => Self::Notebook,
            smbioslib::ChassisType::HandHeld => Self::HandHeld,
            smbioslib::ChassisType::DockingStation => Self::DockingStation,
            smbioslib::ChassisType::AllInOne => Self::AllInOne,
            smbioslib::ChassisType::SubNotebook => Self::SubNotebook,
            smbioslib::ChassisType::SpaceSaving => Self::SpaceSaving,
            smbioslib::ChassisType::LunchBox => Self::LunchBox,
            smbioslib::ChassisType::MainServerChassis => Self::MainServerChassis,
            smbioslib::ChassisType::ExpansionChassis => Self::ExpansionChassis,
            smbioslib::ChassisType::SubChassis => Self::SubChassis,
            smbioslib::ChassisType::BusExpansionChassis => Self::BusExpansionChassis,
            smbioslib::ChassisType::PeripheralChassis => Self::PeripheralChassis,
            smbioslib::ChassisType::RaidChassis => Self::RaidChassis,
            smbioslib::ChassisType::RackMountChassis => Self::RackMountChassis,
            smbioslib::ChassisType::SealedCasePC => Self::SealedCasePC,
            smbioslib::ChassisType::MultiSystemChassis => Self::MultiSystemChassis,
            smbioslib::ChassisType::CompactPci => Self::CompactPci,
            smbioslib::ChassisType::AdvancedTca => Self::AdvancedTca,
            smbioslib::ChassisType::Blade => Self::Blade,
            smbioslib::ChassisType::BladeEnclosure => Self::BladeEnclosure,
            smbioslib::ChassisType::Tablet => Self::Tablet,
            smbioslib::ChassisType::Convertible => Self::Convertible,
            smbioslib::ChassisType::Detachable => Self::Detachable,
            smbioslib::ChassisType::IoTGateway => Self::IoTGateway,
            smbioslib::ChassisType::EmbeddedPC => Self::EmbeddedPC,
            smbioslib::ChassisType::MiniPC => Self::MiniPC,
            smbioslib::ChassisType::StickPC => Self::StickPC,
            smbioslib::ChassisType::None => Self::None,
        }
    }
}

/// Chassis lock presence
#[derive(Debug, Serialize, Clone)]
pub enum ChassisLockPresence {
    Present,
    NotPresent,
}

impl From<smbioslib::ChassisLockPresence> for ChassisLockPresence {
    fn from(value: smbioslib::ChassisLockPresence) -> Self {
        match value {
            smbioslib::ChassisLockPresence::Present => Self::Present,
            smbioslib::ChassisLockPresence::NotPresent => Self::NotPresent,
        }
    }
}

/// Chassis state data
#[derive(Debug, Serialize, Clone)]
pub struct ChassisStateData {
    pub raw: u8,
    pub value: ChassisState,
}

impl From<smbioslib::ChassisStateData> for ChassisStateData {
    fn from(value: smbioslib::ChassisStateData) -> Self {
        Self {
            raw: value.raw,
            value: ChassisState::from(value.value),
        }
    }
}

/// Chassis state
#[derive(Debug, Serialize, Clone)]
pub enum ChassisState {
    Other,
    Unknown,
    Safe,
    Warning,
    Critical,
    NonRecoverable,
    None,
}

impl From<smbioslib::ChassisState> for ChassisState {
    fn from(value: smbioslib::ChassisState) -> Self {
        match value {
            smbioslib::ChassisState::Other => Self::Other,
            smbioslib::ChassisState::Unknown => Self::Unknown,
            smbioslib::ChassisState::Safe => Self::Safe,
            smbioslib::ChassisState::Warning => Self::Warning,
            smbioslib::ChassisState::Critical => Self::Critical,
            smbioslib::ChassisState::NonRecoverable => Self::NonRecoverable,
            smbioslib::ChassisState::None => Self::None,
        }
    }
}

/// Chassis security status data
#[derive(Debug, Serialize, Clone)]
pub struct ChassisSecurityStatusData {
    pub raw: u8,
    pub value: ChassisSecurityStatus,
}

impl From<smbioslib::ChassisSecurityStatusData> for ChassisSecurityStatusData {
    fn from(value: smbioslib::ChassisSecurityStatusData) -> Self {
        Self {
            raw: value.raw,
            value: ChassisSecurityStatus::from(value.value),
        }
    }
}

/// Chassis security status
#[derive(Debug, Serialize, Clone)]
pub enum ChassisSecurityStatus {
    Other,
    Unknown,
    StatusNone,
    ExternalInterfaceLockedOut,
    ExternalInterfaceEnabled,
    None,
}

impl From<smbioslib::ChassisSecurityStatus> for ChassisSecurityStatus {
    fn from(value: smbioslib::ChassisSecurityStatus) -> Self {
        match value {
            smbioslib::ChassisSecurityStatus::Other => Self::Other,
            smbioslib::ChassisSecurityStatus::Unknown => Self::Unknown,
            smbioslib::ChassisSecurityStatus::StatusNone => Self::StatusNone,
            smbioslib::ChassisSecurityStatus::ExternalInterfaceLockedOut => {
                Self::ExternalInterfaceLockedOut
            }
            smbioslib::ChassisSecurityStatus::ExternalInterfaceEnabled => {
                Self::ExternalInterfaceEnabled
            }
            smbioslib::ChassisSecurityStatus::None => Self::None,
        }
    }
}

/// Chassis height
#[derive(Debug, Serialize, Clone)]
pub enum ChassisHeight {
    Unspecified,
    U(u8),
}

impl From<smbioslib::ChassisHeight> for ChassisHeight {
    fn from(value: smbioslib::ChassisHeight) -> Self {
        match value {
            smbioslib::ChassisHeight::Unspecified => Self::Unspecified,
            smbioslib::ChassisHeight::U(u) => Self::U(u),
        }
    }
}

/// Number of Power Cords
#[derive(Debug, Serialize, Clone)]
pub enum PowerCords {
    Unspecified,
    Count(u8),
}

impl From<smbioslib::PowerCords> for PowerCords {
    fn from(value: smbioslib::PowerCords) -> Self {
        match value {
            smbioslib::PowerCords::Unspecified => Self::Unspecified,
            smbioslib::PowerCords::Count(cnt) => Self::Count(cnt),
        }
    }
}

/// Information about system enclosure or chassis
#[derive(Debug, Serialize, Clone)]
pub struct Chassis {
    /// Enclosure/chassis manufacturer
    pub manufacturer: Option<String>,

    /// Chassis type
    pub chassis_type: Option<ChassisTypeData>,

    /// Version
    pub version: Option<String>,

    /// Serial number
    pub serial_number: Option<String>,

    /// Asset tag number
    pub asset_tag_number: Option<String>,

    /// State of the enclosure whet it was last booted
    pub bootup_state: Option<ChassisStateData>,

    /// State of the enclosure's power supply when last booted
    pub power_supply_state: Option<ChassisStateData>,

    /// Thermal state of the enclosure when last booted
    pub thermal_state: Option<ChassisStateData>,

    /// Physical security status of the enclosure when last booted
    pub security_status: Option<ChassisSecurityStatusData>,

    /// OEM- or BIOS vendor-specific information
    pub oem_defined: Option<u32>,

    /// Height of the enclosure, in 'U's
    ///
    /// A U is a standard unit of measure for the height of a rack
    /// or rack-mountable component and is equal to 1.75 inches or
    /// 4.445 cm
    pub height: Option<ChassisHeight>,

    /// Number of power cords associated with the enclosure/chassis
    pub number_of_power_cords: Option<PowerCords>,

    /// Number of Contained Element records that follow, in the
    /// range 0 to 255 Each Contained Element group comprises m
    /// bytes, as specified by the Contained Element Record Length
    /// field that follows. If no Contained Elements are included,
    /// this field is set to 0.
    pub contained_element_count: Option<u8>,

    /// Byte length of eact Contained Element record that follows,
    /// in the range 0 to 255. If no Contained Elements are included,
    /// this field is set to 0
    pub contained_element_record_length: Option<u8>,

    // TODO: this struct doesn't included ContainedElements<'_> iter!
    /// Chassis or enclosure SKU number
    pub sku_number: Option<String>,
}

impl Chassis {
    /// Creates a new instance of `Self`
    ///
    /// It is usually not required, since an instance of this
    /// structure will be created using the method
    /// `Self::new_from_table(table: &SMBiosData)` in the constructor
    /// [`DMITable::new()`].
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Self::new_from_table(&table)
    }

    pub fn new_from_table(table: &SMBiosData) -> Result<Self> {
        let t = table
            .find_map(|f: smbioslib::SMBiosSystemChassisInformation| Some(f))
            .ok_or(anyhow!(
                "Failed to get information about system enclosure/chassis (type 3)!"
            ))?;

        Ok(Self {
            manufacturer: t.manufacturer().ok(),
            chassis_type: match t.chassis_type() {
                Some(ct) => Some(ChassisTypeData::from(ct)),
                None => None,
            },
            version: t.version().ok(),
            serial_number: t.serial_number().ok(),
            asset_tag_number: t.asset_tag_number().ok(),
            bootup_state: match t.bootup_state() {
                Some(bs) => Some(ChassisStateData::from(bs)),
                None => None,
            },
            power_supply_state: match t.power_supply_state() {
                Some(pss) => Some(ChassisStateData::from(pss)),
                None => None,
            },
            thermal_state: match t.thermal_state() {
                Some(ts) => Some(ChassisStateData::from(ts)),
                None => None,
            },
            security_status: match t.security_status() {
                Some(ss) => Some(ChassisSecurityStatusData::from(ss)),
                None => None,
            },
            oem_defined: t.oem_defined(),
            height: match t.height() {
                Some(h) => Some(ChassisHeight::from(h)),
                None => None,
            },
            number_of_power_cords: match t.number_of_power_cords() {
                Some(npc) => Some(PowerCords::from(npc)),
                None => None,
            },
            contained_element_count: t.contained_element_count(),
            contained_element_record_length: t.contained_element_record_length(),
            sku_number: t.sku_number().ok(),
        })
    }
}

impl ToJson for Chassis {}

/// Information about processor
#[derive(Debug, Serialize)]
pub struct Processor {
    /// Socket reference designation
    pub socked_designation: Option<String>,

    /// Processor type
    pub processor_type: Option<smbioslib::ProcessorTypeData>,

    /// Processor family
    pub processor_family: Option<smbioslib::ProcessorFamilyData>,

    /// Processor manufacturer
    pub processor_manufacturer: Option<String>,

    /// Raw processor identification data
    pub processor_id: Option<[u8; 8]>,

    /// Processor version
    pub processor_version: Option<String>,

    /// Processor voltage
    pub voltage: Option<smbioslib::ProcessorVoltage>,

    /// External clock frequency, MHz. If the value is unknown,
    /// the field is set to 0
    pub external_clock: Option<smbioslib::ProcessorExternalClock>,

    /// Maximum CPU speed (in MHz) supported *by the system* for this
    /// processor socket
    pub max_speed: Option<smbioslib::ProcessorSpeed>,

    /// Current speed
    ///
    /// This field identifies the processor's speed at system boot;
    /// the processor may support more than one speed
    pub current_speed: Option<smbioslib::ProcessorSpeed>,

    /// Status bit field
    pub status: Option<smbioslib::ProcessorStatus>,

    /// Processor upgrade
    pub processor_upgrade: Option<smbioslib::ProcessorUpgradeData>,

    /// Attributes of the primary (Level 1) cache for this processor
    pub l1cache_handle: Option<smbioslib::Handle>,

    /// Attributes of the primary (Level 2) cache for this processor
    pub l2cache_handle: Option<smbioslib::Handle>,

    /// Attributes of the primary (Level 3) cache for this processor
    pub l3cache_handle: Option<smbioslib::Handle>,

    /// Serial number of this processor
    pub serial_number: Option<String>,

    /// Asset tag of this proc
    pub asset_tag: Option<String>,

    /// Part number of this processor
    pub part_number: Option<String>,

    /// Number of cores per processor socket
    pub core_count: Option<smbioslib::CoreCount>,

    /// Number of enabled cores per processor socket
    pub cores_enabled: Option<smbioslib::CoresEnabled>,

    /// Number of threads per processor socket
    pub thread_count: Option<smbioslib::ThreadCount>,

    /// Function that processor supports
    pub processors_characteristics: Option<ProcessorCharacteristics>,

    /// Processor family 2
    pub processor_family_2: Option<smbioslib::ProcessorFamilyData2>,

    /// Number of cores per proc socket (if cores > 255)
    pub core_count_2: Option<smbioslib::CoreCount2>,

    /// Number of enabled cores per proc socket (if ecores > 255)
    pub cores_enabled_2: Option<smbioslib::CoresEnabled2>,

    /// Number of threads per proc socket (if threads > 255)
    pub thread_count_2: Option<smbioslib::ThreadCount2>,

    /// Number of threads the BIOS has enabled and available for
    /// OS use
    pub thread_enabled: Option<smbioslib::ThreadEnabled>,
}

impl Processor {
    /// Creates a new instance of `Self`
    ///
    /// It is usually not required, since an instance of this
    /// structure will be created using the method
    /// `Self::new_from_table(table: &SMBiosData)` in the constructor
    /// [`DMITable::new()`].
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Self::new_from_table(&table)
    }

    pub fn new_from_table(table: &SMBiosData) -> Result<Self> {
        let t = table
            .find_map(|f: smbioslib::SMBiosProcessorInformation| Some(f))
            .ok_or(anyhow!("Failed to get information about CPU (type 4)!"))?;

        Ok(Self {
            socked_designation: t.socket_designation().ok(),
            processor_type: t.processor_type(),
            processor_family: t.processor_family(),
            processor_manufacturer: t.processor_manufacturer().ok(),
            processor_id: match t.processor_id() {
                Some(p_id) => Some(*p_id),
                None => None,
            },
            processor_version: t.processor_version().ok(),
            voltage: t.voltage(),
            external_clock: t.external_clock(),
            max_speed: t.max_speed(),
            current_speed: t.current_speed(),
            status: t.status(),
            processor_upgrade: t.processor_upgrade(),
            l1cache_handle: t.l1cache_handle(),
            l2cache_handle: t.l2cache_handle(),
            l3cache_handle: t.l3cache_handle(),
            serial_number: t.serial_number().ok(),
            asset_tag: t.asset_tag().ok(),
            part_number: t.part_number().ok(),
            core_count: t.core_count(),
            cores_enabled: t.cores_enabled(),
            thread_count: t.thread_count(),
            processors_characteristics: match t.processor_characteristics() {
                Some(pc) => Some(ProcessorCharacteristics::from(pc)),
                None => None,
            },
            processor_family_2: t.processor_family_2(),
            core_count_2: t.core_count_2(),
            cores_enabled_2: t.cores_enabled_2(),
            thread_count_2: t.thread_count_2(),
            thread_enabled: t.thread_enabled(),
        })
    }
}

impl ToJson for Processor {}

/// Defines which functions the processor supports
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessorCharacteristics {
    /// Bit 1 unknown
    pub unknown: bool,

    /// 64-bit capable
    pub bit_64capable: bool,

    /// Multi-core
    pub multi_core: bool,

    /// Hardware thread
    pub hardware_thread: bool,

    /// Execute protection
    pub execute_protection: bool,

    /// Enhanced Virtualization
    pub enhanced_virtualization: bool,

    /// Power/perfomance control
    pub power_perfomance_control: bool,

    /// 128-bit capable
    pub bit_128capable: bool,

    /// Arm64 SoC ID
    pub arm_64soc_id: bool,
}

impl From<smbioslib::ProcessorCharacteristics> for ProcessorCharacteristics {
    fn from(value: smbioslib::ProcessorCharacteristics) -> Self {
        Self {
            unknown: value.unknown(),
            bit_64capable: value.bit_64capable(),
            multi_core: value.multi_core(),
            hardware_thread: value.hardware_thread(),
            execute_protection: value.execute_protection(),
            enhanced_virtualization: value.enhanced_virtualization(),
            power_perfomance_control: value.power_performance_control(),
            bit_128capable: value.bit_128capable(),
            arm_64soc_id: value.arm_64soc_id(),
        }
    }
}
impl ToJson for ProcessorCharacteristics {}

/// Attributes of each CPU cache device in the system
#[derive(Debug, Serialize)]
pub struct Caches {
    pub caches: Vec<Cache>,
}

impl Caches {
    /// Creates a new instance of `Self`
    ///
    /// It is usually not required, since an instance of this
    /// structure will be created using the method
    /// `Self::new_from_table(table: &SMBiosData)` in the constructor
    /// [`DMITable::new()`].
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Self::new_from_table(&table)
    }

    pub fn new_from_table(table: &SMBiosData) -> Result<Self> {
        let mut caches = vec![];

        for cache_device in table.collect::<smbioslib::SMBiosCacheInformation>() {
            caches.push(Cache::from(cache_device));
        }

        Ok(Self { caches })
    }
}

impl ToJson for Caches {}

// struct_migrate!(CacheConfiguration, smbioslib::CacheConfiguaration, {
//     raw: u16,
// });

#[derive(Debug, Serialize)]
pub struct CacheConfiguaration {
    pub raw: u16,
}

/// This structure defines the attributes of CPU cache device in the
/// system. One structure is specified for each such device, whether
/// the device is internal to or external to the CPU module.
#[derive(Debug, Serialize)]
pub struct Cache {
    /// String number for reference designation
    pub socket_designation: Option<String>,

    /// Bit fields describing the cache configuration
    pub cache_configuration: Option<CacheConfiguaration>,

    /// Maximum size that can be installed
    pub maximum_cache_size: Option<smbioslib::CacheMemorySize>,

    /// Same format as Max Cache Size field; set 0 if no cache
    /// is installed.
    pub installed_size: Option<smbioslib::CacheMemorySize>,

    /// Supported SRAM type
    pub supported_sram_type: Option<smbioslib::SramTypes>,

    /// Current SRAM type
    pub current_sram_type: Option<smbioslib::SramTypes>,

    /// Cache module speed, in nanosecs. The value is 0 if the
    /// speed is unknown
    pub cache_speed: Option<u8>,

    /// Error-correction scheme supported by this cache component
    pub error_correction_type: Option<smbioslib::ErrorCorrectionTypeData>,

    /// Logical type of cache
    pub system_cache_type: Option<smbioslib::SystemCacheTypeData>,

    /// Associativity of the cache
    pub associativity: Option<smbioslib::CacheAssociativityData>,

    /// Maximum cache size
    pub maximum_cache_size_2: Option<smbioslib::CacheMemorySize>,

    /// Installed cache size
    pub installed_cache_size_2: Option<smbioslib::CacheMemorySize>,
}

impl<'a> From<smbioslib::SMBiosCacheInformation<'a>> for Cache {
    fn from(value: smbioslib::SMBiosCacheInformation) -> Self {
        Self {
            socket_designation: value.socket_designation().ok(),
            cache_configuration: match value.cache_configuration() {
                Some(conf) => Some(CacheConfiguaration { raw: conf.raw }),
                None => None,
            },
            maximum_cache_size: value.maximum_cache_size(),
            installed_size: value.installed_size(),
            supported_sram_type: value.supported_sram_type(),
            current_sram_type: value.current_sram_type(),
            cache_speed: value.cache_speed(),
            error_correction_type: value.error_correction_type(),
            system_cache_type: value.system_cache_type(),
            associativity: value.associativity(),
            maximum_cache_size_2: value.maximum_cache_size_2(),
            installed_cache_size_2: value.installed_cache_size_2(),
        }
    }
}
impl ToJson for Cache {}

/// Attributes of a system port connectors
#[derive(Debug, Serialize)]
pub struct PortConnectors {
    pub ports: Vec<Port>,
}

impl PortConnectors {
    /// Creates a new instance of `Self`
    ///
    /// It is usually not required, since an instance of this
    /// structure will be created using the method
    /// `Self::new_from_table(table: &SMBiosData)` in the constructor
    /// [`DMITable::new()`].
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Self::new_from_table(&table)
    }

    pub fn new_from_table(table: &SMBiosData) -> Result<Self> {
        let mut ports = vec![];

        for port in table.collect::<smbioslib::SMBiosPortConnectorInformation>() {
            ports.push(Port::from(port));
        }

        Ok(Self { ports })
    }
}

impl ToJson for PortConnectors {}

/// Attributes of a system port connector (serial, parallel,
/// keyboard or mouse ports)
#[derive(Debug, Serialize)]
pub struct Port {
    /// Internal reference designator, that is, internal to the
    /// system enclosure
    pub internal_reference_designator: Option<String>,

    /// Internal connector type
    pub internal_connector_type: Option<smbioslib::PortInformationConnectorTypeData>,

    /// External reference designation, external to the system
    /// enclosure
    pub external_reference_designator: Option<String>,

    /// External connector type
    pub external_connector_type: Option<smbioslib::PortInformationConnectorTypeData>,

    /// Function of the port
    pub port_type: Option<smbioslib::PortInformationPortTypeData>,
}

impl<'a> From<smbioslib::SMBiosPortConnectorInformation<'a>> for Port {
    fn from(value: smbioslib::SMBiosPortConnectorInformation) -> Self {
        Self {
            internal_reference_designator: value.internal_reference_designator().ok(),
            internal_connector_type: value.internal_connector_type(),
            external_reference_designator: value.external_reference_designator().ok(),
            external_connector_type: value.external_connector_type(),
            port_type: value.port_type(),
        }
    }
}
impl ToJson for Port {}

/// Collection of memory devices that operate together to form a memory address space
#[derive(Debug, Serialize)]
pub struct MemoryArray {
    /// Physical location of the Memory Array, whether on the system
    /// board or an add-in board
    pub location: Option<smbioslib::MemoryArrayLocationData>,

    /// Which the array is used
    pub usage: Option<smbioslib::MemoryArrayUseData>,

    /// Primary hardware error correction or detection method
    /// supported by this memory array
    pub memory_error_correction: Option<smbioslib::MemoryArrayErrorCorrectionData>,

    /// Maximum memory capacity, in kbytes, for this array
    pub maximum_capacity: Option<smbioslib::MaximumMemoryCapacity>,

    /// Handle, or instance number, associated with any error that
    /// was previously detected for the array
    pub memory_error_information_handle: Option<smbioslib::Handle>,

    /// Number of slots/sockets available for memory devices in
    /// this array
    pub number_of_memory_devices: Option<u16>,

    /// Maximum memory capacity, in bytes, for this array. This
    /// field is only valid when the Maximum Capacity field
    /// contains 8000 0000h.
    pub extended_maximum_capacity: Option<u64>,
}

impl MemoryArray {
    /// Creates a new instance of `Self`
    ///
    /// It is usually not required, since an instance of this
    /// structure will be created using the method
    /// `Self::new_from_table(table: &SMBiosData)` in the constructor
    /// [`DMITable::new()`].
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Self::new_from_table(&table)
    }

    pub fn new_from_table(table: &SMBiosData) -> Result<Self> {
        let t = table
            .find_map(|f: smbioslib::SMBiosPhysicalMemoryArray| Some(f))
            .ok_or(anyhow!(
                "Failed to get information about memory array (type 16)!"
            ))?;

        Ok(Self {
            location: t.location(),
            usage: t.usage(),
            memory_error_correction: t.memory_error_correction(),
            maximum_capacity: t.maximum_capacity(),
            memory_error_information_handle: t.memory_error_information_handle(),
            number_of_memory_devices: t.number_of_memory_devices(),
            extended_maximum_capacity: t.extended_maximum_capacity(),
        })
    }
}

impl ToJson for MemoryArray {}

/// Information about all installed memory devices
#[derive(Debug, Serialize)]
pub struct MemoryDevices {
    pub memory: Vec<MemoryDevice>,
}

impl MemoryDevices {
    /// Creates a new instance of `Self`
    ///
    /// It is usually not required, since an instance of this
    /// structure will be created using the method
    /// `Self::new_from_table(table: &SMBiosData)` in the constructor
    /// [`DMITable::new()`].
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Self::new_from_table(&table)
    }

    pub fn new_from_table(table: &SMBiosData) -> Result<Self> {
        let mut memory = vec![];

        for mem in table.collect::<smbioslib::SMBiosMemoryDevice>() {
            memory.push(MemoryDevice::from(mem));
        }

        Ok(Self { memory })
    }
}

impl ToJson for MemoryDevices {}

/// Information about single memory device
#[derive(Debug, Serialize)]
pub struct MemoryDevice {
    /// Handle or instance number, associated with the physical
    /// memory array to which this device belongs
    pub physical_memory_array_handle: Option<smbioslib::Handle>,

    /// Handle or instance number, associated with any error that
    /// was previously detected for the device. If the system does
    /// not provide the error information structure, the field
    /// containes FFFEH
    pub memory_error_information_handle: Option<smbioslib::Handle>,

    /// Total width, in bits, of this memory device, including any
    /// check or error-correction bits
    pub total_width: Option<u16>,

    /// Data width, in bits, of this memory device
    pub data_width: Option<u16>,

    /// Size of memory device
    pub size: Option<smbioslib::MemorySize>,

    /// Form factor for this memory device
    pub form_factor: Option<smbioslib::MemoryFormFactorData>,

    /// Identifies when the Memory Device is one of a set of
    /// Memory Devices that must be populated with all devices
    /// of the same type and size, and the set to which this
    /// device belongs A value of 0 indicates that the device
    /// is not part of a set; a value of FFh indicates that the
    /// attribute is unknown
    pub device_set: Option<u8>,

    /// Physically-labeled socket or board position where the
    /// memory device is located
    pub device_locator: Option<String>,

    /// Physically-labeled bank where the memory device is located
    pub bank_locator: Option<String>,

    /// Type of memory used in this device
    pub memory_type: Option<smbioslib::MemoryDeviceTypeData>,

    /// Additional detail on the memory device type
    pub type_detail: Option<smbioslib::MemoryTypeDetails>,

    /// The maximum capable speed of the device (MT/s)
    pub speed: Option<smbioslib::MemorySpeed>,

    /// Manufacturer of this memory device
    pub manufacturer: Option<String>,

    /// Serial number of this memory device
    pub serial_number: Option<String>,

    /// Asset tag of this memory device
    pub asset_tag: Option<String>,

    /// Part number of this memory device
    pub part_number: Option<String>,

    /// Bits 7-4: reserved Bits 3-0: rank Value=0 for unknown rank information
    pub attributes: Option<u8>,

    /// Extended suze of the memory device in MB
    pub extended_size: Option<smbioslib::MemorySizeExtended>,

    /// Configured speed of the memory device, in megatransfers per second (MT/s)
    pub configured_memory_speed: Option<smbioslib::MemorySpeed>,

    /// Minimum operating voltage for this device, in millivolts
    pub minimum_voltage: Option<u16>,

    /// Maximum operating voltage for this device, in millivolts
    pub maximum_voltage: Option<u16>,

    /// Configured voltage for this device, in millivolts
    pub configured_voltage: Option<u16>,

    /// Memory technology type for this memory device
    pub memory_technology: Option<smbioslib::MemoryDeviceTechnologyData>,

    /// The operating modes supported by this memory device
    pub memory_operating_mode_capability: Option<smbioslib::MemoryOperatingModeCapabilities>,

    /// Firmware version of this memory device
    pub firmware_version: Option<String>,

    /// Two-byte module manufacturer ID found in the SPD of this
    /// memory device; LSB first
    pub module_manufacturer_id: Option<u16>,

    /// Two-byte module product id found in the SPD of this memory
    /// device; LSB first
    pub module_product_id: Option<u16>,

    /// Two-byte memory subsystem controller manufacturer ID found
    /// in the SPD of this memory device; LSB first
    pub memory_subsystem_controller_manufacturer_id: Option<u16>,

    /// Two-byte memory subsystem controller product ID found in
    /// the SPD of this memory device; LSB first
    pub memory_subsystem_controller_product_id: Option<u16>,

    /// Size of the Non-volatile portion of the memory device in
    /// Bytes, if any
    pub non_volatile_size: Option<smbioslib::MemoryIndicatedSize>,

    /// Size of the Volatile portion of the memory device in
    /// Bytes, if any
    pub volatile_size: Option<smbioslib::MemoryIndicatedSize>,

    /// Size of the Cache portion of the memory device in Bytes,
    /// if any
    pub cache_size: Option<smbioslib::MemoryIndicatedSize>,

    /// Size of the Logical memory device in Bytes
    pub logical_size: Option<smbioslib::MemoryIndicatedSize>,

    /// Extended speed of the memory device (complements the
    /// Speed field at offset 15h). Identifies the maximum capable
    /// speed of the device, in MT/s
    pub extended_speed: Option<smbioslib::MemorySpeedExtended>,

    /// Extended configured memory speed of the memory device
    /// (complements the `configure_memory_speed` field at offset
    /// 20h). Identifies the configured speed of the memory device,
    /// in MT/s
    pub extended_configured_speed: Option<smbioslib::MemorySpeedExtended>,

    /// Two-byte PMIC0 manufacturer ID found in the SPD of this
    /// memory device; LSB first
    pub pmic0_manufacturer_id: Option<u16>,

    /// PMIC 0 Revision Number found in the SPD of this memory
    /// device
    pub pmic0_revision_number: Option<u16>,

    /// Two-byte RCD manufacturer ID found in the SPD of this
    /// memory device; LSB first
    pub rcd_manufacturer_id: Option<u16>,

    /// RCD 0 Revision Number found in the SPD of this memory
    /// device
    pub rcd_revision_number: Option<u16>,
}

impl<'a> From<smbioslib::SMBiosMemoryDevice<'a>> for MemoryDevice {
    fn from(value: smbioslib::SMBiosMemoryDevice) -> Self {
        Self {
            physical_memory_array_handle: value.physical_memory_array_handle(),
            memory_error_information_handle: value.memory_error_information_handle(),
            total_width: value.total_width(),
            data_width: value.data_width(),
            size: value.size(),
            form_factor: value.form_factor(),
            device_set: value.device_set(),
            device_locator: value.device_locator().ok(),
            bank_locator: value.bank_locator().ok(),
            memory_type: value.memory_type(),
            type_detail: value.type_detail(),
            speed: value.speed(),
            manufacturer: value.manufacturer().ok(),
            serial_number: value.serial_number().ok(),
            asset_tag: value.asset_tag().ok(),
            part_number: value.part_number().ok(),
            attributes: value.attributes(),
            extended_size: value.extended_size(),
            configured_memory_speed: value.configured_memory_speed(),
            minimum_voltage: value.minimum_voltage(),
            maximum_voltage: value.maximum_voltage(),
            configured_voltage: value.configured_voltage(),
            memory_technology: value.memory_technology(),
            memory_operating_mode_capability: value.memory_operating_mode_capability(),
            firmware_version: value.firmware_version().ok(),
            module_manufacturer_id: value.module_manufacturer_id(),
            module_product_id: value.module_product_id(),
            memory_subsystem_controller_manufacturer_id: value
                .memory_subsystem_controller_manufacturer_id(),
            memory_subsystem_controller_product_id: value.memory_subsystem_controller_product_id(),
            non_volatile_size: value.non_volatile_size(),
            volatile_size: value.volatile_size(),
            cache_size: value.cache_size(),
            logical_size: value.logical_size(),
            extended_speed: value.extended_speed(),
            extended_configured_speed: value.extended_speed(),
            pmic0_manufacturer_id: value.pmic0_manufacturer_id(),
            pmic0_revision_number: value.pmic0_revision_number(),
            rcd_manufacturer_id: value.rcd_manufacturer_id(),
            rcd_revision_number: value.rcd_revision_number(),
        }
    }
}
impl ToJson for MemoryDevice {}
