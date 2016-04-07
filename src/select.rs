// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct KeyRange {
    // message fields
    low: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    high: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KeyRange {}

impl KeyRange {
    pub fn new() -> KeyRange {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KeyRange {
        static mut instance: ::protobuf::lazy::Lazy<KeyRange> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KeyRange,
        };
        unsafe {
            instance.get(|| {
                KeyRange {
                    low: ::protobuf::SingularField::none(),
                    high: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes low = 1;

    pub fn clear_low(&mut self) {
        self.low.clear();
    }

    pub fn has_low(&self) -> bool {
        self.low.is_some()
    }

    // Param is passed by value, moved
    pub fn set_low(&mut self, v: ::std::vec::Vec<u8>) {
        self.low = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_low<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.low.is_none() {
            self.low.set_default();
        };
        self.low.as_mut().unwrap()
    }

    // Take field
    pub fn take_low(&mut self) -> ::std::vec::Vec<u8> {
        self.low.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_low<'a>(&'a self) -> &'a [u8] {
        match self.low.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes high = 2;

    pub fn clear_high(&mut self) {
        self.high.clear();
    }

    pub fn has_high(&self) -> bool {
        self.high.is_some()
    }

    // Param is passed by value, moved
    pub fn set_high(&mut self, v: ::std::vec::Vec<u8>) {
        self.high = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_high<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.high.is_none() {
            self.high.set_default();
        };
        self.high.as_mut().unwrap()
    }

    // Take field
    pub fn take_high(&mut self) -> ::std::vec::Vec<u8> {
        self.high.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_high<'a>(&'a self) -> &'a [u8] {
        match self.high.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for KeyRange {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.low));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.high));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.low.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.high.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.low.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.high.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<KeyRange>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for KeyRange {
    fn new() -> KeyRange {
        KeyRange::new()
    }

    fn descriptor_static(_: ::std::option::Option<KeyRange>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "low",
                    KeyRange::has_low,
                    KeyRange::get_low,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "high",
                    KeyRange::has_high,
                    KeyRange::get_high,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KeyRange>(
                    "KeyRange",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KeyRange {
    fn clear(&mut self) {
        self.clear_low();
        self.clear_high();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for KeyRange {
    fn eq(&self, other: &KeyRange) -> bool {
        self.low == other.low &&
        self.high == other.high &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for KeyRange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ByItem {
    // message fields
    expr: ::protobuf::SingularPtrField<super::expression::Expr>,
    desc: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ByItem {}

impl ByItem {
    pub fn new() -> ByItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ByItem {
        static mut instance: ::protobuf::lazy::Lazy<ByItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ByItem,
        };
        unsafe {
            instance.get(|| {
                ByItem {
                    expr: ::protobuf::SingularPtrField::none(),
                    desc: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .tipb.Expr expr = 1;

    pub fn clear_expr(&mut self) {
        self.expr.clear();
    }

    pub fn has_expr(&self) -> bool {
        self.expr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expr(&mut self, v: super::expression::Expr) {
        self.expr = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_expr<'a>(&'a mut self) -> &'a mut super::expression::Expr {
        if self.expr.is_none() {
            self.expr.set_default();
        };
        self.expr.as_mut().unwrap()
    }

    // Take field
    pub fn take_expr(&mut self) -> super::expression::Expr {
        self.expr.take().unwrap_or_else(|| super::expression::Expr::new())
    }

    pub fn get_expr<'a>(&'a self) -> &'a super::expression::Expr {
        self.expr.as_ref().unwrap_or_else(|| super::expression::Expr::default_instance())
    }

    // optional bool desc = 2;

    pub fn clear_desc(&mut self) {
        self.desc = ::std::option::Option::None;
    }

    pub fn has_desc(&self) -> bool {
        self.desc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_desc(&mut self, v: bool) {
        self.desc = ::std::option::Option::Some(v);
    }

    pub fn get_desc<'a>(&self) -> bool {
        self.desc.unwrap_or(false)
    }
}

impl ::protobuf::Message for ByItem {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.expr));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.desc = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.expr.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.desc.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.expr.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.desc {
            try!(os.write_bool(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ByItem>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ByItem {
    fn new() -> ByItem {
        ByItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<ByItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "expr",
                    ByItem::has_expr,
                    ByItem::get_expr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "desc",
                    ByItem::has_desc,
                    ByItem::get_desc,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ByItem>(
                    "ByItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ByItem {
    fn clear(&mut self) {
        self.clear_expr();
        self.clear_desc();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ByItem {
    fn eq(&self, other: &ByItem) -> bool {
        self.expr == other.expr &&
        self.desc == other.desc &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ByItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SelectRequest {
    // message fields
    start_ts: ::std::option::Option<i64>,
    table_info: ::protobuf::SingularPtrField<super::schema::TableInfo>,
    index_info: ::protobuf::SingularPtrField<super::schema::IndexInfo>,
    fields: ::protobuf::RepeatedField<super::expression::Expr>,
    ranges: ::protobuf::RepeatedField<KeyRange>,
    distinct: ::std::option::Option<bool>,
    field_where: ::protobuf::SingularPtrField<super::expression::Expr>,
    group_by: ::protobuf::RepeatedField<ByItem>,
    having: ::protobuf::SingularPtrField<super::expression::Expr>,
    order_by: ::protobuf::RepeatedField<ByItem>,
    offset: ::std::option::Option<i64>,
    limit: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SelectRequest {}

impl SelectRequest {
    pub fn new() -> SelectRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SelectRequest {
        static mut instance: ::protobuf::lazy::Lazy<SelectRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SelectRequest,
        };
        unsafe {
            instance.get(|| {
                SelectRequest {
                    start_ts: ::std::option::Option::None,
                    table_info: ::protobuf::SingularPtrField::none(),
                    index_info: ::protobuf::SingularPtrField::none(),
                    fields: ::protobuf::RepeatedField::new(),
                    ranges: ::protobuf::RepeatedField::new(),
                    distinct: ::std::option::Option::None,
                    field_where: ::protobuf::SingularPtrField::none(),
                    group_by: ::protobuf::RepeatedField::new(),
                    having: ::protobuf::SingularPtrField::none(),
                    order_by: ::protobuf::RepeatedField::new(),
                    offset: ::std::option::Option::None,
                    limit: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 start_ts = 1;

    pub fn clear_start_ts(&mut self) {
        self.start_ts = ::std::option::Option::None;
    }

    pub fn has_start_ts(&self) -> bool {
        self.start_ts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_ts(&mut self, v: i64) {
        self.start_ts = ::std::option::Option::Some(v);
    }

    pub fn get_start_ts<'a>(&self) -> i64 {
        self.start_ts.unwrap_or(0)
    }

    // optional .tipb.TableInfo table_info = 2;

    pub fn clear_table_info(&mut self) {
        self.table_info.clear();
    }

    pub fn has_table_info(&self) -> bool {
        self.table_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table_info(&mut self, v: super::schema::TableInfo) {
        self.table_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table_info<'a>(&'a mut self) -> &'a mut super::schema::TableInfo {
        if self.table_info.is_none() {
            self.table_info.set_default();
        };
        self.table_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_table_info(&mut self) -> super::schema::TableInfo {
        self.table_info.take().unwrap_or_else(|| super::schema::TableInfo::new())
    }

    pub fn get_table_info<'a>(&'a self) -> &'a super::schema::TableInfo {
        self.table_info.as_ref().unwrap_or_else(|| super::schema::TableInfo::default_instance())
    }

    // optional .tipb.IndexInfo index_info = 3;

    pub fn clear_index_info(&mut self) {
        self.index_info.clear();
    }

    pub fn has_index_info(&self) -> bool {
        self.index_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index_info(&mut self, v: super::schema::IndexInfo) {
        self.index_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_index_info<'a>(&'a mut self) -> &'a mut super::schema::IndexInfo {
        if self.index_info.is_none() {
            self.index_info.set_default();
        };
        self.index_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_index_info(&mut self) -> super::schema::IndexInfo {
        self.index_info.take().unwrap_or_else(|| super::schema::IndexInfo::new())
    }

    pub fn get_index_info<'a>(&'a self) -> &'a super::schema::IndexInfo {
        self.index_info.as_ref().unwrap_or_else(|| super::schema::IndexInfo::default_instance())
    }

    // repeated .tipb.Expr fields = 4;

    pub fn clear_fields(&mut self) {
        self.fields.clear();
    }

    // Param is passed by value, moved
    pub fn set_fields(&mut self, v: ::protobuf::RepeatedField<super::expression::Expr>) {
        self.fields = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fields<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<super::expression::Expr> {
        &mut self.fields
    }

    // Take field
    pub fn take_fields(&mut self) -> ::protobuf::RepeatedField<super::expression::Expr> {
        ::std::mem::replace(&mut self.fields, ::protobuf::RepeatedField::new())
    }

    pub fn get_fields<'a>(&'a self) -> &'a [super::expression::Expr] {
        &self.fields
    }

    // repeated .tipb.KeyRange ranges = 5;

    pub fn clear_ranges(&mut self) {
        self.ranges.clear();
    }

    // Param is passed by value, moved
    pub fn set_ranges(&mut self, v: ::protobuf::RepeatedField<KeyRange>) {
        self.ranges = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ranges<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<KeyRange> {
        &mut self.ranges
    }

    // Take field
    pub fn take_ranges(&mut self) -> ::protobuf::RepeatedField<KeyRange> {
        ::std::mem::replace(&mut self.ranges, ::protobuf::RepeatedField::new())
    }

    pub fn get_ranges<'a>(&'a self) -> &'a [KeyRange] {
        &self.ranges
    }

    // optional bool distinct = 6;

    pub fn clear_distinct(&mut self) {
        self.distinct = ::std::option::Option::None;
    }

    pub fn has_distinct(&self) -> bool {
        self.distinct.is_some()
    }

    // Param is passed by value, moved
    pub fn set_distinct(&mut self, v: bool) {
        self.distinct = ::std::option::Option::Some(v);
    }

    pub fn get_distinct<'a>(&self) -> bool {
        self.distinct.unwrap_or(false)
    }

    // optional .tipb.Expr where = 7;

    pub fn clear_field_where(&mut self) {
        self.field_where.clear();
    }

    pub fn has_field_where(&self) -> bool {
        self.field_where.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_where(&mut self, v: super::expression::Expr) {
        self.field_where = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_where<'a>(&'a mut self) -> &'a mut super::expression::Expr {
        if self.field_where.is_none() {
            self.field_where.set_default();
        };
        self.field_where.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_where(&mut self) -> super::expression::Expr {
        self.field_where.take().unwrap_or_else(|| super::expression::Expr::new())
    }

    pub fn get_field_where<'a>(&'a self) -> &'a super::expression::Expr {
        self.field_where.as_ref().unwrap_or_else(|| super::expression::Expr::default_instance())
    }

    // repeated .tipb.ByItem group_by = 8;

    pub fn clear_group_by(&mut self) {
        self.group_by.clear();
    }

    // Param is passed by value, moved
    pub fn set_group_by(&mut self, v: ::protobuf::RepeatedField<ByItem>) {
        self.group_by = v;
    }

    // Mutable pointer to the field.
    pub fn mut_group_by<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<ByItem> {
        &mut self.group_by
    }

    // Take field
    pub fn take_group_by(&mut self) -> ::protobuf::RepeatedField<ByItem> {
        ::std::mem::replace(&mut self.group_by, ::protobuf::RepeatedField::new())
    }

    pub fn get_group_by<'a>(&'a self) -> &'a [ByItem] {
        &self.group_by
    }

    // optional .tipb.Expr having = 9;

    pub fn clear_having(&mut self) {
        self.having.clear();
    }

    pub fn has_having(&self) -> bool {
        self.having.is_some()
    }

    // Param is passed by value, moved
    pub fn set_having(&mut self, v: super::expression::Expr) {
        self.having = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_having<'a>(&'a mut self) -> &'a mut super::expression::Expr {
        if self.having.is_none() {
            self.having.set_default();
        };
        self.having.as_mut().unwrap()
    }

    // Take field
    pub fn take_having(&mut self) -> super::expression::Expr {
        self.having.take().unwrap_or_else(|| super::expression::Expr::new())
    }

    pub fn get_having<'a>(&'a self) -> &'a super::expression::Expr {
        self.having.as_ref().unwrap_or_else(|| super::expression::Expr::default_instance())
    }

    // repeated .tipb.ByItem order_by = 10;

    pub fn clear_order_by(&mut self) {
        self.order_by.clear();
    }

    // Param is passed by value, moved
    pub fn set_order_by(&mut self, v: ::protobuf::RepeatedField<ByItem>) {
        self.order_by = v;
    }

    // Mutable pointer to the field.
    pub fn mut_order_by<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<ByItem> {
        &mut self.order_by
    }

    // Take field
    pub fn take_order_by(&mut self) -> ::protobuf::RepeatedField<ByItem> {
        ::std::mem::replace(&mut self.order_by, ::protobuf::RepeatedField::new())
    }

    pub fn get_order_by<'a>(&'a self) -> &'a [ByItem] {
        &self.order_by
    }

    // optional int64 offset = 11;

    pub fn clear_offset(&mut self) {
        self.offset = ::std::option::Option::None;
    }

    pub fn has_offset(&self) -> bool {
        self.offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_offset(&mut self, v: i64) {
        self.offset = ::std::option::Option::Some(v);
    }

    pub fn get_offset<'a>(&self) -> i64 {
        self.offset.unwrap_or(0)
    }

    // optional int64 limit = 12;

    pub fn clear_limit(&mut self) {
        self.limit = ::std::option::Option::None;
    }

    pub fn has_limit(&self) -> bool {
        self.limit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_limit(&mut self, v: i64) {
        self.limit = ::std::option::Option::Some(v);
    }

    pub fn get_limit<'a>(&self) -> i64 {
        self.limit.unwrap_or(0)
    }
}

impl ::protobuf::Message for SelectRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.start_ts = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.table_info));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.index_info));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.fields));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ranges));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.distinct = ::std::option::Option::Some(tmp);
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.field_where));
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.group_by));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.having));
                },
                10 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.order_by));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.offset = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.limit = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.start_ts.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.table_info.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.index_info.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.fields.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.ranges.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.distinct.is_some() {
            my_size += 2;
        };
        for value in self.field_where.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.group_by.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.having.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.order_by.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.offset.iter() {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.limit.iter() {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_ts {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.table_info.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.index_info.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.fields.iter() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.ranges.iter() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.distinct {
            try!(os.write_bool(6, v));
        };
        if let Some(v) = self.field_where.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.group_by.iter() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.having.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.order_by.iter() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.offset {
            try!(os.write_int64(11, v));
        };
        if let Some(v) = self.limit {
            try!(os.write_int64(12, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SelectRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SelectRequest {
    fn new() -> SelectRequest {
        SelectRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SelectRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "start_ts",
                    SelectRequest::has_start_ts,
                    SelectRequest::get_start_ts,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "table_info",
                    SelectRequest::has_table_info,
                    SelectRequest::get_table_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "index_info",
                    SelectRequest::has_index_info,
                    SelectRequest::get_index_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "fields",
                    SelectRequest::get_fields,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "ranges",
                    SelectRequest::get_ranges,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "distinct",
                    SelectRequest::has_distinct,
                    SelectRequest::get_distinct,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "where",
                    SelectRequest::has_field_where,
                    SelectRequest::get_field_where,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "group_by",
                    SelectRequest::get_group_by,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "having",
                    SelectRequest::has_having,
                    SelectRequest::get_having,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "order_by",
                    SelectRequest::get_order_by,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "offset",
                    SelectRequest::has_offset,
                    SelectRequest::get_offset,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "limit",
                    SelectRequest::has_limit,
                    SelectRequest::get_limit,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SelectRequest>(
                    "SelectRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SelectRequest {
    fn clear(&mut self) {
        self.clear_start_ts();
        self.clear_table_info();
        self.clear_index_info();
        self.clear_fields();
        self.clear_ranges();
        self.clear_distinct();
        self.clear_field_where();
        self.clear_group_by();
        self.clear_having();
        self.clear_order_by();
        self.clear_offset();
        self.clear_limit();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SelectRequest {
    fn eq(&self, other: &SelectRequest) -> bool {
        self.start_ts == other.start_ts &&
        self.table_info == other.table_info &&
        self.index_info == other.index_info &&
        self.fields == other.fields &&
        self.ranges == other.ranges &&
        self.distinct == other.distinct &&
        self.field_where == other.field_where &&
        self.group_by == other.group_by &&
        self.having == other.having &&
        self.order_by == other.order_by &&
        self.offset == other.offset &&
        self.limit == other.limit &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SelectRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Row {
    // message fields
    handle: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Row {}

impl Row {
    pub fn new() -> Row {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Row {
        static mut instance: ::protobuf::lazy::Lazy<Row> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Row,
        };
        unsafe {
            instance.get(|| {
                Row {
                    handle: ::protobuf::SingularField::none(),
                    data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes handle = 1;

    pub fn clear_handle(&mut self) {
        self.handle.clear();
    }

    pub fn has_handle(&self) -> bool {
        self.handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_handle(&mut self, v: ::std::vec::Vec<u8>) {
        self.handle = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_handle<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.handle.is_none() {
            self.handle.set_default();
        };
        self.handle.as_mut().unwrap()
    }

    // Take field
    pub fn take_handle(&mut self) -> ::std::vec::Vec<u8> {
        self.handle.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_handle<'a>(&'a self) -> &'a [u8] {
        match self.handle.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data<'a>(&'a self) -> &'a [u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for Row {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.handle));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.handle.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.data.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.handle.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.data.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Row>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Row {
    fn new() -> Row {
        Row::new()
    }

    fn descriptor_static(_: ::std::option::Option<Row>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "handle",
                    Row::has_handle,
                    Row::get_handle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "data",
                    Row::has_data,
                    Row::get_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Row>(
                    "Row",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Row {
    fn clear(&mut self) {
        self.clear_handle();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Row {
    fn eq(&self, other: &Row) -> bool {
        self.handle == other.handle &&
        self.data == other.data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Row {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Error {
    // message fields
    code: ::std::option::Option<i32>,
    msg: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Error {}

impl Error {
    pub fn new() -> Error {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Error {
        static mut instance: ::protobuf::lazy::Lazy<Error> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Error,
        };
        unsafe {
            instance.get(|| {
                Error {
                    code: ::std::option::Option::None,
                    msg: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = ::std::option::Option::None;
    }

    pub fn has_code(&self) -> bool {
        self.code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: i32) {
        self.code = ::std::option::Option::Some(v);
    }

    pub fn get_code<'a>(&self) -> i32 {
        self.code.unwrap_or(0)
    }

    // optional string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.msg.is_none() {
            self.msg.set_default();
        };
        self.msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        self.msg.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_msg<'a>(&'a self) -> &'a str {
        match self.msg.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Error {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.code = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.msg));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.code.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.msg.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.code {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.msg.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Error>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Error {
    fn new() -> Error {
        Error::new()
    }

    fn descriptor_static(_: ::std::option::Option<Error>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "code",
                    Error::has_code,
                    Error::get_code,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "msg",
                    Error::has_msg,
                    Error::get_msg,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Error>(
                    "Error",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Error {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        self.code == other.code &&
        self.msg == other.msg &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SelectResponse {
    // message fields
    error: ::protobuf::SingularPtrField<Error>,
    rows: ::protobuf::RepeatedField<Row>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SelectResponse {}

impl SelectResponse {
    pub fn new() -> SelectResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SelectResponse {
        static mut instance: ::protobuf::lazy::Lazy<SelectResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SelectResponse,
        };
        unsafe {
            instance.get(|| {
                SelectResponse {
                    error: ::protobuf::SingularPtrField::none(),
                    rows: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .tipb.Error error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: Error) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error<'a>(&'a mut self) -> &'a mut Error {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> Error {
        self.error.take().unwrap_or_else(|| Error::new())
    }

    pub fn get_error<'a>(&'a self) -> &'a Error {
        self.error.as_ref().unwrap_or_else(|| Error::default_instance())
    }

    // repeated .tipb.Row rows = 2;

    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    // Param is passed by value, moved
    pub fn set_rows(&mut self, v: ::protobuf::RepeatedField<Row>) {
        self.rows = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rows<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Row> {
        &mut self.rows
    }

    // Take field
    pub fn take_rows(&mut self) -> ::protobuf::RepeatedField<Row> {
        ::std::mem::replace(&mut self.rows, ::protobuf::RepeatedField::new())
    }

    pub fn get_rows<'a>(&'a self) -> &'a [Row] {
        &self.rows
    }
}

impl ::protobuf::Message for SelectResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rows));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.rows.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.rows.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SelectResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SelectResponse {
    fn new() -> SelectResponse {
        SelectResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SelectResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    SelectResponse::has_error,
                    SelectResponse::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "rows",
                    SelectResponse::get_rows,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SelectResponse>(
                    "SelectResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SelectResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_rows();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SelectResponse {
    fn eq(&self, other: &SelectResponse) -> bool {
        self.error == other.error &&
        self.rows == other.rows &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SelectResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0c, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04,
    0x74, 0x69, 0x70, 0x62, 0x1a, 0x10, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0c, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x22, 0x25, 0x0a, 0x08, 0x4b, 0x65, 0x79, 0x52, 0x61, 0x6e, 0x67, 0x65,
    0x12, 0x0b, 0x0a, 0x03, 0x6c, 0x6f, 0x77, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0c, 0x0a,
    0x04, 0x68, 0x69, 0x67, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x30, 0x0a, 0x06, 0x42,
    0x79, 0x49, 0x74, 0x65, 0x6d, 0x12, 0x18, 0x0a, 0x04, 0x65, 0x78, 0x70, 0x72, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x45, 0x78, 0x70, 0x72, 0x12,
    0x0c, 0x0a, 0x04, 0x64, 0x65, 0x73, 0x63, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x22, 0xcf, 0x02,
    0x0a, 0x0d, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x10, 0x0a, 0x08, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x74, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x03, 0x12, 0x23, 0x0a, 0x0a, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x54, 0x61, 0x62,
    0x6c, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x23, 0x0a, 0x0a, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x5f,
    0x69, 0x6e, 0x66, 0x6f, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x74, 0x69, 0x70,
    0x62, 0x2e, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x1a, 0x0a, 0x06, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x74, 0x69,
    0x70, 0x62, 0x2e, 0x45, 0x78, 0x70, 0x72, 0x12, 0x1e, 0x0a, 0x06, 0x72, 0x61, 0x6e, 0x67, 0x65,
    0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x4b,
    0x65, 0x79, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x12, 0x10, 0x0a, 0x08, 0x64, 0x69, 0x73, 0x74, 0x69,
    0x6e, 0x63, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x12, 0x19, 0x0a, 0x05, 0x77, 0x68, 0x65,
    0x72, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e,
    0x45, 0x78, 0x70, 0x72, 0x12, 0x1e, 0x0a, 0x08, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x5f, 0x62, 0x79,
    0x18, 0x08, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x42, 0x79,
    0x49, 0x74, 0x65, 0x6d, 0x12, 0x1a, 0x0a, 0x06, 0x68, 0x61, 0x76, 0x69, 0x6e, 0x67, 0x18, 0x09,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x45, 0x78, 0x70, 0x72,
    0x12, 0x1e, 0x0a, 0x08, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x5f, 0x62, 0x79, 0x18, 0x0a, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x42, 0x79, 0x49, 0x74, 0x65, 0x6d,
    0x12, 0x0e, 0x0a, 0x06, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x03,
    0x12, 0x0d, 0x0a, 0x05, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x03, 0x22,
    0x23, 0x0a, 0x03, 0x52, 0x6f, 0x77, 0x12, 0x0e, 0x0a, 0x06, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0c, 0x22, 0x22, 0x0a, 0x05, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x0c, 0x0a,
    0x04, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0b, 0x0a, 0x03, 0x6d,
    0x73, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x22, 0x45, 0x0a, 0x0e, 0x53, 0x65, 0x6c, 0x65,
    0x63, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1a, 0x0a, 0x05, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x74, 0x69, 0x70, 0x62,
    0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x17, 0x0a, 0x04, 0x72, 0x6f, 0x77, 0x73, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x52, 0x6f, 0x77, 0x42,
    0x19, 0x0a, 0x15, 0x63, 0x6f, 0x6d, 0x2e, 0x70, 0x69, 0x6e, 0x67, 0x63, 0x61, 0x70, 0x2e, 0x74,
    0x69, 0x64, 0x62, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x50, 0x01, 0x4a, 0x8d, 0x15, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x4f, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0c, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x04, 0x00, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x04, 0x00, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12,
    0x03, 0x04, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x04, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x04, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x04, 0x1d,
    0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x05, 0x00, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x08,
    0xe7, 0x07, 0x01, 0x12, 0x03, 0x05, 0x00, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01,
    0x02, 0x12, 0x03, 0x05, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x05, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x05, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x07, 0x12, 0x03,
    0x05, 0x16, 0x2d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x19, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x08, 0x07, 0x15, 0x0a, 0x65, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x0b, 0x00, 0x0e, 0x01, 0x1a, 0x59, 0x20, 0x4b, 0x65, 0x79, 0x52, 0x61, 0x6e, 0x67, 0x65,
    0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x65, 0x64, 0x20,
    0x69, 0x6e, 0x64, 0x65, 0x78, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x2c,
    0x20, 0x6c, 0x6f, 0x77, 0x20, 0x69, 0x73, 0x20, 0x63, 0x6c, 0x6f, 0x73, 0x65, 0x64, 0x2c, 0x20,
    0x68, 0x69, 0x67, 0x68, 0x20, 0x69, 0x73, 0x20, 0x6f, 0x70, 0x65, 0x6e, 0x2e, 0x20, 0x28, 0x6c,
    0x6f, 0x77, 0x20, 0x3c, 0x3d, 0x20, 0x78, 0x20, 0x3c, 0x20, 0x68, 0x69, 0x67, 0x68, 0x29, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x0c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x0c, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x0c, 0x17, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x1d,
    0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0d, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x17, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x0d, 0x1e, 0x1f, 0x0a, 0x34, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x11, 0x00,
    0x14, 0x01, 0x1a, 0x28, 0x20, 0x42, 0x79, 0x49, 0x74, 0x65, 0x6d, 0x20, 0x74, 0x79, 0x70, 0x65,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x20, 0x62, 0x79, 0x20, 0x61, 0x6e,
    0x64, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x20, 0x62, 0x79, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x01, 0x12, 0x03, 0x11, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x12, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x12, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x12, 0x11,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x16, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x12, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x13, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x13, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x13, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x13, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x13,
    0x1d, 0x1e, 0x0a, 0x45, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x17, 0x00, 0x3c, 0x01, 0x1a, 0x39,
    0x20, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x77,
    0x6f, 0x72, 0x6b, 0x73, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6d, 0x70,
    0x6c, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x20, 0x73, 0x74,
    0x61, 0x74, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x17, 0x08, 0x15, 0x0a, 0x2b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x19,
    0x08, 0x24, 0x1a, 0x1e, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x19, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x19, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x17, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x22, 0x23, 0x0a, 0x5f, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x2a, 0x1a, 0x52, 0x20, 0x49, 0x66, 0x20, 0x74, 0x61, 0x62,
    0x6c, 0x65, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x6e,
    0x75, 0x6c, 0x6c, 0x2c, 0x20, 0x69, 0x74, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e,
    0x74, 0x73, 0x20, 0x61, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x73, 0x63, 0x61, 0x6e, 0x2c,
    0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x20, 0x77, 0x6f, 0x75, 0x6c,
    0x64, 0x20, 0x62, 0x65, 0x20, 0x6e, 0x75, 0x6c, 0x6c, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x1c, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x1c, 0x1b, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x1c, 0x28, 0x29, 0x0a, 0x60, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1f, 0x08, 0x2a,
    0x1a, 0x53, 0x20, 0x49, 0x66, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x5f, 0x69, 0x6e, 0x66, 0x6f,
    0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x6e, 0x75, 0x6c, 0x6c, 0x2c, 0x20, 0x69, 0x74,
    0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x69,
    0x6e, 0x64, 0x65, 0x78, 0x20, 0x73, 0x63, 0x61, 0x6e, 0x2c, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x6e,
    0x75, 0x6c, 0x6c, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x1f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12, 0x03, 0x1f, 0x11,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1f, 0x1b, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1f, 0x28, 0x29, 0x0a, 0xab, 0x01,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x23, 0x08, 0x21, 0x1a, 0x9d, 0x01, 0x20, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x6c, 0x65,
    0x63, 0x74, 0x65, 0x64, 0x2c, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x74, 0x79, 0x70,
    0x65, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x20,
    0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x69,
    0x6d, 0x70, 0x6c, 0x65, 0x20, 0x73, 0x63, 0x61, 0x6e, 0x2e, 0x0a, 0x20, 0x6f, 0x72, 0x20, 0x61,
    0x67, 0x67, 0x72, 0x65, 0x67, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x75, 0x6e, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x6e, 0x6f, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x73, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x2c, 0x20, 0x6f, 0x6e, 0x6c,
    0x79, 0x20, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65,
    0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x23, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x23, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x23, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x23, 0x1f, 0x20, 0x0a, 0x34, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x26, 0x08, 0x25,
    0x1a, 0x27, 0x20, 0x64, 0x69, 0x73, 0x6a, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x68, 0x61, 0x6e, 0x64,
    0x6c, 0x65, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20,
    0x73, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x26, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x06,
    0x12, 0x03, 0x26, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x26, 0x1a, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x26, 0x23,
    0x24, 0x0a, 0x1f, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x29, 0x08, 0x23, 0x1a, 0x12,
    0x20, 0x64, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x63, 0x74, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x04, 0x12, 0x03, 0x29, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x05, 0x12, 0x03, 0x29, 0x11, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x29, 0x16, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x29, 0x21, 0x22, 0x0a, 0x1f, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x06, 0x12, 0x03, 0x2c, 0x08, 0x20, 0x1a, 0x12, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x20,
    0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x06, 0x04, 0x12, 0x03, 0x2c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x06, 0x06, 0x12, 0x03, 0x2c, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x01,
    0x12, 0x03, 0x2c, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x03, 0x12, 0x03,
    0x2c, 0x1e, 0x1f, 0x0a, 0x1f, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x07, 0x12, 0x03, 0x2f, 0x08, 0x25,
    0x1a, 0x12, 0x20, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x20, 0x62, 0x79, 0x20, 0x63, 0x6c, 0x61, 0x75,
    0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x04, 0x12, 0x03, 0x2f,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x06, 0x12, 0x03, 0x2f, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03, 0x2f, 0x18, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x03, 0x12, 0x03, 0x2f, 0x23, 0x24, 0x0a, 0x1d, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x08, 0x12, 0x03, 0x32, 0x08, 0x21, 0x1a, 0x10, 0x20, 0x68, 0x61, 0x76, 0x69,
    0x6e, 0x67, 0x20, 0x63, 0x6c, 0x61, 0x75, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x08, 0x04, 0x12, 0x03, 0x32, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x08, 0x06, 0x12, 0x03, 0x32, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x01,
    0x12, 0x03, 0x32, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x03, 0x12, 0x03,
    0x32, 0x1f, 0x20, 0x0a, 0x1f, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x09, 0x12, 0x03, 0x35, 0x08, 0x26,
    0x1a, 0x12, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x20, 0x62, 0x79, 0x20, 0x63, 0x6c, 0x61, 0x75,
    0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x04, 0x12, 0x03, 0x35,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x06, 0x12, 0x03, 0x35, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x01, 0x12, 0x03, 0x35, 0x18, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x03, 0x12, 0x03, 0x35, 0x23, 0x25, 0x0a, 0x24, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x0a, 0x12, 0x03, 0x38, 0x08, 0x23, 0x1a, 0x17, 0x20, 0x6f, 0x66, 0x66, 0x73,
    0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x38, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x38, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x38, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x38, 0x20, 0x22, 0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x0b, 0x12, 0x03, 0x3b, 0x08, 0x22, 0x1a, 0x22, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65,
    0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x3b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x0b, 0x05, 0x12, 0x03, 0x3b, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0b, 0x01,
    0x12, 0x03, 0x3b, 0x17, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0b, 0x03, 0x12, 0x03,
    0x3b, 0x1f, 0x21, 0x0a, 0x2c, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x3f, 0x00, 0x42, 0x01, 0x1a,
    0x20, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x61, 0x6c, 0x6c,
    0x20, 0x69, 0x6e, 0x20, 0x74, 0x65, 0x78, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x3f, 0x08, 0x0b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x40, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x40, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x40, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x40, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x40,
    0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x41, 0x08, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x41, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x41, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x41, 0x17, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x41, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x44,
    0x00, 0x47, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x44, 0x08, 0x0d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x45, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x45, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x45, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x45, 0x17, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x45, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x46, 0x08,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x46, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x46, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x46, 0x18, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x46, 0x1e, 0x1f, 0x0a, 0x29, 0x0a, 0x02, 0x04, 0x05, 0x12,
    0x04, 0x4a, 0x00, 0x4f, 0x01, 0x1a, 0x1d, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x4a, 0x08, 0x16,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x4b, 0x08, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x4b, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x4b, 0x17, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x4b, 0x1f, 0x20, 0x0a, 0x1b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x4e,
    0x08, 0x1e, 0x1a, 0x0e, 0x20, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x20, 0x72, 0x6f, 0x77, 0x73,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4e, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03, 0x4e, 0x11, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4e, 0x15, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4e, 0x1c, 0x1d,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
