import System;
import System.Windows.Forms;
import Fiddler;
import System.Text.RegularExpressions;

class Handlers
{
    
    static var server_ip = "192.168.50.89"//修改为对手机来说你电脑的ip    
    static var server_port = 443//修改为你config里服务器的端口，默认为443不用动
    
    static function OnBeforeRequest(oS: Session) {
        if(oS.host.Contains("google") || oS.host.Contains("amazon") || oS.host.Contains("baidu")){
            if (oS.HTTPMethodIs("CONNECT"))
            {
                oS["x-replywithtunnel"] = "FakeTunnel";
                return;
            }//CONNECT请求处理
            oS.oRequest.headers.UriScheme = "https"
            oS.host = server_ip
            oS.port = server_port
            //屏蔽谷歌以强制游客登陆
        }
        else if(oS.host.Contains("lvdgjosdl")){
            if(oS.fullUrl.Contains("cridata")){
                //啥都不干
            }
            /*
            else{
                if (oS.HTTPMethodIs("CONNECT"))
                {
                    oS["x-replywithtunnel"] = "FakeTunnel";
                    return;
                }//CONNECT请求处理
                oS.host = server_ip
                oS.port = server_port
            }
            //什么都不干
*/
        }
        else if(oS.host.Contains("leiting") || oS.host.Contains("ltgames") || oS.host.Contains("lt")) {
            if (oS.HTTPMethodIs("CONNECT"))
            {
                oS["x-replywithtunnel"] = "FakeTunnel";
                return;
            }//CONNECT请求处理
            oS.host = server_ip
            oS.port = server_port
            if (oS.fullUrl.Contains("cridata")){
                oS.fullUrl = oS.fullUrl.Replace("testasset","songsdata");
            }
        }


    }
    static function OnBeforceResponse(oS: Session){
        
    }

};