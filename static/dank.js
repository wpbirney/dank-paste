function setCookie(cname, cvalue, exdays) {
    var d = new Date();
    d.setTime(d.getTime() + (exdays*24*60*60*1000));
    var expires = "expires="+ d.toUTCString();
    document.cookie = cname + "=" + cvalue + ";" + expires + ";path=/";
}

function getCookie(cname) {
    var name = cname + "=";
    var decodedCookie = decodeURIComponent(document.cookie);
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

function createLinkButton(url, text)	{
	var link = document.createElement('a');
	link.innerHTML = text;
	link.style.marginLeft = '1em';
	link.style.minWidth = '5em';
	link.href = url;
	link.target = '_blank';
	return link;
}

function addUrlEntry(response) {
	var r = JSON.parse(response);

	var list = document.getElementById('paste-url-list');
	var item = document.createElement('li');

	var rlink = createLinkButton(r.raw_url, 'raw');
	var slink = createLinkButton(r.source_url, 'source');

	var id = document.createElement('label');
	id.innerHTML = "Paste Id: " + r.id;

	var hr = document.createElement('hr');

	item.appendChild(id);
	item.appendChild(rlink);
	item.appendChild(slink);
	item.appendChild(hr);
	list.appendChild(item);
}

function addShortUrlEntry(response, old)	{
	var list = document.getElementById('paste-url-list');
	var item = document.createElement('li');

	var link = createLinkButton(response, response);

	item.appendChild(link);
	item.appendChild(document.createElement('hr'));
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
const fileWrapperSelect = "url('/static/img/bg_selectfile.svg')";
const fileWrapperChange = "url('/static/img/bg_changefile.svg')";
const fileWrapperSelectDark = "url('/static/img/bg_selectfile_dark.svg')";
const fileWrapperChangeDark = "url('/static/img/bg_changefile_dark.svg')";


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
				} else if(this.status = 429) {
					alert("quit being a jew and wait a few seconds");
				} else {
					alert("Upload failed!");
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
				} else if(this.status = 429) {
					alert("quit being a jew and wait a few seconds");
				} else {
					alert("Upload failed!");
				}
			}
		};
        xhr.setRequestHeader('expire', expire.value);
        xhr.send(p.value);
	} else {
		alert("Enter some text to paste fool");
	}
});

document.getElementById("submit-url").addEventListener('click', function() {
	var urlentry = document.getElementById("url-entry");
	if(urlentry.value != "") {
		var xhr = new XMLHttpRequest();
		xhr.open('POST', '/shorty', true);
		xhr.onreadystatechange = function() {
			if(this.readyState == 4) {
				if(this.status == 200) {
					//display url list, append url
					pasteUrlContainer.style.display = 'flex';
					addShortUrlEntry(this.responseText, urlentry.value);
				} else if(this.status = 429) {
					alert("quit being a jew and wait a few seconds");
				} else {
					alert("Upload failed!");
				}
			}
		};
		xhr.setRequestHeader('expire', expire.value);
		xhr.send(urlentry.value);
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
	setCookie('theme', currentTheme, 365);
}

document.getElementById('theme-btn').addEventListener('click', toggleTheme);

switch(getCookie('theme'))	{
	case 'light':
		break;
	default:
		toggleTheme();
		break;
}
