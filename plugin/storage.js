var resourcesSuffix="";/*!
 * Copyright 2018 The Rust Project Developers. See the COPYRIGHT
 * file at the top-level directory of this distribution and at
 * http://rust-lang.org/COPYRIGHT.
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */var currentTheme=document.getElementById("themeStyle");var mainTheme=document.getElementById("mainThemeStyle");var savedHref=[];function onEach(arr,func){if(arr&&arr.length>0&&func){for(var i=0;i<arr.length;i++){if(func(arr[i])===true){return true;}}}return false;}function updateLocalStorage(name,value){if(typeof(Storage)!=="undefined"){localStorage[name]=value;}else{}}function getCurrentValue(name){if(typeof(Storage)!=="undefined"&&localStorage[name]!==undefined){return localStorage[name];}return null;}function switchTheme(styleElem,mainStyleElem,newTheme){var fullBasicCss="rustdoc"+resourcesSuffix+".css";var fullNewTheme=newTheme+resourcesSuffix+".css";var newHref=mainStyleElem.href.replace(fullBasicCss,fullNewTheme);if(styleElem.href===newHref){return;}var found=false;if(savedHref.length===0){onEach(document.getElementsByTagName("link"),function(el){savedHref.push(el.href);});}onEach(savedHref,function(el){if(el===newHref){found=true;return true;}});if(found===true){styleElem.href=newHref;updateLocalStorage('rustdoc-theme',newTheme);}}switchTheme(currentTheme,mainTheme,getCurrentValue('rustdoc-theme')||'light');