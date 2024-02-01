#[macro_use]
extern crate rocket;

use rocket::response::content;


#[get("/")]
fn index() -> &'static str {
    "Hello, Docker! Add 'worm' to the URL to play worm..."
}

#[get("/dirtyworm")]
fn dirtyworm() -> content::RawHtml<&'static str> {
    content::RawHtml(
        "<canvas id=\"dddc\" width=\"150\" height=\"150\">
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

                if(!wgl.getProgramParameter(shaderProgram, wgl.LINK_STATUS)) {
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
                    alert(\"shit's fucked, mate... with the shader\");
                    wgl.deleteShader(shader);
                    return null;
                }

                return shader;
            }

            /*
            The following two init functions should be placed in a seperate file, maybe init-buffers.js?
             */
            function init_buffers(wgl) {
                const position_buffer = init_position_buffer(wgl);
                
                return {
                    position: position_buffer,
                };
            }

            function init_position_buffer(wgl) {
                const position_buffer = wgl.createBuffer();
                wgl.bindBuffer(wgl.ARRAY_BUFFER, position_buffer);

                const positions = [1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0];
                wgl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), wgl.STATIC_DRAW);

                return position_buffer;
            }

            /*
            The following two functions should be put into seperate file and exported.
            Now... what would be a good name... something like draw-scene.js!?
             */
            function draw_scene(wgl, program_info, buffers) {
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
                const z_far = 100;
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
            }

            /*
            Remember, main should be in a file with the shader functions, maybe webgl-demo.js...
             */
            function main() {
                const vsSource = `
                    attribute vec4 aVertexPosition;
                    uniform mat4 uModelViewMatrix;
                    uniform mat4 uProjectionMatrix;
                    void main() {
                        gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
                    }
                `;

                const fsSource = `
                    void main() {
                        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
                    }
                `;

                
                const canvas = document.getElementById(\"dddc\");
                if(canvas.getContext) {
                    //const ctx = canvas.getContext(\"2d\");
                    const wgl = canvas.getContext(\"webgl\");
                    const shaderProgram = init_shader_program(wgl, vsSource, fsSource);
                    const programInfo = {
                        program: shaderProgram,
                        attributions: {
                            vertexPosition: wgl.getAttribLocation(shaderProgram, aVertexPosition);
                        },
                        uniformLocations: {
                            projectionMatrix: wgl.getUniformLocation(shaderProgram, uProjectionMatrix),
                            modelViewMatrix: wgl.getUniformLocation(shaderProgram, uModelViewMatrix)
                        }
                    }
                    //ctx.font = \"10px serif\";
                    //ctx.fillText(\"Dirty worm consumes\", 10, 50);
                    wgl.clearColor(0.0, 0.0, 0.0, 1.0);
                    wgl.clear(wgl.COLOR_BUFFER_BIT);
                }
            }

            window.addEventListener(\"load\", main);
        </script>")
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

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/butt", routes![worm])
    .mount("/", routes![dirtyworm])
}