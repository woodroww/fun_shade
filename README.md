This is me trying to understand how to get a basic vertex and fragment shader to
work.
There is little documentation on what can be imported into the shaders from Bevy.


#### bevy example shaders in
bevy/assets/shaders

## 2D
Material2d trait

Material2dPlugin

MaterialMesh2dBundle

Material2dPipeline

### in wgsl code
```
#import bevy_sprite::mesh2d_types
#import bevy_sprite::mesh2d_view_bindings // contains the globals.time
```

#### shaders and code located in
bevy/crates/bevy_sprite/src/render

bevy/crates/bevy_sprite/src/mesh2d

## 3D
Material trait

MaterialPlugin

MaterialMeshBundle

MaterialPipeline

### in wgsl code
```
#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings // contains the globals.time
```

#### shaders and code located in
bevy/crates/bevy_pbr/src/render

