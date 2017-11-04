//toggles between light (default style) and dark themes (class theme-dark)
var currentTheme = 'light';
var changeThemeButton = document.getElementById('theme-btn');
changeThemeButton.addEventListener('click', function() {
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
});
