#[macro_use]
extern crate rocket;

use rocket::response::content;
use rocket::form::Form;
use rocket::http::RawStr;
use rocket::Error;


#[get("/")]
fn index() -> &'static str {
    "Hello, Docker! Add 'worm' to the URL to play worm..."
}

#[get("/dirtyworm")]
fn dirtyworm() -> content::RawHtml<&'static str> {
    content::RawHtml(
        "<canvas id=\"dddc\" width=\"640\" height=\"480\">
            Dirty deeds done dirt cheap    
        </canvas>

        <script
            src=\"https://cdnjs.cloudflare.com/ajax/libs/gl-matrix/2.8.1/gl-matrix-min.js\"
            integrity=\"sha512-zhHQR0/H5SEBL3Wn6yYSaTTZej12z0hVZKOv3TwCUXT1z5qeqGcXJLLrbERYRScEDDpYIJhPC1fk31gqR783iQ==\"
            crossorigin=\"anonymous\"
        defer>
        </script>

        <script>
            /* 
            Create a seperate file for the two shader related functions below and the main function.
            Give it a good name, something like... webgl-demo.js
            */
            function init_shader_program(wgl, vs_source, fs_source) {
                const vertex_shader = load_shader(wgl, wgl.VERTEX_SHADER, vs_source);
                const fragment_shader = load_shader(wgl, wgl.FRAGMENT_SHADER, fs_source);
                const shader_program = wgl.createProgram();

                wgl.attachShader(shader_program, vertex_shader);
                wgl.attachShader(shader_program, fragment_shader);
                wgl.linkProgram(shader_program);

                if(!wgl.getProgramParameter(shader_program, wgl.LINK_STATUS)) {
                    alert(\"shit's fucked, mate... with the program\");
                    return null;
                }

                return shader_program;
            }

            function load_shader(wgl, type, source) {
                const shader = wgl.createShader(type);
                wgl.shaderSource(shader, source);
                wgl.compileShader(shader);

                if (!wgl.getShaderParameter(shader, wgl.COMPILE_STATUS)) {
                    alert(`shit's fucked, mate... with the shader: ${wgl.getShaderInfoLog(shader)}`);
                    wgl.deleteShader(shader);
                    return null;
                }

                return shader;
            }

            /*
            The following init functions should be placed in a seperate file, maybe init-buffers.js?
             */
            function init_buffers(wgl) {
                const position_buffer = init_position_buffer(wgl);
                const color_buffer = init_color_buffer(wgl);
                //const texture_coord_buffer = init_texture_buffer(wgl);
                const index_buffer = init_index_buffer(wgl);
                
                return {
                    position: position_buffer,
                    color: color_buffer,
                    //texture_coord: texture_coord_buffer,
                    indices: index_buffer,
                };
            }

            function init_position_buffer(wgl) {
                const position_buffer = wgl.createBuffer();
                wgl.bindBuffer(wgl.ARRAY_BUFFER, position_buffer);

                const positions = [
                    // Front face
                    -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
                    // Back face
                    -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0,
                    // Top face
                    -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0,
                    // Bottom face
                    -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0,
                    // Right face
                    1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0,
                    // Left face
                    -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0,
                ];
                wgl.bufferData(wgl.ARRAY_BUFFER, new Float32Array(positions), wgl.STATIC_DRAW);

                return position_buffer;
            }

            function init_color_buffer(wgl) {
                /*
                const colors = [
                    1.0,
                    1.0,
                    1.0,
                    1.0, // white
                    1.0,
                    0.0,
                    0.0,
                    1.0, // red
                    0.0,
                    1.0,
                    0.0,
                    1.0, // green
                    0.0,
                    0.0,
                    1.0,
                    1.0, // blue
                ];
                */

                const face_colors = [
                    [1.0, 1.0, 1.0, 1.0], // Front face: white
                    [1.0, 0.0, 0.0, 1.0], // Back face: red
                    [0.0, 1.0, 0.0, 1.0], // Top face: green
                    [0.0, 0.0, 1.0, 1.0], // Bottom face: blue
                    [1.0, 1.0, 0.0, 1.0], // Right face: yellow
                    [1.0, 0.0, 1.0, 1.0], // Left face: purple
                ];

                var colors = [];
                for (var j = 0; j<face_colors.length; ++j) {
                    const c = face_colors[j];
                    colors = colors.concat(c,c,c,c)
                }

                const color_buffer = wgl.createBuffer();
                wgl.bindBuffer(wgl.ARRAY_BUFFER, color_buffer);
                wgl.bufferData(wgl.ARRAY_BUFFER, new Float32Array(colors), wgl.STATIC_DRAW);

                return color_buffer
            }

            function init_index_buffer(wgl) {
                const index_buffer = wgl.createBuffer();
                wgl.bindBuffer(wgl.ELEMENT_ARRAY_BUFFER, index_buffer);

                const indices = [
                    0,1,2,
                    0,2,3, // Front
                    4,5,6,
                    4,6,7, // Back
                    8,9,10,
                    8,10,11, // Top
                    12,13,14,
                    12,14,15, // Bottom
                    16,17,18,
                    16,18,19, // Right
                    20,21,22,
                    20,22,23, // Left
                ];

                // Send the element array to WebGL
                wgl.bufferData(
                    wgl.ELEMENT_ARRAY_BUFFER,
                    new Uint16Array(indices),
                    wgl.STATIC_DRAW,
                );

                return index_buffer
            }

            function init_texture_buffer(wgl) {
                const texture_coord_buffer = wgl.createBuffer();
                wgl.bindBuffer(wgl.ARRAY_BUFFER, texture_coord_buffer);

                const texture_coordinates = [
                    // Front
                    0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
                    // Back
                    0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
                    // Top
                    0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
                    // Bottom
                    0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
                    // Right
                    0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
                    // Left
                    0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
                ];

                wgl.bufferData(
                    wgl.ARRAY_BUFFER,
                    new Float32Array(texture_coordinates),
                    wgl.STATIC_DRAW
                );

                return texture_coord_buffer;
            }

            /*
            The following three functions should be put into seperate file and exported.
            Now... what would be a good name... something like draw-scene.js!?
            If you ever want to try using textures again remember to add that parameter.
             */
            function draw_scene(wgl, program_info, buffers, cube_rotation) {
                wgl.clearColor(0.0, 0.0, 0.0, 1.0); // Clear to black, fully opaque
                wgl.clearDepth(1.0); // Clear everything
                wgl.enable(wgl.DEPTH_TEST);
                wgl.depthFunc(wgl.LEQUAL); // Near things obscure far things

                // Clear the canvas before drawing
                wgl.clear(wgl.COLOR_BUFFER_BIT | wgl.DEPTH_BUFFER_BIT);

                // Create a perspective matrix, a special matrix that is
                // used to simulate the distortion of perspective in a camera.
                // Our field of view is 45 degrees, with a width/height
                // ratio that matches the display size of the canvas
                // and we only want to see objects between 0.1 units
                // and 100 units away from the camera.
                const fov = (45 * Math.PI) / 100; // in radians
                const aspect = wgl.canvas.clientWidth / wgl.canvas.clientHeight;
                const z_near = 0.1;
                const z_far = 100.0;
                const projection_matrix = mat4.create();

                // note: glmatrix.js always has the first argument
                // as the destination to receive the result.
                mat4.perspective(projection_matrix, fov, aspect, z_near, z_far);

                // Set the drawing position to the <quote>identity<unquote> point, which is
                // the center of the scene.
                const model_view_matrix = mat4.create();

                // Move drawing position to where we start drawing the square
                mat4.translate(
                    model_view_matrix, // destination matrix
                    model_view_matrix, // matrix to translate
                    [-0.0, 0.0, -6.0],  // amount to translate
                );

                // Rotate around y axis
                mat4.rotate(
                    model_view_matrix, // destination matrix
                    model_view_matrix, // matrix to rotate
                    cube_rotation * 5, // amount to rotate in radians
                    [0,1,0], // axis to rotate around
                );

                // Rotate around z axis
                mat4.rotate(
                    model_view_matrix, // destination matrix
                    model_view_matrix, // matrix to rotate
                    cube_rotation * .2, // amount to rotate in radians
                    [0,0,1], // axis to rotate around
                );

                // Rotate around x axis
                mat4.rotate(
                    model_view_matrix, // destination matrix
                    model_view_matrix, // matrix to rotate
                    cube_rotation, // amount to rotate in radians
                    [1,0,0], // axis to rotate around
                );

                // Tell WebGL how to pull out the positions from the position
                // buffer into the vertexPosition attribute.
                set_position_attribute(wgl, buffers, program_info);

                // Tell WebGL to use the colors... at least they actually show up.
                set_color_attribute(wgl, buffers, program_info);

                // Tell WebGL to use texture... unfortunately textures don't want to play nice
                //set_texture_attribute(wgl, buffers, program_info);

                // Tell WebGL which indices to use to index the vertices
                wgl.bindBuffer(wgl.ELEMENT_ARRAY_BUFFER, buffers.indices);

                // Tell WebGL to use our program when drawing.
                wgl.useProgram(program_info.program);

                // Set the shader uniforms
                wgl.uniformMatrix4fv(
                    program_info.uniformLocations.projectionMatrix,
                    false,
                    projection_matrix,
                );

                wgl.uniformMatrix4fv(
                    program_info.uniformLocations.modelViewMatrix,
                    false,
                    model_view_matrix,
                );

                // Affect texture unit 0
                //wgl.activeTexture(wgl.TEXTURE0);

                // Bind texture to texture unit 0
                //wgl.bindTexture(wgl.TEXTURE_2D, texture);

                // Tell shader we bound the texture to texture unit 0
                //wgl.uniform1i(program_info.uniformLocations.uSampler, 0);

                {
                    /* 
                    const offset = 0;
                    const vertex_count = 4;
                    wgl.drawArrays(wgl.TRIANGLE_STRIP, offset, vertex_count);
                    */

                    const vertex_count = 36;
                    const type = wgl.UNSIGNED_SHORT;
                    const offset = 0;
                    wgl.drawElements(wgl.TRIANGLES, vertex_count, type, offset)
                }
            }

            function set_position_attribute(wgl, buffers, program_info) {
                const num_components = 3; // pull out 3 values per iteration
                const type = wgl.FLOAT; // the data in the buffer is 32bit floats
                const normalize = false; // don't normalize
                const stride = 0; // how many bytes to get from one set of alues to the next
                const offset = 0; // how many bytes inside the buffer to start from

                wgl.bindBuffer(wgl.ARRAY_BUFFER, buffers.position);
                wgl.vertexAttribPointer(
                    program_info.attribLocations.vertexPosition,
                    num_components,
                    type,
                    normalize,
                    stride,
                    offset,
                );

                wgl.enableVertexAttribArray(program_info.attribLocations.vertexPosition);
            }

            function set_color_attribute(wgl, buffers, program_info) {
                const num_components = 4;
                const type = wgl.FLOAT;
                const normalize = false;
                const stride = 0;
                const offset = 0;
                wgl.bindBuffer(wgl.ARRAY_BUFFER, buffers.color);
                wgl.vertexAttribPointer(
                    program_info.attribLocations.vertexColor,
                    num_components,
                    type,
                    normalize,
                    stride,
                    offset,
                );

                wgl.enableVertexAttribArray(program_info.attribLocations.vertexColor);
            }

            // WebGL needs to know how to pull texture coords from the buffer.
            // Will the following function help?!
            function set_texture_attribute(wgl, buffers, program_info) {
                const num = 2; // coords are composed of 2 values
                const type = wgl.FLOAT; // data in buffer is 32-bit float
                const normalize = false;
                const stride = 0; // bytes from one set to the next
                const offset = 0; // bytes inside buffer to start from

                wgl.bindBuffer(wgl.ARRAY_BUFFER, buffers.textureCoord);
                wgl.vertexAttribPointer(
                    program_info.attribLocations.textureCoord,
                    num,
                    type,
                    normalize,
                    stride,
                    offset,
                );

                wgl.enableVertexAttribArray(program_info.attribLocations.textureCoord);

            }

            // export {draw_scene}  /* Use this when we move to seperate files */

            /*
            Remember, main should be in a file with the shader  and texture functions, maybe webgl-demo.js...
             */
            function main() {
                const vsSource = `
                    attribute vec4 aVertexPosition;
                    attribute vec4 aVertexColor;
                    //attribute vec2 aTextureCoord;

                    uniform mat4 uModelViewMatrix;
                    uniform mat4 uProjectionMatrix;

                    varying lowp vec4 vColor;
                    //varying highp vec2 vTextureCoord

                    void main(void) {
                        gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
                        vColor = aVertexColor;
                        //vTextureCoord = aTextureCoord
                    }
                `;

                const fsSource = `
                    varying lowp vec4 vColor;
                    //varying highp vec2 vTextureCoord;

                    uniform sampler2D uSampler;
                    //out vec4 fragColor;

                    void main(void) {
                        gl_FragColor = vColor;
                        //fragColor = texture2D(uSampler, vTextureCoord);
                    }
                `;

                let cube_rotation = 0.0;
                let deltaTime = 0;
                
                const canvas = document.getElementById(\"dddc\");
                if(canvas.getContext) {
                    //const ctx = canvas.getContext(\"2d\");
                    const wgl = canvas.getContext(\"webgl\");
                    const shaderProgram = init_shader_program(wgl, vsSource, fsSource);
                    const programInfo = {
                        program: shaderProgram,
                        attribLocations: {
                            vertexPosition: wgl.getAttribLocation(shaderProgram, \"aVertexPosition\"),
                            vertexColor: wgl.getAttribLocation(shaderProgram, \"aVertexColor\"),
                            //textureCoord: wgl.getAttribLocation(shaderProgram, \"aTextureCoord\"),
                        },
                        uniformLocations: {
                            projectionMatrix: wgl.getUniformLocation(shaderProgram, \"uProjectionMatrix\"),
                            modelViewMatrix: wgl.getUniformLocation(shaderProgram, \"uModelViewMatrix\"),
                            uSampler: wgl.getUniformLocation(shaderProgram, \"uSampler\"),
                        }
                    }

                    // Call the routine that builds all the objects being drawn
                    const buffers = init_buffers(wgl);

                    // Load texture
                    //const texture = load_texture(wgl, \"cubetexture.png\");
                    // Flip image pixels to bottom-to-top order.  WebGL expects it that way.
                    //wgl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, true);

                    // Draw it, you fool!
                    let then = 0;
                    // Draw the scene repeatedly
                    function render(now) {
                        now *= 0.001; // convert to seconds
                        deltaTime = now - then;
                        then = now;

                        draw_scene(wgl, programInfo, buffers, cube_rotation);
                        cube_rotation += deltaTime;

                        requestAnimationFrame(render);
                    }
                    requestAnimationFrame(render);
                    draw_scene(wgl, programInfo, buffers);

                    //ctx.font = \"10px serif\";
                    //ctx.fillText(\"Dirty worm consumes\", 10, 50);
                    //wgl.clearColor(0.0, 0.0, 0.0, 1.0);
                    //wgl.clear(wgl.COLOR_BUFFER_BIT);
                }
            }

            // Initialize a texture and load an image.  When finished loading copy into the texture.
            function load_texture(wgl, url) {
                const texture = wgl.createTexture();
                wgl.bindTexture(wgl.TEXTURE_2D, texture);


                const level = 0;
                const internal_format = wgl.RGBA;
                const width = 1;
                const height = 1;
                const border = 0;
                const src_format = wgl.RGBA;
                const src_type = wgl.UNSIGNED_BYTE;
                const pixel = new Uint8Array([0,0,255,255]); //opaque blue
                wgl.texImage2D(
                    wgl.TEXTURE_2D,
                    level,
                    internal_format,
                    width,
                    height,
                    border,
                    src_format,
                    src_type,
                    pixel,
                );

                const image = new Image();
                image.onload = () => {
                    wgl.bindTexture(wgl.TEXTURE_2D, texture);
                    wgl.texImage2d(
                        wgl.TEXTURE_2D,
                        level,
                        internal_format,
                        src_format,
                        src_type,
                        image,
                    );

                    // Images that have both width and height values that are powers of 2 are treated differently than those that do not.
                    if(isPowerOf2(image.width) && isPowerOf2(image.height)) {
                        // Generate mips when it's a power of 2.
                        wgl.generateMipmap(wgl.TEXTURE_2D);
                    } else {
                        // Not a power of 2, turn off mips and set wrapping to clamp to edge.
                        wgl.texParameteri(wgl.TEXTURE_2D, wgl.TEXTURE_WRAP_S, wgl.CLAMP_TO_EDGE);
                        wgl.texParameteri(wgl.TEXTURE_2D, wgl.TEXTURE_WRAP_T, wgl.CLAMP_TO_EDGE);
                        wgl.texParameteri(wgl.TEXTURE_2D, wgl.TEXTURE_MIN_FILTER, wgl.LINEAR);
                    }
                };

                image.src = url;

                return texture;
            }

            function isPowerOf2(value) {
                return (value & (value -1)) === 0;
            }


            window.addEventListener(\"load\", main);
        </script>
        <div>
            \"funny cube\"
        </div>")
}

#[get("/worm")]
fn worm() -> content::RawHtml<&'static str> {
    content::RawHtml(
        "<html lang=\"en-US\">
            <head>
            </head>
            <body>
                <canvas id=\"dddc\" width=\"150\" height=\"150\">
                    Dirty deeds done dirt cheap    
                </canvas>
                <script>
                    function draw() {
                        const canvas = document.getElementById(\"dddc\");
                        if(canvas.getContext) {
                            const ctx = canvas.getContext(\"2d\");
                            ctx.font = \"10px serif\";
                            ctx.fillText(\"Silly worm consumes\", 10, 50);
                        }
                    }
                    window.addEventListener(\"load\", draw);
                </script>
            </body>
        </html>"
    )
}

#[derive(Debug, FromForm)]
struct UserInput<'f> {
    value: &'f str
}

#[get("/submit")]
fn submit_input() -> content::RawHtml<&'static str> {
    content::RawHtml("<form action=\"/submit\" method=\"post\" accept-charset=\"utf-8\">
        <label> A place for symbols of language... a textarea?
            <br />
            <textarea name=\"textarea\">type HERE.</textarea>
        </label>
        <br />
        <label>
            <input type=\"submit\" value=\"SUBMIT\">
        </label>
    </form>")
}

#[post("/submit", data="<usertext>")]
fn receive_input(usertext: Result<Form<UserInput>, rocket::form::Errors<'_>>) -> String {
    match usertext {
        Ok(form) => format!("{:?}", &*form),
        Err(form_error) => format!("don't even bother: {:?}", form_error),       
    }
}

fn compute_sum(num_a: i32, num_b: i32) -> i32 {
    num_a + num_b
}

#[get("/addnums")]
fn add_nums() -> String {

    format!("I'm adding 3 + {}. \n The sum is {}", 3, compute_sum(3,3))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/butt", routes![worm])
    .mount("/", routes![dirtyworm])
    .mount("/", routes![add_nums])
    .mount("/", routes![submit_input])
    .mount("/", routes![receive_input])
}