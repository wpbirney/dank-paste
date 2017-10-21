var form = document.getElementById("paste-form");
var fileSelect = document.getElementById("file-select");
var uploadButton = document.getElementById("upload-button");
var pasteUrl = document.getElementById("paste-url");
var pasteUrlList= document.getElementById("paste-url-list");

function addUrlEntry(url, list) {
  var a = document.createElement("A");
  a.target = "_blank";
  a.href = url;
  a.innerHTML = url;
  var li = document.createElement("LI");
  li.append(a);
  list.append(li);
}

function clearUrls()  {
  pasteUrlList.innerHTML = "";
  pasteUrl.hidden = true;
}

form.onsubmit = function(event) {
  event.preventDefault();

  var files = fileSelect.files;
  if(files.length == 0) {
    alert("no file selected");
    return;
  }
  var formData = new FormData();
  formData.append("file", files[0], files[0].name);

  var xhr = new XMLHttpRequest();
  xhr.open("POST", "/upload", true);

  xhr.onreadystatechange = function () {
    if (this.readyState == 4) {
      if(this.status == 200) {
        uploadButton.value = 'upload';
        pasteUrl.hidden = false;
        addUrlEntry(this.responseText, pasteUrlList);
      } else {
        uploadButton.value = 'upload';
        alert("upload failed");
      }
    }
  };

  xhr.send(formData);
  uploadButton.value = 'uploading...';
};

document.getElementById('paste-box').value = "";
var pastebox = document.getElementById('paste-box');
var pastebtn = document.getElementById('paste-btn');

pastebtn.onclick = function() {
    var p = pastebox.value;

    if(p == "") {
        alert("paste is empty nigga");
        return;
    }
    pastebtn.value = "uploading...";

    var xhr = new XMLHttpRequest();
    xhr.open("POST", "/", true);
    xhr.onreadystatechange = function () {
      if (this.readyState == 4) {
        if(this.status == 200) {
            pastebtn.value = 'paste text';
            pasteUrl.hidden = false;
            addUrlEntry(this.responseText, pasteUrlList);
            pastebox.value = '';
        } else {
            pastebtn.value = 'paste text';
            alert("upload failed");
        }
      }
    };

    var buf = {"data": p};
    xhr.send(p);
};
