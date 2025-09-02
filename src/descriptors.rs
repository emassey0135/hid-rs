use crate::descriptor_items::*;
use hut::{AsUsage, AsUsagePage, Usage};
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReportType {
  Input,
  Output,
  Feature,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ReportField {
  Variable {
    size: u32,
    logical_minimum: i32,
    logical_maximum: i32,
    physical_minimum: Option<i32>,
    physical_maximum: Option<i32>,
    unit_exponent: Option<i8>,
    unit: Option<u32>,
    usages: Option<Vec<u32>>,
    constant: bool,
    relative: bool,
    wrap: bool,
    linear: bool,
    preferred_state: bool,
    null_state: bool,
    volatile: Option<bool>,
    buffered_bytes: bool,
  },
  Array {
    size: u32,
    logical_minimum: i32,
    logical_maximum: i32,
    usage_range: Vec<u32>,
    constant: bool,
    relative: bool,
  },
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct GlobalState {
  usage_page: Option<u16>,
  logical_minimum: Option<i32>,
  logical_maximum: Option<i32>,
  physical_minimum: Option<i32>,
  physical_maximum: Option<i32>,
  unit_exponent: Option<i8>,
  unit: Option<u32>,
  report_size: Option<u32>,
  report_id: Option<u8>,
  report_count: Option<u32>,
}
impl Default for GlobalState {
  fn default() -> Self {
    GlobalState {
    usage_page: None,
    logical_minimum: None,
    logical_maximum: None,
    physical_minimum: None,
    physical_maximum: None,
    unit_exponent: None,
    unit: None,
    report_size: None,
    report_id: None,
    report_count: None,
    }
  }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Report {
  pub ty: ReportType,
  pub id: Option<u8>,
  pub fields: Vec<ReportField>,
}
impl Report {
  fn into_descriptor_items(self, state: &mut GlobalState) -> Vec<DescriptorItem> {
    let mut sequence = vec![];
    if let Some(id) = self.id {
      if Some(id) != state.report_id {
        sequence.push(DescriptorItem::ReportId(id));
        state.report_id = Some(id);
      };
    };
    let mut field_sets: Vec<Vec<ReportField>> = vec![];
    if let Some(first_field) = self.fields.first() {
      let mut previous_fields = vec![first_field.clone()];
      let mut previous_field = first_field;
      for field in &self.fields[1..] {
        let same_group = match (previous_field, field) {
          (previous_field, field) if previous_field == field => true,
          (ReportField::Variable { .. }, ReportField::Array { .. }) => false,
          (ReportField::Array { .. }, ReportField::Variable { .. }) => false,
          (ReportField::Variable {
            size: previous_size,
            logical_minimum: previous_logical_minimum,
            logical_maximum: previous_logical_maximum,
            physical_minimum: previous_physical_minimum,
            physical_maximum: previous_physical_maximum,
            unit_exponent: previous_unit_exponent,
            unit: previous_unit,
            usages: _,
            constant: previous_constant,
            relative: previous_relative,
            wrap: previous_wrap,
            linear: previous_linear,
            preferred_state: previous_preferred_state,
            null_state: previous_null_state,
            volatile: previous_volatile,
            buffered_bytes: previous_buffered_bytes,
          }, ReportField::Variable {
            size,
            logical_minimum,
            logical_maximum,
            physical_minimum,
            physical_maximum,
            unit_exponent,
            unit,
            usages: _,
            constant,
            relative,
            wrap,
            linear,
            preferred_state,
            null_state,
            volatile,
            buffered_bytes,
          }) => previous_size == size &&
            previous_logical_minimum == logical_minimum &&
            previous_logical_maximum == logical_maximum &&
            previous_physical_minimum == physical_minimum &&
            previous_physical_maximum == physical_maximum &&
            previous_unit_exponent == unit_exponent &&
            previous_unit == unit &&
            previous_constant == constant &&
            previous_relative == relative &&
            previous_wrap == wrap &&
            previous_linear == linear &&
            previous_preferred_state == preferred_state &&
            previous_null_state == null_state &&
            previous_volatile == volatile &&
            previous_buffered_bytes == buffered_bytes,
          (ReportField::Array { .. }, ReportField::Array { .. }) => false,
        };
        if same_group {
          previous_fields.push(field.clone())
        }
        else {
          field_sets.push(previous_fields);
          previous_fields = vec![field.clone()];
        };
        previous_field = field;
      };
      field_sets.push(previous_fields);
    };
    for set in field_sets {
      if Some(set.len() as u32) != state.report_count {
        sequence.push(DescriptorItem::ReportCount(set.len() as u32));
        state.report_count = Some(set.len() as u32);
      };
      match set.first().unwrap() {
        ReportField::Variable {
          size,
          logical_minimum,
          logical_maximum,
          physical_minimum,
          physical_maximum,
          unit_exponent,
          unit,
          usages,
          constant,
          relative,
          wrap,
          linear,
          preferred_state,
          null_state,
          volatile,
          buffered_bytes,
        } => {
          if Some(*size) != state.report_size {
            sequence.push(DescriptorItem::ReportSize(*size));
            state.report_size = Some(*size);
          };
          if Some(*logical_minimum) != state.logical_minimum {
            sequence.push(DescriptorItem::LogicalMinimum(*logical_minimum));
            state.logical_minimum = Some(*logical_minimum);
          };
          if Some(*logical_maximum) != state.logical_maximum {
            sequence.push(DescriptorItem::LogicalMaximum(*logical_maximum));
            state.logical_maximum = Some(*logical_maximum);
          };
          if *physical_minimum != state.physical_minimum {
            sequence.push(DescriptorItem::PhysicalMinimum(physical_minimum.unwrap_or(0)));
            state.physical_minimum = *physical_minimum;
          };
          if *physical_maximum != state.physical_maximum {
            sequence.push(DescriptorItem::PhysicalMaximum(physical_maximum.unwrap_or(0)));
            state.physical_maximum = *physical_maximum;
          };
          if *unit_exponent != state.unit_exponent {
            sequence.push(DescriptorItem::UnitExponent(unit_exponent.unwrap_or(0)));
            state.unit_exponent = *unit_exponent;
          };
          if *unit != state.unit {
            sequence.push(DescriptorItem::Unit(unit.unwrap_or(0)));
            state.unit = *unit;
          };
          if usages.is_some() {
            let mut usage_sets = vec![];
            for field in set.clone() {
              match field {
                ReportField::Variable { usages, .. } => usage_sets.push(usages.unwrap()),
                _ => {},
              };
            };
            let all_usages = usage_sets.clone().into_iter().flatten().collect::<Vec<u32>>();
            let mut continuous = true;
            if set.len() == all_usages.len() {
              let mut previous_usage_value = *all_usages.first().unwrap();
              for usage_value in &all_usages[1..] {
                let previous_usage = Usage::try_from(previous_usage_value).unwrap();
                let usage = Usage::try_from(*usage_value).unwrap();
                if previous_usage.usage_id_value()+1 != usage.usage_id_value() || previous_usage.usage_page_value() != usage.usage_page_value() {
                  continuous = false;
                  break;
                };
                previous_usage_value = *usage_value;
              };
            }
            else {
              continuous = false;
            };
            if continuous && set.len() > 1 {
              let minimum = Usage::try_from(*all_usages.first().unwrap()).unwrap();
              let maximum = Usage::try_from(*all_usages.last().unwrap()).unwrap();
              if Some(minimum.usage_page_value()) != state.usage_page {
                sequence.push(DescriptorItem::UsagePage(minimum.usage_page_value()));
                state.usage_page = Some(minimum.usage_page_value());
              };
              sequence.push(DescriptorItem::UsageMinimum(UsageSpecifier::Usage(minimum.usage_id_value())));
              sequence.push(DescriptorItem::UsageMaximum(UsageSpecifier::Usage(maximum.usage_id_value())));
            }
            else {
              for (index, usages) in usage_sets.iter().enumerate() {
                if usages.len() != 1 {
                  sequence.push(DescriptorItem::Delimiter(DelimiterFlag::Open));
                };
                for usage_value in usages {
                  let usage = Usage::try_from(*usage_value).unwrap();
                  if Some(usage.usage_page_value()) != state.usage_page {
                    sequence.push(DescriptorItem::UsagePage(usage.usage_page_value()));
                    state.usage_page = Some(usage.usage_page_value());
                  };
                  sequence.push(DescriptorItem::Usage(UsageSpecifier::Usage(usage.usage_id_value())));
                };
                if usages.len() != 1 {
                  sequence.push(DescriptorItem::Delimiter(DelimiterFlag::Close));
                };
                if usage_sets[index..].into_iter().all(|set| set==usages) {
                  break;
                };
              };
            };
          };
          match self.ty {
            ReportType::Input => sequence.push(DescriptorItem::Input {
              constant: if *constant { ReportConstantFlag::Constant } else { ReportConstantFlag::Data },
              layout: ReportLayoutFlag::Variable,
              relative: if *relative { ReportRelativeFlag::Relative } else { ReportRelativeFlag::Absolute },
              wrap: if *wrap { ReportWrapFlag::Wrap } else { ReportWrapFlag::NoWrap },
              linear: if *linear { ReportLinearFlag::Linear } else { ReportLinearFlag::NonLinear },
              preferred_state: if *preferred_state { ReportPreferredStateFlag::PreferredState } else { ReportPreferredStateFlag::NoPreferred },
              null_state: if *null_state { ReportNullStateFlag::NullState } else { ReportNullStateFlag::NoNullPosition },
              buffered_bytes: if *buffered_bytes { ReportBufferedBytesFlag::BufferedBytes } else { ReportBufferedBytesFlag::BitField },
            }),
            ReportType::Output => sequence.push(DescriptorItem::Output {
              constant: if *constant { ReportConstantFlag::Constant } else { ReportConstantFlag::Data },
              layout: ReportLayoutFlag::Variable,
              relative: if *relative { ReportRelativeFlag::Relative } else { ReportRelativeFlag::Absolute },
              wrap: if *wrap { ReportWrapFlag::Wrap } else { ReportWrapFlag::NoWrap },
              linear: if *linear { ReportLinearFlag::Linear } else { ReportLinearFlag::NonLinear },
              preferred_state: if *preferred_state { ReportPreferredStateFlag::PreferredState } else { ReportPreferredStateFlag::NoPreferred },
              null_state: if *null_state { ReportNullStateFlag::NullState } else { ReportNullStateFlag::NoNullPosition },
              volatile: if volatile.unwrap() { ReportVolatileFlag::Volatile } else { ReportVolatileFlag::NonVolatile },
              buffered_bytes: if *buffered_bytes { ReportBufferedBytesFlag::BufferedBytes } else { ReportBufferedBytesFlag::BitField },
            }),
            ReportType::Feature => sequence.push(DescriptorItem::Feature {
              constant: if *constant { ReportConstantFlag::Constant } else { ReportConstantFlag::Data },
              layout: ReportLayoutFlag::Variable,
              relative: if *relative { ReportRelativeFlag::Relative } else { ReportRelativeFlag::Absolute },
              wrap: if *wrap { ReportWrapFlag::Wrap } else { ReportWrapFlag::NoWrap },
              linear: if *linear { ReportLinearFlag::Linear } else { ReportLinearFlag::NonLinear },
              preferred_state: if *preferred_state { ReportPreferredStateFlag::PreferredState } else { ReportPreferredStateFlag::NoPreferred },
              null_state: if *null_state { ReportNullStateFlag::NullState } else { ReportNullStateFlag::NoNullPosition },
              volatile: if volatile.unwrap() { ReportVolatileFlag::Volatile } else { ReportVolatileFlag::NonVolatile },
              buffered_bytes: if *buffered_bytes { ReportBufferedBytesFlag::BufferedBytes } else { ReportBufferedBytesFlag::BitField },
            }),
          };
        },
        ReportField::Array {
          size,
          logical_minimum,
          logical_maximum,
          usage_range,
          constant,
          relative,
        } => {
          if Some(*size) != state.report_size {
            sequence.push(DescriptorItem::ReportSize(*size));
            state.report_size = Some(*size);
          };
          if Some(*logical_minimum) != state.logical_minimum {
            sequence.push(DescriptorItem::LogicalMinimum(*logical_minimum));
            state.logical_minimum = Some(*logical_minimum);
          };
          if Some(*logical_maximum) != state.logical_maximum {
            sequence.push(DescriptorItem::LogicalMaximum(*logical_maximum));
            state.logical_maximum = Some(*logical_maximum);
          };
          let mut continuous = true;
          let mut previous_usage_value = *usage_range.first().unwrap();
          for usage_value in &usage_range[1..] {
            let previous_usage = Usage::try_from(previous_usage_value).unwrap();
            let usage = Usage::try_from(*usage_value).unwrap();
            if previous_usage.usage_id_value()+1 != usage.usage_id_value() || previous_usage.usage_page_value() != usage.usage_page_value() {
              continuous = false;
              break;
            };
            previous_usage_value = *usage_value;
          };
          if continuous {
            let minimum = Usage::try_from(*usage_range.first().unwrap()).unwrap();
            let maximum = Usage::try_from(*usage_range.first().unwrap()).unwrap();
            if Some(minimum.usage_page_value()) != state.usage_page {
              sequence.push(DescriptorItem::UsagePage(minimum.usage_page_value()));
              state.usage_page = Some(minimum.usage_page_value());
            };
            sequence.push(DescriptorItem::UsageMinimum(UsageSpecifier::Usage(minimum.usage_id_value())));
            sequence.push(DescriptorItem::UsageMaximum(UsageSpecifier::Usage(maximum.usage_id_value())));
          }
          else {
            for usage_value in usage_range {
              let usage = Usage::try_from(*usage_value).unwrap();
              if Some(usage.usage_page_value()) != state.usage_page {
                sequence.push(DescriptorItem::UsagePage(usage.usage_page_value()));
                state.usage_page = Some(usage.usage_page_value());
              };
              sequence.push(DescriptorItem::Usage(UsageSpecifier::Usage(usage.usage_id_value())));
            };
          };
          match self.ty {
            ReportType::Input => sequence.push(DescriptorItem::Input {
              constant: if *constant { ReportConstantFlag::Constant } else { ReportConstantFlag::Data },
              layout: ReportLayoutFlag::Array,
              relative: if *relative { ReportRelativeFlag::Relative } else { ReportRelativeFlag::Absolute },
              wrap: ReportWrapFlag::NoWrap,
              linear: ReportLinearFlag::Linear,
              preferred_state: ReportPreferredStateFlag::PreferredState,
              null_state: ReportNullStateFlag::NoNullPosition,
              buffered_bytes: ReportBufferedBytesFlag::BitField,
            }),
            ReportType::Output => sequence.push(DescriptorItem::Output {
              constant: if *constant { ReportConstantFlag::Constant } else { ReportConstantFlag::Data },
              layout: ReportLayoutFlag::Array,
              relative: if *relative { ReportRelativeFlag::Relative } else { ReportRelativeFlag::Absolute },
              wrap: ReportWrapFlag::NoWrap,
              linear: ReportLinearFlag::Linear,
              preferred_state: ReportPreferredStateFlag::PreferredState,
              null_state: ReportNullStateFlag::NoNullPosition,
              volatile: ReportVolatileFlag::NonVolatile,
              buffered_bytes: ReportBufferedBytesFlag::BitField,
            }),
            ReportType::Feature => sequence.push(DescriptorItem::Feature {
              constant: if *constant { ReportConstantFlag::Constant } else { ReportConstantFlag::Data },
              layout: ReportLayoutFlag::Array,
              relative: if *relative { ReportRelativeFlag::Relative } else { ReportRelativeFlag::Absolute },
              wrap: ReportWrapFlag::NoWrap,
              linear: ReportLinearFlag::Linear,
              preferred_state: ReportPreferredStateFlag::PreferredState,
              null_state: ReportNullStateFlag::NoNullPosition,
              volatile: ReportVolatileFlag::NonVolatile,
              buffered_bytes: ReportBufferedBytesFlag::BitField,
            }),
          };
        },
      };
    };
    sequence
  }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Collection {
  pub ty: CollectionType,
  pub usage: Option<u32>,
  pub items: Vec<MainItem>,
}
impl Collection {
  fn into_descriptor_items(self, state: &mut GlobalState) -> Vec<DescriptorItem> {
    let mut sequence = vec![];
    if let Some(usage) = self.usage {
      let usage = Usage::try_from(usage).unwrap();
      if Some(usage.usage_page_value()) != state.usage_page {
        sequence.push(DescriptorItem::UsagePage(usage.usage_page_value()));
        state.usage_page = Some(usage.usage_page_value());
      };
      sequence.push(DescriptorItem::Usage(UsageSpecifier::Usage(usage.usage_id_value())));
    };
    sequence.push(DescriptorItem::Collection(self.ty));
    for item in self.items {
      sequence.append(&mut write_main_item(item, state));
    };
    sequence.push(DescriptorItem::EndCollection);
    sequence
  }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MainItem {
  Report(Report),
  Collection(Collection),
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Descriptor {
  pub items: Vec<MainItem>,
}
impl Descriptor {
  pub fn into_descriptor_items(self) -> Vec<DescriptorItem> {
    let mut sequence = vec![];
    let mut state = GlobalState::default();
    for item in self.items {
      sequence.append(&mut write_main_item(item, &mut state));
    };
    sequence
  }
}
fn write_main_item(item: MainItem, state: &mut GlobalState) -> Vec<DescriptorItem> {
  match item {
    MainItem::Collection(collection) => collection.into_descriptor_items(state),
    MainItem::Report(report) => report.into_descriptor_items(state),
  }
}
