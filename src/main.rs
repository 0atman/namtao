use yate::html;
use color_eyre::Report;

fn index() -> String {
    html! {

<!DOCTYPE html>
<html lang="en">

<head>
    <meta http-equiv="x-clacks-overhead" content="GNU Terry Pratchett" />
    <link rel="icon" href="favicon.png"/>
    <link rel="stylesheet" href="https://unpkg.com/mvp.css"/>

    <meta charset="utf-8"/>
    <meta name="description" content="Namtao music."/>
    <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
    <script src="https://kit.fontawesome.com/333f3de551.js" crossorigin="anonymous"></script>
    <title>"Namtao Music"</title>
    <style>"
        html {height: 100%; min-height: 100%; overflow: auto; }
        body {
            background: black;
            color: white;
            padding: 0;
        }
        a {
            color: 29bec6;
        }
        :root { --color-link: #ffffff; --color-bg: #fff;}
    "</style>
    </head>

        <body>
        <header>
        <nav>
            <ul>
                <li><a href="https://www.soundcloud.com/namtao"><i class="fab fa-soundcloud">" Soundcloud"</i></a></li>
                <li><a href="https://twitter.com/namtaomusic"><i class="fab fa-twitter">" Twitter"</i></a></li>
                <li><a href="https://open.spotify.com/artist/3BdJgEUj08JtFRywQxl0BM?si=JAQ9teVrRBmPizeEvi62TQ"><i class="fab fa-spotify">" Spotify"</i></a></li>
                <li><a href="mailto:contact@namtao.com"><i class="fa fa-envelope">" Contact"</i></a></li>
            </ul>
            </nav>

            </header>
            <main>


            <section>
            <header>
                <h2>"NAMTAO is Tris Oaten"</h2>
                <p>"Podcast and music producer"</p>
            </header>
            <aside>
                    <a href="https://www.lostterminal.com">
            <img alt="Lost Terminal podcast album art" src="lt-logo.png"/>
            <p>"Lost Terminal Podcast"</p>
                    </a>
            </aside>
            <aside>
                    <a href="https://distrokid.com/hyperfollow/namtao/c0ast">
                <img alt="C0ast album art." src="c0ast.jpg"/>
            <p>"C0AST Album"</p>
                    </a>
            </aside>
            <aside>
                    <a href="https://www.modemprometheus.com">
                <img alt="MIT license" src="mp-logo.png"/>
            <p>"Modem Prometheus Podcast"</p>
                    </a>
                <p>
                </p>
            </aside>
        </section>
            </main>
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
