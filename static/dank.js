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

	var d = document.createElement('div');
	d.id = 'paste-res';

	d.appendChild(id);
	d.appendChild(rlink);
	d.appendChild(slink);
	item.appendChild(d);
	item.appendChild(hr);
	list.appendChild(item);
}

function addShortUrlEntry(response, old)	{
	var list = document.getElementById('paste-url-list');
	var item = document.createElement('li');

	var link = createLinkButton(response, response);
	link.id = 'shorturl';

	item.appendChild(link);
	item.appendChild(document.createElement('hr'));
	list.appendChild(item);
}

var uploadButton = document.getElementById('upload-btn');
var pasteButton = document.getElementById('paste-btn');
var submitUrl = document.getElementById('submit-url');
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

class dankPaste {
	constructor(url, expire) {
		this.xhr = new XMLHttpRequest();
		this.xhr.open('POST', url, true);
		var self = this;
		this.xhr.onreadystatechange = function() {
			if(this.readyState == 4) {
				if(this.status == 200) {
					//display url list, append url
					pasteUrlContainer.style.display = 'flex';
					if(typeof self.onsuccess === 'function') {
						self.onsuccess(this.responseText);
					}
				} else if(this.status == 429) {
					alert("quit being a jew and wait a few seconds");
				} else {
					alert("Upload failed!");
				}
			}
		};
		this.xhr.setRequestHeader('expire', expire);
	}
	send(data) {
		this.xhr.send(data);
	}
}


//upload file
uploadButton.addEventListener('click', function() {
	var files = fileInput.files;
	if(files.length > 0) {
		var form = new FormData();
		form.append('file', files[0], files[0].name);
		var dp = new dankPaste('/upload', expire.value);
		dp.onsuccess = function(response) {
			addUrlEntry(response);
		};
		dp.send(form);
	} else {
		alert('Please select a file to upload');
	}
});

//upload pasted text
pasteButton.addEventListener('click', function() {
	var p = document.getElementById('paste-box');
	if(p.value != "") {
		var dp = new dankPaste('/', expire.value);
		dp.onsuccess = function(response) {
			addUrlEntry(response);
			p.value = "";
		};
		dp.send(p.value);
	} else {
		alert("Enter some text to paste fool");
	}
});

submitUrl.addEventListener('click', function() {
	var urlentry = document.getElementById("url-entry");
	if(urlentry.value != "") {
		var dp = new dankPaste('/shorty', expire.value);
		dp.onsuccess = function(response) {
			addShortUrlEntry(response, urlentry.value);
			urlentry.value = "";
		};
		dp.send(urlentry.value);
	} else {
		alert("Enter a url jackass");
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
