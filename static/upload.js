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

//upload file
uploadButton.addEventListener('click', function() {
	var files = document.getElementById('upload-file').files;
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
