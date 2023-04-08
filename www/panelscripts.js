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

function crac_sdk(){
    var usname = prompt("请输入新用户的username：", "");
    var gnname = prompt("请输入新用户的gamename：", "");
    if(usname != null || gnname != null){
        var httpRequeste = new XMLHttpRequest();
        httpRequeste.open("GET","/create_ac/after_sdk/" + usname + "/" +  gnname + "/" + document.getElementById("token_div").textContent,true);
        httpRequeste.send();
        httpRequeste.onreadystatechange = ()=>{
            if(httpRequeste.readyState == 4 && httpRequeste.status == 200){
                data = httpRequeste.responseText
                if(data == "true"){
                    alert("创建成功")
                }
                else{
                    alert("服务器未返回true，可能是出现报错，responsetext: " + data)
                }
            }
        }
    }
    else{
        alert("你说的对，但请不要在应该输入的地方留空")
    }
}

function crac_gst(){
    var usname = prompt("请输入新用户的username：", "");
    if(usname != null){
        var httpRequeste = new XMLHttpRequest();
        httpRequeste.open("GET","/create_ac/guestlogindo/" + usname + "/" + document.getElementById("token_div").textContent,true);
        httpRequeste.send();
        httpRequeste.onreadystatechange = ()=>{
            if(httpRequeste.readyState == 4 && httpRequeste.status == 200){
                data = httpRequeste.responseText
                if(data == "true"){
                    alert("创建成功")
                }
                else{
                    alert("服务器未返回true，可能是出现报错，responsetext: " + data)
                }
            }
        }
    }
    else{
        alert("你说的对，但请不要在应该输入的地方留空")
    }
}

function rmac(){
    var usname = prompt("请输入要删除的用户的username：", "");
    if(usname != null){
        if(window.confirm("真的删除这个用户吗？届时此用户所有的打歌记录将全部丢失且不可恢复，但用户仍可通过自动注册功能注册一个新账号")){
            var httpRequeste = new XMLHttpRequest();
            httpRequeste.open("GET","/delete_ac/" + usname + "/" + document.getElementById("token_div").textContent,true);
            httpRequeste.send();
            httpRequeste.onreadystatechange = ()=>{
                if(httpRequeste.readyState == 4 && httpRequeste.status == 200){
                    data = httpRequeste.responseText
                    if(data == "true"){
                        alert("删除成功")
                    }
                    else{
                        alert("服务器未返回true，可能是出现报错，又或是输入的username出错！responsetext: " + data)
                    }
                }
            }
        }
    }
    else{
        alert("你说的对，但请不要在应该输入的地方留空")
    }
}

function unlock_a_u_t(){
    var usname = prompt("请输入目标用户的username：", "");
    var track = prompt("请输入要解锁歌曲的track（例如track.PastelLines.RekuMochizuki.0，若未显示请检查是否已在sdkloginbase中appear）：", "");
    if(usname != null || track != null){
        var httpRequeste = new XMLHttpRequest();
        httpRequeste.open("GET","/unlock/a/song/" + usname + "/" + track + "/" + document.getElementById("token_div").textContent,true);
        httpRequeste.send();
        httpRequeste.onreadystatechange = ()=>{
            if(httpRequeste.readyState == 4 && httpRequeste.status == 200){
                data = httpRequeste.responseText
                if(data == "true"){
                    alert("操作完毕，若游戏卡在载入中一直不动，可能是track输入错误，需要手动修改accounts.rzpr修复")
                }
                else{
                    alert("服务器未返回true，可能是出现报错，又或是目标username出错或track出错！responsetext: " + data)
                }
            }
        }
    }
    else{
        alert("你说的对，但请不要在应该输入的地方留空")
    }
}

function unlock_a_u(){
    var usname = prompt("请输入要删除的用户的username：", "");
    if(usname != null){
        var httpRequeste = new XMLHttpRequest();
        httpRequeste.open("GET","/unlock/all/song/" + usname + "/" + document.getElementById("token_div").textContent,true);
        httpRequeste.send();
        httpRequeste.onreadystatechange = ()=>{
            if(httpRequeste.readyState == 4 && httpRequeste.status == 200){
                data = httpRequeste.responseText
                if(data == "true"){
                    alert("操作完毕")
                }
                else{
                    alert("服务器未返回true，可能是出现报错，或是username出错！responsetext: " + data)
                }
            }
        }
    }
    else{
        alert("你说的对，但请不要在应该输入的地方留空")
    }
}