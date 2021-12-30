use aoc_prelude::*;
use deku::prelude::*;

#[derive(Debug, PartialEq, Copy, Clone, DekuRead)]
struct Header {
    #[deku(bits = "3")]
    version: u8,
    #[deku(bits = "3")]
    type_id: u8,
}

#[derive(Default, Debug, Copy, Clone, DekuRead)]
struct Number {
    #[deku(bits = "1")]
    cont: u8,
    #[deku(bits = "4")]
    bits: u8,
}

#[derive(Debug, PartialEq, Copy, Clone, DekuRead)]
#[deku(endian = "big")]
#[deku(type = "u8", bits = "1")]
enum OpCount {
    #[deku(id = "0x00")]
    BitCount(#[deku(bits = "15")] u16),

    #[deku(id = "0x01")]
    PacketCount(#[deku(bits = "11")] u16),
}

#[derive(Debug, PartialEq)]
enum PacketType {
    Number(usize),
    Operator(OpCount),
    EndOp,
}

#[derive(Debug)]
struct Decoded {
    header: Header,
    packet: PacketType,
}

#[derive(Debug)]
enum Op {
    Sum,
    Prod,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
    Number(usize),
    _End,
}

impl From<Decoded> for Op {
    fn from(decoded: Decoded) -> Self {
        if matches!(decoded.packet, PacketType::EndOp) {
            return Self::_End;
        }

        match decoded.header.type_id {
            0u8 => Self::Sum,
            1 => Self::Prod,
            2 => Self::Min,
            3 => Self::Max,
            4 => match decoded.packet {
                PacketType::Number(x) => Self::Number(x),
                _ => unreachable!(),
            },
            5 => Self::Gt,
            6 => Self::Lt,
            7 => Self::Eq,
            _ => unreachable!(),
        }
    }
}

impl Op {
    fn eval(&self, nums: impl Iterator<Item = usize>) -> usize {
        match self {
            Self::Sum => nums.sum(),
            Self::Prod => nums.product(),
            Self::Min => nums.min().unwrap(),
            Self::Max => nums.max().unwrap(),
            Self::Gt => {
                let _nums: Vec<_> = nums.collect();
                (_nums[0] > _nums[1]).into()
            }
            Self::Lt => {
                let _nums: Vec<_> = nums.collect();
                (_nums[0] < _nums[1]).into()
            }
            Self::Eq => {
                let _nums: Vec<_> = nums.collect();
                (_nums[0] == _nums[1]).into()
            }
            _ => unreachable!(),
        }
    }
}

type ByteStream<'a> = (&'a [u8], usize);

fn eval(op_stream: Vec<Op>) -> VecDeque<Op> {
    let mut op_stack = VecDeque::<Op>::new();

    for t in op_stream {
        // if we haven't reached an '_End' op keep pushin on that stack
        if !matches!(t, Op::_End) {
            op_stack.push_back(t);
            continue;
        }

        let mut num_stack = VecDeque::<usize>::new();

        // accumulate numbers
        'inner: loop {
            let maybe_num = op_stack.pop_back().unwrap();
            if let Op::Number(x) = maybe_num {
                // numbers come from a stack but we need them in order
                // for eval, so use .push_front
                num_stack.push_front(x);
            } else {
                // we pulled a non-number, put it back!
                op_stack.push_back(maybe_num);
                break 'inner;
            }
        }

        // pop the op, evaluate it and push its result back onto the stack
        let op = op_stack.pop_back().unwrap();
        op_stack.push_back(Op::Number(op.eval(num_stack.into_iter())));
    }

    op_stack
}

fn isolate_bits(stream: ByteStream, num_bits: usize) -> ByteStream {
    // [ .... ...., .... ...., ...., .... .... ]
    //      ^  --------------------->  ^
    //      stream.offset            stream.offset + by
    //
    // first multiple of 8 bigger than offset + by
    let req_bytes: usize = ((stream.1 + num_bits) as f32 / 8f32).ceil() as usize;
    (&stream.0[0..req_bytes], stream.1)
}

fn parse_number(stream: ByteStream) -> Option<(ByteStream, PacketType)> {
    let mut numbers = Vec::<Number>::new();
    let mut inner_stream = stream;

    // parse number segments
    loop {
        let (_loop_rest, number) = Number::from_bytes(inner_stream).ok()?;
        inner_stream = _loop_rest;
        numbers.push(number);
        if number.cont != 1 {
            break;
        }
    }

    // turn into packet struct
    let mut val: usize = numbers[0].bits as usize;
    for n in &numbers[1..] {
        val <<= 4;
        val += n.bits as usize;
    }
    Some((inner_stream, PacketType::Number(val)))
}

fn parse_op<'a>(
    op_count: OpCount,
    stream: ByteStream<'a>,
    packets: &mut Vec<Decoded>,
) -> Option<ByteStream<'a>> {
    let mut _rest = stream;

    match op_count {
        OpCount::PacketCount(p) => {
            for _ in 0..p {
                _rest = parse(_rest, packets)?;
            }
        }
        OpCount::BitCount(b) => {
            // - isolate 'b' number of bits and parse them
            // - advance rest as needed
            let mut parseable = isolate_bits(_rest, b as usize);
            while let Some(_parseable) = parse(parseable, packets) {
                parseable = _parseable;
            }
            _rest = (_rest.0, _rest.1 + b as usize)
        }
    }
    Some(_rest)
}

fn parse<'a>(stream: ByteStream<'a>, packets: &mut Vec<Decoded>) -> Option<ByteStream<'a>> {
    let (_h_rest, header) = Header::from_bytes(stream).ok()?;

    // number
    if header.type_id == 4 {
        let (_rest, number) = parse_number(_h_rest)?;
        packets.push(Decoded {
            header,
            packet: number,
        });
        return Some(_rest);
    }

    // operator
    let (_rest, op_count) = OpCount::from_bytes(_h_rest).ok()?;
    packets.push(Decoded {
        header,
        packet: PacketType::Operator(op_count),
    });

    let _rest = parse_op(op_count, _rest, packets);
    packets.push(Decoded {
        header,
        packet: PacketType::EndOp,
    });

    _rest
}

#[allow(dead_code)]
fn debug(stream: (&[u8], usize)) {
    let mut acc = String::default();
    for x in stream.0 {
        acc.push_str(&format!("{:#010b} ", x));
    }
    println!("bits: {}; offset: {}", acc, stream.1);
}

fn read_input() -> Vec<u8> {
    let s: String = include_str!("../../inputs/day16.txt")
        .trim_end_matches('\n')
        .into();
    Vec::from_hex(s).expect("invalid hex string")
}

aoc_2021::main! {
    let mut packets = Vec::<Decoded>::new();

    let input = read_input();

    parse((input.as_slice(), 0), &mut packets);

    let version_sum: u16 = packets
        .iter()
        .filter(|d| d.packet != PacketType::EndOp)
        .map(|d| d.header.version as u16)
        .sum();

    let op_stream: Vec<Op> = packets.into_iter().map(|x| x.into()).collect();

    let evald = eval(op_stream).pop_back().unwrap();

    (version_sum, format!("{:?}", evald))
}
