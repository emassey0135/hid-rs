use bitvec::prelude::*;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReportConstantFlag {
  Data,
  Constant,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReportLayoutFlag {
  Array,
  Variable,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReportRelativeFlag {
  Absolute,
  Relative,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReportWrapFlag {
  NoWrap,
  Wrap,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReportLinearFlag {
  Linear,
  NonLinear,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReportPreferredStateFlag {
  PreferredState,
  NoPreferred,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReportNullStateFlag {
  NoNullPosition,
  NullState,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReportVolatileFlag {
  NonVolatile,
  Volatile,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReportBufferedBytesFlag {
  BitField,
  BufferedBytes,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CollectionType {
  Physical,
  Application,
  Logical,
  Report,
  NamedArray,
  UsageSwitch,
  UsageModifier,
  VendorDefined(u8),
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Usage {
  Usage(u16),
  ExtendedUsage(u32),
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DelimiterFlag {
  Close,
  Open,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DescriptorItem {
  Input { constant: ReportConstantFlag, layout: ReportLayoutFlag, relative: ReportRelativeFlag, wrap: ReportWrapFlag, linear: ReportLinearFlag, preferred_state: ReportPreferredStateFlag, null_state: ReportNullStateFlag, buffered_bytes: ReportBufferedBytesFlag },
  Output { constant: ReportConstantFlag, layout: ReportLayoutFlag, relative: ReportRelativeFlag, wrap: ReportWrapFlag, linear: ReportLinearFlag, preferred_state: ReportPreferredStateFlag, null_state: ReportNullStateFlag, volatile: ReportVolatileFlag, buffered_bytes: ReportBufferedBytesFlag },
  Feature { constant: ReportConstantFlag, layout: ReportLayoutFlag, relative: ReportRelativeFlag, wrap: ReportWrapFlag, linear: ReportLinearFlag, preferred_state: ReportPreferredStateFlag, null_state: ReportNullStateFlag, volatile: ReportVolatileFlag, buffered_bytes: ReportBufferedBytesFlag },
  Collection { ty: CollectionType },
  EndCollection,
  UsagePage(u16),
  LogicalMinimum(i32),
  LogicalMaximum(i32),
  PhysicalMinimum(i32),
  PhysicalMaximum(i32),
  UnitExponent(i8),
  Unit(u32),
  ReportSize(u32),
  ReportId(u8),
  ReportCount(u32),
  Push,
  Pop,
  Usage(Usage),
  UsageMinimum(Usage),
  UsageMaximum(Usage),
  DesignatorIndex(u32),
  DesignatorMinimum(u32),
  DesignatorMaximum(u32),
  StringIndex(u32),
  StringMinimum(u32),
  StringMaximum(u32),
  Delimiter(DelimiterFlag),
}
impl DescriptorItem {
  pub fn into_bitvec(self) -> BitVec<u8, Msb0> {
    let mut data = bitvec![u8, Msb0; 0; 40];
    match self {
      DescriptorItem::Input { constant, layout, relative, wrap, linear, preferred_state, null_state, buffered_bytes } => {
        data[0..6].store::<u8>(0b1000_00);
        data.set(8, constant==ReportConstantFlag::Constant);
        data.set(9, layout==ReportLayoutFlag::Variable);
        data.set(10, relative==ReportRelativeFlag::Relative);
        data.set(11, wrap==ReportWrapFlag::Wrap);
        data.set(12, linear==ReportLinearFlag::NonLinear);
        data.set(13, preferred_state==ReportPreferredStateFlag::NoPreferred);
        data.set(14, null_state==ReportNullStateFlag::NullState);
        data.set(16, buffered_bytes==ReportBufferedBytesFlag::BufferedBytes);
      },
      DescriptorItem::Output { constant, layout, relative, wrap, linear, preferred_state, null_state, volatile, buffered_bytes } => {
        data[0..6].store::<u8>(0b1001_00);
        data.set(8, constant==ReportConstantFlag::Constant);
        data.set(9, layout==ReportLayoutFlag::Variable);
        data.set(10, relative==ReportRelativeFlag::Relative);
        data.set(11, wrap==ReportWrapFlag::Wrap);
        data.set(12, linear==ReportLinearFlag::NonLinear);
        data.set(13, preferred_state==ReportPreferredStateFlag::NoPreferred);
        data.set(14, null_state==ReportNullStateFlag::NullState);
        data.set(15, volatile==ReportVolatileFlag::Volatile);
        data.set(16, buffered_bytes==ReportBufferedBytesFlag::BufferedBytes);
      },
      DescriptorItem::Feature { constant, layout, relative, wrap, linear, preferred_state, null_state, volatile, buffered_bytes } => {
        data[0..6].store::<u8>(0b1011_00);
        data.set(8, constant==ReportConstantFlag::Constant);
        data.set(9, layout==ReportLayoutFlag::Variable);
        data.set(10, relative==ReportRelativeFlag::Relative);
        data.set(11, wrap==ReportWrapFlag::Wrap);
        data.set(12, linear==ReportLinearFlag::NonLinear);
        data.set(13, preferred_state==ReportPreferredStateFlag::NoPreferred);
        data.set(14, null_state==ReportNullStateFlag::NullState);
        data.set(15, volatile==ReportVolatileFlag::Volatile);
        data.set(16, buffered_bytes==ReportBufferedBytesFlag::BufferedBytes);
      },
      DescriptorItem::Collection { ty } => {
        data[0..6].store::<u8>(0b1010_00);
        data[8..16].store::<u8>(match ty {
          CollectionType::Physical => 0,
          CollectionType::Application => 1,
          CollectionType::Logical => 2,
          CollectionType::Report => 3,
          CollectionType::NamedArray => 4,
          CollectionType::UsageSwitch => 5,
          CollectionType::UsageModifier => 6,
          CollectionType::VendorDefined(n) => n,
        });
      },
      DescriptorItem::EndCollection => {
        data[0..6].store::<u8>(0b1100_00);
      },
      DescriptorItem::UsagePage(page) => {
        data[0..6].store::<u8>(0b0000_01);
        data[8..24].store::<u16>(page);
      },
      DescriptorItem::LogicalMinimum(minimum) => {
        data[0..6].store::<u8>(0b0001_01);
        data[8..40].store::<i32>(minimum);
      },
      DescriptorItem::LogicalMaximum(maximum) => {
        data[0..6].store::<u8>(0b0010_01);
        data[8..40].store::<i32>(maximum);
      },
      DescriptorItem::PhysicalMinimum(minimum) => {
        data[0..6].store::<u8>(0b0011_01);
        data[8..40].store::<i32>(minimum);
      },
      DescriptorItem::PhysicalMaximum(maximum) => {
        data[0..6].store::<u8>(0b0100_01);
        data[8..40].store::<i32>(maximum);
      },
      DescriptorItem::UnitExponent(exponent) => {
        data[0..6].store::<u8>(0b0101_01);
        data[8..16].store::<i8>(exponent);
      },
      DescriptorItem::Unit(unit) => {
        data[0..6].store::<u8>(0b0110_01);
        data[8..40].store::<u32>(unit);
      },
      DescriptorItem::ReportSize(size) => {
        data[0..6].store::<u8>(0b0111_01);
        data[8..40].store::<u32>(size);
      },
      DescriptorItem::ReportId(id) => {
        data[0..6].store::<u8>(0b1000_01);
        data[8..16].store::<u8>(id);
      },
      DescriptorItem::ReportCount(count) => {
        data[0..6].store::<u8>(0b1001_01);
        data[8..40].store::<u32>(count);
      },
      DescriptorItem::Push => {
        data[0..6].store::<u8>(0b1010_01);
      },
      DescriptorItem::Pop => {
        data[0..6].store::<u8>(0b1011_01);
      },
      DescriptorItem::Usage(usage) => {
        data[0..6].store::<u8>(0b0000_10);
        match usage {
          Usage::Usage(usage) => data[8..24].store::<u16>(usage),
          Usage::ExtendedUsage(usage) => data[8..40].store::<u32>(usage),
        };
      },
      DescriptorItem::UsageMinimum(usage) => {
        data[0..6].store::<u8>(0b0001_10);
        match usage {
          Usage::Usage(usage) => data[8..24].store::<u16>(usage),
          Usage::ExtendedUsage(usage) => data[8..40].store::<u32>(usage),
        };
      },
      DescriptorItem::UsageMaximum(usage) => {
        data[0..6].store::<u8>(0b0010_10);
        match usage {
          Usage::Usage(usage) => data[8..24].store::<u16>(usage),
          Usage::ExtendedUsage(usage) => data[8..40].store::<u32>(usage),
        };
      },
      DescriptorItem::DesignatorIndex(index) => {
        data[0..6].store::<u8>(0b0011_10);
        data[8..40].store::<u32>(index);
      },
      DescriptorItem::DesignatorMinimum(minimum) => {
        data[0..6].store::<u8>(0b0100_10);
        data[8..40].store::<u32>(minimum);
      },
      DescriptorItem::DesignatorMaximum(maximum) => {
        data[0..6].store::<u8>(0b0101_10);
        data[8..40].store::<u32>(maximum);
      },
      DescriptorItem::StringIndex(index) => {
        data[0..6].store::<u8>(0b0111_10);
        data[8..40].store::<u32>(index);
      },
      DescriptorItem::StringMinimum(minimum) => {
        data[0..6].store::<u8>(0b1000_10);
        data[8..40].store::<u32>(minimum);
      },
      DescriptorItem::StringMaximum(maximum) => {
        data[0..6].store::<u8>(0b1001_10);
        data[8..40].store::<u32>(maximum);
      },
      DescriptorItem::Delimiter(flag) => {
        data[0..6].store::<u8>(0b1010_10);
        data.set(8, flag==DelimiterFlag::Open);
      },
    };
    let mut has_value = vec![false; 4];
    for i in 0..4 {
      has_value[i] = data[8+i*8..16+i*8].load::<u8>()==0;
    };
    let size: u8 = match has_value[..] {
      [false, false, false, false] => 0,
      [true, false, false, false] => 1,
      [true, true, false, false] => 2,
      _ => 4,
    };
    data.truncate((size+1) as usize);
    data[6..8].store::<u8>(match size {
      4 => 3,
      size => size,
    });
    data
  }
  pub fn from_bitvec(item: BitVec<u8, Msb0>) -> Self {
    if item.len() < 8 || item.len()%8 != 0 {
      panic!("HID descriptor items must be at least one byte long and their size must be a multiple of bytes");
    };
    let size = match item[6..8].load::<u8>() {
      3 => 4,
      size => size,
    };
    if item.len() != (8+size*8) as usize {
      panic!("The size field does not match the length of the data");
    };
    let mut data = bitvec![u8, Msb0; 0; 32];
    for i in 0..(size as usize) {
      data[8*i..8+8*i].store::<u8>(item[8+8*i..16+8*i].load::<u8>());
    };
    match item[0..6].load::<u8>() {
      0b1000_00 => DescriptorItem::Input {
        constant: if *data.get(0).unwrap() { ReportConstantFlag::Constant } else { ReportConstantFlag::Data },
        layout: if *data.get(1).unwrap() { ReportLayoutFlag::Variable } else { ReportLayoutFlag::Array },
        relative: if *data.get(2).unwrap() { ReportRelativeFlag::Relative } else { ReportRelativeFlag::Absolute },
        wrap: if *data.get(3).unwrap() { ReportWrapFlag::Wrap } else { ReportWrapFlag::NoWrap },
        linear: if *data.get(4).unwrap() { ReportLinearFlag::NonLinear } else { ReportLinearFlag::Linear },
        preferred_state: if *data.get(5).unwrap() { ReportPreferredStateFlag::NoPreferred } else { ReportPreferredStateFlag::PreferredState },
        null_state: if *data.get(6).unwrap() { ReportNullStateFlag::NullState } else { ReportNullStateFlag::NoNullPosition },
        buffered_bytes: if *data.get(8).unwrap() { ReportBufferedBytesFlag::BufferedBytes } else { ReportBufferedBytesFlag::BitField },
      },
      0b1001_00 => DescriptorItem::Output {
        constant: if *data.get(0).unwrap() { ReportConstantFlag::Constant } else { ReportConstantFlag::Data },
        layout: if *data.get(1).unwrap() { ReportLayoutFlag::Variable } else { ReportLayoutFlag::Array },
        relative: if *data.get(2).unwrap() { ReportRelativeFlag::Relative } else { ReportRelativeFlag::Absolute },
        wrap: if *data.get(3).unwrap() { ReportWrapFlag::Wrap } else { ReportWrapFlag::NoWrap },
        linear: if *data.get(4).unwrap() { ReportLinearFlag::NonLinear } else { ReportLinearFlag::Linear },
        preferred_state: if *data.get(5).unwrap() { ReportPreferredStateFlag::NoPreferred } else { ReportPreferredStateFlag::PreferredState },
        null_state: if *data.get(6).unwrap() { ReportNullStateFlag::NullState } else { ReportNullStateFlag::NoNullPosition },
        volatile: if *data.get(7).unwrap() { ReportVolatileFlag::Volatile } else { ReportVolatileFlag::NonVolatile },
        buffered_bytes: if *data.get(8).unwrap() { ReportBufferedBytesFlag::BufferedBytes } else { ReportBufferedBytesFlag::BitField },
      },
      0b1011_00 => DescriptorItem::Feature {
        constant: if *data.get(0).unwrap() { ReportConstantFlag::Constant } else { ReportConstantFlag::Data },
        layout: if *data.get(1).unwrap() { ReportLayoutFlag::Variable } else { ReportLayoutFlag::Array },
        relative: if *data.get(2).unwrap() { ReportRelativeFlag::Relative } else { ReportRelativeFlag::Absolute },
        wrap: if *data.get(3).unwrap() { ReportWrapFlag::Wrap } else { ReportWrapFlag::NoWrap },
        linear: if *data.get(4).unwrap() { ReportLinearFlag::NonLinear } else { ReportLinearFlag::Linear },
        preferred_state: if *data.get(5).unwrap() { ReportPreferredStateFlag::NoPreferred } else { ReportPreferredStateFlag::PreferredState },
        null_state: if *data.get(6).unwrap() { ReportNullStateFlag::NullState } else { ReportNullStateFlag::NoNullPosition },
        volatile: if *data.get(7).unwrap() { ReportVolatileFlag::Volatile } else { ReportVolatileFlag::NonVolatile },
        buffered_bytes: if *data.get(8).unwrap() { ReportBufferedBytesFlag::BufferedBytes } else { ReportBufferedBytesFlag::BitField },
      },
      0b1010_00 => DescriptorItem::Collection {
        ty: match data.load::<u32>() {
          0 => CollectionType::Physical,
          1 => CollectionType::Application,
          2 => CollectionType::Logical,
          3 => CollectionType::Report,
          4 => CollectionType::NamedArray,
          5 => CollectionType::UsageSwitch,
          6 => CollectionType::UsageModifier,
          n if n >= 128 && n <= 255 => CollectionType::VendorDefined(n as u8),
          _ => panic!("Invalid collection type"),
        },
      },
      0b1100_00 => DescriptorItem::EndCollection,
      0b0000_01 => DescriptorItem::UsagePage(data[0..16].load::<u16>()),
      0b0001_01 => DescriptorItem::LogicalMinimum(data.load::<i32>()),
      0b0010_01 => DescriptorItem::LogicalMaximum(data.load::<i32>()),
      0b0011_01 => DescriptorItem::PhysicalMinimum(data.load::<i32>()),
      0b0100_01 => DescriptorItem::PhysicalMaximum(data.load::<i32>()),
      0b0101_01 => DescriptorItem::UnitExponent(data[0..8].load::<i8>()),
      0b0110_01 => DescriptorItem::Unit(data.load::<u32>()),
      0b0111_01 => DescriptorItem::ReportSize(data.load::<u32>()),
      0b1000_01 => DescriptorItem::ReportId(data[0..8].load::<u8>()),
      0b1001_01 => DescriptorItem::ReportCount(data.load::<u32>()),
      0b1010_01 => DescriptorItem::Push,
      0b1011_01 => DescriptorItem::Pop,
      0b0000_10 => DescriptorItem::Usage(match size {
        1 | 2 => Usage::Usage(data[0..16].load::<u16>()),
        _ => Usage::ExtendedUsage(data.load::<u32>()),
      }),
      0b0001_10 => DescriptorItem::UsageMinimum(match size {
        1 | 2 => Usage::Usage(data[0..16].load::<u16>()),
        _ => Usage::ExtendedUsage(data.load::<u32>()),
      }),
      0b0010_10 => DescriptorItem::UsageMaximum(match size {
        1 | 2 => Usage::Usage(data[0..16].load::<u16>()),
        _ => Usage::ExtendedUsage(data.load::<u32>()),
      }),
      0b0011_10 => DescriptorItem::DesignatorIndex(data.load::<u32>()),
      0b0100_10 => DescriptorItem::DesignatorMinimum(data.load::<u32>()),
      0b0101_10 => DescriptorItem::DesignatorMaximum(data.load::<u32>()),
      0b0111_10 => DescriptorItem::StringIndex(data.load::<u32>()),
      0b1000_10 => DescriptorItem::StringMinimum(data.load::<u32>()),
      0b1001_10 => DescriptorItem::StringMaximum(data.load::<u32>()),
      0b1010_10 => DescriptorItem::Delimiter(if *data.get(0).unwrap() { DelimiterFlag::Open } else { DelimiterFlag::Close }),
      _ => panic!("Invalid descriptor item"),
    }
  }
}
