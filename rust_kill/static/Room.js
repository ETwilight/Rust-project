
var slider = document.getElementById("myRange");
slider.onchange = function() {
    let i = 1;
    for(;i<=this.value;){
        document.querySelector(".player"+(i).toString()).style.visibility = "visible";
        i++;
    }
}
   
