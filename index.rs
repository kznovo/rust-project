use http::{header, Request, Response, StatusCode};

fn handler(request: Request<()>) -> http::Result<Response<String>> {
  let response = Response::builder()
    .status(StatusCode::OK)
    .header(header::CONTENT_TYPE, "text/html")
    .body("<!DOCTYPE html><html lang='en'><head><meta charset='UTF-8' /><meta name='viewport' content='width=device-width, initial-scale=1.0' /><meta http-equiv='X-UA-Compatible' content='ie=edge' /><link rel='stylesheet' href='//fonts.googleapis.com/css?family=Merriweather' /><title>Kznovo</title><style>body {font-family: 'Merriweather', sans-serif;margin: auto;padding: 2%;max-width: 45rem;}</style></head><body><h1>Hello I'm Kazuya Hatta</h1><p>I'm studying rust :)</p><button onclick='clicked()'>Good job!</button><hr /><li><a href='http://twitter.com/KazuyaHatta' target='_blank' rel='noopener noreferrer'>Twitter</a></li><li><a href='http://kznovo.github.io' target='_blank' rel='noopener noreferrer'>Main Web Site</a></li><script>function clicked() {window.alert('Thanks!');}</script></body></html>".to_string())
    .expect("failed to render response");
  Ok(response)
}
