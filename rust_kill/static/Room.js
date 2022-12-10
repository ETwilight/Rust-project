var content = document.getElementById("content");  
function scrollToBottom() {
    setTimeout(function(){
        content.scrollTop = content.scrollHeight;
    }, 1000);   
}
   
let newMessageForm = document.getElementById('new-message');
newMessageForm.addEventListener("submit", scrollToBottom);


var e = document.getElementById("killoption");
document.querySelector("#killconfirm").addEventListener('click', () => {
    console.log(e.value)
})



//init

/* => GAME LOOP*/ 

/*Utilities*/
const heightOutput = document.querySelector("#height");
const widthOutput = document.querySelector("#width");
function reportWindowSize() {
    heightOutput.textContent = window.innerHeight;
    widthOutput.textContent = window.innerWidth;
}
window.onresize = reportWindowSize;


var slider = document.getElementById("myRange");
slider.onchange = function() {
    let i = 1;
    for(;i<=this.value;){
        document.querySelector(".player"+(i).toString()).style.visibility = "visible";
        i++;
    }
}
