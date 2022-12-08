use super::Generator;
use crate::field::FieldKind;
use std::borrow::Cow;

pub struct RustGenerator {
    text: String,
    last_offset: usize,
    offset: usize,
}

impl Default for RustGenerator {
    fn default() -> Self {
        Self {
            text: format!(
                "// Generated by YClass {}\n// Made by @ItsEthra\n\n",
                env!("YCLASS_VERSION")
            ),
            last_offset: 0,
            offset: 0,
        }
    }
}

impl Generator for RustGenerator {
    fn begin_class(&mut self, name: &str) {
        self.text += &format!("#[repr(C)]\npub struct {name} {{\n");
    }

    fn end_class(&mut self) {
        self.text += "}\n\n";
        self.offset = 0;
        self.last_offset = 0;
    }

    fn add_field(&mut self, name: &str, kind: FieldKind, metadata: Option<&str>) {
        let size = kind.size();
        if self.offset != self.last_offset {
            self.text += &format!(
                "    _pad_0x{:x}: [u8; 0x{:x}],\n",
                self.offset,
                self.offset - self.last_offset
            );
        }

        self.text += &format!("    pub {name}: {},\n", kind_to_type(kind, metadata));

        self.offset += size;
        self.last_offset = self.offset;
    }

    fn add_offset(&mut self, offset: usize) {
        self.offset += offset;
    }

    fn finilize(&mut self) -> String {
        std::mem::take(&mut self.text)
    }
}

fn kind_to_type(kind: FieldKind, metadata: Option<&str>) -> Cow<'static, str> {
    match kind {
        FieldKind::Unk8 | FieldKind::Unk16 | FieldKind::Unk32 | FieldKind::Unk64 => unreachable!(),
        FieldKind::I8 => "i8".into(),
        FieldKind::U8 => "u8".into(),
        FieldKind::I16 => "i16".into(),
        FieldKind::U16 => "u16".into(),
        FieldKind::I32 => "i32".into(),
        FieldKind::U32 => "u32".into(),
        FieldKind::I64 => "i64".into(),
        FieldKind::U64 => "u64".into(),
        FieldKind::F32 => "f32".into(),
        FieldKind::F64 => "f64".into(),
        FieldKind::Ptr => format!("Option<&'static {}>", metadata.unwrap()).into(),
        FieldKind::Bool => "bool".into(),
    }
}