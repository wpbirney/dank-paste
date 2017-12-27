/*
    Handles keepn' shit pretty
*/
var pick = document.getElementById("lang-pick");
var link = document.getElementById('link');
var codebox = document.getElementById("code");

var orig = document.location.href;
orig = orig.split('?')[0];
var ID = orig.split('/').reverse()[0];
var plugins = ['lang-rust', 'lang-css'];

function removeAllPlugins()    {
    for ( var i = 0; i < plugins.length; ++i )    {
        codebox.classList.remove(plugins[i]);
    }
}

function updateLang(reload)    {
    switch(pick.value)    {
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
if(q != null)    {
    pick.value = q;
}
updateLang(false);

document.getElementById("lang-pick").addEventListener('change', function() {
    updateLang(true);
});

var link = document.getElementById('link');
var _link = link.innerHTML;

link.addEventListener('click', function() {
    let x = document.getElementById('sel');
    x.hidden = false;
    x.value = document.location.href;
    x.select();
    document.execCommand('copy');
    x.hidden = true;
    link.innerHTML = 'Copied!';
    setTimeout(function() {
        link.innerHTML = _link;
    }, 3000);
});
