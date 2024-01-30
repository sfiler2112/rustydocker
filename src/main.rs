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
            function draw() {
                const canvas = document.getElementById(\"dddc\");
                if(canvas.getContext) {
                    //const ctx = canvas.getContext(\"2d\");
                    const wgl = canvas.getContext(\"webgl\");

                    //ctx.font = \"10px serif\";
                    //ctx.fillText(\"Dirty worm consumes\", 10, 50);
                    wgl.clearColor(0.0, 0.0, 0.0, 1.0);
                    wgl.clear(wgl.COLOR_BUFFER_BIT);
                }
            }
            window.addEventListener(\"load\", draw);
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