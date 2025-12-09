#![allow(missing_docs)]

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(from = "alox_48::Userdata", into = "alox_48::Userdata")]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl From<alox_48::Userdata> for Color {
    fn from(value: alox_48::Userdata) -> Self {
        *bytemuck::from_bytes(&value.data)
    }
}

impl From<Color> for alox_48::Userdata {
    fn from(value: Color) -> Self {
        alox_48::Userdata {
            class: "Color".into(),
            data: bytemuck::bytes_of(&value).to_vec(),
        }
    }
}

impl From<Color> for alox_48::Value {
    fn from(value: Color) -> Self {
        Self::Userdata(value.into())
    }
}

// Default values
impl Default for Color {
    fn default() -> Self {
        Self {
            red: 255.0,
            green: 255.0,
            blue: 255.0,
            alpha: 255.0,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(from = "alox_48::Userdata", into = "alox_48::Userdata")]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Tone {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub gray: f64,
}

impl From<alox_48::Userdata> for Tone {
    fn from(value: alox_48::Userdata) -> Self {
        *bytemuck::from_bytes(&value.data)
    }
}

impl From<Tone> for alox_48::Userdata {
    fn from(value: Tone) -> Self {
        alox_48::Userdata {
            class: "Tone".into(),
            data: bytemuck::bytes_of(&value).to_vec(),
        }
    }
}

impl From<Tone> for alox_48::Value {
    fn from(value: Tone) -> Self {
        Self::Userdata(value.into())
    }
}

/// Normal RGSS has dynamically dimensioned arrays, but in practice that does not map well to Rust.
/// We don't particularly need dynamically sized arrays anyway.
/// 1D Table.
#[derive(Debug, Default, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(from = "alox_48::Userdata", into = "alox_48::Userdata")]
pub struct Table1 {
    xsize: usize,
    data: Vec<i16>,
}

impl From<alox_48::Userdata> for Table1 {
    fn from(value: alox_48::Userdata) -> Self {
        let u32_slice: &[u32] =
            bytemuck::cast_slice(&value.data[0..std::mem::size_of::<u32>() * 5]);

        assert_eq!(u32_slice[0], 1);
        let xsize = u32_slice[1] as usize;
        let ysize = u32_slice[2] as usize;
        let zsize = u32_slice[3] as usize;
        let len = u32_slice[4] as usize;

        assert_eq!(xsize * ysize * zsize, len);
        let data = bytemuck::cast_slice(&value.data[(std::mem::size_of::<u32>() * 5)..]).to_vec();
        assert_eq!(data.len(), len);

        Self { xsize, data }
    }
}

impl From<Table1> for alox_48::Userdata {
    fn from(value: Table1) -> Self {
        let header = &[1, value.xsize as u32, 1, 1, value.data.len() as u32];
        let mut data = bytemuck::pod_collect_to_vec(header);
        data.extend_from_slice(bytemuck::cast_slice(&value.data));

        Self {
            class: "Table".into(),
            data,
        }
    }
}

/// 2D table. See [`Table1`].
#[derive(Debug, Default, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(from = "alox_48::Userdata", into = "alox_48::Userdata")]
pub struct Table2 {
    xsize: usize,
    ysize: usize,
    data: Vec<i16>,
}

impl From<alox_48::Userdata> for Table2 {
    fn from(value: alox_48::Userdata) -> Self {
        let u32_slice: &[u32] =
            bytemuck::cast_slice(&value.data[0..std::mem::size_of::<u32>() * 5]);

        assert_eq!(u32_slice[0], 2);
        let xsize = u32_slice[1] as usize;
        let ysize = u32_slice[2] as usize;
        let zsize = u32_slice[3] as usize;
        let len = u32_slice[4] as usize;

        assert_eq!(xsize * ysize * zsize, len);
        let data = bytemuck::cast_slice(&value.data[(std::mem::size_of::<u32>() * 5)..]).to_vec();
        assert_eq!(data.len(), len);

        Self { xsize, ysize, data }
    }
}

impl From<Table2> for alox_48::Userdata {
    fn from(value: Table2) -> Self {
        let header = &[
            2,
            value.xsize as u32,
            value.ysize as u32,
            1,
            value.data.len() as u32,
        ];
        let mut data = bytemuck::pod_collect_to_vec(header);
        data.extend_from_slice(bytemuck::cast_slice(&value.data));

        Self {
            class: "Table".into(),
            data,
        }
    }
}

#[derive(Debug, Default, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(from = "alox_48::Userdata", into = "alox_48::Userdata")]
/// 3D table. See [`Table2`].
pub struct Table3 {
    xsize: usize,
    ysize: usize,
    zsize: usize,
    data: Vec<i16>,
}

impl From<alox_48::Userdata> for Table3 {
    fn from(value: alox_48::Userdata) -> Self {
        let u32_slice: &[u32] =
            bytemuck::cast_slice(&value.data[0..std::mem::size_of::<u32>() * 5]);

        assert_eq!(u32_slice[0], 3);
        let xsize = u32_slice[1] as usize;
        let ysize = u32_slice[2] as usize;
        let zsize = u32_slice[3] as usize;
        let len = u32_slice[4] as usize;

        assert_eq!(xsize * ysize * zsize, len);
        let data = bytemuck::cast_slice(&value.data[(std::mem::size_of::<u32>() * 5)..]).to_vec();
        assert_eq!(data.len(), len);

        Self {
            xsize,
            ysize,
            zsize,
            data,
        }
    }
}

impl From<Table3> for alox_48::Userdata {
    fn from(value: Table3) -> Self {
        let header = &[
            3,
            value.xsize as u32,
            value.ysize as u32,
            value.zsize as u32,
            value.data.len() as u32,
        ];
        let mut data = bytemuck::pod_collect_to_vec(header);
        data.extend_from_slice(bytemuck::cast_slice(&value.data));

        Self {
            class: "Table".into(),
            data,
        }
    }
}
