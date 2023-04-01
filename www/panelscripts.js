function std_rzpr(){
    if(window.confirm("真的要停止 RizPS-Reborn 的运行吗？若要再次启动，需要在服务器中进行操作")){
        var httpRequest = new XMLHttpRequest();
        httpRequest.open("GET","/shutdown_rizps/" + document.getElementById("token_div").textContent,true);
        httpRequest.send();
        alert("停止请求已发送")
    }
}