/*
	handles themes inside dank-paste, currently there is just light/dark
	props to josh for the html/css/theming, coopercomputer.net
*/

function setCookie(cname, cvalue, exdays) {
    var d = new Date();
    d.setTime(d.getTime() + (exdays*24*60*60*1000));
    var expires = "expires="+ d.toUTCString();
    document.cookie = cname + "=" + cvalue + ";" + expires + ";path=/";
}

function getCookie(cname) {
    var name = cname + "=";
    var decodedCookie = decodeURIComponent(document.cookie);
	var cooks = document.cookie;
    var ca = decodedCookie.split(';');
    for(var i = 0; i <ca.length; i++) {
        var c = ca[i];
        while (c.charAt(0) == ' ') {
            c = c.substring(1);
        }
        if (c.indexOf(name) == 0) {
            return c.substring(name.length, c.length);
        }
    }
    return "";
}

var currentTheme = "";

switch(getCookie("theme"))	{
	case "light":
		currentTheme = "dark";
		break;
	default:
		currentTheme = "light";
		break;
}

function toggleTheme() {
	var themedElements = ['body', '.dp-container'];
	for(var i = 0; i < themedElements.length; i++) {
		var elem = document.querySelector(themedElements[i]);
		if(currentTheme == 'light') {
			elem.classList.add('theme-dark');
		} else {
			elem.classList.remove('theme-dark');
		}
	}
	currentTheme = (currentTheme == 'light') ? 'dark' : 'light';
	setCookie("theme", currentTheme, 180);
}

toggleTheme();
document.getElementById('theme-btn').addEventListener('click', toggleTheme);
