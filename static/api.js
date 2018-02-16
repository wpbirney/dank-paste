function pasteFile(formData, expire, retain_name, callback) {
    var xhr = new XMLHttpRequest();
    xhr.open('POST', '/upload', true);
    xhr.setRequestHeader('expire', expire);
    xhr.setRequestHeader('retain-filename', retain_name);
    xhr.responseType = 'json';
    xhr.onreadystatechange = function() {
        if(this.readyState == 4) {
            callback(this);
        }
    }
    xhr.send(formData);
}

var uploadbtn = document.getElementById('upload-btn');
var fileInput = document.getElementById('file-input');

uploadbtn.addEventListener('click', function() {
    var files = fileInput.files;
    var form = new FormData();
    form.append('file', files[0], files[0].name);
    pasteFile(form, '30', false, function(resp) {
        alert(JSON.stringify(resp.response));
    });
});
