// I'm sorry

var path = document.currentScript.getAttribute("src");
var nest_count = (path.match(/\.\./g)||[]).length; 

var base_path = "";
for (var i = 0; i < nest_count; i++) {
    base_path += "../";
}

var sidebar = document.getElementsByClassName("sidebar")[0];

var node = document.createElement("div");
node.innerHTML = '\
  <h2 class="location">\
      <a href="' + base_path + 'language/index.html">dynasm-rs</a>\
  </h2>\
  <h3>Components</h3>\
  <ul class = "block crate">\
    <li>\
      <a href="' + base_path + 'language/index.html">Syntax</a>\
    </li>\
    <li>\
      <a href="' + base_path + 'dynasm/index.html">Plugin (dynasm)</a>\
    </li>\
    <li>\
      <a href="' + base_path + 'dynasmrt/index.html">Runtime (dynasmrt)</a>\
    </li>\
  </ul>';

node.setAttribute("class", "sidebar-elems")

sidebar.insertBefore(node, sidebar.firstChild);

