use http::{header, Request, Response, StatusCode};

fn handler(request: Request<()>) -> http::Result<Response<String>> {
  let response = Response::builder()
    .status(StatusCode::OK)
    .header(header::CONTENT_TYPE, "text/html")
    .body("<!DOCTYPE html>
<html lang='en'>
  <head>
    <meta charset='UTF-8' />
    <meta name='viewport' content='width=device-width, initial-scale=1.0' />
    <meta http-equiv='X-UA-Compatible' content='ie=edge' />
    <link
      rel='stylesheet'
      href='//fonts.googleapis.com/css?family=Merriweather'
    />
    <title>Kznovo</title>
    <style>
      body {
        font-family: 'Merriweather', sans-serif;
        margin: auto;
        padding: 2%;
        max-width: 45rem;
      }
    </style>
  </head>
  <body>
    <h1>Hello I'm Kazuya Hatta</h1>
    <p>Thanks for stopping by!</p>
    <p>I'm studying rust :)</p>
    <p>
      <a
        href='https://github.com/kznovo/rust-project'
        target='_blank'
        rel='noopener noreferrer'
        >Here</a
      >
      is the source code.
    </p>
    <br/>
    <br/>
    <h3>Some Projects</h3>
    <li>
      <a
        href='https://wasm-conways-game-of-life.kznovo.now.sh/'
        target='_blank'
        rel='noopener noreferrer'
        >Wasm Conway's Game Of Life</a
      >
    </li>

    <br/>
    <button onclick='clicked()'>Good job!</button>
    <hr />
    <li>
      <a
        href='http://twitter.com/KazuyaHatta'
        target='_blank'
        rel='noopener noreferrer'
        >Twitter</a
      >
    </li>
    <li>
      <a
        href='http://kznovo.github.io'
        target='_blank'
        rel='noopener noreferrer'
        >Main Web Site</a
      >
    </li>
    <script>
      function clicked() {
        window.alert('Thanks!');
      }
    </script>
  </body>
</html>
".to_string())
    .expect("failed to render response");
  Ok(response)
}
