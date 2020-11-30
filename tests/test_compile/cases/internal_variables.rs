use deku::prelude::*;

#[derive(DekuRead, DekuWrite)]
struct TestCount {
    field_a: u8,
    #[deku(count = "byte_offset")]
    field_b: Vec<u8>
}

#[derive(DekuRead, DekuWrite)]
struct TestBitRead {
    field_a: u8,
    #[deku(bits_read = "bit_offset")]
    field_b: Vec<u8>
}

#[derive(DekuRead, DekuWrite)]
struct TestBytesRead {
    field_a: u8,
    #[deku(bytes_read = "bit_offset")]
    field_b: Vec<u8>
}

#[derive(DekuRead, DekuWrite)]
struct TestUntil {
    field_a: u8,
    #[deku(until = "|v| *v as usize == bit_offset")]
    field_b: Vec<u8>
}

#[derive(DekuRead, DekuWrite)]
struct TestCond {
    field_a: u8,
    #[deku(cond = "bit_offset == *field_a as usize")]
    field_b: u8
}

#[derive(DekuRead, DekuWrite)]
struct TestDefault {
    field_a: u8,
    #[deku(skip, default = "byte_offset")]
    field_b: usize
}

#[derive(DekuRead, DekuWrite)]
struct TestMap {
    field_a: u8,
    #[deku(map = "|v: u8| -> Result<_, DekuError> { Ok(v as usize + byte_offset) }")]
    field_b: usize
}

fn dummy_reader(
    offset: usize,
    rest: &BitSlice<Msb0, u8>,
) -> Result<(&BitSlice<Msb0, u8>, usize), DekuError> {
    Ok((rest, offset))
}
#[derive(DekuRead, DekuWrite)]
struct TestReader {
    field_a: u8,
    #[deku(reader = "dummy_reader(byte_offset, rest)")]
    field_b: usize
}

#[derive(DekuRead, DekuWrite)]
#[deku(ctx = "_byte_size: usize, _bit_size: usize")]
struct ChildCtx {
}
#[derive(DekuRead, DekuWrite)]
struct TestCtx {
    field_a: u8,
    #[deku(ctx = "byte_offset, bit_offset")]
    field_b: ChildCtx
}

fn dummy_writer(
    _offset: usize,
    _output: &mut BitVec<Msb0, u8>,
) -> Result<(), DekuError> {
    Ok(())
}
#[derive(DekuRead, DekuWrite)]
struct TestWriter {
    field_a: u8,
    #[deku(writer = "dummy_writer(byte_offset, output)")]
    field_b: usize
}

fn main() {
    fail
}
