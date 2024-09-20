what?
===
A simple json/javascript expression parser.

why?
===
Playing with lexers/parsers in rust.

run
===

evaluate an expression:
```
~/tmp/rust/js-light (main) » cargo run -q -- --expr "var x = 10; var z = x+25;"
parsed prg:
var x = 10;
var z = x + 25;

executing prg
vars (2):
  x = 10
  z = 35
```

evaluate a file:
```
~/tmp/rust/js-light (main) » cargo run -q -- --file resources/ex1.js_new
parsed prg:
var v1 = 10;
var v2 = 2 + 3;
var v3 = 2 + 3 + 6;
var v4 = 2 + (3 * 6);
var v5 = (2 + 3) * 6;
var v6 = 2 + 3 + 6 + 7;
var name = "didier";
var greetings = "hello " + name + "!!";
var v8 = greetings + ", welcome";
var v9 = v1 + v2 + v3;
var card = {"name": name, "greetings": greetings, "res": v4 + v6};
var dir = "resources";
var filename = "ex1.json";
var file = load_json(dir + "/" + filename);
var special1 = min(2, 30);
var special2 = max(2, 30);

executing prg
vars (16):
  card = {"name": "didier", "greetings": "hello didier!!", "res": 38}
  dir = "resources"
  file = {"nesting": {"inner object": {}}, "an array": [1.5, true, null, 0.000001], "string with escaped double quotes": "\"quick brown foxes\""}
  filename = "ex1.json"
  greetings = "hello didier!!"
  name = "didier"
  special1 = 2
  special2 = 30
  v1 = 10
  v2 = 5
  v3 = 11
  v4 = 20
  v5 = 30
  v6 = 18
  v8 = "hello didier!!, welcome"
  v9 = 26
```
