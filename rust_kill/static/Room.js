//main functions
   
var messages = document.getElementById("content"); 
function scrollToBottom() {
    messages.scrollTop = messages.scrollHeight;
}





//utilities
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