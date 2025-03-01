use lsp_types::CompletionItemKind;

use super::{CompletionProvider, completion_provider::new_completion_item};
pub struct BuiltinCompletions;

impl CompletionProvider for BuiltinCompletions {
    fn get_completions(&self, _position: &lsp_types::Position) -> Vec<lsp_types::CompletionItem> {
        [].iter()
            .chain(VALUE_CONSTRUCTORS)
            .chain(OTHER)
            .chain(NUMERIC)
            .chain(DERIVATIVE)
            .chain(TEXTURE)
            .chain(ATOMIC)
            .chain(DATA_PACKING)
            .chain(DATA_UNPACKING)
            .chain(SYNCHRONIZATION)
            .chain(SUBGROUP)
            .chain(QUAD)
            .map(|name| new_completion_item(*name, CompletionItemKind::FUNCTION))
            .collect()
    }
}

const VALUE_CONSTRUCTORS: &[&str] = &[
    "array", "bool", "f16", "f32", "i32", "u32", "mat2x2", "mat2x3", "mat2x4", "mat3x2", "mat3x3",
    "mat3x4", "mat4x2", "mat4x3", "mat4x4", "vec2", "vec3", "vec4",
];

const OTHER: &[&str] = &["bitcast", "all", "any", "select", "arrayLength"];

const NUMERIC: &[&str] = &[
    "abs",
    "acos",
    "acosh",
    "asin",
    "asinh",
    "atan",
    "atanh",
    "atan2",
    "ceil",
    "clamp",
    "cos",
    "cosh",
    "countLeadingZeros",
    "countOneBits",
    "countTrailingZeros",
    "cross",
    "degrees",
    "determinant",
    "distance",
    "dot",
    "dot4U8Packed",
    "dot4I8Packed",
    "exp",
    "exp2",
    "extractBits",
    "faceForward",
    "firstLeadingBit",
    "firstTrailingBit",
    "floor",
    "fma",
    "fract",
    "frexp",
    "insertBits",
    "inverseSqrt",
    "ldexp",
    "length",
    "log",
    "log2",
    "max",
    "min",
    "mix",
    "modf",
    "normalize",
    "pow",
    "quantizeToF16",
    "radians",
    "reflect",
    "refract",
    "reverseBits",
    "round",
    "saturate",
    "sign",
    "sin",
    "sinh",
    "smoothstep",
    "sqrt",
    "step",
    "tan",
    "tanh",
    "transpose",
    "trunc",
];

const DERIVATIVE: &[&str] = &[
    "dpdx",
    "dpdxCoarse",
    "dpdxFine",
    "dpdy",
    "dpdyCoarse",
    "dpdyFine",
    "fwidth",
    "fwidthCoarse",
    "fwidthFine",
];

const TEXTURE: &[&str] = &[
    "textureDimensions",
    "textureGather",
    "textureGatherCompare",
    "textureLoad",
    "textureNumLayers",
    "textureNumLevels",
    "textureNumSamples",
    "textureSample",
    "textureSampleBias",
    "textureSampleCompare",
    "textureSampleCompareLevel",
    "textureSampleGrad",
    "textureSampleLevel",
    "textureSampleBaseClampToEdge",
    "textureStore",
];

const ATOMIC: &[&str] = &[
    "atomicLoad",
    "atomicStore",
    "atomicAdd",
    "atomicSub",
    "atomicMax",
    "atomicMin",
    "atomicAnd",
    "atomicOr",
    "atomicXor",
    "atomicExchange",
    "atomicCompareExchangeWeak",
];

const DATA_PACKING: &[&str] = &[
    "pack4x8snorm",
    "pack4x8unorm",
    "pack4xI8",
    "pack4xU8",
    "pack4xI8Clamp",
    "pack4xU8Clamp",
    "pack2x16snorm",
    "pack2x16unorm",
    "pack2x16float",
];

const DATA_UNPACKING: &[&str] = &[
    "unpack4x8snorm",
    "unpack4x8unorm",
    "unpack4xI8",
    "unpack4xU8",
    "unpack2x16snorm",
    "unpack2x16unorm",
    "unpack2x16float",
];

const SYNCHRONIZATION: &[&str] = &[
    "storageBarrier",
    "textureBarrier",
    "workgroupBarrier",
    "workgroupUniformLoad",
];

const SUBGROUP: &[&str] = &[
    "subgroupAdd",
    "subgroupExclusiveAdd",
    "subgroupInclusiveAdd",
    "subgroupAll",
    "subgroupAnd",
    "subgroupAny",
    "subgroupBallot",
    "subgroupBroadcast",
    "subgroupBroadcastFirst",
    "subgroupElect",
    "subgroupMax",
    "subgroupMin",
    "subgroupMul",
    "subgroupExclusiveMul",
    "subgroupInclusiveMul",
    "subgroupOr",
    "subgroupShuffle",
    "subgroupShuffleDown",
    "subgroupShuffleUp",
    "subgroupShuffleXor",
    "subgroupXor",
];

const QUAD: &[&str] = &[
    "quadBroadcast",
    "quadSwapDiagonal",
    "quadSwapX",
    "quadSwapY",
];
