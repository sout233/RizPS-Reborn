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
                    document.getElementById("load_div").innerHTML = document.getElementById("load_div").innerHTML + "<button onclick=\"go_deatil()\" acname=\"" + btarray[i] + "\">" + btarray[i] + "</button><br/>"
                }
            }
        }
}