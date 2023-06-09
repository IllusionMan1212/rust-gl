- ui
    - [x] integrate imgui in the program
    - [x] basic imgui menus and stuff with default docking
    - [x] render scene to texture and display inside imgui window
    - [x] figure out what to put in the main menu bar
    - [ ] more settings to control the scene (arbitrary but i just need something on the todo)

- graphics
    - [x] better grid
    - [x] get the basic material properties ($clr.diffuse, $clr.ambient, etc..) from the mesh material and apply them to the shader
    - [ ] (FEAT) get the rest of the material properties
    - [ ] (FEAT) some models (.glb only i think) have textures under the materials but they don't have names and no heights (investigate)
    - [ ] (FEAT) other models have the textures as material properties with a "$tex." prefix for the property key
    - [x] default normals and tex coords incase they are missing (default to 0.0 for all)
    - [|] models with more than a single mesh have all their meshes loaded at 0.0, 0.0, 0.0 with a default rotation of 0.0. apply transforms to meshes
        - [x] some gltf models seem to have a weird transform applied to them (investigate) (possibly related to rotations, no idea if assimp is parsing them incorrectly or im doing something wrong)
        - [x] seems like the position and scale is applied correctly now, still need to properly extract the rotation from the transform matrix and apply that as well
        - [ ] (BUG?) some gltf models still don't have the correct transform applied to them (rotations ??) (made some progress by fixing rotations)
        - [ ] (CONFIRMED BUG) I know for a fact that there's some sign fuckery going on when applying transformations, because when importing ABeautifulGame.glb the white knights have their z axis negative when it's supposed to be positive (online gltf viewers show the z as negative but the mesh is placed correctly??)
    - [x] russimp panics when unwrapping an option that relates to materials when loading the ToyCar.glb/gltf model
    - [ ] (FEAT) normalize shininess value (seems to range anywhere from 0.0 to 500.0+) (read assimp docs)
    - [ ] (FEAT) parse nodes and show them in the UI instead of only showing meshes
    - [ ] (FEAT) when parsing a model, if the nodes have a parent list them under it, otherwise just throw the nodes as they are in the scene
    - [ ] (FEAT) .fbx (and others ??) that are exported from blender (or is this just how the format is?) have metadata that contains the proper axes for the model (up, front) and unit scale factors and other data
    - [x] when scaling a node that is the child of another node, it applies both a scale and a translation to the child node (apparently they are SUPPOSED to be like this)
        - [ ] (BUG?) scaling is currently done relative to 0.0, 0.0, 0.0, but in blender (and other programs?) it's relative to where the origin point of the mesh is
        - [ ] (BUG?) i think hierarchical transformations are still not being applied correctly
        - [ ] (BUG?) im also convinced rotations lose data when converted from a 3x3 matrix to euler angles (investigate)
        - [ ] (BUG?) current rotation implementation is supposed to only work for non-negative scale factors, a better implementation is here https://math.stackexchange.com/a/3554913
        - [ ] (BUG?) parent transformations need to be stored in each child's struct (or perhaps the nodes need to be linked lists so we can easily access the parent/child's properties if we need them) this is needed so when a parent changes transformation we can apply the parent's transformation as well
    - [ ] (FEAT) object selection with outline



    - (CONFIRMED BUG) the problem with incorrect positioning of meshes is related to rotations being applied around 0.0, 0.0, 0.0 instead of the mesh's origin point. if we fix this I'm 100% sure (at least) ABeautifulGame.glb will have it's knights positions fixed and the (maybe) Buggy.glb will have its meshes positions fixed (russimp doesn't seem like it can get me the origin points of the meshes tho)
