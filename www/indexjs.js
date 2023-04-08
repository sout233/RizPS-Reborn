function auth(){
    var httpRequest = new XMLHttpRequest();
    httpRequest.open("POST","/auth",true);
    httpRequest.send(document.getElementById("pwd_input").value.toString());
    httpRequest.onreadystatechange = ()=>{
        if(httpRequest.readyState == 4 && httpRequest.status == 200){
            var data = httpRequest.responseText;
            if(data == "pwd_err"){
                alert("密码错误");
            }
            else{
                var httpRequeste = new XMLHttpRequest();
                httpRequeste.open("GET","/panel/" + data,true);
                httpRequeste.send();
                httpRequeste.onreadystatechange = ()=>{
                if(httpRequeste.readyState == 4 && httpRequeste.status == 200){
                    document.write("<html><head><meta charset=\"utf-8\"><title>RizPS-Reborn Control Panel</title></head><body><div id=\"token_div\" style=\"display: none;\">"+ data + "</div>" + httpRequeste.responseText + "<script src=\"" + "/panel/js/" + data + "\"></script>")
                }
            }
        }
    }
}
}