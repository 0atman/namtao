use yate::html;
use color_eyre::Report;

fn content() -> String {
    html! { <div>"content"</div> }
}

fn index() -> String {
    html! {
        <!DOCTYPE html>
        <html>
            <head></head>
            <body>
                {content()}
            </body>
        </html>
    }
}

fn main() -> Result<(), Report> {
    std::fs::create_dir_all("dist")?;
    std::fs::write("dist/index.html", index())?;

    println!("Built site OK!");
    Ok(())
}
