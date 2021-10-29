use yate::html;
use color_eyre::Report;

fn index() -> String {
    html! {

<!DOCTYPE html>
<html lang="en">

<head>
    <link rel="icon" href="favicon.png"/>
    <link rel="stylesheet" href="https://unpkg.com/mvp.css"/>

    <meta charset="utf-8"/>
    <meta name="description" content="Modem Prometheus is a forthcoming podcast of modern folktales. These are the stories of the kerbside, the transmitter mast, the shadows between buildings."/>
    <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
    <script src="https://kit.fontawesome.com/333f3de551.js" crossorigin="anonymous"></script>
    <title>"Modem Prometheus Podcast"</title>
    <style>"
        body, html {height: 100%;}
        body {
        background-image: url(\"bg.jpg\");

        /* Full height */
        height: 100%;

        /* Center and scale the image nicely */
        background-position: center;
        background-repeat: no-repeat;
        background-size: cover;
        }
        :root {
            --active-brightness: 0.85;
            --border-radius: 5px;
            --box-shadow: 2px 2px 10px;
            --color: #118bee;
            --color-accent: #118bee15;
            --color-bg: #fff;
            --color-bg-secondary: #e9e9e9;
            --color-link: #29bec6;
            --color-secondary: #920de9;
            --color-secondary-accent: #920de90b;
            --color-shadow: #f4f4f4;
            --color-table: #118bee;
            --color-text: #FFF;
            --color-text-secondary: #999;
            --font-family: -apple-system, sans-serif;
            --hover-brightness: 1.2;
            --justify-important: center;
            --justify-normal: left;
            --line-height: 1.5;
            --width-card: 285px;
            --width-card-medium: 460px;
            --width-card-wide: 800px;
            --width-content: 1080px;
        }

    "</style>
    </head>

    <body>
        <header>
        <nav>
            <ul>
                <li><a href="https://www.spreaker.com/show/modem-prometheus"><i class="fa fa-podcast">" Listen"</i></a></li>
                <li><a href="https://twitter.com/modemprometheus"><i class="fab fa-twitter">" Twitter"</i></a></li>
                //<li><a href="#"><i class="fa fa-user-astronaut">" Credits"</i></a></li>
                <li><a href="https://www.patreon.com/modemprometheus"><i class="fab fa-patreon">" Support us on Patreon"</i></a></li>
            </ul>
            </nav>

        <section>
            <figure>
                <img alt="Logo" src="logo.png"/>
            </figure>
            <h1>"Modem Prometheus is a forthcoming podcast of modern folktales."</h1>
            <p>"These are the stories of the kerbside, the transmitter mast, the shadows between buildings."</p>
            <br/>
            <p>
            <a href="https://www.youtube.com/watch?v=rci101Zkgds"><b>"Start Here &rarr;"</b></a></p>

        </section>
        </header>
            </body>
        </html>
    }
}


fn main() -> Result<(), Report> {
    std::fs::create_dir_all("docs")?;
    std::fs::write("docs/index.html", index())?;

    println!("Built site OK!");
    Ok(())
}
