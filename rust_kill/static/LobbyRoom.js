

function openClient() {
document.getElementById("f2").style.display = "block";
}

function openHost() {
document.getElementById("f1").style.display = "block";
}

function closeClient() {
document.getElementById("f2").style.display = "none";
}

function closeHost() {
document.getElementById("f1").style.display = "none";
}

function save() {
    let form = document.querySelector('#hostform');
    let data = new FormData(form);
    var object = {};
    data.forEach(function(value, key){
    object[key] = value;
    });
    var jsondat = JSON.stringify(object);
    
} 

function save2() {
    let form = document.querySelector('#clientform');
    let data = new FormData(form);
    var object = {};
    data.forEach(function(value, key){
    object[key] = value;
    });
    var jsondat = JSON.stringify(object);
    let userObj = JSON.parse(jsondat);
} 

