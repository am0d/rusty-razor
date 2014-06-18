
use std::io::IoResult;
use super::Action;
pub struct Index<'a> {
    model:  Vec<(int, String)>
}

impl<'a> Index<'a> {
    pub fn new(m:  Vec<(int, String)>) -> Index<'a> {
        Index {
            model: m
        }
    }
}

impl<'a> Action for Index<'a> {
    fn render(&self, out: &mut Writer) -> IoResult<()> {
        let ref model = self.model;
        try!(out.write_str(r###"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="description" content="">
    <meta name="author" content="">
    <link rel="shortcut icon" href="/assets/ico/favicon.png">

    <title>Starter Template for Bootstrap</title>

    <!-- Bootstrap core CSS-->
    <link href="/assets/css/bootstrap.css" rel="stylesheet">

    <!-- Custom styles for this template-->
    <link href="/assets/css/starter-template.css" rel="stylesheet">

    <!-- HTML5 shim and Respond.js IE8 support of HTML5 elements and media queries -->
    <!--[if lt IE 9]>
      <script src="/assets/js/html5shiv.js"></script>
      <script src="/assets/js/respond.min.js"></script>
    <![endif]-->
  </head>

  <body>

    <div class="navbar navbar-inverse navbar-fixed-top">
      <div class="container">
        <div class="navbar-header">
          <button type="button" class="navbar-toggle" data-toggle="collapse" data-target=".navbar-collapse">
            <span class="icon-bar"></span>
            <span class="icon-bar"></span>
            <span class="icon-bar"></span>
          </button>
          <a class="navbar-brand" href="#">Project name</a>
        </div>
        <div class="collapse navbar-collapse">
          <ul class="nav navbar-nav">
            <li class="active"><a href="#">Home</a></li>
            <li><a href="/todos">Todos</a></li>
            <li><a href="#contact">Contact</a></li>
          </ul>
        </div><!--/.nav-collapse -->
      </div>
    </div>

    <div class="container">

      <div class="starter-template">
        <h1>Bootstrap starter template</h1>
        <p class="lead">Use this document as a way to quickly start any new project.
            <br> All you get is this text and a mostly barebones HTML document.
        </p>
        <span>What happens when I put "###));
        try!(out.write_str(r###"@"###));
        try!(out.write_str(r###"LastName here?</span>
        <p>
            My contact: test@example.com
        </p>

        <ul>
            "###));
        for &(id, ref description) in model.iter() {
        try!(out.write_str(r###"
            <li><a href="todos/"###));
        try!(write!(out, "{}", id));
        try!(out.write_str(r###"/edit">"###));
        try!(write!(out, "{}", description));
        try!(out.write_str(r###"</a></li>
            "###));
        }
        try!(out.write_str(r###"
        </ul>
      </div>

    </div><!-- /.container -->


    <!-- Bootstrap core JavaScript
    ================================================== -->
    <!-- Placed at the end of the document so the pages load faster -->
    <script src="/assets/js/jquery.js"></script>
    <script src="/assets/js/bootstrap.min.js"></script>
  </body>
</html>
"###));
    Ok(())
    }
}
