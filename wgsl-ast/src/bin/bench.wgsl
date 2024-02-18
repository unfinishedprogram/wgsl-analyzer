// This snapshot tests accessing various containers, dereferencing pointers.

struct GlobalConst {
    a: u32,
    b: vec3<u32>,
    c: i32,
}
// tests msl padding insertion for global constants
var<private> global_const: GlobalConst = GlobalConst(0u, vec3<u32>(0u, 0u, 0u), 0);

struct AlignedWrapper {
	@align(8) value: i32
}

struct Bar {
	_matrix: mat4x3<f32>,
	matrix_array: array<mat2x2<f32>, 2>,
	atom: atomic<i32>,
	atom_arr: array<atomic<i32>, 10>,
	arr: array<vec2<u32>, 2>,
	data: array<AlignedWrapper>,
}

@group(0) @binding(0)
var<storage,read_write> bar: Bar;

struct Baz {
	m: mat3x2<f32>,
}

@group(0) @binding(1)
var<uniform> baz: Baz;

@group(0) @binding(2)
var<storage,read_write> qux: vec2<i32>;

fn test_matrix_within_struct_accesses() {
	var idx = 1;

    idx--;

	// loads
    let l0 = baz.m;
    let l1 = baz.m[0];
    let l2 = baz.m[idx];
    let l3 = baz.m[0][1];
    let l4 = baz.m[0][idx];
    let l5 = baz.m[idx][1];
    let l6 = baz.m[idx][idx];

    var t = Baz(mat3x2<f32>(vec2<f32>(1.0), vec2<f32>(2.0), vec2<f32>(3.0)));

    idx++;

	// stores
    t.m = mat3x2<f32>(vec2<f32>(6.0), vec2<f32>(5.0), vec2<f32>(4.0));
    t.m[0] = vec2<f32>(9.0);
    t.m[idx] = vec2<f32>(90.0);
    t.m[0][1] = 10.0;
    t.m[0][idx] = 20.0;
    t.m[idx][1] = 30.0;
    t.m[idx][idx] = 40.0;
}

struct MatCx2InArray {
	am: array<mat4x2<f32>, 2>,
}

@group(0) @binding(3)
var<uniform> nested_mat_cx2: MatCx2InArray;

fn test_matrix_within_array_within_struct_accesses() {
	var idx = 1;

    idx--;

	// loads
    let l0 = nested_mat_cx2.am;
    let l1 = nested_mat_cx2.am[0];
    let l2 = nested_mat_cx2.am[0][0];
    let l3 = nested_mat_cx2.am[0][idx];
    let l4 = nested_mat_cx2.am[0][0][1];
    let l5 = nested_mat_cx2.am[0][0][idx];
    let l6 = nested_mat_cx2.am[0][idx][1];
    let l7 = nested_mat_cx2.am[0][idx][idx];

    var t = MatCx2InArray(array<mat4x2<f32>, 2>());

    idx++;

	// stores
    t.am = array<mat4x2<f32>, 2>();
    t.am[0] = mat4x2<f32>(vec2<f32>(8.0), vec2<f32>(7.0), vec2<f32>(6.0), vec2<f32>(5.0));
    t.am[0][0] = vec2<f32>(9.0);
    t.am[0][idx] = vec2<f32>(90.0);
    t.am[0][0][1] = 10.0;
    t.am[0][0][idx] = 20.0;
    t.am[0][idx][1] = 30.0;
    t.am[0][idx][idx] = 40.0;
}

fn read_from_private(foo: ptr<function, f32>) -> f32 {
    return *foo;
}

fn test_arr_as_arg(a: array<array<f32, 10>, 5>) -> f32 {
    return a[4][9];
}

@vertex
fn foo_vert(@builtin(vertex_index) vi: u32) -> @builtin(position) vec4<f32> {
    var foo: f32 = 0.0;
    // We should check that backed doesn't skip this expression
    let baz: f32 = foo;
    foo = 1.0;

	test_matrix_within_struct_accesses();
	test_matrix_within_array_within_struct_accesses();

    // test storage loads
	let _matrix = bar._matrix;
	let arr = bar.arr;
	let index = 3u;
	let b = bar._matrix[index].x;
	let a = bar.data[arrayLength(&bar.data) - 2u].value;
	let c = qux;

	// test pointer types
	let data_pointer: ptr<storage, i32, read_write> = &bar.data[0].value;
	let foo_value = read_from_private(&foo);

	// test array indexing
	var c2 = array<i32, 5>(a, i32(b), 3, 4, 5);
	c2[vi + 1u] = 42;
	let value = c2[vi];

	test_arr_as_arg(array<array<f32, 10>, 5>());

	return vec4<f32>(_matrix * vec4<f32>(vec4<i32>(value)), 2.0);
}

@fragment
fn foo_frag() -> @location(0) vec4<f32> {
	// test storage stores
	bar._matrix[1].z = 1.0;
	bar._matrix = mat4x3<f32>(vec3<f32>(0.0), vec3<f32>(1.0), vec3<f32>(2.0), vec3<f32>(3.0));
	bar.arr = array<vec2<u32>, 2>(vec2<u32>(0u), vec2<u32>(1u));
	bar.data[1].value = 1;
	qux = vec2<i32>();

	return vec4<f32>(0.0);
}

fn assign_through_ptr_fn(p: ptr<function, u32>) {
    *p = 42u;
}

fn assign_array_through_ptr_fn(foo: ptr<function, array<vec4<f32>, 2>>) {
    *foo = array<vec4<f32>, 2>(vec4(1.0), vec4(2.0));
}

@compute @workgroup_size(1)
fn assign_through_ptr() {
    var val = 33u;
    assign_through_ptr_fn(&val);

	var arr = array<vec4<f32>, 2>(vec4(6.0), vec4(7.0));
    assign_array_through_ptr_fn(&arr);
}
struct Ah {
    inner: array<f32, 2>,
};
@group(0) @binding(0)
var<storage> ah: Ah;

@compute @workgroup_size(1)
fn cs_main() {
    let ah = ah;
}
fn ret_array() -> array<f32, 2> {
    return array<f32, 2>(1.0, 2.0);
}

@fragment
fn main() -> @location(0) vec4<f32> {
    let a = ret_array();
    return vec4<f32>(a[0], a[1], 0.0, 1.0);
}
const SIZE: u32 = 128u;

@group(0) @binding(0)
var<storage,read_write> arr_i32: array<atomic<i32>, SIZE>;
@group(0) @binding(1)
var<storage,read_write> arr_u32: array<atomic<u32>, SIZE>;

@compute @workgroup_size(1)
fn test_atomic_compare_exchange_i32() {
    for(var i = 0u; i < SIZE; i++) {
        var old = atomicLoad(&arr_i32[i]);
        var exchanged = false;
        while(!exchanged) {
            let new_ = bitcast<i32>(bitcast<f32>(old) + 1.0);
            let result = atomicCompareExchangeWeak(&arr_i32[i], old, new_);
            old = result.old_value;
            exchanged = result.exchanged;
        }
    }
}

@compute @workgroup_size(1)
fn test_atomic_compare_exchange_u32() {
    for(var i = 0u; i < SIZE; i++) {
        var old = atomicLoad(&arr_u32[i]);
        var exchanged = false;
        while(!exchanged) {
            let new_ = bitcast<u32>(bitcast<f32>(old) + 1.0);
            let result = atomicCompareExchangeWeak(&arr_u32[i], old, new_);
            old = result.old_value;
            exchanged = result.exchanged;
        }
    }
}
// This test covers the cross product of:
//
// * All atomic operations.
// * On all applicable scopes (storage read-write, workgroup).
// * For all shapes of modeling atomic data.

struct Struct {
    atomic_scalar: atomic<u32>,
    atomic_arr: array<atomic<i32>, 2>,
}

@group(0) @binding(0)
var<storage, read_write> storage_atomic_scalar: atomic<u32>;
@group(0) @binding(1)
var<storage, read_write> storage_atomic_arr: array<atomic<i32>, 2>;
@group(0) @binding(2)
var<storage, read_write> storage_struct: Struct;

var<workgroup> workgroup_atomic_scalar: atomic<u32>;
var<workgroup> workgroup_atomic_arr: array<atomic<i32>, 2>;
var<workgroup> workgroup_struct: Struct;

@compute
@workgroup_size(2)
fn cs_main(@builtin(local_invocation_id) id: vec3<u32>) {
    atomicStore(&storage_atomic_scalar, 1u);
    atomicStore(&storage_atomic_arr[1], 1i);
    atomicStore(&storage_struct.atomic_scalar, 1u);
    atomicStore(&storage_struct.atomic_arr[1], 1i);
    atomicStore(&workgroup_atomic_scalar, 1u);
    atomicStore(&workgroup_atomic_arr[1], 1i);
    atomicStore(&workgroup_struct.atomic_scalar, 1u);
    atomicStore(&workgroup_struct.atomic_arr[1], 1i);

    workgroupBarrier();

    let l0 = atomicLoad(&storage_atomic_scalar);
    let l1 = atomicLoad(&storage_atomic_arr[1]);
    let l2 = atomicLoad(&storage_struct.atomic_scalar);
    let l3 = atomicLoad(&storage_struct.atomic_arr[1]);
    let l4 = atomicLoad(&workgroup_atomic_scalar);
    let l5 = atomicLoad(&workgroup_atomic_arr[1]);
    let l6 = atomicLoad(&workgroup_struct.atomic_scalar);
    let l7 = atomicLoad(&workgroup_struct.atomic_arr[1]);

    workgroupBarrier();

    atomicAdd(&storage_atomic_scalar, 1u);
    atomicAdd(&storage_atomic_arr[1], 1i);
    atomicAdd(&storage_struct.atomic_scalar, 1u);
    atomicAdd(&storage_struct.atomic_arr[1], 1i);
    atomicAdd(&workgroup_atomic_scalar, 1u);
    atomicAdd(&workgroup_atomic_arr[1], 1i);
    atomicAdd(&workgroup_struct.atomic_scalar, 1u);
    atomicAdd(&workgroup_struct.atomic_arr[1], 1i);

    workgroupBarrier();

    atomicSub(&storage_atomic_scalar, 1u);
    atomicSub(&storage_atomic_arr[1], 1i);
    atomicSub(&storage_struct.atomic_scalar, 1u);
    atomicSub(&storage_struct.atomic_arr[1], 1i);
    atomicSub(&workgroup_atomic_scalar, 1u);
    atomicSub(&workgroup_atomic_arr[1], 1i);
    atomicSub(&workgroup_struct.atomic_scalar, 1u);
    atomicSub(&workgroup_struct.atomic_arr[1], 1i);

    workgroupBarrier();

    atomicMax(&storage_atomic_scalar, 1u);
    atomicMax(&storage_atomic_arr[1], 1i);
    atomicMax(&storage_struct.atomic_scalar, 1u);
    atomicMax(&storage_struct.atomic_arr[1], 1i);
    atomicMax(&workgroup_atomic_scalar, 1u);
    atomicMax(&workgroup_atomic_arr[1], 1i);
    atomicMax(&workgroup_struct.atomic_scalar, 1u);
    atomicMax(&workgroup_struct.atomic_arr[1], 1i);

    workgroupBarrier();

    atomicMin(&storage_atomic_scalar, 1u);
    atomicMin(&storage_atomic_arr[1], 1i);
    atomicMin(&storage_struct.atomic_scalar, 1u);
    atomicMin(&storage_struct.atomic_arr[1], 1i);
    atomicMin(&workgroup_atomic_scalar, 1u);
    atomicMin(&workgroup_atomic_arr[1], 1i);
    atomicMin(&workgroup_struct.atomic_scalar, 1u);
    atomicMin(&workgroup_struct.atomic_arr[1], 1i);

    workgroupBarrier();

    atomicAnd(&storage_atomic_scalar, 1u);
    atomicAnd(&storage_atomic_arr[1], 1i);
    atomicAnd(&storage_struct.atomic_scalar, 1u);
    atomicAnd(&storage_struct.atomic_arr[1], 1i);
    atomicAnd(&workgroup_atomic_scalar, 1u);
    atomicAnd(&workgroup_atomic_arr[1], 1i);
    atomicAnd(&workgroup_struct.atomic_scalar, 1u);
    atomicAnd(&workgroup_struct.atomic_arr[1], 1i);

    workgroupBarrier();

    atomicOr(&storage_atomic_scalar, 1u);
    atomicOr(&storage_atomic_arr[1], 1i);
    atomicOr(&storage_struct.atomic_scalar, 1u);
    atomicOr(&storage_struct.atomic_arr[1], 1i);
    atomicOr(&workgroup_atomic_scalar, 1u);
    atomicOr(&workgroup_atomic_arr[1], 1i);
    atomicOr(&workgroup_struct.atomic_scalar, 1u);
    atomicOr(&workgroup_struct.atomic_arr[1], 1i);

    workgroupBarrier();

    atomicXor(&storage_atomic_scalar, 1u);
    atomicXor(&storage_atomic_arr[1], 1i);
    atomicXor(&storage_struct.atomic_scalar, 1u);
    atomicXor(&storage_struct.atomic_arr[1], 1i);
    atomicXor(&workgroup_atomic_scalar, 1u);
    atomicXor(&workgroup_atomic_arr[1], 1i);
    atomicXor(&workgroup_struct.atomic_scalar, 1u);
    atomicXor(&workgroup_struct.atomic_arr[1], 1i);

    atomicExchange(&storage_atomic_scalar, 1u);
    atomicExchange(&storage_atomic_arr[1], 1i);
    atomicExchange(&storage_struct.atomic_scalar, 1u);
    atomicExchange(&storage_struct.atomic_arr[1], 1i);
    atomicExchange(&workgroup_atomic_scalar, 1u);
    atomicExchange(&workgroup_atomic_arr[1], 1i);
    atomicExchange(&workgroup_struct.atomic_scalar, 1u);
    atomicExchange(&workgroup_struct.atomic_arr[1], 1i);

    // // TODO: https://github.com/gpuweb/gpuweb/issues/2021
    // atomicCompareExchangeWeak(&storage_atomic_scalar, 1u);
    // atomicCompareExchangeWeak(&storage_atomic_arr[1], 1i);
    // atomicCompareExchangeWeak(&storage_struct.atomic_scalar, 1u);
    // atomicCompareExchangeWeak(&storage_struct.atomic_arr[1], 1i);
    // atomicCompareExchangeWeak(&workgroup_atomic_scalar, 1u);
    // atomicCompareExchangeWeak(&workgroup_atomic_arr[1], 1i);
    // atomicCompareExchangeWeak(&workgroup_struct.atomic_scalar, 1u);
    // atomicCompareExchangeWeak(&workgroup_struct.atomic_arr[1], 1i);
}
struct UniformIndex {
    index: u32
};

@group(0) @binding(0)
var texture_array_unbounded: binding_array<texture_2d<f32>>;
@group(0) @binding(1)
var texture_array_bounded: binding_array<texture_2d<f32>, 5>;
@group(0) @binding(2)
var texture_array_2darray: binding_array<texture_2d_array<f32>, 5>;
@group(0) @binding(3)
var texture_array_multisampled: binding_array<texture_multisampled_2d<f32>, 5>;
@group(0) @binding(4)
var texture_array_depth: binding_array<texture_depth_2d, 5>;
@group(0) @binding(5)
var texture_array_storage: binding_array<texture_storage_2d<rgba32float, write>, 5>;
@group(0) @binding(6)
var samp: binding_array<sampler, 5>;
@group(0) @binding(7)
var samp_comp: binding_array<sampler_comparison, 5>;
@group(0) @binding(8)
var<uniform> uni: UniformIndex;

struct FragmentIn {
    @location(0) index: u32,
};

@fragment
fn main(fragment_in: FragmentIn) -> @location(0) vec4<f32> {
    let uniform_index = uni.index;
    let non_uniform_index = fragment_in.index;

    var u1 = 0u;
    var u2 = vec2<u32>(0u);
    var v1 = 0.0;
    var v4 = vec4<f32>(0.0);
    
    // This example is arranged in the order of the texture definitions in the wgsl spec
    // 
    // The first function uses texture_array_unbounded, the rest use texture_array_bounded to make sure
    // they both show up in the output. Functions that need depth use texture_array_2darray.
    //
    // We only test 2D f32 textures here as the machinery for binding indexing doesn't care about
    // texture format or texture dimension.

    let uv = vec2<f32>(0.0);
    let pix = vec2<i32>(0);

    u2 += textureDimensions(texture_array_unbounded[0]);
    u2 += textureDimensions(texture_array_unbounded[uniform_index]);
    u2 += textureDimensions(texture_array_unbounded[non_uniform_index]);

    v4 += textureGather(0, texture_array_bounded[0], samp[0], uv);
    v4 += textureGather(0, texture_array_bounded[uniform_index], samp[uniform_index], uv);
    v4 += textureGather(0, texture_array_bounded[non_uniform_index], samp[non_uniform_index], uv); 

    v4 += textureGatherCompare(texture_array_depth[0], samp_comp[0], uv, 0.0);
    v4 += textureGatherCompare(texture_array_depth[uniform_index], samp_comp[uniform_index], uv, 0.0);
    v4 += textureGatherCompare(texture_array_depth[non_uniform_index], samp_comp[non_uniform_index], uv, 0.0); 

    v4 += textureLoad(texture_array_unbounded[0], pix, 0);
    v4 += textureLoad(texture_array_unbounded[uniform_index], pix, 0);
    v4 += textureLoad(texture_array_unbounded[non_uniform_index], pix, 0);

    u1 += textureNumLayers(texture_array_2darray[0]);
    u1 += textureNumLayers(texture_array_2darray[uniform_index]);
    u1 += textureNumLayers(texture_array_2darray[non_uniform_index]);

    u1 += textureNumLevels(texture_array_bounded[0]);
    u1 += textureNumLevels(texture_array_bounded[uniform_index]);
    u1 += textureNumLevels(texture_array_bounded[non_uniform_index]);

    u1 += textureNumSamples(texture_array_multisampled[0]);
    u1 += textureNumSamples(texture_array_multisampled[uniform_index]);
    u1 += textureNumSamples(texture_array_multisampled[non_uniform_index]);

    v4 += textureSample(texture_array_bounded[0], samp[0], uv);
    v4 += textureSample(texture_array_bounded[uniform_index], samp[uniform_index], uv);
    v4 += textureSample(texture_array_bounded[non_uniform_index], samp[non_uniform_index], uv);

    v4 += textureSampleBias(texture_array_bounded[0], samp[0], uv, 0.0);
    v4 += textureSampleBias(texture_array_bounded[uniform_index], samp[uniform_index], uv, 0.0);
    v4 += textureSampleBias(texture_array_bounded[non_uniform_index], samp[non_uniform_index], uv, 0.0);

    v1 += textureSampleCompare(texture_array_depth[0], samp_comp[0], uv, 0.0);
    v1 += textureSampleCompare(texture_array_depth[uniform_index], samp_comp[uniform_index], uv, 0.0);
    v1 += textureSampleCompare(texture_array_depth[non_uniform_index], samp_comp[non_uniform_index], uv, 0.0);

    v1 += textureSampleCompareLevel(texture_array_depth[0], samp_comp[0], uv, 0.0);
    v1 += textureSampleCompareLevel(texture_array_depth[uniform_index], samp_comp[uniform_index], uv, 0.0);
    v1 += textureSampleCompareLevel(texture_array_depth[non_uniform_index], samp_comp[non_uniform_index], uv, 0.0);

    v4 += textureSampleGrad(texture_array_bounded[0], samp[0], uv, uv, uv);
    v4 += textureSampleGrad(texture_array_bounded[uniform_index], samp[uniform_index], uv, uv, uv);
    v4 += textureSampleGrad(texture_array_bounded[non_uniform_index], samp[non_uniform_index], uv, uv, uv);

    v4 += textureSampleLevel(texture_array_bounded[0], samp[0], uv, 0.0);
    v4 += textureSampleLevel(texture_array_bounded[uniform_index], samp[uniform_index], uv, 0.0);
    v4 += textureSampleLevel(texture_array_bounded[non_uniform_index], samp[non_uniform_index], uv, 0.0);

    textureStore(texture_array_storage[0], pix, v4);
    textureStore(texture_array_storage[uniform_index], pix, v4);
    textureStore(texture_array_storage[non_uniform_index], pix, v4);

    let v2 = vec2<f32>(u2 + vec2<u32>(u1));

    return v4 + vec4<f32>(v2.x, v2.y, v2.x, v2.y) + v1;
}
struct UniformIndex {
    index: u32
}

struct Foo { x: u32 }
@group(0) @binding(0)
var<storage, read> storage_array: binding_array<Foo, 1>;
@group(0) @binding(10)
var<uniform> uni: UniformIndex;

struct FragmentIn {
    @location(0) index: u32,
}

@fragment
fn main(fragment_in: FragmentIn) -> @location(0) u32 {
    let uniform_index = uni.index;
    let non_uniform_index = fragment_in.index;

    var u1 = 0u;

    u1 += storage_array[0].x;
    u1 += storage_array[uniform_index].x;
    u1 += storage_array[non_uniform_index].x;

    return u1;
}
@compute @workgroup_size(1)
fn main() {
    var i2 = vec2<i32>(0);
    var i3 = vec3<i32>(0);
    var i4 = vec4<i32>(0);

    var u2 = vec2<u32>(0u);
    var u3 = vec3<u32>(0u);
    var u4 = vec4<u32>(0u);

    var f2 = vec2<f32>(0.0);
    var f3 = vec3<f32>(0.0);
    var f4 = vec4<f32>(0.0);

    u2 = bitcast<vec2<u32>>(i2);
    u3 = bitcast<vec3<u32>>(i3);
    u4 = bitcast<vec4<u32>>(i4);

    i2 = bitcast<vec2<i32>>(u2);
    i3 = bitcast<vec3<i32>>(u3);
    i4 = bitcast<vec4<i32>>(u4);

    f2 = bitcast<vec2<f32>>(i2);
    f3 = bitcast<vec3<f32>>(i3);
    f4 = bitcast<vec4<f32>>(i4);
}
@compute @workgroup_size(1)
fn main() {
    var i = 0;
    var i2 = vec2<i32>(0);
    var i3 = vec3<i32>(0);
    var i4 = vec4<i32>(0);
    var u = 0u;
    var u2 = vec2<u32>(0u);
    var u3 = vec3<u32>(0u);
    var u4 = vec4<u32>(0u);
    var f2 = vec2<f32>(0.0);
    var f4 = vec4<f32>(0.0);
    u = pack4x8snorm(f4);
    u = pack4x8unorm(f4);
    u = pack2x16snorm(f2);
    u = pack2x16unorm(f2);
    u = pack2x16float(f2);
    f4 = unpack4x8snorm(u);
    f4 = unpack4x8unorm(u);
    f2 = unpack2x16snorm(u);
    f2 = unpack2x16unorm(u);
    f2 = unpack2x16float(u);
    i = insertBits(i, i, 5u, 10u);
    i2 = insertBits(i2, i2, 5u, 10u);
    i3 = insertBits(i3, i3, 5u, 10u);
    i4 = insertBits(i4, i4, 5u, 10u);
    u = insertBits(u, u, 5u, 10u);
    u2 = insertBits(u2, u2, 5u, 10u);
    u3 = insertBits(u3, u3, 5u, 10u);
    u4 = insertBits(u4, u4, 5u, 10u);
    i = extractBits(i, 5u, 10u);
    i2 = extractBits(i2, 5u, 10u);
    i3 = extractBits(i3, 5u, 10u);
    i4 = extractBits(i4, 5u, 10u);
    u = extractBits(u, 5u, 10u);
    u2 = extractBits(u2, 5u, 10u);
    u3 = extractBits(u3, 5u, 10u);
    u4 = extractBits(u4, 5u, 10u);
    i = firstTrailingBit(i);
    u2 = firstTrailingBit(u2);
    i3 = firstLeadingBit(i3);
    u3 = firstLeadingBit(u3);
    i = firstLeadingBit(i);
    u = firstLeadingBit(u);
    i = countOneBits(i);
    i2 = countOneBits(i2);
    i3 = countOneBits(i3);
    i4 = countOneBits(i4);
    u = countOneBits(u);
    u2 = countOneBits(u2);
    u3 = countOneBits(u3);
    u4 = countOneBits(u4);
    i = reverseBits(i);
    i2 = reverseBits(i2);
    i3 = reverseBits(i3);
    i4 = reverseBits(i4);
    u = reverseBits(u);
    u2 = reverseBits(u2);
    u3 = reverseBits(u3);
    u4 = reverseBits(u4);
}
const NUM_PARTICLES: u32 = 1500u;

struct Particle {
  pos : vec2<f32>,
  vel : vec2<f32>,
}

struct SimParams {
  deltaT : f32,
  rule1Distance : f32,
  rule2Distance : f32,
  rule3Distance : f32,
  rule1Scale : f32,
  rule2Scale : f32,
  rule3Scale : f32,
}

struct Particles {
  particles : array<Particle>
}

@group(0) @binding(0) var<uniform> params : SimParams;
@group(0) @binding(1) var<storage> particlesSrc : Particles;
@group(0) @binding(2) var<storage,read_write> particlesDst : Particles;

// https://github.com/austinEng/Project6-Vulkan-Flocking/blob/master/data/shaders/computeparticles/particle.comp
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_invocation_id : vec3<u32>) {
  let index : u32 = global_invocation_id.x;
  if index >= NUM_PARTICLES {
    return;
  }

  var vPos = particlesSrc.particles[index].pos;
  var vVel = particlesSrc.particles[index].vel;

  var cMass = vec2<f32>(0.0, 0.0);
  var cVel = vec2<f32>(0.0, 0.0);
  var colVel = vec2<f32>(0.0, 0.0);
  var cMassCount : i32 = 0;
  var cVelCount : i32 = 0;

  var pos : vec2<f32>;
  var vel : vec2<f32>;
  var i : u32 = 0u;
  loop {
    if i >= NUM_PARTICLES {
      break;
    }
    if i == index {
      continue;
    }

    pos = particlesSrc.particles[i].pos;
    vel = particlesSrc.particles[i].vel;

    if distance(pos, vPos) < params.rule1Distance {
      cMass = cMass + pos;
      cMassCount = cMassCount + 1;
    }
    if distance(pos, vPos) < params.rule2Distance {
      colVel = colVel - (pos - vPos);
    }
    if distance(pos, vPos) < params.rule3Distance {
      cVel = cVel + vel;
      cVelCount = cVelCount + 1;
    }

    continuing {
      i = i + 1u;
    }
  }
  if cMassCount > 0 {
    cMass = cMass / f32(cMassCount) - vPos;
  }
  if cVelCount > 0 {
    cVel = cVel / f32(cVelCount);
  }

  vVel = vVel + (cMass * params.rule1Scale) +
      (colVel * params.rule2Scale) +
      (cVel * params.rule3Scale);

  // clamp velocity for a more pleasing simulation
  vVel = normalize(vVel) * clamp(length(vVel), 0.0, 0.1);

  // kinematic update
  vPos = vPos + (vVel * params.deltaT);

  // Wrap around boundary
  if vPos.x < -1.0 {
    vPos.x = 1.0;
  }
  if vPos.x > 1.0 {
    vPos.x = -1.0;
  }
  if vPos.y < -1.0 {
    vPos.y = 1.0;
  }
  if vPos.y > 1.0 {
    vPos.y = -1.0;
  }

  // Write back
  particlesDst.particles[index].pos = vPos;
  particlesDst.particles[index].vel = vVel;
}
@group(0) @binding(0)
var image_1d: texture_1d<f32>;

fn test_textureLoad_1d(coords: i32, level: i32) -> vec4<f32> {
   return textureLoad(image_1d, coords, level);
}

@group(0) @binding(1)
var image_2d: texture_2d<f32>;

fn test_textureLoad_2d(coords: vec2<i32>, level: i32) -> vec4<f32> {
   return textureLoad(image_2d, coords, level);
}

@group(0) @binding(2)
var image_2d_array: texture_2d_array<f32>;

fn test_textureLoad_2d_array_u(coords: vec2<i32>, index: u32, level: i32) -> vec4<f32> {
   return textureLoad(image_2d_array, coords, index, level);
}

fn test_textureLoad_2d_array_s(coords: vec2<i32>, index: i32, level: i32) -> vec4<f32> {
   return textureLoad(image_2d_array, coords, index, level);
}

@group(0) @binding(3)
var image_3d: texture_3d<f32>;

fn test_textureLoad_3d(coords: vec3<i32>, level: i32) -> vec4<f32> {
   return textureLoad(image_3d, coords, level);
}

@group(0) @binding(4)
var image_multisampled_2d: texture_multisampled_2d<f32>;

fn test_textureLoad_multisampled_2d(coords: vec2<i32>, _sample: i32) -> vec4<f32> {
   return textureLoad(image_multisampled_2d, coords, _sample);
}

@group(0) @binding(5)
var image_depth_2d: texture_depth_2d;

fn test_textureLoad_depth_2d(coords: vec2<i32>, level: i32) -> f32 {
   return textureLoad(image_depth_2d, coords, level);
}

@group(0) @binding(6)
var image_depth_2d_array: texture_depth_2d_array;

fn test_textureLoad_depth_2d_array_u(coords: vec2<i32>, index: u32, level: i32) -> f32 {
   return textureLoad(image_depth_2d_array, coords, index, level);
}

fn test_textureLoad_depth_2d_array_s(coords: vec2<i32>, index: i32, level: i32) -> f32 {
   return textureLoad(image_depth_2d_array, coords, index, level);
}

@group(0) @binding(7)
var image_depth_multisampled_2d: texture_depth_multisampled_2d;

fn test_textureLoad_depth_multisampled_2d(coords: vec2<i32>, _sample: i32) -> f32 {
   return textureLoad(image_depth_multisampled_2d, coords, _sample);
}

@group(0) @binding(8)
var image_storage_1d: texture_storage_1d<rgba8unorm, write>;

fn test_textureStore_1d(coords: i32, value: vec4<f32>) {
    textureStore(image_storage_1d, coords, value);
}

@group(0) @binding(9)
var image_storage_2d: texture_storage_2d<rgba8unorm, write>;

fn test_textureStore_2d(coords: vec2<i32>, value: vec4<f32>) {
    textureStore(image_storage_2d, coords, value);
}

@group(0) @binding(10)
var image_storage_2d_array: texture_storage_2d_array<rgba8unorm, write>;

fn test_textureStore_2d_array_u(coords: vec2<i32>, array_index: u32, value: vec4<f32>) {
 textureStore(image_storage_2d_array, coords, array_index, value);
}

fn test_textureStore_2d_array_s(coords: vec2<i32>, array_index: i32, value: vec4<f32>) {
 textureStore(image_storage_2d_array, coords, array_index, value);
}

@group(0) @binding(11)
var image_storage_3d: texture_storage_3d<rgba8unorm, write>;

fn test_textureStore_3d(coords: vec3<i32>, value: vec4<f32>) {
    textureStore(image_storage_3d, coords, value);
}

// GLSL output requires that we identify an entry point, so
// that it can tell what "in" and "out" globals to write.
@fragment
fn fragment_shader() -> @location(0) vec4<f32> {
    test_textureLoad_1d(0, 0);
    test_textureLoad_2d(vec2<i32>(), 0);
    test_textureLoad_2d_array_u(vec2<i32>(), 0u, 0);
    test_textureLoad_2d_array_s(vec2<i32>(), 0, 0);
    test_textureLoad_3d(vec3<i32>(), 0);
    test_textureLoad_multisampled_2d(vec2<i32>(), 0);
    // Not yet implemented for GLSL:
    // test_textureLoad_depth_2d(vec2<i32>(), 0);
    // test_textureLoad_depth_2d_array_u(vec2<i32>(), 0u, 0);
    // test_textureLoad_depth_2d_array_s(vec2<i32>(), 0, 0);
    // test_textureLoad_depth_multisampled_2d(vec2<i32>(), 0);
    test_textureStore_1d(0, vec4<f32>());
    test_textureStore_2d(vec2<i32>(), vec4<f32>());
    test_textureStore_2d_array_u(vec2<i32>(), 0u, vec4<f32>());
    test_textureStore_2d_array_s(vec2<i32>(), 0, vec4<f32>());
    test_textureStore_3d(vec3<i32>(), vec4<f32>());

    return vec4<f32>(0.,0.,0.,0.);
}
@group(0) @binding(0)
var image_1d: texture_1d<f32>;

fn test_textureLoad_1d(coords: i32, level: i32) -> vec4<f32> {
   return textureLoad(image_1d, coords, level);
}

@group(0) @binding(1)
var image_2d: texture_2d<f32>;

fn test_textureLoad_2d(coords: vec2<i32>, level: i32) -> vec4<f32> {
   return textureLoad(image_2d, coords, level);
}

@group(0) @binding(2)
var image_2d_array: texture_2d_array<f32>;

fn test_textureLoad_2d_array_u(coords: vec2<i32>, index: u32, level: i32) -> vec4<f32> {
   return textureLoad(image_2d_array, coords, index, level);
}

fn test_textureLoad_2d_array_s(coords: vec2<i32>, index: i32, level: i32) -> vec4<f32> {
   return textureLoad(image_2d_array, coords, index, level);
}

@group(0) @binding(3)
var image_3d: texture_3d<f32>;

fn test_textureLoad_3d(coords: vec3<i32>, level: i32) -> vec4<f32> {
   return textureLoad(image_3d, coords, level);
}

@group(0) @binding(4)
var image_multisampled_2d: texture_multisampled_2d<f32>;

fn test_textureLoad_multisampled_2d(coords: vec2<i32>, _sample: i32) -> vec4<f32> {
   return textureLoad(image_multisampled_2d, coords, _sample);
}

@group(0) @binding(5)
var image_depth_2d: texture_depth_2d;

fn test_textureLoad_depth_2d(coords: vec2<i32>, level: i32) -> f32 {
   return textureLoad(image_depth_2d, coords, level);
}

@group(0) @binding(6)
var image_depth_2d_array: texture_depth_2d_array;

fn test_textureLoad_depth_2d_array_u(coords: vec2<i32>, index: u32, level: i32) -> f32 {
   return textureLoad(image_depth_2d_array, coords, index, level);
}

fn test_textureLoad_depth_2d_array_s(coords: vec2<i32>, index: i32, level: i32) -> f32 {
   return textureLoad(image_depth_2d_array, coords, index, level);
}

@group(0) @binding(7)
var image_depth_multisampled_2d: texture_depth_multisampled_2d;

fn test_textureLoad_depth_multisampled_2d(coords: vec2<i32>, _sample: i32) -> f32 {
   return textureLoad(image_depth_multisampled_2d, coords, _sample);
}

@group(0) @binding(8)
var image_storage_1d: texture_storage_1d<rgba8unorm, write>;

fn test_textureStore_1d(coords: i32, value: vec4<f32>) {
    textureStore(image_storage_1d, coords, value);
}

@group(0) @binding(9)
var image_storage_2d: texture_storage_2d<rgba8unorm, write>;

fn test_textureStore_2d(coords: vec2<i32>, value: vec4<f32>) {
    textureStore(image_storage_2d, coords, value);
}

@group(0) @binding(10)
var image_storage_2d_array: texture_storage_2d_array<rgba8unorm, write>;

fn test_textureStore_2d_array_u(coords: vec2<i32>, array_index: u32, value: vec4<f32>) {
 textureStore(image_storage_2d_array, coords, array_index, value);
}

fn test_textureStore_2d_array_s(coords: vec2<i32>, array_index: i32, value: vec4<f32>) {
 textureStore(image_storage_2d_array, coords, array_index, value);
}

@group(0) @binding(11)
var image_storage_3d: texture_storage_3d<rgba8unorm, write>;

fn test_textureStore_3d(coords: vec3<i32>, value: vec4<f32>) {
    textureStore(image_storage_3d, coords, value);
}

// GLSL output requires that we identify an entry point, so
// that it can tell what "in" and "out" globals to write.
@fragment
fn fragment_shader() -> @location(0) vec4<f32> {
    test_textureLoad_1d(0, 0);
    test_textureLoad_2d(vec2<i32>(), 0);
    test_textureLoad_2d_array_u(vec2<i32>(), 0u, 0);
    test_textureLoad_2d_array_s(vec2<i32>(), 0, 0);
    test_textureLoad_3d(vec3<i32>(), 0);
    test_textureLoad_multisampled_2d(vec2<i32>(), 0);
    // Not yet implemented for GLSL:
    // test_textureLoad_depth_2d(vec2<i32>(), 0);
    // test_textureLoad_depth_2d_array_u(vec2<i32>(), 0u, 0);
    // test_textureLoad_depth_2d_array_s(vec2<i32>(), 0, 0);
    // test_textureLoad_depth_multisampled_2d(vec2<i32>(), 0);
    test_textureStore_1d(0, vec4<f32>());
    test_textureStore_2d(vec2<i32>(), vec4<f32>());
    test_textureStore_2d_array_u(vec2<i32>(), 0u, vec4<f32>());
    test_textureStore_2d_array_s(vec2<i32>(), 0, vec4<f32>());
    test_textureStore_3d(vec3<i32>(), vec4<f32>());

    return vec4<f32>(0.,0.,0.,0.);
}
// Tests for `naga::back::BoundsCheckPolicy::Restrict`.

struct Globals {
    a: array<f32, 10>,
    v: vec4<f32>,
    m: mat3x4<f32>,
    d: array<f32>,
}

@group(0) @binding(0) var<storage, read_write> globals: Globals;

fn index_array(i: i32) -> f32 {
   return globals.a[i];
}

fn index_dynamic_array(i: i32) -> f32 {
   return globals.d[i];
}

fn index_vector(i: i32) -> f32 {
   return globals.v[i];
}

fn index_vector_by_value(v: vec4<f32>, i: i32) -> f32 {
   return v[i];
}

fn index_matrix(i: i32) -> vec4<f32> {
   return globals.m[i];
}

fn index_twice(i: i32, j: i32) -> f32 {
   return globals.m[i][j];
}

fn index_expensive(i: i32) -> f32 {
   return globals.a[i32(sin(f32(i) / 100.0) * 100.0)];
}

fn index_in_bounds() -> f32 {
   return globals.a[9] + globals.v[3] + globals.m[2][3];
}

fn set_array(i: i32, v: f32) {
   globals.a[i] = v;
}

fn set_dynamic_array(i: i32, v: f32) {
   globals.d[i] = v;
}

fn set_vector(i: i32, v: f32) {
   globals.v[i] = v;
}

fn set_matrix(i: i32, v: vec4<f32>) {
   globals.m[i] = v;
}

fn set_index_twice(i: i32, j: i32, v: f32) {
   globals.m[i][j] = v;
}

fn set_expensive(i: i32, v: f32) {
   globals.a[i32(sin(f32(i) / 100.0) * 100.0)] = v;
}

fn set_in_bounds(v: f32) {
   globals.a[9] = v;
   globals.v[3] = v;
   globals.m[2][3] = v;
}
// Tests for `naga::back::BoundsCheckPolicy::ReadZeroSkipWrite` for atomic types.

// These are separate from `bounds-check-zero.wgsl because SPIR-V does not yet
// support `ReadZeroSkipWrite` for atomics. Once it does, the test files could
// be combined.

struct Globals {
    a: atomic<u32>,
    b: array<atomic<u32>, 10>,
    c: array<atomic<u32>>,
}

@group(0) @binding(0) var<storage, read_write> globals: Globals;

fn fetch_add_atomic() -> u32 {
   return atomicAdd(&globals.a, 1u);
}

fn fetch_add_atomic_static_sized_array(i: i32) -> u32 {
   return atomicAdd(&globals.b[i], 1u);
}

fn fetch_add_atomic_dynamic_sized_array(i: i32) -> u32 {
   return atomicAdd(&globals.c[i], 1u);
}

fn exchange_atomic() -> u32 {
   return atomicExchange(&globals.a, 1u);
}

fn exchange_atomic_static_sized_array(i: i32) -> u32 {
   return atomicExchange(&globals.b[i], 1u);
}

fn exchange_atomic_dynamic_sized_array(i: i32) -> u32 {
   return atomicExchange(&globals.c[i], 1u);
}

// Tests for `naga::back::BoundsCheckPolicy::ReadZeroSkipWrite`.

struct Globals {
    a: array<f32, 10>,
    v: vec4<f32>,
    m: mat3x4<f32>,
    d: array<f32>,
}

@group(0) @binding(0) var<storage, read_write> globals: Globals;

fn index_array(i: i32) -> f32 {
   return globals.a[i];
}

fn index_dynamic_array(i: i32) -> f32 {
   return globals.d[i];
}

fn index_vector(i: i32) -> f32 {
   return globals.v[i];
}

fn index_vector_by_value(v: vec4<f32>, i: i32) -> f32 {
   return v[i];
}

fn index_matrix(i: i32) -> vec4<f32> {
   return globals.m[i];
}

fn index_twice(i: i32, j: i32) -> f32 {
   return globals.m[i][j];
}

fn index_expensive(i: i32) -> f32 {
   return globals.a[i32(sin(f32(i) / 100.0) * 100.0)];
}

fn index_in_bounds() -> f32 {
   return globals.a[9] + globals.v[3] + globals.m[2][3];
}

fn set_array(i: i32, v: f32) {
   globals.a[i] = v;
}

fn set_dynamic_array(i: i32, v: f32) {
   globals.d[i] = v;
}

fn set_vector(i: i32, v: f32) {
   globals.v[i] = v;
}

fn set_matrix(i: i32, v: vec4<f32>) {
   globals.m[i] = v;
}

fn set_index_twice(i: i32, j: i32, v: f32) {
   globals.m[i][j] = v;
}

fn set_expensive(i: i32, v: f32) {
   globals.a[i32(sin(f32(i) / 100.0) * 100.0)] = v;
}

fn set_in_bounds(v: f32) {
   globals.a[9] = v;
   globals.v[3] = v;
   globals.m[2][3] = v;
}
@compute @workgroup_size(1)
fn main() {}

fn breakIfEmpty() {
    loop {
        continuing {
            break if true;
        }
    }
}

fn breakIfEmptyBody(a: bool) {
    loop {
        continuing {
            var b = a;
            var c = a != b;

            break if a == c;
        }
    }
}

fn breakIf(a: bool) {
    loop {
        var d = a;
        var e = a != d;

        continuing {
            break if a == e;
        }
    }
}
struct PrimeIndices {
    data: array<u32>
} // this is used as both input and output for convenience

@group(0) @binding(0)
var<storage,read_write> v_indices: PrimeIndices;

// The Collatz Conjecture states that for any integer n:
// If n is even, n = n/2
// If n is odd, n = 3n+1
// And repeat this process for each new n, you will always eventually reach 1.
// Though the conjecture has not been proven, no counterexample has ever been found.
// This function returns how many times this recurrence needs to be applied to reach 1.
fn collatz_iterations(n_base: u32) -> u32 {
    var n = n_base;
    var i: u32 = 0u;
    while n > 1u {
        if n % 2u == 0u {
            n = n / 2u;
        }
        else {
            n = 3u * n + 1u;
        }
        i = i + 1u;
    }
    return i;
}

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    v_indices.data[global_id.x] = collatz_iterations(v_indices.data[global_id.x]);
}
const TWO: u32 = 2u;
const THREE: i32 = 3i;

@compute @workgroup_size(TWO, THREE, TWO - 1u)
fn main() {
    swizzle_of_compose();
    index_of_compose();
    compose_three_deep();
    non_constant_initializers();
    splat_of_constant();
    compose_of_constant();
}

// Swizzle the value of nested Compose expressions.
fn swizzle_of_compose() {
    var out = vec4(vec2(1, 2), vec2(3, 4)).wzyx; // should assign vec4(4, 3, 2, 1);
}

// Index the value of nested Compose expressions.
fn index_of_compose() {
    var out = vec4(vec2(1, 2), vec2(3, 4))[1]; // should assign 2
}

// Index the value of Compose expressions nested three deep
fn compose_three_deep() {
    var out = vec4(vec3(vec2(6, 7), 8), 9)[0]; // should assign 6
}

// While WGSL allows local variables to be declared anywhere in the function,
// Naga treats them all as appearing at the top of the function. To ensure that
// WGSL initializer expressions are evaluated at the right time, in the general
// case they need to be turned into Naga `Store` statements executed at the
// point of the WGSL declaration.
//
// When a variable's initializer is a constant expression, however, it can be
// evaluated at any time. The WGSL front end thus renders locals with
// initializers that are constants as Naga locals with initializers. This test
// checks that Naga local variable initializers are only used when safe.
fn non_constant_initializers() {
    var w = 10 + 20;
    var x = w;
    var y = x;
    var z = 30 + 40;

    var out = vec4(w, x, y, z);
}

// Constant evaluation should be able to see through constants to
// their values.
const FOUR: i32 = 4;

const FOUR_ALIAS: i32 = FOUR;

const TEST_CONSTANT_ADDITION: i32 = FOUR + FOUR;
const TEST_CONSTANT_ALIAS_ADDITION: i32 = FOUR_ALIAS + FOUR_ALIAS;

fn splat_of_constant() {
    var out = -vec4(FOUR);
}

fn compose_of_constant() {
    var out = -vec4(FOUR, FOUR, FOUR, FOUR);
}

const PI: f32 = 3.141;
const phi_sun: f32 = PI * 2.0;

const DIV: vec4f = vec4(4.0 / 9.0, 0.0, 0.0, 0.0);

const TEXTURE_KIND_REGULAR: i32 = 0;
const TEXTURE_KIND_WARP: i32 = 1;
const TEXTURE_KIND_SKY: i32 = 2;

fn map_texture_kind(texture_kind: i32) -> u32 {
    switch (texture_kind) {
        case TEXTURE_KIND_REGULAR: { return 10u; }
        case TEXTURE_KIND_WARP: { return 20u; }
        case TEXTURE_KIND_SKY: { return 30u; }
        default: { return 0u; }
    }
}
struct Foo {
    a: vec4<f32>,
    b: i32,
}

// const const1 = vec3<f32>(0.0); // TODO: this is now a splat and we need to const eval it
const const2 = vec3(0.0, 1.0, 2.0);
const const3 = mat2x2<f32>(0.0, 1.0, 2.0, 3.0);
const const4 = array<mat2x2<f32>, 1>(mat2x2<f32>(0.0, 1.0, 2.0, 3.0));

// zero value constructors
const cz0 = bool();
const cz1 = i32();
const cz2 = u32();
const cz3 = f32();
const cz4 = vec2<u32>();
const cz5 = mat2x2<f32>();
const cz6 = array<Foo, 3>();
const cz7 = Foo();

// constructors that infer their type from their parameters
// TODO: these also contain splats
// const cp1 = vec2(0u);
// const cp2 = mat2x2(vec2(0.), vec2(0.));
const cp3 = array(0, 1, 2, 3);

@compute @workgroup_size(1)
fn main() {
    var foo: Foo;
    foo = Foo(vec4<f32>(1.0), 1);

    let m0 = mat2x2<f32>(
        1.0, 0.0,
        0.0, 1.0,
    );
    let m1 = mat4x4<f32>(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    // zero value constructors
    let zvc0 = bool();
    let zvc1 = i32();
    let zvc2 = u32();
    let zvc3 = f32();
    let zvc4 = vec2<u32>();
    let zvc5 = mat2x2<f32>();
    let zvc6 = array<Foo, 3>();
    let zvc7 = Foo();

    // constructors that infer their type from their parameters
    let cit0 = vec2(0u);
    let cit1 = mat2x2(vec2(0.), vec2(0.));
    let cit2 = array(0, 1, 2, 3);

    // identity constructors
    let ic0 = bool(bool());
    let ic1 = i32(i32());
    let ic2 = u32(u32());
    let ic3 = f32(f32());
    let ic4 = vec2<u32>(vec2<u32>());
    let ic5 = mat2x3<f32>(mat2x3<f32>());
    let ic6 = vec2(vec2<u32>());
    let ic7 = mat2x3(mat2x3<f32>());
}
@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    //TODO: execution-only barrier?
    storageBarrier();
    workgroupBarrier();

    var pos: i32;
    // switch without cases
    switch 1 {
        default: {
            pos = 1;
        }
    }

    // non-empty switch *not* in last-statement-in-function position
    // (return statements might be inserted into the switch cases otherwise)
    switch pos {
        case 1: {
            pos = 0;
            break;
        }
        case 2: {
            pos = 1;
        }
        case 3, 4: {
            pos = 2;
        }
        case 5: {
            pos = 3;
        }
        case default, 6: {
            pos = 4;
        }
    }

	// switch with unsigned integer selectors
	switch(0u) {
		case 0u: {
		}
        default: {
        }
	}

    // non-empty switch in last-statement-in-function position
    switch pos {
        case 1: {
            pos = 0;
            break;
        }
        case 2: {
            pos = 1;
        }
        case 3: {
            pos = 2;
        }
        case 4: {}
        default: {
            pos = 3;
        }
    }
}

fn switch_default_break(i: i32) {
    switch i {
        default: {
            break;
        }
    }
}

fn switch_case_break() {
    switch(0) {
        case 0: {
            break;
        }
        default: {}
    }
    return;
}

fn loop_switch_continue(x: i32) {
    loop {
        switch x {
            case 1: {
                continue;
            }
            default: {}
        }
    }
}
@group(0) @binding(4)
var point_shadow_textures: texture_depth_cube_array;
@group(0) @binding(5)
var point_shadow_textures_sampler: sampler_comparison;

@fragment
fn fragment() -> @location(0) vec4<f32> {
    let frag_ls = vec4<f32>(1., 1., 2., 1.).xyz;
    let a = textureSampleCompare(point_shadow_textures, point_shadow_textures_sampler, frag_ls, i32(1), 1.);

    return vec4<f32>(a, 1., 1., 1.);
}
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = in.color;
    for (var i = 0; i < 10; i += 1) {
        var ii = f32(i);
        color.x += ii*0.001;
        color.y += ii*0.002;
    }

    return vec4<f32>(color, 1.0);
}// Taken from https://github.com/sotrh/learn-wgpu/blob/11820796f5e1dbce42fb1119f04ddeb4b167d2a0/code/intermediate/tutorial13-terrain/src/terrain.wgsl
// ============================
// Terrain Generation
// ============================

// https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39
//  MIT License. (c) Ian McEwan, Stefan Gustavson, Munrocket
// - Less condensed glsl implementation with comments can be found at https://weber.itn.liu.se/~stegu/jgt2012/article.pdf

fn permute3(x: vec3<f32>) -> vec3<f32> { return (((x * 34.) + 1.) * x) % vec3<f32>(289.); }

fn snoise2(v: vec2<f32>) -> f32 {
    let C = vec4<f32>(0.211324865405187, 0.366025403784439, -0.577350269189626, 0.024390243902439);
    var i: vec2<f32> = floor(v + dot(v, C.yy));
    let x0 = v - i + dot(i, C.xx);
    // I flipped the condition here from > to < as it fixed some artifacting I was observing
    var i1: vec2<f32> = select(vec2<f32>(1., 0.), vec2<f32>(0., 1.), (x0.x < x0.y));
    var x12: vec4<f32> = x0.xyxy + C.xxzz - vec4<f32>(i1, 0., 0.);
    i = i % vec2<f32>(289.);
    let p = permute3(permute3(i.y + vec3<f32>(0., i1.y, 1.)) + i.x + vec3<f32>(0., i1.x, 1.));
    var m: vec3<f32> = max(0.5 - vec3<f32>(dot(x0, x0), dot(x12.xy, x12.xy), dot(x12.zw, x12.zw)), vec3<f32>(0.));
    m = m * m;
    m = m * m;
    let x = 2. * fract(p * C.www) - 1.;
    let h = abs(x) - 0.5;
    let ox = floor(x + 0.5);
    let a0 = x - ox;
    m = m * (1.79284291400159 - 0.85373472095314 * (a0 * a0 + h * h));
    let g = vec3<f32>(a0.x * x0.x + h.x * x0.y, a0.yz * x12.xz + h.yz * x12.yw);
    return 130. * dot(m, g);
}


fn fbm(p: vec2<f32>) -> f32 {
    let NUM_OCTAVES: u32 = 5u;
    var x = p * 0.01;
    var v = 0.0;
    var a = 0.5;
    let shift = vec2<f32>(100.0);
    let cs = vec2<f32>(cos(0.5), sin(0.5));
    let rot = mat2x2<f32>(cs.x, cs.y, -cs.y, cs.x);

    for (var i = 0u; i < NUM_OCTAVES; i = i + 1u) {
        v = v + a * snoise2(x);
        x = rot * x * 2.0 + shift;
        a = a * 0.5;
    }

    return v;
}

struct ChunkData {
    chunk_size: vec2<u32>,
    chunk_corner: vec2<i32>,
    min_max_height: vec2<f32>,
}

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct VertexBuffer {
    data: array<Vertex>, // stride: 32
}

struct IndexBuffer {
    data: array<u32>,
}

@group(0) @binding(0) var<uniform> chunk_data: ChunkData;
@group(0) @binding(1) var<storage, read_write> vertices: VertexBuffer;
@group(0) @binding(2) var<storage, read_write> indices: IndexBuffer;

fn terrain_point(p: vec2<f32>, min_max_height: vec2<f32>) -> vec3<f32> {
    return vec3<f32>(
        p.x,
        mix(min_max_height.x, min_max_height.y, fbm(p)),
        p.y,
    );
}

fn terrain_vertex(p: vec2<f32>, min_max_height: vec2<f32>) -> Vertex {
    let v = terrain_point(p, min_max_height);

    let tpx = terrain_point(p + vec2<f32>(0.1, 0.0), min_max_height) - v;
    let tpz = terrain_point(p + vec2<f32>(0.0, 0.1), min_max_height) - v;
    let tnx = terrain_point(p + vec2<f32>(-0.1, 0.0), min_max_height) - v;
    let tnz = terrain_point(p + vec2<f32>(0.0, -0.1), min_max_height) - v;

    let pn = normalize(cross(tpz, tpx));
    let nn = normalize(cross(tnz, tnx));

    let n = (pn + nn) * 0.5;

    return Vertex(v, n);
}

fn index_to_p(vert_index: u32, chunk_size: vec2<u32>, chunk_corner: vec2<i32>) -> vec2<f32> {
    return vec2(
        f32(vert_index) % f32(chunk_size.x + 1u),
        f32(vert_index / (chunk_size.x + 1u)),
    ) + vec2<f32>(chunk_corner);
}

@compute @workgroup_size(64)
fn gen_terrain_compute(
    @builtin(global_invocation_id) gid: vec3<u32>
) {
    // Create vert_component
    let vert_index = gid.x;

    let p = index_to_p(vert_index, chunk_data.chunk_size, chunk_data.chunk_corner);

    vertices.data[vert_index] = terrain_vertex(p, chunk_data.min_max_height);

    // Create indices
    let start_index = gid.x * 6u; // using TriangleList

    if (start_index >= (chunk_data.chunk_size.x * chunk_data.chunk_size.y * 6u)) { return; }

    let v00 = vert_index + gid.x / chunk_data.chunk_size.x;
    let v10 = v00 + 1u;
    let v01 = v00 + chunk_data.chunk_size.x + 1u;
    let v11 = v01 + 1u;

    indices.data[start_index] = v00;
    indices.data[start_index + 1u] = v01;
    indices.data[start_index + 2u] = v11;
    indices.data[start_index + 3u] = v00;
    indices.data[start_index + 4u] = v11;
    indices.data[start_index + 5u] = v10;
}

// ============================
// Terrain Gen (Fragment Shader)
// ============================

struct GenData {
    chunk_size: vec2<u32>,
    chunk_corner: vec2<i32>,
    min_max_height: vec2<f32>,
    texture_size: u32,
    start_index: u32,
}
@group(0)
@binding(0)
var<uniform> gen_data: GenData;

struct GenVertexOutput {
    @location(0)
    index: u32,
    @builtin(position)
    position: vec4<f32>,
    @location(1)
    uv: vec2<f32>,
};

@vertex
fn gen_terrain_vertex(@builtin(vertex_index) vindex: u32) -> GenVertexOutput {
    let u = f32(((vindex + 2u) / 3u) % 2u);
    let v = f32(((vindex + 1u) / 3u) % 2u);
    let uv = vec2<f32>(u, v);

    let position = vec4<f32>(-1.0 + uv * 2.0, 0.0, 1.0);

    // TODO: maybe replace this with u32(dot(uv, vec2(f32(gen_data.texture_dim.x))))
    let index = u32(uv.x * f32(gen_data.texture_size) + uv.y * f32(gen_data.texture_size)) + gen_data.start_index;

    return GenVertexOutput(index, position, uv);
}


struct GenFragmentOutput {
    @location(0) vert_component: u32,
    @location(1) index: u32,
}

@fragment
fn gen_terrain_fragment(in: GenVertexOutput) -> GenFragmentOutput {
    let i = u32(in.uv.x * f32(gen_data.texture_size) + in.uv.y * f32(gen_data.texture_size * gen_data.texture_size)) + gen_data.start_index;
    let vert_index = u32(floor(f32(i) / 6.));
    let comp_index = i % 6u;

    let p = index_to_p(vert_index, gen_data.chunk_size, gen_data.chunk_corner);
    let v = terrain_vertex(p, gen_data.min_max_height);

    var vert_component: f32 = 0.;
    
    switch comp_index {
        case 0u: { vert_component = v.position.x; }
        case 1u: { vert_component = v.position.y; }
        case 2u: { vert_component = v.position.z; }
        case 3u: { vert_component = v.normal.x; }
        case 4u: { vert_component = v.normal.y; }
        case 5u: { vert_component = v.normal.z; }
        default: {}
    }

    let v00 = vert_index + vert_index / gen_data.chunk_size.x;
    let v10 = v00 + 1u;
    let v01 = v00 + gen_data.chunk_size.x + 1u;
    let v11 = v01 + 1u;

    var index = 0u;
    switch comp_index {
        case 0u, 3u: { index = v00; }
        case 2u, 4u: { index = v11; }
        case 1u: { index = v01; }
        case 5u: { index = v10; }
        default: {}
    }
    index = in.index;
    // index = gen_data.start_index;
    // indices.data[start_index] = v00;
    // indices.data[start_index + 1u] = v01;
    // indices.data[start_index + 2u] = v11;
    // indices.data[start_index + 3u] = v00;
    // indices.data[start_index + 4u] = v11;
    // indices.data[start_index + 5u] = v10;

    let ivert_component = bitcast<u32>(vert_component);
    return GenFragmentOutput(ivert_component, index);
}

// ============================
// Terrain Rendering
// ============================

struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
}
@group(0) @binding(0)
var<uniform> camera: Camera;

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
}
@group(1) @binding(0)
var<uniform> light: Light;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) world_pos: vec3<f32>,
}

@vertex
fn vs_main(
    vertex: Vertex,
) -> VertexOutput {
    let clip_position = camera.view_proj * vec4<f32>(vertex.position, 1.);
    let normal = vertex.normal;
    return VertexOutput(clip_position, normal, vertex.position);
}

@group(2) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(2) @binding(1)
var s_diffuse: sampler;
@group(2) @binding(2)
var t_normal: texture_2d<f32>;
@group(2) @binding(3)
var s_normal: sampler;

fn color23(p: vec2<f32>) -> vec3<f32> {
    return vec3<f32>(
        snoise2(p) * 0.5 + 0.5,
        snoise2(p + vec2<f32>(23., 32.)) * 0.5 + 0.5,
        snoise2(p + vec2<f32>(-43., 3.)) * 0.5 + 0.5,
    );
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = smoothstep(vec3<f32>(0.0), vec3<f32>(0.1), fract(in.world_pos));
    color = mix(vec3<f32>(0.5, 0.1, 0.7), vec3<f32>(0.2, 0.2, 0.2), vec3<f32>(color.x * color.y * color.z));

    let ambient_strength = 0.1;
    let ambient_color = light.color * ambient_strength;

    let light_dir = normalize(light.position - in.world_pos);
    let view_dir = normalize(camera.view_pos.xyz - in.world_pos);
    let half_dir = normalize(view_dir + light_dir);

    let diffuse_strength = max(dot(in.normal, light_dir), 0.0);
    let diffuse_color = diffuse_strength * light.color;

    let specular_strength = pow(max(dot(in.normal, half_dir), 0.0), 32.0);
    let specular_color = specular_strength * light.color;

    let result = (ambient_color + diffuse_color + specular_color) * color;

    return vec4<f32>(result, 1.0);
}/* Simple test for multiple output sources from fragment shaders */
struct FragmentOutput{
    @location(0) color: vec4<f32>,
    @location(0) mask: vec4<f32>,
}
@fragment
fn main(@builtin(position) position: vec4<f32>) -> FragmentOutput {
    var color = vec4(0.4,0.3,0.2,0.1);
    var mask = vec4(0.9,0.8,0.7,0.6);
    return FragmentOutput(color, mask);
}
@compute @workgroup_size(1)
fn main() {}
struct PushConstants {
    index: u32,
    double: vec2<f64>,
}
var<push_constant> pc: PushConstants;

struct FragmentIn {
    @location(0) color: vec4<f32>,
    @builtin(primitive_index) primitive_index: u32,
}

@fragment
fn main(in: FragmentIn) -> @location(0) vec4<f32> {
    if in.primitive_index == pc.index {
        return in.color;
    } else {
        return vec4<f32>(vec3<f32>(1.0) - in.color.rgb, in.color.a);
    }
}
// AUTHOR: REASY
// ISSUE: https://github.com/gfx-rs/wgpu/issues/3179
// FIX: https://github.com/gfx-rs/wgpu/pull/3440
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32((in_vertex_index) | 1u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
// Split up because some output languages limit number of locations to 8.
struct FragmentOutputVec4Vec3 {
    @location(0) vec4f: vec4<f32>,
    @location(1) vec4i: vec4<i32>,
    @location(2) vec4u: vec4<u32>,
    @location(3) vec3f: vec3<f32>,
    @location(4) vec3i: vec3<i32>,
    @location(5) vec3u: vec3<u32>,
}
@fragment
fn main_vec4vec3() -> FragmentOutputVec4Vec3 {
    var output: FragmentOutputVec4Vec3;
    output.vec4f = vec4<f32>(0.0);
    output.vec4i = vec4<i32>(0);
    output.vec4u = vec4<u32>(0u);
    output.vec3f = vec3<f32>(0.0);
    output.vec3i = vec3<i32>(0);
    output.vec3u = vec3<u32>(0u);
    return output;
}

struct FragmentOutputVec2Scalar {
    @location(0) vec2f: vec2<f32>,
    @location(1) vec2i: vec2<i32>,
    @location(2) vec2u: vec2<u32>,
    @location(3) scalarf: f32,
    @location(4) scalari: i32,
    @location(5) scalaru: u32,
}

@fragment
fn main_vec2scalar() -> FragmentOutputVec2Scalar {
    var output: FragmentOutputVec2Scalar;
    output.vec2f = vec2<f32>(0.0);
    output.vec2i = vec2<i32>(0);
    output.vec2u = vec2<u32>(0u);
    output.scalarf = 0.0;
    output.scalari = 0;
    output.scalaru = 0u;
    return output;
}
fn test_fma() -> vec2<f32> {
    let a = vec2<f32>(2.0, 2.0);
    let b = vec2<f32>(0.5, 0.5);
    let c = vec2<f32>(0.5, 0.5);

    return fma(a, b, c);
}


@fragment
fn main() {
    let a = test_fma();
}
fn test_fma() -> vec2<f32> {
    let a = vec2<f32>(2.0, 2.0);
    let b = vec2<f32>(0.5, 0.5);
    let c = vec2<f32>(0.5, 0.5);

    // Hazard: HLSL needs a different intrinsic function for f32 and f64
    // See: https://github.com/gfx-rs/naga/issues/1579
    return fma(a, b, c);
}

fn test_integer_dot_product() -> i32 {
    let a_2 = vec2<i32>(1);
    let b_2 = vec2<i32>(1);
    let c_2: i32 = dot(a_2, b_2);

    let a_3 = vec3<u32>(1u);
    let b_3 = vec3<u32>(1u);
    let c_3: u32 = dot(a_3, b_3);

    // test baking of arguments
    let c_4: i32 = dot(vec4<i32>(4), vec4<i32>(2));
    return c_4;
}

@compute @workgroup_size(1)
fn main() {
    let a = test_fma();
    let b = test_integer_dot_product();
}
// Global variable & constant declarations

const Foo: bool = true;

var<workgroup> wg : array<f32, 10u>;
var<workgroup> at: atomic<u32>;

struct FooStruct {
    v3: vec3<f32>,
    // test packed vec3
    v1: f32,
}
@group(0) @binding(1)
var<storage, read_write> alignment: FooStruct;

@group(0) @binding(2)
var<storage> dummy: array<vec2<f32>>;

@group(0) @binding(3)
var<uniform> float_vecs: array<vec4<f32>, 20>;

@group(0) @binding(4)
var<uniform> global_vec: vec3<f32>;

@group(0) @binding(5)
var<uniform> global_mat: mat3x2<f32>;

@group(0) @binding(6)
var<uniform> global_nested_arrays_of_matrices_2x4: array<array<mat2x4<f32>, 2>, 2>;

@group(0) @binding(7)
var<uniform> global_nested_arrays_of_matrices_4x2: array<array<mat4x2<f32>, 2>, 2>;

fn test_msl_packed_vec3_as_arg(arg: vec3<f32>) {}

fn test_msl_packed_vec3() {
    // stores
    alignment.v3 = vec3<f32>(1.0);
    var idx = 1;
    alignment.v3.x = 1.0;
    alignment.v3[0] = 2.0;
    alignment.v3[idx] = 3.0;

    // force load to happen here
    let data = alignment;

    // loads
    let l0 = data.v3;
    let l1 = data.v3.zx;
    test_msl_packed_vec3_as_arg(data.v3);

    // matrix vector multiplication
    let mvm0 = data.v3 * mat3x3<f32>();
    let mvm1 = mat3x3<f32>() * data.v3;

    // scalar vector multiplication
    let svm0 = data.v3 * 2.0;
    let svm1 = 2.0 * data.v3;
}

@compute @workgroup_size(1)
fn main() {
    test_msl_packed_vec3();

    wg[7] = (global_nested_arrays_of_matrices_4x2[0][0] * global_nested_arrays_of_matrices_2x4[0][0][0]).x;
    wg[6] = (global_mat * global_vec).x;
    wg[5] = dummy[1].y;
    wg[4] = float_vecs[0].w;
    wg[3] = alignment.v1;
    wg[2] = alignment.v3.x;
    alignment.v1 = 4.0;
    wg[1] = f32(arrayLength(&dummy));
    atomicStore(&at, 2u);

    // Valid, Foo and at is in function scope
    var Foo: f32 = 1.0;
    var at: bool = true;
}
@fragment
fn fs_main() -> @location(0) vec4f {
    // Make sure case-insensitive keywords are escaped in HLSL.
    var Pass = vec4(1.0,1.0,1.0,1.0);
    return Pass;
}@group(0) @binding(0)
var image_mipmapped_src: texture_2d<u32>;
@group(0) @binding(3)
var image_multisampled_src: texture_multisampled_2d<u32>;
@group(0) @binding(4)
var image_depth_multisampled_src: texture_depth_multisampled_2d;
@group(0) @binding(1)
var image_storage_src: texture_storage_2d<rgba8uint, read>;
@group(0) @binding(5)
var image_array_src: texture_2d_array<u32>;
@group(0) @binding(6)
var image_dup_src: texture_storage_1d<r32uint,read>; // for #1307
@group(0) @binding(7)
var image_1d_src: texture_1d<u32>;
@group(0) @binding(2)
var image_dst: texture_storage_1d<r32uint,write>;

@compute @workgroup_size(16)
fn main(@builtin(local_invocation_id) local_id: vec3<u32>) {
    let dim = textureDimensions(image_storage_src);
    let itc = vec2<i32>(dim * local_id.xy) % vec2<i32>(10, 20);
    // loads with ivec2 coords.
    let value1 = textureLoad(image_mipmapped_src, itc, i32(local_id.z));
    let value2 = textureLoad(image_multisampled_src, itc, i32(local_id.z));
    let value4 = textureLoad(image_storage_src, itc);
    let value5 = textureLoad(image_array_src, itc, local_id.z, i32(local_id.z) + 1);
    let value6 = textureLoad(image_array_src, itc, i32(local_id.z), i32(local_id.z) + 1);
    let value7 = textureLoad(image_1d_src, i32(local_id.x), i32(local_id.z));
    // loads with uvec2 coords.
    let value1u = textureLoad(image_mipmapped_src, vec2<u32>(itc), i32(local_id.z));
    let value2u = textureLoad(image_multisampled_src, vec2<u32>(itc), i32(local_id.z));
    let value4u = textureLoad(image_storage_src, vec2<u32>(itc));
    let value5u = textureLoad(image_array_src, vec2<u32>(itc), local_id.z, i32(local_id.z) + 1);
    let value6u = textureLoad(image_array_src, vec2<u32>(itc), i32(local_id.z), i32(local_id.z) + 1);
    let value7u = textureLoad(image_1d_src, u32(local_id.x), i32(local_id.z));
    // store with ivec2 coords.
    textureStore(image_dst, itc.x, value1 + value2 + value4 + value5 + value6);
    // store with uvec2 coords.
    textureStore(image_dst, u32(itc.x), value1u + value2u + value4u + value5u + value6u);
}

@compute @workgroup_size(16, 1, 1)
fn depth_load(@builtin(local_invocation_id) local_id: vec3<u32>) {
    let dim: vec2<u32> = textureDimensions(image_storage_src);
    let itc: vec2<i32> = (vec2<i32>(dim * local_id.xy) % vec2<i32>(10, 20));
    let val: f32 = textureLoad(image_depth_multisampled_src, itc, i32(local_id.z));
    textureStore(image_dst, itc.x, vec4<u32>(u32(val)));
    return;
}

@group(0) @binding(0)
var image_1d: texture_1d<f32>;
@group(0) @binding(1)
var image_2d: texture_2d<f32>;
@group(0) @binding(2)
var image_2d_u32: texture_2d<u32>;
@group(0) @binding(3)
var image_2d_i32: texture_2d<i32>;
@group(0) @binding(4)
var image_2d_array: texture_2d_array<f32>;
@group(0) @binding(5)
var image_cube: texture_cube<f32>;
@group(0) @binding(6)
var image_cube_array: texture_cube_array<f32>;
@group(0) @binding(7)
var image_3d: texture_3d<f32>;
@group(0) @binding(8)
var image_aa: texture_multisampled_2d<f32>;

@vertex
fn queries() -> @builtin(position) vec4<f32> {
    let dim_1d = textureDimensions(image_1d);
    let dim_1d_lod = textureDimensions(image_1d, i32(dim_1d));
    let dim_2d = textureDimensions(image_2d);
    let dim_2d_lod = textureDimensions(image_2d, 1);
    let dim_2d_array = textureDimensions(image_2d_array);
    let dim_2d_array_lod = textureDimensions(image_2d_array, 1);
    let dim_cube = textureDimensions(image_cube);
    let dim_cube_lod = textureDimensions(image_cube, 1);
    let dim_cube_array = textureDimensions(image_cube_array);
    let dim_cube_array_lod = textureDimensions(image_cube_array, 1);
    let dim_3d = textureDimensions(image_3d);
    let dim_3d_lod = textureDimensions(image_3d, 1);
    let dim_2s_ms = textureDimensions(image_aa);

    let sum = dim_1d + dim_2d.y + dim_2d_lod.y + dim_2d_array.y + dim_2d_array_lod.y + 
        dim_cube.y + dim_cube_lod.y + dim_cube_array.y + dim_cube_array_lod.y +
        dim_3d.z + dim_3d_lod.z;
    return vec4<f32>(f32(sum));
}

@vertex
fn levels_queries() -> @builtin(position) vec4<f32> {
    let num_levels_2d = textureNumLevels(image_2d);
    let num_levels_2d_array = textureNumLevels(image_2d_array);
    let num_layers_2d = textureNumLayers(image_2d_array);
    let num_levels_cube = textureNumLevels(image_cube);
    let num_levels_cube_array = textureNumLevels(image_cube_array);
    let num_layers_cube = textureNumLayers(image_cube_array);
    let num_levels_3d = textureNumLevels(image_3d);
    let num_samples_aa = textureNumSamples(image_aa);

    let sum = num_layers_2d + num_layers_cube + num_samples_aa +
        num_levels_2d + num_levels_2d_array + num_levels_3d + num_levels_cube + num_levels_cube_array;
    return vec4<f32>(f32(sum));
}

@group(1) @binding(0)
var sampler_reg: sampler;

@fragment
fn texture_sample() -> @location(0) vec4<f32> {
    let tc = vec2<f32>(0.5);
    let tc3 = vec3<f32>(0.5);
    let level = 2.3;
    var a: vec4<f32>;
    a += textureSample(image_1d, sampler_reg, tc.x);
    a += textureSample(image_2d, sampler_reg, tc);
    a += textureSample(image_2d, sampler_reg, tc, vec2<i32>(3, 1));
    a += textureSampleLevel(image_2d, sampler_reg, tc, level);
    a += textureSampleLevel(image_2d, sampler_reg, tc, level, vec2<i32>(3, 1));
    a += textureSampleBias(image_2d, sampler_reg, tc, 2.0, vec2<i32>(3, 1));
    a += textureSample(image_2d_array, sampler_reg, tc, 0u);
    a += textureSample(image_2d_array, sampler_reg, tc, 0u, vec2<i32>(3, 1));
    a += textureSampleLevel(image_2d_array, sampler_reg, tc, 0u, level);
    a += textureSampleLevel(image_2d_array, sampler_reg, tc, 0u, level, vec2<i32>(3, 1));
    a += textureSampleBias(image_2d_array, sampler_reg, tc, 0u, 2.0, vec2<i32>(3, 1));
    a += textureSample(image_2d_array, sampler_reg, tc, 0);
    a += textureSample(image_2d_array, sampler_reg, tc, 0, vec2<i32>(3, 1));
    a += textureSampleLevel(image_2d_array, sampler_reg, tc, 0, level);
    a += textureSampleLevel(image_2d_array, sampler_reg, tc, 0, level, vec2<i32>(3, 1));
    a += textureSampleBias(image_2d_array, sampler_reg, tc, 0, 2.0, vec2<i32>(3, 1));
    a += textureSample(image_cube_array, sampler_reg, tc3, 0u);
    a += textureSampleLevel(image_cube_array, sampler_reg, tc3, 0u, level);
    a += textureSampleBias(image_cube_array, sampler_reg, tc3, 0u, 2.0);
    a += textureSample(image_cube_array, sampler_reg, tc3, 0);
    a += textureSampleLevel(image_cube_array, sampler_reg, tc3, 0, level);
    a += textureSampleBias(image_cube_array, sampler_reg, tc3, 0, 2.0);
    return a;
}

@group(1) @binding(1)
var sampler_cmp: sampler_comparison;
@group(1) @binding(2)
var image_2d_depth: texture_depth_2d;
@group(1) @binding(3)
var image_2d_array_depth: texture_depth_2d_array;
@group(1) @binding(4)
var image_cube_depth: texture_depth_cube;

@fragment
fn texture_sample_comparison() -> @location(0) f32 {
    let tc = vec2<f32>(0.5);
    let tc3 = vec3<f32>(0.5);
    let dref = 0.5;
    var a: f32;
    a += textureSampleCompare(image_2d_depth, sampler_cmp, tc, dref);
    a += textureSampleCompare(image_2d_array_depth, sampler_cmp, tc, 0u, dref);
    a += textureSampleCompare(image_2d_array_depth, sampler_cmp, tc, 0, dref);
    a += textureSampleCompare(image_cube_depth, sampler_cmp, tc3, dref);
    a += textureSampleCompareLevel(image_2d_depth, sampler_cmp, tc, dref);
    a += textureSampleCompareLevel(image_2d_array_depth, sampler_cmp, tc, 0u, dref);
    a += textureSampleCompareLevel(image_2d_array_depth, sampler_cmp, tc, 0, dref);
    a += textureSampleCompareLevel(image_cube_depth, sampler_cmp, tc3, dref);
    return a;
}

@fragment
fn gather() -> @location(0) vec4<f32> {
    let tc = vec2<f32>(0.5);
    let dref = 0.5;
    let s2d = textureGather(1, image_2d, sampler_reg, tc);
    let s2d_offset = textureGather(3, image_2d, sampler_reg, tc, vec2<i32>(3, 1));
    let s2d_depth = textureGatherCompare(image_2d_depth, sampler_cmp, tc, dref);
    let s2d_depth_offset = textureGatherCompare(image_2d_depth, sampler_cmp, tc, dref, vec2<i32>(3, 1));

    let u = textureGather(0, image_2d_u32, sampler_reg, tc);
    let i = textureGather(0, image_2d_i32, sampler_reg, tc);
    let f = vec4<f32>(u) + vec4<f32>(i);

    return s2d + s2d_offset + s2d_depth + s2d_depth_offset + f;
}

@fragment
fn depth_no_comparison() -> @location(0) vec4<f32> {
    let tc = vec2<f32>(0.5);
    let s2d = textureSample(image_2d_depth, sampler_reg, tc);
    let s2d_gather = textureGather(image_2d_depth, sampler_reg, tc);
    return s2d + s2d_gather;
}
// Testing various parts of the pipeline interface: locations, built-ins, and entry points

struct VertexOutput {
    @builtin(position) @invariant position: vec4<f32>,
    @location(1) _varying: f32,
}

@vertex
fn vertex(
    @builtin(vertex_index) vertex_index: u32,
    @builtin(instance_index) instance_index: u32,
    @location(10) color: u32,
) -> VertexOutput {
    let tmp = vertex_index + instance_index + color;
    return VertexOutput(vec4<f32>(1.0), f32(tmp));
}

struct FragmentOutput {
    @builtin(frag_depth) depth: f32,
    @builtin(sample_mask) sample_mask: u32,
    @location(0) color: f32,
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) front_facing: bool,
    @builtin(sample_index) sample_index: u32,
    @builtin(sample_mask) sample_mask: u32,
) -> FragmentOutput {
    let mask = sample_mask & (1u << sample_index);
    let color = select(0.0, 1.0, front_facing);
    return FragmentOutput(in._varying, mask, color);
}

var<workgroup> output: array<u32, 1>;

@compute @workgroup_size(1)
fn compute(
    @builtin(global_invocation_id) global_id: vec3<u32>,
    @builtin(local_invocation_id) local_id: vec3<u32>,
    @builtin(local_invocation_index) local_index: u32,
    @builtin(workgroup_id) wg_id: vec3<u32>,
    @builtin(num_workgroups) num_wgs: vec3<u32>,
) {
    output[0] = global_id.x + local_id.x + local_index + wg_id.x + num_wgs.x;
}

struct Input1 {
    @builtin(vertex_index) index: u32,
}

struct Input2 {
    @builtin(instance_index) index: u32,
}

@vertex
fn vertex_two_structs(in1: Input1, in2: Input2) -> @builtin(position) @invariant vec4<f32> {
    var index = 2u;
    return vec4<f32>(f32(in1.index), f32(in2.index), f32(index), 0.0);
}
//TODO: merge with "interface"?

struct FragmentInput {
  @builtin(position) position: vec4<f32>,
  @location(0) @interpolate(flat) _flat : u32,
  @location(1) @interpolate(linear) _linear : f32,
  @location(2) @interpolate(linear, centroid) linear_centroid : vec2<f32>,
  @location(3) @interpolate(linear, sample) linear_sample : vec3<f32>,
  @location(4) @interpolate(perspective) perspective : vec4<f32>,
  @location(5) @interpolate(perspective, centroid) perspective_centroid : f32,
  @location(6) @interpolate(perspective, sample) perspective_sample : f32,
}

@vertex
fn vert_main() -> FragmentInput {
   var out: FragmentInput;

   out.position = vec4<f32>(2.0, 4.0, 5.0, 6.0);
   out._flat = 8u;
   out._linear = 27.0;
   out.linear_centroid = vec2<f32>(64.0, 125.0);
   out.linear_sample = vec3<f32>(216.0, 343.0, 512.0);
   out.perspective = vec4<f32>(729.0, 1000.0, 1331.0, 1728.0);
   out.perspective_centroid = 2197.0;
   out.perspective_sample = 2744.0;

   return out;
}

@fragment
fn frag_main(val : FragmentInput) { }
@vertex
fn vs() -> @builtin(position) @invariant vec4<f32> {
    return vec4<f32>(0.0);
}

@fragment
fn fs(@builtin(position) @invariant position: vec4<f32>) { }
fn blockLexicalScope(a: bool) {
    {
        let a = 2;
        {
            let a = 2.0;
        }
        let test: i32 = a;
    }
    let test: bool = a;
}

fn ifLexicalScope(a: bool) {
    if (a) {
        let a = 2.0;
    }
    let test: bool = a;
}


fn loopLexicalScope(a: bool) {
    loop {
        let a = 2.0;
    }
    let test: bool = a;
}

fn forLexicalScope(a: f32) {
    for (var a = 0; a < 1; a++) {
        let a = true;
    }
    let test: f32 = a;
}

fn whileLexicalScope(a: i32) {
    while (a > 2) {
        let a = false;
    }
    let test: i32 = a;
}

fn switchLexicalScope(a: i32) {
    switch (a) {
        case 0 {
            let a = false;
        }
        case 1 {
            let a = 2.0;
        }
        default {
            let a = true;
        }
    }
    let test = a == 2;
}
@fragment
fn main() {
    let f = 1.0;
    let v = vec4<f32>(0.0);
    let a = degrees(f);
    let b = radians(f);
    let c = degrees(v);
    let d = radians(v);
    let e = saturate(v);
    let g = refract(v, v, f);
    let sign_a = sign(-1);
    let sign_b = sign(vec4(-1));
    let sign_c = sign(-1.0);
    let sign_d = sign(vec4(-1.0));
    let const_dot = dot(vec2<i32>(), vec2<i32>());
    let first_leading_bit_abs = firstLeadingBit(abs(0u));
    let flb_a = firstLeadingBit(-1);
    let flb_b = firstLeadingBit(vec2(-1));
    let flb_c = firstLeadingBit(vec2(1u));
    let ftb_a = firstTrailingBit(-1);
    let ftb_b = firstTrailingBit(1u);
    let ftb_c = firstTrailingBit(vec2(-1));
    let ftb_d = firstTrailingBit(vec2(1u));
    let ctz_a = countTrailingZeros(0u);
    let ctz_b = countTrailingZeros(0);
    let ctz_c = countTrailingZeros(0xFFFFFFFFu);
    let ctz_d = countTrailingZeros(-1);
    let ctz_e = countTrailingZeros(vec2(0u));
    let ctz_f = countTrailingZeros(vec2(0));
    let ctz_g = countTrailingZeros(vec2(1u));
    let ctz_h = countTrailingZeros(vec2(1));
    let clz_a = countLeadingZeros(-1);
    let clz_b = countLeadingZeros(1u);
    let clz_c = countLeadingZeros(vec2(-1));
    let clz_d = countLeadingZeros(vec2(1u));
    let lde_a = ldexp(1.0, 2);
    let lde_b = ldexp(vec2(1.0, 2.0), vec2(3, 4));
    let modf_a = modf(1.5);
    let modf_b = modf(1.5).fract;
    let modf_c = modf(1.5).whole;
    let modf_d = modf(vec2(1.5, 1.5));
    let modf_e = modf(vec4(1.5, 1.5, 1.5, 1.5)).whole.x;
    let modf_f: f32 = modf(vec2(1.5, 1.5)).fract.y;
    let frexp_a = frexp(1.5);
    let frexp_b = frexp(1.5).fract;
    let frexp_c: i32 = frexp(1.5).exp;
    let frexp_d: i32 = frexp(vec4(1.5, 1.5, 1.5, 1.5)).exp.x;
}
fn call() {
    statement();
    let x: S = returns();
    let vf = f32(Value);
    let s = textureSample(Texture, Sampler, Vec2(vf));
}

fn statement() {}

fn returns() -> S {
    return S(Value);
}

struct S {
    x: i32,
}

const Value: i32 = 1;

@group(0) @binding(0)
var Texture: texture_2d<f32>;

@group(0) @binding(1)
var Sampler: sampler;

alias Vec2 = vec2<f32>;
struct Vertex {
    @location(0) position: vec2f
}

struct NoteInstance {
    @location(1) position: vec2f
}

struct VertexOutput {
    @builtin(position) position: vec4f
}

@vertex
fn vs_main(vertex: Vertex, note: NoteInstance) -> VertexOutput {
    var out: VertexOutput;
    return out;
}

@fragment
fn fs_main(in: VertexOutput, note: NoteInstance) -> @location(0) vec4f {
    let position = vec3(1f);
    return in.position;
}
@fragment
fn main(@builtin(view_index) view_index: i32) {}
@fragment
fn main(@builtin(view_index) view_index: i32) {}
const v_f32_one = vec4<f32>(1.0, 1.0, 1.0, 1.0);
const v_f32_zero = vec4<f32>(0.0, 0.0, 0.0, 0.0);
const v_f32_half = vec4<f32>(0.5, 0.5, 0.5, 0.5);
const v_i32_one = vec4<i32>(1, 1, 1, 1);

fn builtins() -> vec4<f32> {
    // select()
    let condition = true;
    let s1 = select(0, 1, condition);
    let s2 = select(v_f32_zero, v_f32_one, condition);
    let s3 = select(v_f32_one, v_f32_zero, vec4<bool>(false, false, false, false));
    // mix()
    let m1 = mix(v_f32_zero, v_f32_one, v_f32_half);
    let m2 = mix(v_f32_zero, v_f32_one, 0.1);
    // bitcast()
    let b1 = bitcast<f32>(v_i32_one.x);
    let b2 = bitcast<vec4<f32>>(v_i32_one);
    // convert
    let v_i32_zero = vec4<i32>(v_f32_zero);
    // done
    return vec4<f32>(vec4<i32>(s1) + v_i32_zero) + s2 + m1 + m2 + b1 + b2;
}

fn splat() -> vec4<f32> {
    let a = (1.0 + vec2<f32>(2.0) - 3.0) / 4.0;
    let b = vec4<i32>(5) % 2;
    return a.xyxy + vec4<f32>(b);
}

fn splat_assignment() -> vec2<f32> {
    var a = vec2<f32>(2.0);
    a += 1.0;
    a -= 3.0;
    a /= 4.0;
    return a;
}

fn bool_cast(x: vec3<f32>) -> vec3<f32> {
    let y = vec3<bool>(x);
    return vec3<f32>(y);
}

fn logical() {
    let t = true;
    let f = false;

    // unary
    let neg0 = !t;
    let neg1 = !vec2(t);

    // binary
    let or = t || f;
    let and = t && f;
    let bitwise_or0 = t | f;
    let bitwise_or1 = vec3(t) | vec3(f);
    let bitwise_and0 = t & f;
    let bitwise_and1 = vec4(t) & vec4(f);
}

fn arithmetic() {
    let one_i = 1i;
    let one_u = 1u;
    let one_f = 1.0;
    let two_i = 2i;
    let two_u = 2u;
    let two_f = 2.0;

    // unary
    let neg0 = -one_f;
    let neg1 = -vec2(one_i);
    let neg2 = -vec2(one_f);

    // binary
    // Addition
    let add0 = two_i + one_i;
    let add1 = two_u + one_u;
    let add2 = two_f + one_f;
    let add3 = vec2(two_i) + vec2(one_i);
    let add4 = vec3(two_u) + vec3(one_u);
    let add5 = vec4(two_f) + vec4(one_f);

    // Subtraction
    let sub0 = two_i - one_i;
    let sub1 = two_u - one_u;
    let sub2 = two_f - one_f;
    let sub3 = vec2(two_i) - vec2(one_i);
    let sub4 = vec3(two_u) - vec3(one_u);
    let sub5 = vec4(two_f) - vec4(one_f);

    // Multiplication
    let mul0 = two_i * one_i;
    let mul1 = two_u * one_u;
    let mul2 = two_f * one_f;
    let mul3 = vec2(two_i) * vec2(one_i);
    let mul4 = vec3(two_u) * vec3(one_u);
    let mul5 = vec4(two_f) * vec4(one_f);

    // Division
    let div0 = two_i / one_i;
    let div1 = two_u / one_u;
    let div2 = two_f / one_f;
    let div3 = vec2(two_i) / vec2(one_i);
    let div4 = vec3(two_u) / vec3(one_u);
    let div5 = vec4(two_f) / vec4(one_f);

    // Remainder
    let rem0 = two_i % one_i;
    let rem1 = two_u % one_u;
    let rem2 = two_f % one_f;
    let rem3 = vec2(two_i) % vec2(one_i);
    let rem4 = vec3(two_u) % vec3(one_u);
    let rem5 = vec4(two_f) % vec4(one_f);

    // Binary arithmetic expressions with mixed scalar and vector operands
    {
        let add0 = vec2(two_i) + one_i;
        let add1 = two_i + vec2(one_i);
        let add2 = vec2(two_u) + one_u;
        let add3 = two_u + vec2(one_u);
        let add4 = vec2(two_f) + one_f;
        let add5 = two_f + vec2(one_f);

        let sub0 = vec2(two_i) - one_i;
        let sub1 = two_i - vec2(one_i);
        let sub2 = vec2(two_u) - one_u;
        let sub3 = two_u - vec2(one_u);
        let sub4 = vec2(two_f) - one_f;
        let sub5 = two_f - vec2(one_f);

        let mul0 = vec2(two_i) * one_i;
        let mul1 = two_i * vec2(one_i);
        let mul2 = vec2(two_u) * one_u;
        let mul3 = two_u * vec2(one_u);
        let mul4 = vec2(two_f) * one_f;
        let mul5 = two_f * vec2(one_f);

        let div0 = vec2(two_i) / one_i;
        let div1 = two_i / vec2(one_i);
        let div2 = vec2(two_u) / one_u;
        let div3 = two_u / vec2(one_u);
        let div4 = vec2(two_f) / one_f;
        let div5 = two_f / vec2(one_f);

        let rem0 = vec2(two_i) % one_i;
        let rem1 = two_i % vec2(one_i);
        let rem2 = vec2(two_u) % one_u;
        let rem3 = two_u % vec2(one_u);
        let rem4 = vec2(two_f) % one_f;
        let rem5 = two_f % vec2(one_f);
    }

    // Matrix arithmetic
    let add = mat3x3<f32>() + mat3x3<f32>();
    let sub = mat3x3<f32>() - mat3x3<f32>();

    let mul_scalar0 = mat3x3<f32>() * one_f;
    let mul_scalar1 = two_f * mat3x3<f32>();

    let mul_vector0 = mat4x3<f32>() * vec4(one_f);
    let mul_vector1 = vec3f(two_f) * mat4x3f();

    let mul = mat4x3<f32>() * mat3x4<f32>();
}

fn bit() {
    let one_i = 1i;
    let one_u = 1u;
    let two_i = 2i;
    let two_u = 2u;

    // unary
    let flip0 = ~one_i;
    let flip1 = ~one_u;
    let flip2 = ~vec2(one_i);
    let flip3 = ~vec3(one_u);

    // binary
    let or0 = two_i | one_i;
    let or1 = two_u | one_u;
    let or2 = vec2(two_i) | vec2(one_i);
    let or3 = vec3(two_u) | vec3(one_u);

    let and0 = two_i & one_i;
    let and1 = two_u & one_u;
    let and2 = vec2(two_i) & vec2(one_i);
    let and3 = vec3(two_u) & vec3(one_u);

    let xor0 = two_i ^ one_i;
    let xor1 = two_u ^ one_u;
    let xor2 = vec2(two_i) ^ vec2(one_i);
    let xor3 = vec3(two_u) ^ vec3(one_u);

    let shl0 = two_i << one_u;
    let shl1 = two_u << one_u;
    let shl2 = vec2(two_i) << vec2(one_u);
    let shl3 = vec3(two_u) << vec3(one_u);

    let shr0 = two_i >> one_u;
    let shr1 = two_u >> one_u;
    let shr2 = vec2(two_i) >> vec2(one_u);
    let shr3 = vec3(two_u) >> vec3(one_u);
}

fn comparison() {
    let one_i = 1i;
    let one_u = 1u;
    let one_f = 1.0;
    let two_i = 2i;
    let two_u = 2u;
    let two_f = 2.0;

    let eq0 = two_i == one_i;
    let eq1 = two_u == one_u;
    let eq2 = two_f == one_f;
    let eq3 = vec2(two_i) == vec2(one_i);
    let eq4 = vec3(two_u) == vec3(one_u);
    let eq5 = vec4(two_f) == vec4(one_f);

    let neq0 = two_i != one_i;
    let neq1 = two_u != one_u;
    let neq2 = two_f != one_f;
    let neq3 = vec2(two_i) != vec2(one_i);
    let neq4 = vec3(two_u) != vec3(one_u);
    let neq5 = vec4(two_f) != vec4(one_f);

    let lt0 = two_i < one_i;
    let lt1 = two_u < one_u;
    let lt2 = two_f < one_f;
    let lt3 = vec2(two_i) < vec2(one_i);
    let lt4 = vec3(two_u) < vec3(one_u);
    let lt5 = vec4(two_f) < vec4(one_f);

    let lte0 = two_i <= one_i;
    let lte1 = two_u <= one_u;
    let lte2 = two_f <= one_f;
    let lte3 = vec2(two_i) <= vec2(one_i);
    let lte4 = vec3(two_u) <= vec3(one_u);
    let lte5 = vec4(two_f) <= vec4(one_f);

    let gt0 = two_i > one_i;
    let gt1 = two_u > one_u;
    let gt2 = two_f > one_f;
    let gt3 = vec2(two_i) > vec2(one_i);
    let gt4 = vec3(two_u) > vec3(one_u);
    let gt5 = vec4(two_f) > vec4(one_f);

    let gte0 = two_i >= one_i;
    let gte1 = two_u >= one_u;
    let gte2 = two_f >= one_f;
    let gte3 = vec2(two_i) >= vec2(one_i);
    let gte4 = vec3(two_u) >= vec3(one_u);
    let gte5 = vec4(two_f) >= vec4(one_f);
}

fn assignment() {
    let zero_i = 0i;
    let one_i = 1i;
    let one_u = 1u;
    let two_u = 2u;

    var a = one_i;

    a += one_i;
    a -= one_i;
    a *= a;
    a /= a;
    a %= one_i;
    a &= zero_i;
    a |= zero_i;
    a ^= zero_i;
    a <<= two_u;
    a >>= one_u;

    a++;
    a--;

    var vec0: vec3<i32> = vec3<i32>();
    vec0[one_i]++;
    vec0[one_i]--;
}

@compute @workgroup_size(1)
fn main() {
    builtins();
    splat();
    bool_cast(v_f32_one.xyz);

    logical();
    arithmetic();
    bit();
    comparison();
    assignment();
}

fn negation_avoids_prefix_decrement() {
    let x = 1;
    let p0 = -x;
    let p1 = - -x;
    let p2 = -(-x);
    let p3 = -(- x);
    let p4 = - - -x;
    let p5 = - - - - x;
    let p6 = - - -(- -x);
    let p7 = (- - - - -x);
}
struct S {
    a: vec3<f32>,
}

struct Test {
    a: S,
    b: f32, // offset: 16
}

struct Test2 {
    a: array<vec3<f32>, 2>,
    b: f32, // offset: 32
}

struct Test3 {
    a: mat4x3<f32>,
    b: f32, // offset: 64
}

@group(0) @binding(0)
var<uniform> input1: Test;

@group(0) @binding(1)
var<uniform> input2: Test2;

@group(0) @binding(2)
var<uniform> input3: Test3;


@vertex
fn vertex() -> @builtin(position) vec4<f32> {
    return vec4<f32>(1.0) * input1.b * input2.b * input3.b;
}
fn f() {
   var v: vec2<i32>;
   let px = &v.x;
   *px = 10;
}

struct DynamicArray {
    arr: array<u32>
}

@group(0) @binding(0)
var<storage, read_write> dynamic_array: DynamicArray;

fn index_unsized(i: i32, v: u32) {
   let p: ptr<storage, DynamicArray, read_write> = &dynamic_array;

   let val = (*p).arr[i];
   (*p).arr[i] = val + v;
}

fn index_dynamic_array(i: i32, v: u32) {
   let p: ptr<storage, array<u32>, read_write> = &dynamic_array.arr;

   let val = (*p)[i];
   (*p)[i] = val + v;
}
// Tests that the index, buffer, and texture bounds checks policies are
// implemented separately.

// Storage and Uniform storage classes
struct InStorage {
  a: array<vec4<f32>, 10>
}
@group(0) @binding(0) var<storage> in_storage: InStorage;

struct InUniform {
  a: array<vec4<f32>, 20>
}
@group(0) @binding(1) var<uniform> in_uniform: InUniform;

// Textures automatically land in the `handle` storage class.
@group(0) @binding(2) var image_2d_array: texture_2d_array<f32>;

// None of the above.
var<workgroup> in_workgroup: array<f32, 30>;
var<private> in_private: array<f32, 40>;

fn mock_function(c: vec2<i32>, i: i32, l: i32) -> vec4<f32> {
  var in_function: array<vec4<f32>, 2> =
    array<vec4<f32>, 2>(vec4<f32>(0.707, 0.0, 0.0, 1.0),
                        vec4<f32>(0.0, 0.707, 0.0, 1.0));

  return (in_storage.a[i] +
          in_uniform.a[i] +
          textureLoad(image_2d_array, c, i, l) +
          in_workgroup[i] +
          in_private[i] +
          in_function[i]);
}
struct PushConstants {
    multiplier: f32
}
var<push_constant> pc: PushConstants;

struct FragmentIn {
    @location(0) color: vec4<f32>
}

@vertex
fn vert_main(
  @location(0) pos : vec2<f32>,
  @builtin(vertex_index) vi: u32,
) -> @builtin(position) vec4<f32> {
    return vec4<f32>(f32(vi) * pc.multiplier * pos, 0.0, 1.0);
}

@fragment
fn main(in: FragmentIn) -> @location(0) vec4<f32> {
    return in.color * pc.multiplier;
}
// vertex
const c_scale: f32 = 1.2;

struct VertexOutput {
  @location(0) uv : vec2<f32>,
  @builtin(position) position : vec4<f32>,
}

@vertex
fn vert_main(
  @location(0) pos : vec2<f32>,
  @location(1) uv : vec2<f32>,
) -> VertexOutput {
  return VertexOutput(uv, vec4<f32>(c_scale * pos, 0.0, 1.0));
}

// fragment
@group(0) @binding(0) var u_texture : texture_2d<f32>;
@group(0) @binding(1) var u_sampler : sampler;

@fragment
fn frag_main(@location(0) uv : vec2<f32>) -> @location(0) vec4<f32> {
  let color = textureSample(u_texture, u_sampler, uv);
  if color.a == 0.0 {
    discard;
  }
  // forcing the expression here to be emitted in order to check the
  // uniformity of the control flow a bit more strongly.
  let premultiplied = color.a * color;
  return premultiplied;
}


// We need to make sure that backends are successfully handling multiple entry points for the same shader stage.
@fragment
fn fs_extra() -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.5, 0.0, 0.5);
}
@group(0) @binding(0)
var acc_struct: acceleration_structure;

/*
let RAY_FLAG_NONE = 0x00u;
let RAY_FLAG_OPAQUE = 0x01u;
let RAY_FLAG_NO_OPAQUE = 0x02u;
let RAY_FLAG_TERMINATE_ON_FIRST_HIT = 0x04u;
let RAY_FLAG_SKIP_CLOSEST_HIT_SHADER = 0x08u;
let RAY_FLAG_CULL_BACK_FACING = 0x10u;
let RAY_FLAG_CULL_FRONT_FACING = 0x20u;
let RAY_FLAG_CULL_OPAQUE = 0x40u;
let RAY_FLAG_CULL_NO_OPAQUE = 0x80u;
let RAY_FLAG_SKIP_TRIANGLES = 0x100u;
let RAY_FLAG_SKIP_AABBS = 0x200u;

let RAY_QUERY_INTERSECTION_NONE = 0u;
let RAY_QUERY_INTERSECTION_TRIANGLE = 1u;
let RAY_QUERY_INTERSECTION_GENERATED = 2u;
let RAY_QUERY_INTERSECTION_AABB = 4u;

struct RayDesc {
    flags: u32,
    cull_mask: u32,
    t_min: f32,
    t_max: f32,
    origin: vec3<f32>,
    dir: vec3<f32>,
}

struct RayIntersection {
    kind: u32,
    t: f32,
    instance_custom_index: u32,
    instance_id: u32,
    sbt_record_offset: u32,
    geometry_index: u32,
    primitive_index: u32,
    barycentrics: vec2<f32>,
    front_face: bool,
    object_to_world: mat4x3<f32>,
    world_to_object: mat4x3<f32>,
}
*/

struct Output {
    visible: u32,
    normal: vec3<f32>,
}

@group(0) @binding(1)
var<storage, read_write> output: Output;

fn get_torus_normal(world_point: vec3<f32>, intersection: RayIntersection) -> vec3<f32> {
    let local_point = intersection.world_to_object * vec4<f32>(world_point, 1.0);
    let point_on_guiding_line = normalize(local_point.xy) * 2.4;
    let world_point_on_guiding_line = intersection.object_to_world * vec4<f32>(point_on_guiding_line, 0.0, 1.0);
    return normalize(world_point - world_point_on_guiding_line);
}

@compute @workgroup_size(1)
fn main() {
    var rq: ray_query;

    let dir = vec3<f32>(0.0, 1.0, 0.0);
    rayQueryInitialize(&rq, acc_struct, RayDesc(RAY_FLAG_TERMINATE_ON_FIRST_HIT, 0xFFu, 0.1, 100.0, vec3<f32>(0.0), dir));

    while (rayQueryProceed(&rq)) {}

    let intersection = rayQueryGetCommittedIntersection(&rq);
    output.visible = u32(intersection.kind == RAY_QUERY_INTERSECTION_NONE);
    output.normal = get_torus_normal(dir * intersection.t, intersection);
}
@group(0) @binding(0) var t1: texture_2d<f32>;
@group(0) @binding(1) var t2: texture_2d<f32>;
@group(0) @binding(2) var s1: sampler;
@group(0) @binding(3) var s2: sampler;

@group(0) @binding(4) var<uniform> uniformOne: vec2<f32>;
@group(1) @binding(0) var<uniform> uniformTwo: vec2<f32>;

@fragment
fn entry_point_one(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    return textureSample(t1, s1, pos.xy);
}

@fragment
fn entry_point_two() -> @location(0) vec4<f32> {
    return textureSample(t1, s1, uniformOne);
}

@fragment
fn entry_point_three() -> @location(0) vec4<f32> {
    return textureSample(t1, s1, uniformTwo + uniformOne) +
           textureSample(t2, s2, uniformOne);
}
struct DataStruct {
    data: f32,
    data_vec: vec4<f32>,
}

struct Struct {
    data: array<DataStruct>,
};

struct PrimitiveStruct {
    data: array<f32>,
};
// only available in the fragment stage
fn derivatives() {
    let x = dpdx(0.0);
    let y = dpdy(0.0);
    let width = fwidth(0.0);
}

// only available in the compute stage
fn barriers() {
    storageBarrier();
    workgroupBarrier();
}

@fragment
fn fragment() -> @location(0) vec4<f32> {
    derivatives();
    return vec4<f32>();
}

@compute @workgroup_size(1)
fn compute() {
    barriers();
}struct Globals {
    view_proj: mat4x4<f32>,
    num_lights: vec4<u32>,
}

@group(0)
@binding(0)
var<uniform> u_globals: Globals;

struct Entity {
    world: mat4x4<f32>,
    color: vec4<f32>,
}

@group(1)
@binding(0)
var<uniform> u_entity: Entity;

/* Not useful for testing
@vertex
fn vs_bake(@location(0) position: vec4<i32>) -> @builtin(position) vec4<f32> {
    return u_globals.view_proj * u_entity.world * vec4<f32>(position);
}
*/

struct VertexOutput {
    @builtin(position) proj_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    @location(1) world_position: vec4<f32>,
}

@vertex
fn vs_main(
    @location(0) position: vec4<i32>,
    @location(1) normal: vec4<i32>,
) -> VertexOutput {
    let w = u_entity.world;
    let world_pos = u_entity.world * vec4<f32>(position);
    var out: VertexOutput;
    out.world_normal = mat3x3<f32>(w.x.xyz, w.y.xyz, w.z.xyz) * vec3<f32>(normal.xyz);
    out.world_position = world_pos;
    out.proj_position = u_globals.view_proj * world_pos;
    return out;
}

// fragment shader

struct Light {
    proj: mat4x4<f32>,
    pos: vec4<f32>,
    color: vec4<f32>,
}

@group(0)
@binding(1)
var<storage, read> s_lights: array<Light>;
@group(0)
@binding(1)
var<uniform> u_lights: array<Light, 10>; // Used when storage types are not supported
@group(0)
@binding(2)
var t_shadow: texture_depth_2d_array;
@group(0)
@binding(3)
var sampler_shadow: sampler_comparison;

fn fetch_shadow(light_id: u32, homogeneous_coords: vec4<f32>) -> f32 {
    if (homogeneous_coords.w <= 0.0) {
        return 1.0;
    }
    // compensate for the Y-flip difference between the NDC and texture coordinates
    let flip_correction = vec2<f32>(0.5, -0.5);
    // compute texture coordinates for shadow lookup
    let proj_correction = 1.0 / homogeneous_coords.w;
    let light_local = homogeneous_coords.xy * flip_correction * proj_correction + vec2<f32>(0.5, 0.5);
    // do the lookup, using HW PCF and comparison
    return textureSampleCompareLevel(t_shadow, sampler_shadow, light_local, i32(light_id), homogeneous_coords.z * proj_correction);
}

const c_ambient: vec3<f32> = vec3<f32>(0.05, 0.05, 0.05);
const c_max_lights: u32 = 10u;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(in.world_normal);
    // accumulate color
    var color: vec3<f32> = c_ambient;
    for(var i = 0u; i < min(u_globals.num_lights.x, c_max_lights); i++) {
        let light = s_lights[i];
        // project into the light space
        let shadow = fetch_shadow(i, light.proj * in.world_position);
        // compute Lambertian diffuse term
        let light_dir = normalize(light.pos.xyz - in.world_position.xyz);
        let diffuse = max(0.0, dot(normal, light_dir));
        // add light contribution
        color += shadow * diffuse * light.color.xyz;
    }
    // multiply the light by material color
    return vec4<f32>(color, 1.0) * u_entity.color;
}

// The fragment entrypoint used when storage buffers are not available for the lights
@fragment
fn fs_main_without_storage(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(in.world_normal);
    var color: vec3<f32> = c_ambient;
    for(var i = 0u; i < min(u_globals.num_lights.x, c_max_lights); i++) {
        // This line is the only difference from the entrypoint above. It uses the lights
        // uniform instead of the lights storage buffer
        let light = u_lights[i];
        let shadow = fetch_shadow(i, light.proj * in.world_position);
        let light_dir = normalize(light.pos.xyz - in.world_position.xyz);
        let diffuse = max(0.0, dot(normal, light_dir));
        color += shadow * diffuse * light.color.xyz;
    }
    return vec4<f32>(color, 1.0) * u_entity.color;
}
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec3<f32>,
}

struct Data {
    proj_inv: mat4x4<f32>,
    view: mat4x4<f32>,
}
@group(0) @binding(0)
var<uniform> r_data: Data;

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    // hacky way to draw a large triangle
    var tmp1 = i32(vertex_index) / 2;
    var tmp2 = i32(vertex_index) & 1;
    let pos = vec4<f32>(
        f32(tmp1) * 4.0 - 1.0,
        f32(tmp2) * 4.0 - 1.0,
        0.0,
        1.0,
    );

    let inv_model_view = transpose(mat3x3<f32>(r_data.view.x.xyz, r_data.view.y.xyz, r_data.view.z.xyz));
    let unprojected = r_data.proj_inv * pos;
    return VertexOutput(pos, inv_model_view * unprojected.xyz);
}

@group(0) @binding(1)
var r_texture: texture_cube<f32>;
@group(0) @binding(2)
var r_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(r_texture, r_sampler, in.uv);
}
@group(0) @binding(0) var u_texture : texture_2d<f32>;
@group(0) @binding(1) var u_sampler : sampler;

@fragment
fn main(@location(0) uv : vec2<f32>) -> @location(0) vec4<f32> {
  return textureSample(u_texture, u_sampler, uv);
}
// Standard functions.

fn test_any_and_all_for_bool() -> bool {
    let a = any(true);
    return all(a);
}


@fragment
fn derivatives(@builtin(position) foo: vec4<f32>) -> @location(0) vec4<f32> {
    var x = dpdxCoarse(foo);
    var y = dpdyCoarse(foo);
    var z = fwidthCoarse(foo);

    x = dpdxFine(foo);
    y = dpdyFine(foo);
    z = fwidthFine(foo);

    x = dpdx(foo);
    y = dpdy(foo);
    z = fwidth(foo);

    let a = test_any_and_all_for_bool();

    return (x + y) * z;
}
@group(0) @binding(0)
var Texture: texture_2d<f32>;
@group(0) @binding(1)
var Sampler: sampler;

fn test(Passed_Texture: texture_2d<f32>, Passed_Sampler: sampler) -> vec4<f32> {
    return textureSample(Passed_Texture, Passed_Sampler, vec2<f32>(0.0, 0.0));
}

@fragment
fn main() -> @location(0) vec4<f32> {
    return test(Texture, Sampler);
}
alias FVec3 = vec3<f32>;
alias IVec3 = vec3i;
alias Mat2 = mat2x2<f32>;
alias Mat3 = mat3x3f;

fn main() {
    let a = FVec3(0.0, 0.0, 0.0);
    let c = FVec3(0.0);
    let b = FVec3(vec2<f32>(0.0), 0.0);
    let d = FVec3(vec2<f32>(0.0), 0.0);
    let e = IVec3(d);

    let f = Mat2(1.0, 2.0, 3.0, 4.0);
    let g = Mat3(a, a, a);
}

const SIZE: u32 = 128u;

var<workgroup> arr_i32: array<i32, SIZE>;

@compute @workgroup_size(4)
fn test_workgroupUniformLoad(@builtin(workgroup_id) workgroup_id: vec3<u32>) {
    let x = &arr_i32[workgroup_id.x];
    let val = workgroupUniformLoad(x);
    if val > 10 {
        workgroupBarrier();
    }
}
struct WStruct {
    arr: array<u32, 512>,
    atom: atomic<i32>,
    atom_arr: array<array<atomic<i32>, 8>, 8>,
}

var<workgroup> w_mem: WStruct;

@group(0) @binding(0)
var<storage, read_write> output: array<u32, 512>;

@compute @workgroup_size(1)
fn main() {
    output = w_mem.arr;
}
