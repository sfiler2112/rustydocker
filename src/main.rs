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
        <script>
            function draw() {
                const canvas = document.getElementById(\"dddc\");
                if(canvas.getContext) {
                    const ctx = canvas.getContext(\"2d\");
                    ctx.font = \"10px serif\";
                    ctx.fillText(\"Dirty worm consumes\", 10, 50);
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