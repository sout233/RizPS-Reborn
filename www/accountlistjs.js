function get_username_list(){
    var httpRequest = new XMLHttpRequest();
        httpRequest.open("GET","/get_username_list/" + document.getElementById("token_div").textContent,true);
        httpRequest.send();
        httpRequest.onreadystatechange = ()=>{
            if(httpRequest.readyState == 4 && httpRequest.status == 200){
                var btarray = httpRequest.responseText.split(">^<")
                document.getElementById("load_div").innerHTML = "<br/>"
                for(var i=0;i<btarray.length-1;i++){
                    console.log(document.getElementById("load_div").innerHTML)
                    console.log(btarray[i])
                    document.getElementById("load_div").innerHTML = document.getElementById("load_div").innerHTML + "<button onclick=\"go_deatil(this)\" acname=\"" + btarray[i] + "\">" + btarray[i] + "</button><br/>"
                }
            }
        }
}

function go_deatil(data){
    console.log(data.innerHTML)
    window.open("/ac_deatil/" + data.innerHTML + "/" + document.getElementById("token_div").innerHTML)
}

function back_panel(){
    var httpRequeste = new XMLHttpRequest();
    httpRequeste.open("GET","/panel/" + document.getElementById("token_div").innerHTML,true);
    httpRequeste.send();
    httpRequeste.onreadystatechange = ()=>{
        if(httpRequeste.readyState == 4 && httpRequeste.status == 200){
            document.body.innerHTML = httpRequeste.responseText + "<div id=\"token_div\" style=\"display: none;\">"+ document.getElementById("token_div").innerHTML + "</div>" + "<script src=\"" + "/panel/js/" + document.getElementById("token_div").innerHTML + "\"></script>"
        }
    }
}