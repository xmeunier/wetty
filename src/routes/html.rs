use handlebars::Handlebars;
use serde_json::json;
use std::sync::Arc;
use warp::Reply;

pub fn render(hbs: Arc<Handlebars>, title: String, base: String) -> impl Reply {
    let value = json!({
        "title": title,
        "base": base,
        "js": ["wetty"],
        "css": ["styles", "options", "overlay", "terminal"]
    });
    warp::reply::html(
        hbs.render_template(INDEX_HTML, &value)
            .unwrap_or_else(|err| err.to_string()),
    )
}

pub static INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=no">
    <link rel="icon" type="image/x-icon" href="{{base}}/favicon.ico">
    <title>{{title}}</title>
    {{#each css}}
    <link rel="stylesheet" href="{{base}}/assets/css/{{this}}.css" />
    {{/each}}
  </head>
  <body>
    <div id="overlay">
      <div class="error">
        <div id="msg"></div>
        <input type="button" onclick="location.reload();" value="reconnect" />
      </div>
    </div>
    <div id="options">
      <a class="toggler"
         href="\#"
         alt="Toggle options"
       ><i class="fas fa-cogs"></i></a>
      <textarea class="editor"></textarea>
    </div>
    <div id="terminal"></div>
    {{#each js}}
    <script type="module" src="{{base}}/client/{{this}}.js"></script>
    {{/each}}
  </body>
</html>"#;
