use std::{fmt, u8};
use serde::Deserialize;
use serde::de::{
    self, DeserializeSeed, IntoDeserializer,
    MapAccess, SeqAccess, VariantAccess, Visitor,
    Expected, Unexpected,
};

// Avalon doesn't even return proper JSON,
// it return JSON containing a *custom* data format
// This is a custom deserializer for that data format

static POW10: [f64; 309] = [
    1e000, 1e001, 1e002, 1e003, 1e004, 1e005, 1e006, 1e007, 1e008, 1e009, //
    1e010, 1e011, 1e012, 1e013, 1e014, 1e015, 1e016, 1e017, 1e018, 1e019, //
    1e020, 1e021, 1e022, 1e023, 1e024, 1e025, 1e026, 1e027, 1e028, 1e029, //
    1e030, 1e031, 1e032, 1e033, 1e034, 1e035, 1e036, 1e037, 1e038, 1e039, //
    1e040, 1e041, 1e042, 1e043, 1e044, 1e045, 1e046, 1e047, 1e048, 1e049, //
    1e050, 1e051, 1e052, 1e053, 1e054, 1e055, 1e056, 1e057, 1e058, 1e059, //
    1e060, 1e061, 1e062, 1e063, 1e064, 1e065, 1e066, 1e067, 1e068, 1e069, //
    1e070, 1e071, 1e072, 1e073, 1e074, 1e075, 1e076, 1e077, 1e078, 1e079, //
    1e080, 1e081, 1e082, 1e083, 1e084, 1e085, 1e086, 1e087, 1e088, 1e089, //
    1e090, 1e091, 1e092, 1e093, 1e094, 1e095, 1e096, 1e097, 1e098, 1e099, //
    1e100, 1e101, 1e102, 1e103, 1e104, 1e105, 1e106, 1e107, 1e108, 1e109, //
    1e110, 1e111, 1e112, 1e113, 1e114, 1e115, 1e116, 1e117, 1e118, 1e119, //
    1e120, 1e121, 1e122, 1e123, 1e124, 1e125, 1e126, 1e127, 1e128, 1e129, //
    1e130, 1e131, 1e132, 1e133, 1e134, 1e135, 1e136, 1e137, 1e138, 1e139, //
    1e140, 1e141, 1e142, 1e143, 1e144, 1e145, 1e146, 1e147, 1e148, 1e149, //
    1e150, 1e151, 1e152, 1e153, 1e154, 1e155, 1e156, 1e157, 1e158, 1e159, //
    1e160, 1e161, 1e162, 1e163, 1e164, 1e165, 1e166, 1e167, 1e168, 1e169, //
    1e170, 1e171, 1e172, 1e173, 1e174, 1e175, 1e176, 1e177, 1e178, 1e179, //
    1e180, 1e181, 1e182, 1e183, 1e184, 1e185, 1e186, 1e187, 1e188, 1e189, //
    1e190, 1e191, 1e192, 1e193, 1e194, 1e195, 1e196, 1e197, 1e198, 1e199, //
    1e200, 1e201, 1e202, 1e203, 1e204, 1e205, 1e206, 1e207, 1e208, 1e209, //
    1e210, 1e211, 1e212, 1e213, 1e214, 1e215, 1e216, 1e217, 1e218, 1e219, //
    1e220, 1e221, 1e222, 1e223, 1e224, 1e225, 1e226, 1e227, 1e228, 1e229, //
    1e230, 1e231, 1e232, 1e233, 1e234, 1e235, 1e236, 1e237, 1e238, 1e239, //
    1e240, 1e241, 1e242, 1e243, 1e244, 1e245, 1e246, 1e247, 1e248, 1e249, //
    1e250, 1e251, 1e252, 1e253, 1e254, 1e255, 1e256, 1e257, 1e258, 1e259, //
    1e260, 1e261, 1e262, 1e263, 1e264, 1e265, 1e266, 1e267, 1e268, 1e269, //
    1e270, 1e271, 1e272, 1e273, 1e274, 1e275, 1e276, 1e277, 1e278, 1e279, //
    1e280, 1e281, 1e282, 1e283, 1e284, 1e285, 1e286, 1e287, 1e288, 1e289, //
    1e290, 1e291, 1e292, 1e293, 1e294, 1e295, 1e296, 1e297, 1e298, 1e299, //
    1e300, 1e301, 1e302, 1e303, 1e304, 1e305, 1e306, 1e307, 1e308,
];

macro_rules! overflow {
    ($a:ident * 10 + $b:ident, $c:expr) => {
        match $c {
            c => $a >= c / 10 && ($a > c / 10 || $b > c % 10),
        }
    };
}

#[derive(Debug)]
pub enum ErrorCode {
    Message(String),

    Eof,
    Syntax,
    ExpectedUnsignedInteger,
    ExpectedMap,
    ExpectedSequence,
    TrailingCharacters,
    Overflow,
    InvalidNumber,
    NumberOutOfRange,
    InvalidIdentifier,
    ExpectedMapBracket,
    ExpectedMapColon,
    ExpectedChar(u8, u8),
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCode::Message(msg) => write!(f, "{}", msg),
            ErrorCode::Eof => write!(f, "unexpected end of input"),
            ErrorCode::Syntax => write!(f, "syntax error"),
            ErrorCode::ExpectedUnsignedInteger => write!(f, "expected unsigned integer"),
            ErrorCode::ExpectedMap => write!(f, "expected map"),
            ErrorCode::ExpectedSequence => write!(f, "expected sequence"),
            ErrorCode::TrailingCharacters => write!(f, "trailing characters"),
            ErrorCode::Overflow => write!(f, "overflow"),
            ErrorCode::InvalidNumber => write!(f, "invalid number"),
            ErrorCode::NumberOutOfRange => write!(f, "number out of range"),
            ErrorCode::InvalidIdentifier => write!(f, "invalid identifier"),
            ErrorCode::ExpectedMapBracket => write!(f, "expected map bracket"),
            ErrorCode::ExpectedMapColon => write!(f, "expected map colon"),
            ErrorCode::ExpectedChar(c, c2) => write!(f, "expected char '{}' found '{}'", *c as char, *c2 as char),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub code: ErrorCode,
    pub column: usize,
}

impl Error {
    pub fn new(code: ErrorCode, column: usize) -> Self {
        Error { code, column }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at column {}", self.code, self.column)
    }
}

impl std::error::Error for Error {}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error {
            code: ErrorCode::Message(msg.to_string()),
            column: 0,
        }
    }
}

// Some float values are represented as integers * 100
pub enum Number {
    F64(f64),
    U64(u64),
    I64(i64),
}

impl Number {
    fn visit_u64<'de, V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Number::F64(f) => Err(Error::new(ErrorCode::InvalidNumber, 0)),
            Number::U64(u) => visitor.visit_u64(u),
            Number::I64(i) => visitor.visit_u64(i as u64),
        }
    }

    fn visit_f64<'de, V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Number::F64(f) => visitor.visit_f64(f),
            Number::U64(u) => visitor.visit_f64(u as f64 / 100.0),
            Number::I64(i) => visitor.visit_f64(i as f64 / 100.0),
        }
    }

    fn visit_i64<'de, V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Number::F64(f) => Err(Error::new(ErrorCode::InvalidNumber, 0)),
            Number::U64(u) => visitor.visit_i64(u as i64),
            Number::I64(i) => visitor.visit_i64(i),
        }
    }

    fn neg(&self) -> Self {
        match self {
            Number::F64(f) => Number::F64(-f),
            Number::U64(u) => Number::I64(-(*u as i64)),
            Number::I64(i) => Number::I64(-i),
        }
    }
}

// Handles separation of list elements
// These are space delimited
struct ListSeparator<'a, 'de> {
    de: &'a mut MsgDeserializer<'de>,
    first: bool,
}

impl<'a, 'de> ListSeparator<'a, 'de> {
    fn new(de: &'a mut MsgDeserializer<'de>) -> Self {
        ListSeparator { de, first: true }
    }
}

impl<'de, 'a> SeqAccess<'de> for ListSeparator<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.de.peek_some()? == b']' {
            return Ok(None)
        }

        // Sepecial case for a space then ending bracket
        if let Some((b' ', b']')) = self.de.peek2() {
            return Ok(None)
        }

        if self.first {
            self.first = false;
        } else {
            // Optionally theres a comma here, in the event its a list of strings
            if self.de.peek_some()? == b',' {
                self.de.expect(b',')?;
            }
            self.de.expect(b' ')?;
        }

        // OPTIONALLY YET ANOTHER SPACE
        if self.de.peek_some()? == b' ' {
            self.de.expect(b' ')?;
        }

        seed.deserialize(&mut *self.de).map(Some)
    }
}

struct RootMap<'a, 'de> {
    de: &'a mut MsgDeserializer<'de>,
    first: bool,
}

impl<'a, 'de> RootMap<'a, 'de> {
    fn new(de: &'a mut MsgDeserializer<'de>) -> Self {
        RootMap { de, first: true }
    }
}

impl<'de, 'a> MapAccess<'de> for RootMap<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.de.peek().is_none() {
            return Ok(None)
        }

        // If this is the first element, we don't need to check for a space
        if self.first {
            self.first = false;
        } else {
            self.de.expect(b' ')?;
        }

        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error>
    where
        V: DeserializeSeed<'de>,
    {
        if self.de.next_some()? != b'[' {
            return Err(self.de.error(ErrorCode::ExpectedMapBracket));
        }
        let r = seed.deserialize(&mut *self.de);

        // There is optionally a space here .-.
        if self.de.peek_some()? == b' ' {
            self.de.expect(b' ')?;
        }
        self.de.expect(b']')?;
        r
    }
}

struct ChildMap<'a, 'de> {
    de: &'a mut MsgDeserializer<'de>,
    first: bool,
}

impl<'a, 'de> ChildMap<'a, 'de> {
    fn new(de: &'a mut MsgDeserializer<'de>) -> Self {
        ChildMap { de, first: true }
    }
}

impl<'de, 'a> MapAccess<'de> for ChildMap<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.de.peek_some()? == b']' {
            return Ok(None)
        }

        // Sepecial case for a space then ending bracket
        if let Some((b' ', b']')) = self.de.peek2() {
            return Ok(None)
        }

        // If this is the first element, we don't need to check for a space
        if self.first {
            self.first = false;
        } else {
            // Optionally theres a comma here, in the event the values are strings
            if self.de.peek_some()? == b',' {
                self.de.discard();
            }
            self.de.expect(b' ')?;
        }
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error>
    where
        V: DeserializeSeed<'de>,
    {
        self.de.expect(b':')?;
        // Optionally there is a space here, in the event the values are strings
        if self.de.peek_some()? == b' ' {
            self.de.discard();
        }
        seed.deserialize(&mut *self.de)
    }
}

pub struct MsgDeserializer<'de> {
    input: &'de [u8],
    index: usize,
    depth: usize,
}

impl<'de> MsgDeserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        MsgDeserializer { input: input.as_bytes(), index: 0, depth: 0 }
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    let mut deserializer = MsgDeserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.peek().is_none() {
        Ok(t)
    } else {
        Err(deserializer.error(ErrorCode::TrailingCharacters))
    }
}

impl<'de> MsgDeserializer<'de> {
    fn pos(&self) -> usize {
        self.index-1
    }

    fn error(&self, err: ErrorCode) -> Error {
        Error::new(err, self.pos())
    }

    fn peek(&mut self) -> Option<u8> {
        if self.index < self.input.len() {
            Some(self.input[self.index])
        } else {
            None
        }
    }

    fn peek_some(&mut self) -> Result<u8, Error> {
        self.peek().ok_or_else(|| self.error(ErrorCode::Eof))
    }

    fn peek_or_null(&mut self) -> u8 {
        self.peek().unwrap_or(b'\0')
    }

    fn peek2(&mut self) -> Option<(u8, u8)> {
        if self.index + 1 < self.input.len() {
            Some((self.input[self.index], self.input[self.index+1]))
        } else {
            None
        }
    }

    fn next(&mut self) -> Option<u8> {
        if self.index < self.input.len() {
            self.index += 1;
            Some(self.input[self.index-1])
        } else {
            None
        }
    }

    fn next_some(&mut self) -> Result<u8, Error> {
        self.next().ok_or_else(|| self.error(ErrorCode::Eof))
    }

    fn discard(&mut self) {
        self.index += 1;
    }

    fn expect(&mut self, c: u8) -> Result<(), Error> {
        let ch = self.next_some()?;
        if c == ch {
            Ok(())
        } else {
            Err(self.error(ErrorCode::ExpectedChar(c, ch)))
        }
    }

    fn parse_any_number(&mut self) -> Result<Number, Error> {
        let next = match self.next() {
            Some(b) => b,
            None => {
                return Err(self.error(ErrorCode::Eof));
            }
        };

        match next {
            c @ b'0'..=b'9' => {
                let mut significand = (c - b'0') as u64;

                loop {
                    match self.peek_or_null() {
                        c @ b'0'..=b'9' => {
                            let digit = (c - b'0') as u64;

                            // We need to be careful with overflow.
                            if overflow!(significand * 10 + digit, u64::max_value()) {
                                return Err(self.error(ErrorCode::Overflow));
                            }

                            self.discard();
                            significand = significand * 10 + digit;
                        }
                        b'.' => {
                            return Ok(Number::F64(self.parse_decimal(significand, 0)?));
                        }
                        b'%' => {
                            self.discard();
                            return Ok(Number::F64(significand as f64 / 100.0));
                        }
                        _ => {
                            return Ok(Number::U64(significand));
                        }
                    }
                }
            }
            _ => Err(self.error(ErrorCode::ExpectedUnsignedInteger)),
        }
    }

    fn parse_decimal(
        &mut self,
        mut significand: u64,
        mut exponent: i32,
    ) -> Result<f64, Error> {
        self.discard();

        while let c @ b'0'..=b'9' = self.peek_or_null() {
            let digit = (c - b'0') as u64;

            if overflow!(significand * 10 + digit, u64::max_value()) {
                return Err(self.error(ErrorCode::Overflow));
            }

            self.discard();
            significand = significand * 10 + digit;
            exponent -= 1;
        }

        if let Some(b'%') = self.peek() {
            self.discard();
            exponent -= 2;
        }

        // Error if there is not at least one digit after the decimal point.
        if exponent == 0 {
            match self.peek() {
                Some(_) => return Err(self.error(ErrorCode::InvalidNumber)),
                None => return Err(self.error(ErrorCode::Eof)),
            }
        }

        self.f64_from_parts(significand, exponent)
    }

    fn f64_from_parts(
        &mut self,
        significand: u64,
        mut exponent: i32,
    ) -> Result<f64, Error> {
        let mut f = significand as f64;
        loop {
            match POW10.get(exponent.wrapping_abs() as usize) {
                Some(&pow) => {
                    if exponent >= 0 {
                        f *= pow;
                        if f.is_infinite() {
                            return Err(self.error(ErrorCode::NumberOutOfRange));
                        }
                    } else {
                        f /= pow;
                    }
                    break;
                }
                None => {
                    if f == 0.0 {
                        break;
                    }
                    if exponent >= 0 {
                        return Err(self.error(ErrorCode::NumberOutOfRange));
                    }
                    f /= 1e308;
                    exponent += 308;
                }
            }
        }
        Ok(f)
    }

    fn deserialize_number(&mut self) -> Result<Number, Error> {
        let peek = self.peek_some()?;

        match peek {
            b'-' => {
                self.discard();
                self.parse_any_number().map(|n| n.neg())
            }
            b'0'..=b'9' => self.parse_any_number(),
            _ => Err(self.error(ErrorCode::InvalidNumber)),
        }
    }

    // Used to grab both strings and identifiers
    fn parse_str(&mut self) -> Result<&'de str, Error> {
        let start = self.index;
        let mut negoff = 0;
        while let Some(c) = self.peek() {
            // Strings may not contain "[],"
            match c {
                b'[' | b']' | b',' => {
                    break;
                }
                // Need to deal with trailing space .-.
                b' ' => {
                    self.discard();
                    if self.peek() == Some(b']') {
                        negoff += 1;
                        break;
                    }
                }
                _ => {
                    self.discard();
                }
            }
        }
        // This is safe because we know we started with a valid UTF-8 string.
        Ok(unsafe { std::str::from_utf8_unchecked(&self.input[start..(self.index-negoff)]) })
    }

    fn parse_identifier(&mut self) -> Result<&'de str, Error> {
        let start = self.index;
        while let Some(c) = self.peek() {
            // Identifiers may not contain "[:],"
            match c {
                b'[' | b':' | b']' | b',' => {
                    break;
                }
                _ => {
                    self.discard();
                }
            }
        }
        // This is safe because we know we started with a valid UTF-8 string.
        Ok(unsafe { std::str::from_utf8_unchecked(&self.input[start..self.index]) })
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut MsgDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        self.deserialize_number()?.visit_i64(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        self.deserialize_number()?.visit_i64(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        self.deserialize_number()?.visit_i64(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        self.deserialize_number()?.visit_i64(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        self.deserialize_number()?.visit_u64(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        self.deserialize_number()?.visit_u64(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        self.deserialize_number()?.visit_u64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        self.deserialize_number()?.visit_u64(visitor)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        self.deserialize_number()?.visit_f64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        self.deserialize_number()?.visit_f64(visitor)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        visitor.visit_borrowed_str(self.parse_str()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        visitor.visit_borrowed_str(self.parse_str()?)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        visitor.visit_seq(ListSeparator::new(self))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        // There are 3 map variants
        // 1. key[value]
        // 2. [key: value key: value]
        // 3. [key: value, key: value]
        // We need to check which one it is

        // 2 & 3 start at [
        self.depth += 1;
        let r = if self.depth > 1 {
            Ok(visitor.visit_map(ChildMap::new(self))?)
        } else {
            // 1 starts with a key
            Ok(visitor.visit_map(RootMap::new(self))?)
        };
        self.depth -= 1;
        r
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        visitor.visit_borrowed_str(self.parse_identifier()?)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        // We need to advanced the deserializer to the next token
        // Theory says this function is only called when a map value is ignored
        // So lets see what happens if we parse_str
        self.parse_str()?;
        visitor.visit_unit()
    }

    fn is_human_readable(&self) -> bool {
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[test]
    fn root_map_works() {
        let input = r#"a[asdf] b[potato] c[test]"#;

        #[derive(Debug, Deserialize, PartialEq)]
        struct Test {
            a: String,
            b: String,
            c: String,
        }

        let test: Test = from_str(input).unwrap();
        assert_eq!(
            test,
            Test {
                a: "asdf".to_string(),
                b: "potato".to_string(),
                c: "test".to_string(),
            }
        );
    }

    #[test]
    fn child_map_works() {
        let input = r#"foo[bar:255 bar2:60]"#;

        #[derive(Debug, Deserialize, PartialEq)]
        struct Foo {
            bar: u64,
            bar2: u64,
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct Test {
            foo: Foo
        }

        let test: Test = from_str(input).unwrap();
        assert_eq!(
            test,
            Test {
                foo: Foo {
                    bar: 255,
                    bar2: 60,
                }
            }
        );
    }

    #[test]
    fn list_works() {
        let input = r#"foo[1 2 3 5 8]"#;

        #[derive(Debug, Deserialize, PartialEq)]
        struct Test {
            foo: Vec<u64>,
        }

        let test: Test = from_str(input).unwrap();
        assert_eq!(
            test,
            Test {
                foo: vec![1, 2, 3, 5, 8],
            }
        );
    }

    #[test]
    fn float_works() {
        let input = r#"foo[1.234] bar[0.735%]"#;

        #[derive(Debug, Deserialize, PartialEq)]
        struct Test {
            foo: f64,
            bar: f64,
        }

        let test: Test = from_str(input).unwrap();
        assert_eq!(
            test,
            Test {
                foo: 1.234,
                bar: 0.00735,
            }
        );
    }

    #[test]
    fn strings_work() {
        let input = r#"foo[foo: hello there, bar bar: 8]"#;

        #[derive(Debug, Deserialize, PartialEq)]
        struct Foo {
            foo: String,
            #[serde(rename = "bar bar")]
            bar: u64,
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct Test {
            foo: Foo,
        }

        let test: Test = from_str(input).unwrap();
        assert_eq!(
            test,
            Test {
                foo: Foo {
                    foo: "hello there".to_string(),
                    bar: 8,
                }
            }
        );
    }

    #[test]
    fn test_ignore() {
        let test = r#"foo[bar1: 255, bar2: 4153, bar3: 1352] bar[foo: foo, bar: bar]"#;

        #[derive(Debug, Deserialize, PartialEq)]
        struct Foo {
            bar1: u64,
            bar3: u64,
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct Bar {
            foo: String,
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct Test {
            foo: Foo,
            bar: Bar,
        }

        let test: Test = from_str(test).unwrap();
        assert_eq!(
            test,
            Test {
                foo: Foo {
                    bar1: 255,
                    bar3: 1352,
                },
                bar: Bar {
                    foo: "foo".to_string(),
                }
            }
        );
    }

    #[test]
    fn test_whitespace() {
        // You would think in a space delimited format, it wouldn't have excess spaces, but you'd be wrong.
        let test = r#"foo[bar1: 153, bar2: 245 ] bar[ 4  5  6] bar2[6 7 8 ] bar3[a1 132:185:153 a2 153:134:64:1685 ]"#;

        #[derive(Debug, Deserialize, PartialEq)]
        struct Foo {
            bar1: u64,
            bar2: u64,
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct Test {
            foo: Foo,
            bar: Vec<u64>,
            bar2: Vec<u64>,
            bar3: String,
        }

        let test: Test = from_str(test).unwrap();
        assert_eq!(
            test,
            Test {
                foo: Foo {
                    bar1: 153,
                    bar2: 245,
                },
                bar: vec![4, 5, 6],
                bar2: vec![6, 7, 8],
                bar3: "a1 132:185:153 a2 153:134:64:1685".to_string(),
            }
        );
    }

    #[test]
    fn test_floats() {
        let test = r#"foo[50%] bar[0.5%] baz[0.5]"#;

        #[derive(Debug, Deserialize, PartialEq)]
        struct Test {
            foo: f64,
            bar: f64,
            baz: f64,
        }

        let test: Test = from_str(test).unwrap();
        assert_eq!(
            test,
            Test {
                foo: 0.5,
                bar: 0.005,
                baz: 0.5,
            }
        );
    }

    #[test]
    fn test_ints() {
        let test = r#"foo[-5 -3 -4 -6 81]"#;

        #[derive(Debug, Deserialize, PartialEq)]
        struct Test {
            foo: Vec<i64>,
        }

        let test: Test = from_str(test).unwrap();
        assert_eq!(
            test,
            Test {
                foo: vec![-5, -3, -4, -6, 81],
            }
        );
    }
}
