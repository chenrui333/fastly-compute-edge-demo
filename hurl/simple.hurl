GET https://fastly-compute-edge-demo.edgecompute.app

HTTP/2 200
```
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta http-equiv="x-ua-compatible" content="ie=edge" />
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1, shrink-to-fit=no"
    />
    <title>fastly compute@edge demo</title>
  </head>
  <body>
    This is demo app.
    Source code in https://github.com/chenrui333/fastly-compute-edge-demo
  </body>
</html>
```

[Asserts]
header "Content-Type" == "text/html; charset=utf-8"
