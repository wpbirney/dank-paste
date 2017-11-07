function addUrlEntry(url) {
	var list = document.getElementById('paste-url-list');
	var item = document.createElement('li');
	var link = document.createElement('a');
	var linkText = document.createTextNode(url);
	link.target = '_blank';
	link.href = url;
	link.appendChild(linkText);
	item.appendChild(link);
	list.appendChild(item);
}

var uploadButton = document.getElementById('upload-btn');
var pasteButton = document.getElementById('paste-btn');
var clearUrlButton = document.getElementById('url-clear-btn');
var pasteUrlContainer = document.getElementById('paste-url');
var expire = document.getElementById('expire');
var fileInput = document.getElementById('upload-file');
var fileWrapper = document.getElementsByClassName('file-wrapper')[0];

//file wrapper background urls
const fileWrapperSelect = "url('/static/bg_selectfile.svg')";
const fileWrapperChange = "url('/static/bg_changefile.svg')";
const fileWrapperSelectDark = "url('/static/bg_selectfile_dark.svg')";
const fileWrapperChangeDark = "url('/static/bg_changefile_dark.svg')";


//upload file
uploadButton.addEventListener('click', function() {
	var files = fileInput.files;
	if(files.length > 0) {
		var form = new FormData();
		form.append('file', files[0], files[0].name);
		var xhr = new XMLHttpRequest();
		xhr.open('POST', '/upload', true);
		xhr.onreadystatechange = function() {
			if(this.readyState == 4) {
				if(this.status == 200) {
					//display url list, append url
					pasteUrlContainer.style.display = 'flex';
					addUrlEntry(this.responseText);
				} else {
					alert('Upload failed!');
				}
			}
		};
		xhr.setRequestHeader('expire', expire.value);
		xhr.send(form);
	} else {
		alert('Please select a file to upload');
	}
});

//upload pasted text
pasteButton.addEventListener('click', function() {
	var p = document.getElementById('paste-box');
	if(p.value != "") {
		var xhr = new XMLHttpRequest();
		xhr.open('POST', '/', true);
		xhr.onreadystatechange = function() {
			if(this.readyState == 4) {
				if(this.status == 200) {
					//display url list, append url
					pasteUrlContainer.style.display = 'flex';
					addUrlEntry(this.responseText);
                    p.value = "";
				}
			}
		};
        xhr.setRequestHeader('expire', expire.value);
        xhr.send(p.value);
	} else {
		alert("Enter some text to paste fool");
	}
});

//clear urls
clearUrlButton.addEventListener('click', function() {
	var pasteUrlList = document.getElementById('paste-url-list');
	//remove all child nodes
	while(pasteUrlList.hasChildNodes()) {
		pasteUrlList.removeChild(pasteUrlList.lastChild);
	}
	//hide the url list container
	pasteUrlContainer.style.display = 'none';
});

//things specific to custom-styled file input
function updateFileWrapper() {
	if(document.getElementsByClassName('dp-container')[0].classList.contains('theme-dark')) {
		document.getElementsByClassName('file-wrapper')[0].style.backgroundImage = (document.getElementById('upload-file').files.length > 0) ? fileWrapperChangeDark : fileWrapperSelectDark;
	} else {
		document.getElementsByClassName('file-wrapper')[0].style.backgroundImage = (document.getElementById('upload-file').files.length > 0) ? fileWrapperChange : fileWrapperSelect;
	}
}
fileInput.addEventListener('change', updateFileWrapper);
//update text if file is already selected on page load
updateFileWrapper();

//toggles between light (default style) and dark themes (class theme-dark)
var currentTheme = 'light';
var changeThemeButton = document.getElementById('theme-btn');

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
	updateFileWrapper();
	currentTheme = (currentTheme == 'light') ? 'dark' : 'light';
}

changeThemeButton.addEventListener('click', toggleTheme);
