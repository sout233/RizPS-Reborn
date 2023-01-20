import System;
import System.Windows.Forms;
import Fiddler;
import System.Text.RegularExpressions;

class Handlers
{
    static function OnBeforeRequest(oS: Session) {
        if(oS.host.Contains("google")){
            oS.host = "114.51.4.1"
            oS.port = 233
            //屏蔽谷歌以强制游客登陆
        }
        else if(oS.host.Contains("leiting") || oS.host.Contains("ltgames")) {
            if (oS.HTTPMethodIs("CONNECT"))
            {
                oS["x-replywithtunnel"] = "FakeTunnel";
                return;
            }//CONNECT请求处理
            oS.oRequest.headers.UriScheme = "https";
            oS.host = "192.168.50.89"; // 你自己电脑的IP
            oS.port = 443
            oS.fullUrl = oS.fullUrl.Replace("https","https");
        }
    }
    static function OnBeforceResponse(oS: Session){
        
    }
};