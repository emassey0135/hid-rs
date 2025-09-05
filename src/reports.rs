use bitvec::prelude::*;
use crate::descriptors::*;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReportFieldValue {
  UnsignedVariable(Option<u32>),
  SignedVariable(Option<i32>),
  Array(Option<u32>),
}
pub fn write_report(report: Report, values: Vec<ReportFieldValue>) -> BitVec<u8, Lsb0> {
  let mut data = BitVec::<u8, Lsb0>::new();
  let mut cursor: usize = 0;
  if let Some(id) = report.id {
    data.resize(8, false);
    data.store::<u8>(id);
    cursor = 8;
  };
  if report.fields.len() != values.len() {
    panic!("Either not enough or too many values provided");
  };
  for (field, value) in report.fields.into_iter().zip(values.into_iter()) {
    match (field, value) {
      (ReportField::Variable { size, logical_minimum, logical_maximum, .. }, ReportFieldValue::UnsignedVariable(value)) if logical_minimum >= 0 && logical_maximum >= 0 => {
        if let Some(value) = value {
          if value < logical_minimum as u32 || value > logical_maximum as u32 {
            panic!("Value is out of the specified range")
          };
          data.resize(cursor+(size as usize), false);
          data[cursor..cursor+(size as usize)].store_be::<u32>(value);
          cursor += size as usize;
        }
        else {
          let null_value: u32 = if logical_minimum != 0 {
            0
            }
          else {
            if logical_maximum as u32 >= 2u32.pow(size)-1 {
              panic!("The logical minimum and logical maximum contain all possible values")
            };
            (logical_maximum as u32)+1
          };
          data.resize(cursor+(size as usize), false);
          data[cursor..cursor+(size as usize)].store_be::<u32>(null_value);
          cursor += size as usize;
        };
      },
      (ReportField::Variable { size, logical_minimum, logical_maximum, .. }, ReportFieldValue::SignedVariable(value)) if logical_minimum < 0 || logical_maximum < 0 => {
        if let Some(value) = value {
          if value < logical_minimum || value > logical_maximum {
            panic!("Value is out of the specified range")
          };
          data.resize(cursor+(size as usize), false);
          data[cursor..cursor+(size as usize)].store_be::<i32>(value);
          cursor += size as usize;
        }
        else {
          let null_value: i32 = if logical_minimum >= -2i32.pow(size)+1 {
            logical_minimum-1
            }
          else {
            if logical_maximum >= 2i32.pow(size)-1 {
              panic!("The logical minimum and logical maximum contain all possible values")
            };
            logical_maximum+1
          };
          data.resize(cursor+(size as usize), false);
          data[cursor..cursor+(size as usize)].store_be::<i32>(null_value);
          cursor += size as usize;
        };
      },
      (ReportField::Array { size, logical_minimum, logical_maximum, usage_range, .. }, ReportFieldValue::Array(usage)) => {
        if let Some(usage) = usage {
          if !usage_range.contains(&usage) {
            panic!("Usage is out of the specified range")
          };
          let value = usage_range.iter().position(|usage2| usage==*usage2).unwrap() as u32 + logical_minimum as u32;
          if value > logical_maximum as u32 {
            panic!("Value is out of the specified range")
          };
          data.resize(cursor+(size as usize), false);
          data[cursor..cursor+(size as usize)].store_be::<u32>(value);
          cursor += size as usize;
        }
        else {
          let null_value: u32 = if logical_minimum != 0 {
            0
            }
          else {
            if logical_maximum as u32 >= 2u32.pow(size)-1 {
              panic!("The logical minimum and logical maximum contain all possible values")
            };
            (logical_maximum as u32)+1
          };
          data.resize(cursor+(size as usize), false);
          data[cursor..cursor+(size as usize)].store_be::<u32>(null_value);
          cursor += size as usize;
        };
      },
      _ => panic!("Report fields and values do not match"),
    };
  };
  data
    .chunks(8)
    .rev()
    .flatten()
    .collect::<BitVec<u8, Lsb0>>()
}
pub fn read_report(report: Report, data: BitVec<u8, Lsb0>) -> Vec<ReportFieldValue> {
  let data = data
    .chunks(8)
    .rev()
    .flatten()
    .collect::<BitVec<u8, Lsb0>>();
  let mut values = vec![];
  let mut cursor: usize = 0;
  if let Some(id) = report.id {
    if data[0..8].load::<u8>() != id {
      panic!("ID does not match");
    };
    cursor += 8;
  };
  for field in report.fields {
    match field {
      ReportField::Variable { size, logical_minimum, logical_maximum, .. } => {
        if logical_minimum < 0 || logical_maximum < 0 {
          let value = data[cursor..cursor+(size as usize)].load::<i32>();
          if value < logical_minimum || value > logical_maximum {
            values.push(ReportFieldValue::SignedVariable(None));
          }
          else {
            values.push(ReportFieldValue::SignedVariable(Some(value)));
          };
        }
        else {
          let value = data[cursor..cursor+(size as usize)].load::<u32>();
          if value < logical_minimum as u32 || value > logical_maximum as u32 {
            values.push(ReportFieldValue::UnsignedVariable(None));
          }
          else {
            values.push(ReportFieldValue::UnsignedVariable(Some(value)));
          };
        };
        cursor += size as usize;
      },
      ReportField::Array { size, logical_minimum, logical_maximum, usage_range, .. } => {
        let value = data[cursor..cursor+(size as usize)].load::<u32>();
        if value < logical_minimum as u32 || value > logical_maximum as u32 {
          values.push(ReportFieldValue::Array(None));
        }
        else {
          let usage = usage_range.get((value-logical_minimum as u32) as usize).unwrap();
          values.push(ReportFieldValue::Array(Some(*usage)));
        };
        cursor += size as usize;
      },
    };
  };
  values
}
