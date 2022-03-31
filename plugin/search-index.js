var searchIndex = JSON.parse('{\
"dynasm":{"doc":"The dynasm crate contains the procedural macros that power …","t":[14,14],"n":["dynasm","dynasm_backwards"],"q":["dynasm",""],"d":["The whole point. This macro compiles given assembly/rust …","Similar to <code>dynasm!</code>, but the calls to the assembler are …"],"i":[0,0],"f":[null,null],"p":[]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};// I'm sorry
var path = document.getElementsByClassName("logo-container")[0].childNodes[0].getAttribute("src");
var nest_count = (path.match(/\.\./g)||[]).length + 1; 

var base_path = "";
for (i = 0; i < nest_count; i++) {
    base_path += "../";
}

var sidebar = document.getElementsByClassName("sidebar")[0];

var node = document.createElement("div");
node.innerHTML = '\
  <p class="location">\
      <a href="' + base_path + 'language/index.html">dynasm-rs</a>\
  </p>\
  <div class = "block modules">\
    <h3>Components</h3>\
    <ul>\
      <li>\
        <a href="' + base_path + 'language/index.html">Syntax</a>\
      </li>\
      <li>\
        <a href="' + base_path + 'plugin/dynasm/index.html">Plugin (dynasm)</a>\
      </li>\
      <li>\
        <a href="' + base_path + 'runtime/dynasmrt/index.html">Runtime (dynasmrt)</a>\
      </li>\
    </ul>\
  </div>';

sidebar.insertBefore(node, sidebar.childNodes[2]);
