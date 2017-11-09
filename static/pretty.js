var pick = document.getElementById("lang-pick");

function updateLang(reload)	{
	var c = document.getElementById("code");
	if(pick.value == "rust")	{
		c.classList.add('lang-rust');
	} else if (pick.value == "css") {
		c.classList.add('lang-css');
	} else {
		c.classList.remove('lang-rust');
		c.classList.remove('lang-css');
	}
	if(reload) {
		document.location.reload();
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

document.getElementById('raw-btn').addEventListener('click', function() {
	window.open('https://ganja.ml', '_blank');
});
