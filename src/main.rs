use yate::html;
use color_eyre::Report;

fn index() -> String {
    html! {

<!DOCTYPE html>
<html lang="en">

<head>
    <link rel="icon" href="https://via.placeholder.com/70x70"/>
    <link rel="stylesheet" href="https://unpkg.com/mvp.css"/>

    <meta charset="utf-8"/>
    <meta name="description" content="My description"/>
    <meta name="viewport" content="width=device-width, initial-scale=1.0"/>

            <title>"Modem Prometheus"</title>
            <style>"
                body, html {height: 100%;}
                body {
                font-family: Garamond, serif;
                background: black;

                background-image: url(\"bg.jpg\");


                /* Full height */
                height: 100%;

                /* Center and scale the image nicely */
                background-position: center;
                background-repeat: no-repeat;
                background-size: cover;
                }
                "</style>
</head>

<body>
    <header>
        <nav>
            <ul>
                <li><a href="#">"Listen"</a></li>
                <li><a href="#">"Twitter"</a></li>
                <li><a href="#">"Credits"</a></li>
                <li><a href="#">"Support us on Patreon"</a></li>
            </ul>
            </nav>

        {content()}

    </header>
    <main>
            </main>
            <footer>"
            foot
            "</footer>
            </body>
        </html>
    }
}


fn content() -> String {
    html! {
        <section>
            <figure>
                <img alt="Logo" src="logo.png"/>
            </figure>
            <h1>"Modem Prometheus is a forthcoming podcast of modern folktales."</h1>
            <p>"These are the stories of the kerbside, the transmitter mast, the shadows between buildings."</p>
            <br/>
            <p>
            <a href="#">
                <i>"Watch promo here"</i>
            </a>
            <a href="#"><b>"Start Here &rarr;"</b></a></p>

        </section>
}
}


fn main() -> Result<(), Report> {
    std::fs::create_dir_all("dist")?;
    std::fs::write("dist/index.html", index())?;

    println!("Built site OK!");
    Ok(())
}
