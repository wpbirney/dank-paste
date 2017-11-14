/*
	Handles keepn' shit pretty
*/
var pick = document.getElementById("lang-pick");
var link = document.getElementById('link');
var codebox = document.getElementById("code");

var orig = link.href;
var ID = orig.split('/').reverse()[0];
var plugins = ['lang-rust', 'lang-css'];

function removeAllPlugins()	{
	for ( var i = 0; i < plugins.length; ++i )	{
		codebox.classList.remove(plugins[i]);
	}
}

function updateLang(reload)	{
	switch(pick.value)	{
		case "rust":
			codebox.classList.add('lang-rust');
			link.href = orig + '?lang=rust';
			break;
		case "css":
			codebox.classList.add('lang-css');
			link.href = orig + '?lang=css';
			break;
		default:
			removeAllPlugins();
			link.href = orig;
			break;
	}

	if(reload) {
		document.location = link.href;
	}
}

var params = new URLSearchParams(window.location.search);

var q = params.get('lang');
if(q != null)	{
	pick.value = q;
}
updateLang(false);

document.getElementById("lang-pick").addEventListener('change', function() {
	updateLang(true);
});
