// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[macro_use]
extern crate objc;

use metal::*;

use cocoa::foundation::NSAutoreleasePool;

const PROGRAM: &'static str = "
    #include <metal_stdlib>\n\

    using namespace metal;\n\

    typedef struct {\n\
        float2 position;\n\
        float3 color;\n\
    } vertex_t;\n\

    struct ColorInOut {\n\
        float4 position [[position]];\n\
        float4 color;\n\
    };\n\

    vertex ColorInOut vs(device vertex_t* vertex_array [[ buffer(0) ]],\n\
                                      unsigned int vid [[ vertex_id ]])\n\
    {\n\
        ColorInOut out;\n\

        out.position = float4(float2(vertex_array[vid].position), 0.0, 1.0);\n\
        out.color = float4(float3(vertex_array[vid].color), 1.0);\n\

        return out;\n\
    }\n\

    fragment float4 ps(ColorInOut in [[stage_in]])\n\
    {\n\
        return in.color;\n\
    };\n\
";

fn main() {
    let pool = unsafe { NSAutoreleasePool::new(cocoa::base::nil) };

    let device = Device::system_default();

    let options = CompileOptions::new();
    let library = device.new_library_with_source(PROGRAM, &options).unwrap();
    let (vs, ps) = (
        library.get_function("vs", None).unwrap(),
        library.get_function("ps", None).unwrap(),
    );

    let vertex_desc = VertexDescriptor::new();

    let desc = RenderPipelineDescriptor::new();
    desc.set_vertex_function(Some(&vs));
    desc.set_fragment_function(Some(&ps));
    desc.set_vertex_descriptor(Some(vertex_desc));

    println!("{:?}", desc);

    #[cfg(features = "private")]
    let _reflection = unsafe {
        RenderPipelineReflection::new(
            desc.serialize_vertex_data(),
            desc.serialize_fragment_data(),
            vertex_desc.serialize_descriptor(),
            &device,
            0x8,
            0x0,
        )
    };

    unsafe {
        msg_send![pool, release];
    }
}
