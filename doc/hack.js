var path = $(".location").text();
var nest_count;
if (path) {
    nest_count = path.split("::").length + 1;
} else {
    nest_count = 1;
}
if (window.location.pathname.endsWith("index.html")) {
  nest_count += 1;
}

var base_path = "";
for (i = 0; i < nest_count; i++) {
    base_path += "../";
}

$(".sidebar").prepend('\
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
  </div>');
