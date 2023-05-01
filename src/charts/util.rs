use substring::Substring;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl From<(f64, f64)> for Point {
    fn from(val: (f64, f64)) -> Self {
        Point { x: val.0, y: val.1 }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Box {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
impl Box {
    pub fn width(&self) -> f64 {
        self.right - self.left
    }
    pub fn height(&self) -> f64 {
        self.bottom - self.top
    }
}

impl From<f64> for Box {
    fn from(val: f64) -> Self {
        Box {
            left: val,
            top: val,
            right: val,
            bottom: val,
        }
    }
}
impl From<(f64, f64)> for Box {
    fn from(val: (f64, f64)) -> Self {
        Box {
            left: val.0,
            top: val.1,
            right: val.0,
            bottom: val.1,
        }
    }
}
impl From<(f64, f64, f64)> for Box {
    fn from(val: (f64, f64, f64)) -> Self {
        Box {
            left: val.0,
            top: val.1,
            right: val.2,
            bottom: val.1,
        }
    }
}
impl From<(f64, f64, f64, f64)> for Box {
    fn from(val: (f64, f64, f64, f64)) -> Self {
        Box {
            left: val.0,
            top: val.1,
            right: val.2,
            bottom: val.3,
        }
    }
}

pub fn format_float(value: f64) -> String {
    let str = format!("{:.1}", value);
    if str.ends_with(".0") {
        return str.substring(0, str.len() - 2).to_string();
    }
    str
}

#[derive(Clone, Debug, Default)]
pub(crate) struct AxisValueParams {
    pub data_list: Vec<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub split_number: usize,
    pub reverse: Option<bool>,
}
#[derive(Clone, Debug, Default)]
pub(crate) struct AxisValues {
    pub data: Vec<String>,
    pub min: f64,
    pub max: f64,
}

impl AxisValues {
    fn get_offset(&self) -> f64 {
        self.max - self.min
    }
    pub(crate) fn get_offset_height(&self, value: f64, max_height: f64) -> f64 {
        let percent = (value - self.min) / self.get_offset();
        max_height - percent * max_height
    }
}

pub(crate) fn get_axis_values(params: AxisValueParams) -> AxisValues {
    let mut min = 0.0;
    let mut max = f64::MIN;

    for item in params.data_list.iter() {
        let value = item.to_owned();
        if value > max {
            max = value;
        }
        if value < min {
            min = value;
        }
    }
    if let Some(value) = params.min {
        if value < min {
            min = value;
        }
    }
    if let Some(value) = params.max {
        if value > max {
            max = value;
        }
    }
    let mut unit = ((max - min) / params.split_number as f64) as i32;
    let mut base = 1;
    while unit >= 10 {
        unit /= 10;
        base *= 10;
    }
    let unit = if unit <= 1 {
        base
    } else if unit <= 2 {
        base * 2
    } else if unit <= 5 {
        base * 5
    } else {
        base * 10
    };

    let mut data = vec![];
    for i in 0..=params.split_number {
        let value = min + (i * unit) as f64;
        data.push(format_float(value));
    }
    if params.reverse.unwrap_or_default() {
        data.reverse();
    }

    AxisValues {
        data,
        min: min,
        max: min + (unit * params.split_number) as f64,
    }
}
