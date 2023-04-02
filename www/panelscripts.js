function std_rzpr(){
    if(window.confirm("真的要停止 RizPS-Reborn 的运行吗？若要再次启动，需要在服务器中进行操作")){
        var httpRequest = new XMLHttpRequest();
        httpRequest.open("GET","/shutdown_rizps/" + document.getElementById("token_div").textContent,true);
        httpRequest.send();
        alert("停止请求已发送")
    }
}

function open_account_list_page(){
    var httpRequeste = new XMLHttpRequest();
    httpRequeste.open("GET","/aclist/" + document.getElementById("token_div").textContent,true);
    httpRequeste.send();
    httpRequeste.onreadystatechange = ()=>{
        if(httpRequeste.readyState == 4 && httpRequeste.status == 200){
            data = httpRequeste.responseText
            document.getElementById("panelMain").innerHTML = ""
            document.write(httpRequeste.responseText + "<script src=\"" + "/aclist/js/" + document.getElementById("token_div").textContent + "\"></script>")
        }
    }
}