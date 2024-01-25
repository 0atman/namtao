use color_eyre::Report;
use yate::html;

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
        <title>"NAMTAO Productions"</title>
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
                    <li><a href="https://mastodon.social/deck/@0atman"><i class="fab fa-mastodon">" Mastodon"</i></a></li>
                    <li><a href="https://open.spotify.com/artist/3BdJgEUj08JtFRywQxl0BM?si=JAQ9teVrRBmPizeEvi62TQ"><i class="fab fa-spotify">" Spotify"</i></a></li>
                    <li><a href="mailto:contact@namtao.com"><i class="fa fa-envelope">" Contact"</i></a></li>
                </ul>
                </nav>

                </header>
                <main>


                <section>
                <aside>
                    <iframe src="https://www.youtube.com/embed/p3bDE9kszMc?si=fvdaHMEdlwrtHc30" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>
                    <a href="https://lostterminal.com">
                        <p>"Lost Terminal"</p>
                    </a>
                    <p>"A hard sci-fi hopepunk podcast following the journey of a little satellite trying to understand what has happened after Earth stops returning his calls."</p>

                </aside>
                <aside>
                    <iframe src="https://www.youtube.com/embed/_SDnUVHAC44?si=FYLNBYLD6pOE-5bC" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>
                    <a href="https://www.modemprometheus.com">
                        <p>"Modem Prometheus"</p>
                    </a>
                    <p>"Modem Prometheus is an anthology podcast of urban folktales.
                            This is where any roundabout could be a gateway to Faerie. Where trains that don’t exist run from the foundations of skyscrapers. Where there is no deeper dark than the shadows between buildings.
                            The transmitter mast is calling. Come home."</p>
                </aside>
                <aside>
                    <iframe src="https://www.youtube.com/embed/y3wVvDbJHrU?si=bo7oAARmF09FEH1A" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>
                    <a href="https://phosphenecatalogue.com">
                        <p>"The Phosphene Catalogue"</p>
                    </a>
                    <p>"An urban fantasy podcast of tape recordings by the curator of a secretive London-based art auction house.
    The Phosphene Catalogue is a 1970s mail-order catalogue, specialising in those items that cannot be sold at other auction houses: Paintings of lost origin, statues that are too grotesque for public display, and books better left unread..."
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
